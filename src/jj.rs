use color_eyre::eyre::{ContextCompat, Result, WrapErr};
use jj_lib::{
    backend::CommitId,
    config::{ConfigLayer, ConfigSource, StackedConfig},
    gitignore::GitIgnoreFile,
    local_working_copy::{LocalWorkingCopy, LocalWorkingCopyFactory},
    matchers::{EverythingMatcher, NothingMatcher},
    op_store::RefTarget,
    ref_name::RefNameBuf,
    repo::{Repo, StoreFactories},
    revset::{RevsetExpression, SymbolResolver, SymbolResolverExtension},
    settings::UserSettings,
    working_copy::SnapshotOptions,
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
/// Returns the `CommitId` of the newly written (described) commit.
pub async fn commit(message: &str) -> Result<CommitId> {
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

    // Lock the working copy and snapshot the on-disk state before rewriting the
    // commit. Without this, any file changes made since the last `jj` command
    // ran would be missing from the committed tree because jj-lib only records
    // the working copy lazily (on explicit snapshot).

    // Load .gitignore from the workspace root so the snapshot skips ignored
    // paths (e.g. target/, .git/) instead of hashing them all.
    // Must be done before start_working_copy_mutation() takes a mutable borrow.
    let root_gitignore = GitIgnoreFile::empty()
        .chain_with_file("", workspace.workspace_root().join(".gitignore"))
        .wrap_err("Failed to load .gitignore")?;

    let mut locked_ws = workspace
        .start_working_copy_mutation()
        .wrap_err("Failed to lock working copy")?;

    let snapshot_options = SnapshotOptions {
        base_ignores: root_gitignore,
        progress: None,
        // Auto-track all new untracked files, matching jj's default behaviour
        // (snapshot.auto-track = "all()").
        start_tracking_matcher: &EverythingMatcher,
        // Never force-track ignored or oversized files.
        force_tracking_matcher: &NothingMatcher,
        max_new_file_size: u64::MAX,
    };
    let (snapshot_tree, _stats) = locked_ws
        .locked_wc()
        .snapshot(&snapshot_options)
        .await
        .wrap_err("Failed to snapshot working copy")?;

    // Start a mutable transaction
    let mut tx = repo.start_transaction();
    let repo = tx.repo_mut();

    // Rewrite the WC commit with the snapshotted tree and the new description
    let new_commit = repo
        .rewrite_commit(&wc_commit)
        .set_tree(snapshot_tree)
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
    let new_wc_commit = repo
        .check_out(workspace_name.clone(), &new_commit)
        .await
        .wrap_err("Failed to check out new commit")?;

    // Commit the transaction
    let new_repo = tx
        .commit("commit")
        .await
        .wrap_err("Failed to commit transaction")?;

    // Point the on-disk working copy to the new empty WC commit and release the lock.
    // This replaces the previous `workspace.check_out()` call; doing it
    // through the already-held lock avoids a redundant re-lock.
    locked_ws
        .locked_wc()
        .check_out(&new_wc_commit)
        .await
        .wrap_err("Failed to update working copy to new commit")?;
    locked_ws
        .finish(new_repo.op_id().clone())
        .wrap_err("Failed to finish working copy mutation")?;

    Ok(new_commit.id().clone())
}

/// Walk the first-parent ancestor chain starting from the parents of the
/// current working-copy commit and return the names of all local bookmarks
/// found on the *nearest* ancestor that has at least one.
///
/// Skips the WC commit itself (typically an empty, open change).
/// Returns `None` if no ancestor has any local bookmark.
pub async fn find_nearest_ancestor_bookmarks() -> Result<Option<Vec<String>>> {
    let workspace = load_workspace()?;
    let repo = workspace
        .repo_loader()
        .load_at_head()
        .await
        .wrap_err("Failed to load jj repo")?;

    let workspace_name = workspace.workspace_name().to_owned();
    let wc_commit_id = repo
        .view()
        .get_wc_commit_id(&workspace_name)
        .cloned()
        .wrap_err("No working-copy commit found for this workspace")?;

    let wc_commit = repo
        .store()
        .get_commit(&wc_commit_id)
        .wrap_err("Failed to get working-copy commit")?;

    // Start from the WC's first parent, walking first-parent only.
    let first_parent_id = match wc_commit.parent_ids().first() {
        Some(id) => id.clone(),
        None => return Ok(None),
    };

    let mut current = repo
        .store()
        .get_commit(&first_parent_id)
        .wrap_err("Failed to get parent commit")?;

    loop {
        let names: Vec<String> = repo
            .view()
            .local_bookmarks_for_commit(current.id())
            .map(|(name, _)| name.as_str().to_owned())
            .collect();

        if !names.is_empty() {
            return Ok(Some(names));
        }

        // Advance to the first parent.
        let parent_ids = current.parent_ids();
        if parent_ids.is_empty() {
            return Ok(None);
        }

        current = repo
            .store()
            .get_commit(&parent_ids[0])
            .wrap_err("Failed to get ancestor commit")?;
    }
}

/// Move local bookmark `name` to point to `commit_id`.
pub async fn advance_bookmark(name: &str, commit_id: &CommitId) -> Result<()> {
    let mut workspace = load_workspace()?;
    let repo = workspace
        .repo_loader()
        .load_at_head()
        .await
        .wrap_err("Failed to load jj repo")?;

    let mut tx = repo.start_transaction();
    let repo = tx.repo_mut();

    let ref_name: RefNameBuf = name.into();
    let target = RefTarget::normal(commit_id.clone());
    repo.set_local_bookmark_target(&ref_name, target);

    let new_repo = tx
        .commit("advance bookmark")
        .await
        .wrap_err("Failed to commit bookmark transaction")?;

    // Update working copy so the on-disk state is consistent
    let workspace_name = workspace.workspace_name().to_owned();
    let wc_commit_id = new_repo
        .view()
        .get_wc_commit_id(&workspace_name)
        .cloned()
        .wrap_err("No working-copy commit found after bookmark advance")?;
    let wc_commit = new_repo
        .store()
        .get_commit(&wc_commit_id)
        .wrap_err("Failed to get working-copy commit after bookmark advance")?;

    workspace
        .check_out(new_repo.op_id().clone(), None, &wc_commit)
        .await
        .wrap_err("Failed to update working copy after bookmark advance")?;

    Ok(())
}
