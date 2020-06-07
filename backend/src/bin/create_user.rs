extern crate red_drink;

use red_drink::models::user::User;
use red_drink::db;
use std::env;

/**
 * コマンドライン引数で受け取ったGitHubIDを用いて、ユーザを登録する。
 * GitHubのアカウントが存在するかの確認を行わない。
 */
fn main() {
    if let Some(id) = env::args().collect::<Vec<String>>().get(1).and_then(|id| { id.parse::<i32>().ok() }) {
        let message = match db::connect().get() {
            Ok(connection) => match User::create_with_github_id(id, &connection) {
                Some(user) => format!("github_id: {}, user_id: {}で作成しました。", id, user.id),
                None => format!("作成に失敗しました。")
            },
            Err(_) => format!("DBへの接続に失敗しました。")
        };

        println!("{}", message);
    }
    println!("GitHubのユーザIDを指定してください");
}