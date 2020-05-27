aws の利用料金を slack のチャンネルに通知します。

# 使い方

## 準備

1. プロジェクトの root 配下に、以下のような .env ファイルを用意。

```
AWS_ACCESS_KEY_ID=********************
AWS_SECRET_ACCESS_KEY=****************************************
SLACK_WEBHOOK_URL=https://hooks.slack.com/services/*********/***********/************************
```

2. [CostExplorer を有効にする](https://docs.aws.amazon.com/ja_jp/awsaccountbilling/latest/aboutv2/ce-enable.html)

## 実行方法

これで、今月分の aws の料金が slack に送信される。

```
$ cargo run -- cost
```

### 期間の指定

`yyyy-mm-dd` の形式で期間を指定できる。
指定する場合は `-s` と `-e` の両方を指定すること。片方だけの指定だと、デフォルト期間（今月分）での検索になる。

```
$ cargo run -- cost -s 2020-02-01 -e 2020-04-01
```

また、CostExplorer の仕様上、開始日は 1 年以上前の日付は指定できない。終了日は翌月移行の日付は指定できない。

### チャンネルの指定

`-c` で送信するチャンネルを指定できる。指定なしだと `cost` チャンネルに送信する。

```
$ cargo run -- cost -c hoge
```
