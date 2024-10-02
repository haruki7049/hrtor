# Contribution Guide

This sentence was translated by [Deepl](https://www.deepl.com/translator). If you have trouble understanding the meaning of this sentence, please post it in the [GitHub Discussions](https://github.com/haruki7049/hrtor/discussions). Our collaborators will respond.

If you have questions, please post them in [GitHub Discussion for hrtor](https://github.com/haruki7049/hrtor/discussions) as [Q&A Category](https://github.com/haruki7049/hrtor/discussions/categories/q-a).

## Creating an Issue

- Before creating an Issue, please make sure that there is no similar Issue. If the content of the Issue is similar, close the Issue as `Not Planned`.
- If you want to group issues, consider creating milestones.

## Creating a PullRequest

- Always create a PullRequest after you have created an Issue. Otherwise, the purpose of the PullRequest will be lost. For related information, please read \[code modification section\](#Code modification).

## Creating Milestones

- Milestones can only be created by hrtor collaborators. Therefore, if you want to create a milestone, first go to [GitHub Discussions for hrtor](https://github.com/haruki7049/hrtor/discussions), then to [Ideas Category](https://github.com/haruki7049/hrtor/discussions/categories/ideas). Our collaborators will comment.

```md
<! -- Title -->.
Create a new milestone for hrtor:v0.1.1

<! -- Body -->
Wanna create a new milestone for hrtor:v0.1.1?

<! -- You must write Reasons to create a new milestone -->
## Reasons
**ANY REASONS!!! **
```

## Fixing the code

Follow the flow below:

1. Create an Issue.
1. Create a PullRequest for the Issue. See [GitHub Docs](https://docs.github.com/en/issues/tracking-your-work-with-issues/linking-a-pull-request-to-an-issue) for how to do this.
1. Have the PullRequest reviewed and merge it.

## Adding CI/CD

Currently, only rust-checker is working for GitHub Actions. The following is a description of rust-checker.

- Run cargo-test.
- Run cargo-clippy and check.
- Run cargo-fmt to check for proper indentation, etc.

If you have other work that should be done using CI/CD, please post an Issue with a note to that effect.

## Review

After getting a review and being able to merge, please check the following points.

- Merge with `Create a merge commit`.
- Delete the working branch after you are done.
