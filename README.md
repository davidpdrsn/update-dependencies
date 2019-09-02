# `update-dependencies`

Script for automating merging dozen dependabot pull requests.

How it works:

1. Run `update-dependencies foo bar baz`
2. It'll create a branch from `master` called `update-dependencies-year-month-day`
3. It'll merge `foo`, `bar`, and `baz` into that branch
4. If something does wrong it'll show you the git error and the command you can run to continue merging after you have fixed the error
5. It'll push the branch to GitHub so CI can run

## Install

1. [Install `rust`](https://www.rust-lang.org/tools/install)
2. Install this with `cargo install --force --git https://github.com/tonsser/update-dependencies.git`
