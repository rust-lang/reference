# Contribution process

## Before contributing

We encourage people to discuss their changes before opening a PR. This gives the Reference team a chance to understand your idea better and ensure it fits with the intended direction of the Reference. Typically, you should file an issue or start a thread on [Zulip](#zulip) before submitting a pull request.

## Contributing process overview

The general outline of a contribution is as follows:

1. [Check out the source.](../tooling/building.md#checking-out-the-source)
2. [Install mdbook.](../tooling/building.md#installing-mdbook)
3. [Learn to build the book locally.](../tooling/building.md#running-mdbook)
4. Make your changes to the source files. Be sure to follow all the guidelines in this book for styling, conventions, etc.
5. [Run the tests.](../tests.md)
6. [Submit a pull request](#submitting-a-pull-request)
7. The PR will go through the review process.
    - There is a [guide](../review-policy.md) for the types of reviews it may undergo.
    - This may take a while, as the team has limited time.
8. Once approved, a team member will merge the change.
    - The team may apply editorial changes before merging.
    - It may take a few weeks for the change to appear on the [nightly website](https://doc.rust-lang.org/nightly/reference/). See [the publishing chapter](../publishing.md) for more details.

## Office hours

The lang-docs team holds office hours on Tuesdays at [3:30 PM US/Eastern](https://dateful.com/convert/est-edt-eastern-time?t=330pm). We meet on [Jitsi Meet](https://meet.jit.si/rust-t-lang-docs). Check the [Zulip](#zulip) channel for the latest status and availability.

## Zulip

There are channels on Zulip for more direct discussions about the Reference:

- [`#t-lang-docs`](https://rust-lang.zulipchat.com/#narrow/channel/237824-t-lang-docs) --- Used by the lang docs team.
- [`#t-lang-docs/reference`](https://rust-lang.zulipchat.com/#narrow/channel/520709-t-lang-docs.2Freference) --- Discussion about the Reference specifically.

## Working on issues

Issues labeled with [Help Wanted] are ones the team wants to see addressed and is asking for contributions.

If you want to work on an issue, you can assign yourself by commenting `@rustbot claim`. See [issue assignment] for more information.

[Help Wanted]: https://github.com/rust-lang/reference/issues?q=state%3Aopen%20label%3A%22Help%20Wanted%22
[issue assignment]: https://forge.rust-lang.org/triagebot/issue-assignment.html

## New features

See the [stabilization chapter] for information on how to document new features.

[stabilization chapter]: stabilization.md

## Minor changes

Minor changes, like small corrections, wording cleanup, or formatting fixes, can be made simply by opening a PR.

## Major changes

Major changes, such as large rewrites, reorganization, or new chapters, should be approved by the Reference team first. Open an issue (if there isn't already one) to discuss the kinds of changes you are interested in. If the Reference team is able, they can work with you to approve the change.

## Submitting a pull request

When submitting a pull request, please follow these guidelines:

- Include a clear description of what the change is and why it is being made.
- Try to keep a clean git history, with each commit explaining the reason for the change.
- Use [GitHub’s keywords] in the description to automatically link to an issue if the PR resolves it. For example, `Closes #1234` will link issue #1234 to the PR. When the PR is merged, GitHub will automatically close the issue.

When your PR is submitted, GitHub automatically runs all tests. The GitHub interface shows a green checkmark if it passes, or a red X if it fails. Links to the logs are available on the PR page to diagnose any issues.

[GitHub’s keywords]: https://docs.github.com/en/github/managing-your-work-on-github/linking-a-pull-request-to-an-issue

### PR labeling

PRs are marked with [labels] like [`S-waiting-on-review`] or [`S-waiting-on-author`] to indicate their status. Anyone can use the [`@rustbot`] bot to adjust the labels. If a PR is marked as `S-waiting-on-author` and you have pushed new changes that you would like reviewed, you can comment on the PR with `@rustbot ready`. The bot will switch the labels on the PR.

More information about these commands can be found at the [shortcuts documentation].

[`@rustbot`]: https://github.com/rustbot
[`S-waiting-on-author`]: https://github.com/rust-lang/reference/labels/S-waiting-on-author
[`S-waiting-on-review`]: https://github.com/rust-lang/reference/labels/S-waiting-on-review
[labels]: https://github.com/rust-lang/reference/labels
[shortcuts documentation]: https://forge.rust-lang.org/triagebot/shortcuts.html
