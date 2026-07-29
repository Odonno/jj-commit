<div align="center">

[![CI](https://github.com/Odonno/jj-commit/actions/workflows/ci.yml/badge.svg)](https://github.com/Odonno/jj-commit/actions/workflows/ci.yml)
[![Crates.io version](https://img.shields.io/crates/v/jj-commit)](https://crates.io/crates/jj-commit)
[![Latest release](https://img.shields.io/github/v/release/Odonno/jj-commit)](https://github.com/Odonno/jj-commit/releases/latest)
[![Rust version](https://img.shields.io/badge/rust-1.94.0%2B-orange)](https://www.rust-lang.org)
[![License](https://img.shields.io/github/license/Odonno/jj-commit)](./LICENSE)

</div>

# jj-commit

> A guided, convention-aware commit-message builder for [Jujutsu](https://github.com/martinvonz/jj).

`jjc` wraps the `jj` workflow with interactive prompts that enforce a consistent commit style — either [Conventional Commits](https://www.conventionalcommits.org) or [Gitmoji](https://gitmoji.dev) — so you never have to remember the format again.

## Table of contents

- [Features](#features)
- [Install](#install)
- [Get started](#get-started)
  - [Auto-detect convention](#auto-detect-convention)
  - [Specify a convention explicitly](#specify-a-convention-explicitly)
  - [Pre-fill the commit type](#pre-fill-the-commit-type)
  - [Pre-fill scopes](#pre-fill-scopes)
  - [Pre-fill from an existing message](#pre-fill-from-an-existing-message)
  - [Gitmoji workflow](#gitmoji-workflow)
- [Bookmarks](#bookmarks)
  - [Advance the nearest ancestor bookmark](#advance-the-nearest-ancestor-bookmark)
  - [Create or move named bookmarks](#create-or-move-named-bookmarks)
  - [Combine the two](#combine-the-two)
- [Configuration](#configuration)
- [Supported conventions](#supported-conventions)
  - [Conventional Commits types](#conventional-commits-types)
  - [Gitmoji](#gitmoji)
- [How it works](#how-it-works)
- [Contributing](#contributing)
- [License](#license)

---

## Features

- **🎯 Convention auto-detection** — Inspects the last 10 commits and picks the convention used most often. No flags needed for the common case.
- **✍️ Two conventions, one tool** — Full support for [Conventional Commits](https://www.conventionalcommits.org) (`feat:`, `fix(scope):`, …) and [Gitmoji](https://gitmoji.dev) (`✨`, `🐛`, …).
- **🔀 Interactive prompts** — Type, scope(s), and description are gathered through prompts with sensible defaults and pre-fill support.
- **🧩 Pre-fill everything** — Pass `--message`, `--type`, and `--scopes` to seed the prompts or skip them entirely. Great foraliases and editor integrations.
- **🏷️ Bookmarks** — Advance the nearest ancestor bookmark, or create/move named bookmarks to the new commit. Supports interactive selection when multiple ancestors match.

---

## Install

```sh
cargo install --git https://github.com/Odonno/jj-commit
```

Or build from source:

```sh
git clone https://github.com/Odonno/jj-commit
cd jj-commit
cargo build --release
# binary is at ./target/release/jjc
```

---

## Get started

### Auto-detect convention

With no flags, `jjc` inspects the last 10 commits and picks the convention used most often.

```sh
jjc
```

```
? Commit type
> feat
  fix
  chore
  docs
  style
  refactor
  perf
[↑↓ to move, enter to select]

? Scope (leave empty to finish): auth

? Description: add OAuth2 login support
```

Resulting commit message:

```
feat(auth): add OAuth2 login support
```

### Specify a convention explicitly

Skip auto-detection and force a convention:

```sh
jjc --convention conventional
jjc --convention gitmoji
```

### Pre-fill the commit type

Skip the type prompt entirely by passing `--type` (Conventional Commits only):

```sh
jjc --type fix
```

```
? Scope (leave empty to finish):

? Description: handle null pointer in user resolver
```

Resulting commit message:

```
fix: handle null pointer in user resolver
```

### Pre-fill scopes

Pass one or more `--scopes` flags to seed the scope list (Conventional Commits only):

```sh
jjc --type feat --scopes api --scopes ui
```

```
? Scope (leave empty to finish):   ← api and ui already added

? Description: expose dark mode toggle
```

Resulting commit message:

```
feat(api,ui): expose dark mode toggle
```

### Pre-fill from an existing message

Use `--message` to parse an existing commit string into the prompts so you can review and amend each field:

```sh
jjc --message "fix(auth): wrong token expiry"
```

```
? Commit type  [fix]
? Scope        [auth]
? Description  [wrong token expiry]
```

`--message` works with Gitmoji too, accepting both the shortcode (`:bug:`) and raw emoji (`🐛`) forms.

### Gitmoji workflow

```sh
jjc --convention gitmoji
```

```
? Gitmoji
> ✨  Introduce new features.
  🐛  Fix a bug.
  🚑️  Critical hotfix.
  📝  Add or update documentation.
  ♻️   Refactor code.
  🔥  Remove code or files.
[↑↓ to move, enter to select]

? Description: streaming support for chat API
```

Resulting commit message:

```
✨ streaming support for chat API
```

---

## Bookmarks

`jjc` can manage [bookmarks](https://jj-vcs.github.io/jj/latest/bookmarks/) as part of your commit.

### Advance the nearest ancestor bookmark

Use `--advance-bookmark` (`-a`) to find the closest ancestor that holds a local bookmark and move it onto the newly created commit — handy for keeping a moving "main"-style bookmark pinned to your latest work:

```sh
jjc -a
```

If that ancestor carries **several** bookmarks, you get an interactive multi-select:

```
? Select bookmarks to advance to the new commit:
  > [x] main
    [x] release
    [ ] wip
[↑↓ to move, space to toggle, enter to confirm]
```

With a single bookmark the choice is applied automatically. If no ancestor has any bookmark, `jjc` prints a warning instead of failing.

### Create or move named bookmarks

Use `--bookmarks` (`-b`, repeatable) to point one or more bookmarks at the new commit, creating them if they don't exist:

```sh
jjc -b feature-x -b v2
```

### Combine the two

The two flags cooperate: `--bookmarks` destinations are applied directly, while `--advance-bookmark` discovers ancestors interactively. They can be used together in a single invocation:

```sh
jjc --type feat --scopes ui --advance-bookmark --bookmarks release
```

---

## Configuration

`jjc` reads the same configuration the `jj` CLI does, in the same order:

1. **Built-in defaults** from `jj-lib`.
2. **User config file** — `$JJ_CONFIG` (colon-separated, like `$PATH`), or otherwise `$XDG_CONFIG_HOME/jj/config.toml` (falling back to `~/.config/jj/config.toml`), the legacy `~/.jjconfig.toml`, and `%APPDATA%\jj\config.toml` on Windows.
3. **Environment overrides** — `JJ_USER` sets `user.name` and `JJ_EMAIL` sets `user.email`, taking precedence over the config file.

So if you've already configured `jj`, you're configured for `jjc` — nothing extra to do.

---

## Supported conventions

### Conventional Commits types

The `--type` / `--convention conventional` flow knows these types:

| Type       | Use for                                               |
| ---------- | ----------------------------------------------------- |
| `feat`     | A new feature                                         |
| `fix`      | A bug fix                                             |
| `chore`    | Maintenance tasks that don't touch src/docs           |
| `docs`     | Documentation only changes                            |
| `style`    | Formatting, whitespace, semicolons, etc.              |
| `refactor` | Code changes that neither fix a bug nor add a feature |
| `perf`     | Performance improvements                              |
| `test`     | Adding or correcting tests                            |
| `build`    | Build system or external dependencies                 |
| `ci`       | CI configuration files and scripts                    |
| `revert`   | Reverting a previous commit                           |

The breaking-change marker (`feat!:`) is parsed from `--message` but not added by a prompt — use `--message` when you need it.

### Gitmoji

The Gitmoji flow ships the full [gitmoji.dev](https://gitmoji.dev) table (80+ entries), presented with their description. Both the shortcode (`:sparkles:`) and raw emoji (`✨`) forms are recognized when pre-filling with `--message`.

---

## How it works

Unlike a thin wrapper that calls out to the `jj` binary, `jjc` links against `jj-lib` and performs the commit transaction in-process:

1. **Loads** your real `jj` stacked config and workspace (mirroring the `jj` CLI's lookup rules).
2. **Snapshots** the working copy — respecting `.gitignore` and auto-tracking new files, just like `jj`'s default `snapshot.auto-track = "all()"`.
3. **Rewrites** the open working-copy commit with the snapshotted tree and your crafted message.
4. **Rebases** any descendants, then checks out a fresh empty working-copy commit on top.
5. **Syncs** the Git index and `HEAD` for co-located Git repos so the Git view matches Jujutsu.
6. Optionally **advances bookmarks** before or after the commit lands.

This means you get the same resulting topology as `jj commit` — without spawning `jj`.

---

## Contributing

Before contributing a change, please run:

```sh
cargo fmt
cargo clippy -- -D warnings
cargo test
```

---

## License

See [LICENSE](./LICENSE).
