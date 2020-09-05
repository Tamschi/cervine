# Contribution Guidelines

## Issues

This repository contains issue templates for [bugs] and [feature requests].  
Unclear documentation or error messages are considered bugs.

For anything else, please use the ["Custom issue"] template.

[bugs]: https://github.com/Tamschi/cervine/issues/new?assignees=&labels=bug&template=bug_report.md&title=
[feature requests]: https://github.com/Tamschi/cervine/issues/new?assignees=&labels=enhancement&template=feature_request.md&title=
["Custom issue"]: https://github.com/Tamschi/cervine/issues/new?assignees=&labels=&template=custom_issue.md&title=

## Pull Requests

This repository uses fairly extensive CI to make sure everything is in order.  
Travis CI will automatically build and test your pull requests.

**I recommend working on branches with a `-` in their name.**  
The CI is configured slightly differently for them to make WIP code a bit easier.

Additionally, when you run `cargo test` for the first time, [cargo-husky] sets up a Git pre-push hook to run tests.  
This includes a branch name check, which is ignored on any branches that have a `-` in their name.  
You can still push failing builds using `git push --no-verify`.

Warnings are only denied on `develop`, but the CI should still detect them for pull requests towards that branch.

Please add yourself to each copyright holders list of [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) when contributing.

[cargo-husky]: https://lib.rs/crates/cargo-husky
