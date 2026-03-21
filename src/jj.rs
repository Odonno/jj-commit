use color_eyre::eyre::{ContextCompat, Result, WrapErr};
use jj_lib::{
    config::{ConfigLayer, ConfigSource, StackedConfig},
    local_working_copy::{LocalWorkingCopy, LocalWorkingCopyFactory},
    repo::{Repo, StoreFactories},
    revset::{RevsetExpression, SymbolResolver, SymbolResolverExtension},
    settings::UserSettings,
    workspace::{WorkingCopyFactories, Workspace},
};
use std::{env, path::PathBuf};

/// Build a `StackedConfig` that mirrors what the real `jj` CLI loads:
///   1. jj-lib built-in defaults  (`user.name = ""`, `user.email = ""`, …)
///   2. User config file           (`$JJ_CONFIG`, `~/.config/jj/config.toml`, or `~/.jjconfig.toml`)
///   3. Env overrides              (`$JJ_USER` → `user.name`,`$JJ_EMAIL` → `user.email`)
fn load_config() -> Result<StackedConfig> {
    let mut config = StackedConfig::with_defaults();

    // --- User config layer ---
    // Respect $JJ_CONFIG if set (colon-separated list of paths, like $PATH).
    if let Ok(jj_config) = env::var("JJ_CONFIG") {
        for path in env::split_paths(&jj_config) {
            if path.is_dir() {
                config
                    .load_dir(ConfigSource::User, &path)
                    .wrap_err("Failed to load user config directory from $JJ_CONFIG")?;
            } else if path.exists() {
                config
                    .load_file(ConfigSource::User, path)
                    .wrap_err("Failed to load user config file from $JJ_CONFIG")?;
            }
        }
    } else {
        // XDG / platform path: $XDG_CONFIG_HOME/jj/config.toml
        //   fallback:          $HOME/.config/jj/config.toml
        let xdg_base = env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")));
        if let Some(platform_config) = xdg_base.map(|d| d.join("jj").join("config.toml"))
            && platform_config.exists()
        {
            config
                .load_file(ConfigSource::User, platform_config)
                .wrap_err("Failed to load platform jj config")?;
        }

        // Legacy path: $HOME/.jjconfig.toml
        if let Some(legacy_config) =
            env::var_os("HOME").map(|h| PathBuf::from(h).join(".jjconfig.toml"))
            && legacy_config.exists()
        {
            config
                .load_file(ConfigSource::User, legacy_config)
                .wrap_err("Failed to load ~/.jjconfig.toml")?;
        }
    }

    // --- Env overrides layer ---
    // $JJ_USER and $JJ_EMAIL take precedence over the config file, matching
    // the behaviour of the real jj CLI.
    let mut overrides = ConfigLayer::empty(ConfigSource::EnvOverrides);
    if let Ok(name) = env::var("JJ_USER") {
        overrides
            .set_value("user.name", name)
            .wrap_err("Failed to set user.name from $JJ_USER")?;
    }
    if let Ok(email) = env::var("JJ_EMAIL") {
        overrides
            .set_value("user.email", email)
            .wrap_err("Failed to set user.email from $JJ_EMAIL")?;
    }
    config.add_layer(overrides);

    Ok(config)
}

fn load_workspace() -> Result<Workspace> {
    let cwd = env::current_dir().wrap_err("Failed to get current directory")?;

    let settings =
        UserSettings::from_config(load_config()?).wrap_err("Failed to load jj settings")?;
    let store_factories = StoreFactories::default();
    let mut wc_factories = WorkingCopyFactories::default();
    wc_factories.insert(
        LocalWorkingCopy::name().to_owned(),
        Box::new(LocalWorkingCopyFactory {}),
    );

    Workspace::load(&settings, &cwd, &store_factories, &wc_factories)
        .wrap_err("Failed to load jj workspace")
}

/// Fetch the last `n` commit descriptions from the jj repository.
pub async fn fetch_commit_messages(n: usize) -> Result<Vec<String>> {
    let workspace = load_workspace()?;
    let repo = workspace
        .repo_loader()
        .load_at_head()
        .await
        .wrap_err("Failed to load jj repo")?;

    let workspace_name = workspace.workspace_name().to_owned();
    let wc_expr = RevsetExpression::working_copy(workspace_name);
    // ancestors_range(0..n) gives n commits: @ itself plus n-1 ancestors
    let ancestors_expr = wc_expr.ancestors_range(0..n as u64);

    // SymbolResolver needs an empty slice of resolver extensions
    let extensions: Vec<Box<dyn SymbolResolverExtension>> = vec![];
    let symbol_resolver = SymbolResolver::new(repo.as_ref(), &extensions);
    let resolved = ancestors_expr
        .resolve_user_expression(repo.as_ref(), &symbol_resolver)
        .wrap_err("Failed to resolve revset expression")?;

    let revset = resolved
        .evaluate(repo.as_ref())
        .wrap_err("Failed to evaluate revset")?;

    let mut messages = Vec::new();
    for commit_id in revset.iter() {
        let commit_id = commit_id.wrap_err("Error iterating revset")?;
        let commit = repo
            .store()
            .get_commit(&commit_id)
            .wrap_err("Failed to get commit")?;
        let desc = commit.description().trim().to_string();
        if !desc.is_empty() {
            messages.push(desc);
        }
    }

    Ok(messages)
}

/// Create a new commit with the given message using jj-lib directly.
pub async fn commit(message: &str) -> Result<()> {
    let mut workspace = load_workspace()?;
    let repo = workspace
        .repo_loader()
        .load_at_head()
        .await
        .wrap_err("Failed to load jj repo")?;

    let workspace_name = workspace.workspace_name().to_owned();

    // Get the current working-copy commit
    let wc_commit_id = repo
        .view()
        .get_wc_commit_id(&workspace_name)
        .cloned()
        .wrap_err("No working-copy commit found for this workspace")?;
    let wc_commit = repo
        .store()
        .get_commit(&wc_commit_id)
        .wrap_err("Failed to get working-copy commit")?;

    // Start a mutable transaction
    let mut tx = repo.start_transaction();
    let repo = tx.repo_mut();

    // Rewrite the WC commit with the new description
    let new_commit = repo
        .rewrite_commit(&wc_commit)
        .set_description(message)
        .write()
        .await
        .wrap_err("Failed to write new commit")?;

    // Rebase any descendants of the rewritten commit
    // (required by jj-lib before committing a transaction that contains rewrites)
    repo.rebase_descendants()
        .await
        .wrap_err("Failed to rebase descendants")?;

    // Check out onto the new commit (creates a new empty WC commit on top)
    repo.check_out(workspace_name.clone(), &new_commit)
        .await
        .wrap_err("Failed to check out new commit")?;

    // Commit the transaction
    let new_repo = tx
        .commit("commit")
        .await
        .wrap_err("Failed to commit transaction")?;

    // Update the working copy on disk
    workspace
        .check_out(new_repo.op_id().clone(), None, &new_commit)
        .await
        .wrap_err("Failed to update working copy")?;

    Ok(())
}
