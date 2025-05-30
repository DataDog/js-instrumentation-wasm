# Welcome

Welcome! We are glad you are interested in contributing to `@datadog/js-instrumentation-wasm`. This guide will help you understand the requirements and guidelines to improve your contributor experience.

## Contributing to code

### New features

If you want to contribute with a new feature, before start writing any code, you will need to get your proposal accepted by the maintainers. This is to avoid going through the effort of writing the code and getting it rejected because it is already being worked on in a different way, or it is outside the scope of the project.

Open a new issue with the title "[RFC] Title of your proposal". In the description explain carefully why you think this feature is needed, why it is useful, and how you plan to implement it. We recommend to use the RFC issue template we provide.

The maintainers will label the issue as `type/feature` or `type/major_change` and `rfc/discussion` and will start a conversation with you to discuss it. If the proposal gets accepted it will be tagged as `rfc/approved`. Feel free to start coding at that point and propose a PR, linking it to the issue.

During the RFC process, your change proposal, alongside with implementation approaches, will get discussed with the maintainers, ensuring that you don’t waste time with wrong approaches or features that are out of scope for the project.

If, after the discussion, the proposal gets rejected, the team will give you an explanation, label the issue as `rfc/rejected` and close the issue.

### Bug fixes

If you have identified an issue that is already labeled as `type/bug` that hasn’t been assigned to anyone, feel free to claim it, and ask a maintainer to add you as assignee.
Once you have some code ready, open a PR, [linking it to the issue](https://docs.github.com/en/issues/tracking-your-work-with-issues/linking-a-pull-request-to-an-issue#manually-linking-a-pull-request-to-an-issue-using-the-pull-request-sidebar). Take into account that if the changes to fix the bug are not trivial, you need to follow the RFC process as well to discuss the options with the maintainers.

### Setting up your development environment

To get starting, you'll need to install `yarn`, `rustup` and `wasm-pack` with your system package manager, and use `rustup` to install a Rust toolchain.

```bash
# Install yarn, rustup and wasm-pack
brew install yarn rustup wasm-pack
```

Once you've installed those core prerequisites, use the following commands to install dependencies and build the project:

```bash
yarn
yarn build
```

### Testing your changes

You can run the full test suite using this command:

```bash
yarn test
```

It's also possible to run a subset of the tests:

```bash
yarn test:unit:rust
yarn test:unit:js
yarn test:integration
```

If you make changes that affect the generated code, or if you modify the test applications used in the integration tests, you may need to update the test snapshots. You can do this by running this command:

```bash
yarn update:snapshots
```

Note that for integration tests, the snapshots are generated with instrumentation *disabled*, because their goal is to ensure that instrumentation doesn't change the behavior of the generated code. If you run `yarn update:snapshots` and these tests are still failing, this probably indicates that there's a bug in the instrumentation, or in the tests themselves.

If you modify an existing test fixture, you should be able to just rerun the tests without issue, but newly added fixtures may not be picked up due to caching. You can ensure that a newly added fixture is visible by running this command:

```bash
yarn clean
```

## Contributing to issues

### Contributing to reporting bugs

If you think you have found a bug in `@datadog/js-instrumentation-wasm` feel free to report it. When creating issues, you will be presented with a template to fill. Please, fill as much as you can from that template, including steps to reproduce your issue, so we can address it quicker.

### Contributing to triaging issues

Triaging issues is a great way to contribute to an open source project. Some actions you can perform on an open by someone else issue that will help addressing it sooner:

- Trying to reproduce the issue. If you can reproduce the issue following the steps the reporter provided, add a comment specifying that you could reproduce the issue.
- Finding duplicates. If there is a bug, there might be a chance that it was already reported in a different issue. If you find an already reported issue that is the same one as the one you are triaging, add a comment with "Duplicate of" followed by the issue number of the original one.
- Asking the reporter for more information if needed. Sometimes the reporter of an issue doesn’t include enough information to work on the fix, i.e. lack of steps to reproduce, not specifying the affected version, etc. If you find a bug that doesn’t have enough information, add a comment tagging the reporter asking for the missing information.

## Releasing a new version

Just open a PR that updates the version number in `package.json` and merge it; Github Actions should do the rest.
