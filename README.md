# jj-commit

> A guided, convention-aware commit-message builder for [Jujutsu](https://github.com/martinvonz/jj).

`jjc` wraps the `jj` workflow with interactive prompts that enforce a consistent commit style — either [Conventional Commits](https://www.conventionalcommits.org) or [Gitmoji](https://gitmoji.dev) — so you never have to remember the format again. It auto-detects which convention your project uses by inspecting recent commit history.

---

## Install

```sh
cargo install --git https://github.com/Odonno/jjcc
```

Or build from source:

```sh
git clone https://github.com/Odonno/jjcc
cd jjcc
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

---

### Specify a convention explicitly

```sh
jjc --convention conventional
```

```sh
jjc --convention gitmoji
```

---

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

---

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

---

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

---

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
