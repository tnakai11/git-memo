# git-memo

`git memo` is a CLI tool for recording short notes directly in your repository using Git itself. Each note is stored as an empty commit under a dedicated reference so you can browse or replay memos in order by category. This provides a lightweight way to keep memos alongside your code without any external database.

## Vision

The idea is to treat the repository as a lightweight journal. Whenever you want to capture an idea or task, you run a command that creates an empty commit whose message is your memo. Categories such as `todo` or `idea` are tracked under references like `refs/memo/todo`. Because memos are commits, they can be pushed, pulled and searched with standard Git tools.

Because memos live in Git, their history survives checkouts or merges. This lets any tooling or team share notes through normal Git operations without a separate service.

## Example workflow

```
# record a memo under refs/memo/todo
$ git memo add todo "Finish writing README"
# read a multi-line message from stdin
$ cat msg.txt | git memo add todo -

When the message is `-`, `git memo` reads the memo text from standard input.

# show the log of todo memos
$ git log refs/memo/todo

# alternatively use the built-in list subcommand
$ git memo list todo

# remove all todo memos
$ git memo remove todo

# list existing memo categories
$ git memo categories
# alias form
$ git memo list-categories

Categories are printed in alphabetical order for easy scanning.
For machine-readable output, pass `--json` to either command:

```bash
$ git memo list todo --json
$ git memo categories --json
```

# edit the latest memo message
$ git memo edit todo "updated message"

# archive a category
$ git memo archive todo

The archive command renames `refs/memo/todo` to `refs/archive/todo` so
the category can be hidden without deleting its history.

# list archived categories
$ git memo archive-categories
$ git memo archive-categories --json
```

Each memo is an empty commit so repository history is unaffected. Categories live under their own refs and can be removed or archived independently.

You can search memo messages across all categories using the `grep` subcommand:

```bash
$ git memo grep hello
hello world
```

## Organizing categories

Categories are simple names under `refs/memo/`. Keep them short (e.g. `todo`, `idea`, `bug`) so that Git ref names remain valid. You can create as many categories as needed and list or remove them independently.

To share memos with collaborators or automation, push the memo references just like branches:

```
# push a single category
git push origin refs/memo/todo

# push all memo categories
git memo push origin
```

Fetching works the same way so notes stay in sync across machines.

## Automating remote pushes

For collaborative setups it's convenient to push memo references immediately
after they are written. Run `git memo push <remote>` or use the helper script
`scripts/push-memos.sh` which pushes all memo categories to a remote (defaults
to `origin`). You can call this script manually or install it as a Git hook.

```sh
# push all memos to the default remote
scripts/push-memos.sh

# install as a post-commit hook
ln -s ../../scripts/push-memos.sh .git/hooks/post-commit
```

Running the script ensures `refs/memo/*` are kept in sync with your remote
repository.

## Setup

First install the Rust toolchain with
[rustup](https://rustup.rs) if it is not already available:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
```

Then clone the repository and build the release binary:

```sh
git clone https://github.com/yourname/git-memo.git
cd git-memo
cargo build --release
```

The resulting executable resides at `target/release/git-memo`. Add this
location to your `PATH` or install it system-wide:

```sh
cargo install --path .
```

Optionally install the helper script as a post-commit hook so memo refs are
pushed automatically:

```sh
ln -s ../../scripts/push-memos.sh .git/hooks/post-commit
```

Before recording memos, configure your Git username. Setting `user.email` is
optional:

```sh
git config --global user.name "Your Name"
git config --global user.email "you@example.com" # optional
```

## Dependencies

- Rust (edition 2024)
- Cargo
- [git2-rs](https://github.com/rust-lang/git2-rs) and [serde_json](https://github.com/serde-rs/json)

Contributions and feedback are welcome!

