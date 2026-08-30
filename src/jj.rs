use color_eyre::eyre::{ContextCompat, Result, WrapErr, bail};
use futures::StreamExt;
use jj_lib::{
    backend::CommitId,
    config::{ConfigLayer, ConfigSource, StackedConfig},
    default_backend_factories::default_backend_factories,
    fileset,
    fileset::{FilesetAliasesMap, FilesetDiagnostics, FilesetExpression, FilesetParseContext},
    gitignore::GitIgnoreFile,
    local_working_copy::{LocalWorkingCopy, LocalWorkingCopyFactory},
    matchers::{EverythingMatcher, Matcher, NothingMatcher},
    merged_tree_builder::MergedTreeBuilder,
    op_store::RefTarget,
    ref_name::RefNameBuf,
    repo::Repo,
    repo_path::{RepoPath, RepoPathUiConverter},
    revset::{RevsetExpression, SymbolResolver, SymbolResolverExtension},
    settings::UserSettings,
    working_copy::SnapshotOptions,
    workspace::{WorkingCopyFactories, Workspace},
};
use std::{
    env,
    path::{Path, PathBuf},
};

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

        // Windows platform path: %APPDATA%\jj\config.toml
        #[cfg(windows)]
        if let Some(win_config) =
            env::var_os("APPDATA").map(|a| PathBuf::from(a).join("jj").join("config.toml"))
            && win_config.exists()
        {
            config
                .load_file(ConfigSource::User, win_config)
                .wrap_err("Failed to load Windows jj config")?;
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

fn find_workspace_root(cwd: &std::path::Path) -> Result<std::path::PathBuf> {
    let mut dir = cwd;
    loop {
        if dir.join(".jj").is_dir() {
            return Ok(dir.to_path_buf());
        }
        dir = dir
            .parent()
            .wrap_err("There is no Jujutsu repo in the current directory or any parent")?;
    }
}

fn load_workspace_at(cwd: &Path) -> Result<Workspace> {
    let workspace_root = find_workspace_root(cwd)?;

    let settings =
        UserSettings::from_config(load_config()?).wrap_err("Failed to load jj settings")?;
    let store_factories = default_backend_factories();
    let mut wc_factories = WorkingCopyFactories::default();
    wc_factories.insert(
        LocalWorkingCopy::name().to_owned(),
        Box::new(LocalWorkingCopyFactory {}),
    );

    Workspace::load(&settings, &workspace_root, &store_factories, &wc_factories)
        .wrap_err("Failed to load jj workspace")
}

/// Fetch the last `n` commit descriptions from the jj repository.
pub async fn fetch_commit_messages(n: usize) -> Result<Vec<String>> {
    let cwd = env::current_dir().wrap_err("Failed to get current directory")?;
    let workspace = load_workspace_at(&cwd)?;
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

    let mut stream = revset.stream();
    let mut messages = Vec::new();
    while let Some(result) = stream.next().await {
        let commit_id = result.wrap_err("Error iterating revset")?;
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

/// Parse cwd-relative fileset expressions (e.g. `src/`, `*.rs`, `a|b`) into
/// a single matcher, exactly like `jj commit <paths>` resolves them.
fn build_path_matcher(
    paths: &[String],
    cwd: &Path,
    workspace_root: &Path,
) -> Result<Box<dyn Matcher>> {
    let cwd = dunce::canonicalize(cwd).wrap_err("failed to canonicalize cwd")?;
    let base =
        dunce::canonicalize(workspace_root).wrap_err("failed to canonicalize workspace root")?;
    let path_converter = RepoPathUiConverter::Fs { cwd, base };
    let aliases_map = FilesetAliasesMap::new();
    let mut diagnostics = FilesetDiagnostics::new();
    let expressions: Vec<FilesetExpression> = paths
        .iter()
        .map(|text| {
            fileset::parse_maybe_bare(
                &mut diagnostics,
                text,
                &FilesetParseContext {
                    aliases_map: &aliases_map,
                    path_converter: &path_converter,
                },
            )
            .wrap_err_with(|| format!("invalid fileset expression: {text}"))
        })
        .collect::<Result<_>>()?;
    Ok(FilesetExpression::union_all(expressions).to_matcher())
}

/// Create a new commit with the given message using jj-lib directly.
/// Returns the `CommitId` of the newly written (described) commit.
pub async fn commit(message: &str, paths: &[String]) -> Result<CommitId> {
    let cwd = env::current_dir().wrap_err("Failed to get current directory")?;
    commit_at(message, &cwd, paths).await
}

pub async fn commit_at(message: &str, cwd: &Path, paths: &[String]) -> Result<CommitId> {
    let mut workspace = load_workspace_at(cwd)?;
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
        .chain_with_file(
            RepoPath::root(),
            workspace.workspace_root().join(".gitignore"),
        )
        .wrap_err("Failed to load .gitignore")?;

    // Resolve path matching before locking the working copy (the lock takes a mutable borrow of the workspace).
    let path_matcher = if paths.is_empty() {
        None
    } else {
        Some(build_path_matcher(paths, cwd, workspace.workspace_root())?)
    };

    let mut locked_ws = workspace
        .start_working_copy_mutation()
        .await
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

    // If paths are given, the new commit gets only the matching changes;
    // the rest stay behind in the new working-copy commit (jj `commit <paths>` split).
    let commit_tree = if let Some(matcher) = path_matcher.as_deref() {
        let mut tree_builder = MergedTreeBuilder::new(wc_commit.tree());
        let mut matched = false;
        let mut diff_stream = wc_commit.tree().diff_stream(&snapshot_tree, matcher);
        while let Some(entry) = diff_stream.next().await {
            let after = entry.values.wrap_err("Failed to read diff values")?.after;
            tree_builder.set_or_remove(entry.path, after);
            matched = true;
        }
        if !matched {
            bail!("no matching files: {}", paths.join(" "));
        }
        tree_builder
            .write_tree()
            .await
            .wrap_err("Failed to write tree")?
    } else {
        snapshot_tree.clone()
    };

    // Start a mutable transaction
    let mut tx = repo.start_transaction();
    let repo = tx.repo_mut();

    // Rewrite the WC commit with the snapshotted tree and the new description
    let new_commit = repo
        .rewrite_commit(&wc_commit)
        .set_tree(commit_tree)
        .set_description(message)
        .write()
        .await
        .wrap_err("Failed to write new commit")?;

    // Rebase any descendants of the rewritten commit
    // (required by jj-lib before committing a transaction that contains rewrites)
    repo.rebase_descendants()
        .await
        .wrap_err("Failed to rebase descendants")?;

    // Check out onto the new commit (creates a new empty WC commit on top).
    // With paths, the new WC commit keeps the full snapshotted tree so the
    // unselected changes remain in the working copy.
    let new_wc_commit = if path_matcher.is_none() {
        repo.check_out(workspace_name.clone(), &new_commit)
            .await
            .wrap_err("Failed to check out new commit")?
    } else {
        let wc_commit = repo
            .new_commit(vec![new_commit.id().clone()], snapshot_tree)
            .write()
            .await
            .wrap_err("Failed to write new working-copy commit")?;
        repo.edit(workspace_name.clone(), &wc_commit)
            .await
            .wrap_err("Failed to set new working-copy commit")?;
        wc_commit
    };

    // Update git HEAD and reset the index so co-located git repos stay in sync.
    // Skipped silently for non-git-backed workspaces.
    if jj_lib::git::get_git_backend(repo.store()).is_ok() {
        jj_lib::git::reset_head(repo, &new_wc_commit)
            .await
            .wrap_err("Failed to reset git HEAD and index")?;
    }

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
        .await
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
    let cwd = env::current_dir().wrap_err("Failed to get current directory")?;
    let workspace = load_workspace_at(&cwd)?;
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
    let cwd = env::current_dir().wrap_err("Failed to get current directory")?;
    let mut workspace = load_workspace_at(&cwd)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    async fn init_test_repo(dir: &Path) -> Result<()> {
        // ponytail: env vars set per-test; tests must not run in parallel (cargo test -- --test-threads=1 if needed)
        unsafe {
            std::env::set_var("JJ_USER", "Test User");
            std::env::set_var("JJ_EMAIL", "test@example.com");
        }
        let settings = UserSettings::from_config(load_config()?)?;
        Workspace::init_colocated_git(&settings, dir, gix_hash::Kind::Sha1).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_commit_at_updates_git_head() -> Result<()> {
        let tmp = tempfile::TempDir::new()?;
        init_test_repo(tmp.path()).await?;

        fs::write(tmp.path().join("test.ts"), "export const x = 1;")?;

        commit_at("test: add test.ts", tmp.path(), &[]).await?;

        // Git HEAD should now point to the commit that contains test.ts
        let output = std::process::Command::new("git")
            .args([
                "-C",
                tmp.path().to_str().unwrap(),
                "show",
                "--name-only",
                "--format=",
                "HEAD",
            ])
            .output()?;

        let stdout = String::from_utf8(output.stdout)?;
        assert!(
            stdout.trim().contains("test.ts"),
            "expected test.ts in git HEAD, git show output: {stdout:?}"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_commit_at_with_paths_splits_changes() -> Result<()> {
        let tmp = tempfile::TempDir::new()?;
        init_test_repo(tmp.path()).await?;

        fs::write(tmp.path().join("committed.txt"), "committed")?;
        fs::write(tmp.path().join("leftover.txt"), "leftover")?;

        commit_at(
            "test: only committed.txt",
            tmp.path(),
            &["committed.txt".to_string()],
        )
        .await?;

        // In a colocated repo, git HEAD points at the parent of the jj WC commit,
        // which is the newly described commit: it must contain only the selected path.
        let output = std::process::Command::new("git")
            .args([
                "-C",
                tmp.path().to_str().unwrap(),
                "show",
                "--name-only",
                "--format=",
                "HEAD",
            ])
            .output()?;
        let stdout = String::from_utf8(output.stdout)?;
        assert!(
            stdout.contains("committed.txt"),
            "expected committed.txt in committed change, git show output: {stdout:?}"
        );
        assert!(
            !stdout.contains("leftover.txt"),
            "leftover.txt must not be in the committed change, git show output: {stdout:?}"
        );

        // The leftover change must remain in the new working-copy commit
        let workspace = load_workspace_at(tmp.path())?;
        let repo = workspace.repo_loader().load_at_head().await?;
        let wc_commit_id = repo
            .view()
            .get_wc_commit_id(workspace.workspace_name())
            .cloned()
            .wrap_err("No working-copy commit after commit_at")?;
        let wc_commit = repo.store().get_commit(&wc_commit_id)?;
        let leftover_path = RepoPath::from_internal_string("leftover.txt")?;
        let value = wc_commit
            .tree()
            .path_value(leftover_path)
            .await?
            .into_resolved()
            .map_err(|_| color_eyre::eyre::eyre!("leftover.txt is conflicted in the WC commit"))?;
        assert!(
            value.is_some(),
            "leftover.txt must still be tracked in the working-copy commit"
        );

        // And still on disk
        assert_eq!(
            fs::read_to_string(tmp.path().join("leftover.txt"))?,
            "leftover"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_commit_at_no_matching_paths_errors() -> Result<()> {
        let tmp = tempfile::TempDir::new()?;
        init_test_repo(tmp.path()).await?;

        fs::write(tmp.path().join("real.txt"), "x")?;

        let err = commit_at("test: no match", tmp.path(), &["nope.txt".to_string()]).await;
        assert!(err.is_err(), "expected an error when no paths match");

        Ok(())
    }
}
