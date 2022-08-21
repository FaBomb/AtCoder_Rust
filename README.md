# AtCoder_Rust

## Snippet
スニペットを更新したのち以下のシェルスクリプトを実行することで自動で
```.vscode/cargo_snippet.code-snippets```が更新されるツール

```sh snippet.sh```

## Open Task
コンテスト名と問題を指定したら問題のページが開かれるツール

ex.)
```sh web_task.sh abc264 a```
>>> 
https://atcoder.jp/contests/abc264/tasks/abc219_a
が開かれる

## Create Source File
新規問題のソースファイルをテンプレートから生成し、
Cargo.tomlの設定まで自動でされるツール

ex.)
```./contest.py abc264 a b c```