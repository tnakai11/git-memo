# git-memo

`git memo` is a CLI tool for recording short notes directly in your repository using Git itself. Notes are saved as empty commits organized under special references, making it easy to browse or replay them chronologically by category. It aims to provide efficient, stable memory for long‑lived agents such as LLM assistants that operate over a code base.

## Vision

The idea is to treat the repository as a lightweight journal. Whenever you want to capture an idea or task, you run a command that creates an empty commit whose message is your memo. Categories such as `todo` or `idea` are tracked under references like `refs/memo/todo`. Because memos are commits, they can be pushed, pulled and searched with standard Git tools.

By storing memos in Git, agents gain a stable history that survives checkouts or merges. This is particularly useful for language model agents that need persistent context without relying on external databases.

## Example workflow

```
# record a memo under refs/memo/todo
$ git memo add todo "Finish writing README"

# show the log of todo memos
$ git log refs/memo/todo

# alternatively use the built-in list subcommand
$ git memo list todo

# remove all todo memos
$ git memo remove todo
```

Each memo is an empty commit so repository history is unaffected. Categories live under their own refs and can be removed or archived independently.

## Organizing categories

Categories are simple names under `refs/memo/`. Keep them short (e.g. `todo`, `idea`, `bug`) so that Git ref names remain valid. You can create as many categories as needed and list or remove them independently.

To share memos with collaborators or other agents, push the memo references just like branches:

```
# push a single category
git push origin refs/memo/todo

# push all memo categories
git push origin 'refs/memo/*:refs/memo/*'
```

Fetching works the same way and allows agents to sync their memory across machines.

## Setup

The project is planned as a Rust CLI distributed through Cargo:

```
$ cargo build --release
```

Once built, make sure the resulting binary is in your PATH so you can call `git memo`.

Before recording memos, configure your Git username and email so commits can be created:

```
$ git config --global user.name "Your Name"
$ git config --global user.email "you@example.com"
```

## Planned dependencies

- Rust (edition 2024)
- Cargo for building and running tests
- libgit2 bindings (via [git2-rs](https://github.com/rust-lang/git2-rs)) to manipulate Git repositories

This repository is still in the bootstrapping phase. Contributions and feedback are welcome!

