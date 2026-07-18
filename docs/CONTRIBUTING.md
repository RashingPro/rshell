# Contributing Guide

Although I'm always happy when someone wants to improve the project, there are some rules you should follow:

## I Have a Question

Before creating an issue:

1. Read the documentation.
2. Make sure you use the latest version.
3. Try searching for solution on Reddit, StackOverflow etc.

If nothing helped - create an issue and describe your question in details. Include RShell version you use.

## I Found a Bug

Create an issue. Describe steps to reproduce, include RShell version you use, if relevant - your OS.

## I Found a Vulnerability

If you found security vulnerability, do **not** create a public issue. Instead, report it on my email:
`rashingpro@yandex.ru`

## I want to submit a Pull Request

1. Take your time when writing pull request to clearly explain your point.
2. If your pull request introduces a breaking (back-incompatible) change - mention it.
3. Always use `cargo clippy` (or `cargo clippy --fix`) to lint and `cargo fmt` to format the
   code.
4. Read [`CODE_STYLE.md`](./CODE_STYLE.md)
