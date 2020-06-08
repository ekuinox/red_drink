# backend

GitHubのアカウントでログインしてきたら、そのGitHubのIDを使ってユーザと紐付けて認証する感じにやる
ログイン時にGitHubのトークンをフロントに投げて、それを使い回すけど、認証したトークンだけパチられてめちゃくちゃされたら死ぬのだわ

## .env一気にexportする

`$ while read line; do export ; done < .env`
