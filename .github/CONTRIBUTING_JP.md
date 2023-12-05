# Contribution Guide

This file written in Japanese for JP Developer. If you want the English version, browse [here](./CONTRIBUTING.md).

質問がある場合には[GitHub Discussion for hrtor](https://github.com/haruki7049/hrtor/discussions)に[Q&A Category](https://github.com/haruki7049/hrtor/discussions/categories/q-a)として投稿してください。

## Issueの作成

- Issueを立てる前には必ず同様のIssueが存在しない事を確認してください。もしもIssueの内容が被っていた場合には`Not Planned`としてIssueを閉じてください。
- Issueをグループにしたい場合はマイルストーンを作成する事を検討してください。

## PullRequestの作成

- PullRequestは必ずIssueを立ててから作成してください。なぜならばそうしないと、そのPullRequestの目的が分からなくなるからです。関連情報として、[コードの修正節](#コードの修正)を読んでください。

## マイルストーンの作成

- マイルストーンは、hrtorのコラボレーターでないと作成出来ません。従って、マイルストーンを作成したい場合はまず[GitHub Discussions for hrtor](https://github.com/haruki7049/hrtor/discussions)に[Ideas Category](https://github.com/haruki7049/hrtor/discussions/categories/ideas)として投稿してください。私たちコラボレーターがコメントします。

```md
<!-- Title -->
Create a new milestone for hrtor:v0.1.1

<!-- Body -->
Wanna create a new milestone for hrtor:v0.1.1?

<!-- You must write Reasons to create a new milestone -->
## Reasons
**ANY REASONS!!**
```

## コードの修正

以下のフローに従ってください。

1. Issueを作成する。
2. Issueに対応させたPullRequestを作成する。作成方法は[GitHub Docs](https://docs.github.com/en/issues/tracking-your-work-with-issues/linking-a-pull-request-to-an-issue)を参照してください。
3. PullRequestにレビューをもらい、マージ。

この際、CI/CDが成功している事を確認してからPullRequestを作成しましょう。

## CI/CDの追加

現在、GitHub Actionsはrust-checkerのみ動作しています。以下にrust-checkerの説明を記載します。

- cargo-testの実行。
- cargo-clippyを実行、チェックする。
- cargo-fmtによって適切にインデント等されているかの確認。

もしも他にCI/CDを使うべき作業がある場合はその旨を書いたIssueを立ててください。

## Review

レビューを貰う前には必ずCI/CDが成功している事を確認してください。
レビューを貰いマージが出来るようになった後には以下の点を確認してください。

- `Create a merge commit`でマージしてください。
- 作業後には作業ブランチを削除してください。
