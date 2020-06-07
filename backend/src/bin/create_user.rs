extern crate red_drink;

use red_drink::models::user::User;
use red_drink::db;
use std::env;

/**
 * コマンドライン引数で受け取ったGitHubIDを用いて、ユーザを登録する。
 * GitHubのアカウントが存在するかの確認を行わない。
 */
fn main() {
    println!("{}", match env::args().collect::<Vec<String>>().get(1).and_then(|id| { id.parse::<i32>().ok() }) {
        Some(github_id) => match db::connect().get() {
            Ok(connection) => match User::create_with_github_id(github_id, &connection) {
                Some(user) => format!("github_id: {}, user_id: {}で作成しました。", github_id, user.id),
                None => format!("作成に失敗しました。")
            },
            Err(_) => format!("DBへの接続に失敗しました。")
        },
        None => format!("GitHubのユーザIDを指定してください")
    })
}