extern crate red_drink;

use red_drink::models::user::User;
use red_drink::db;
use std::env;

/**
 * コマンドライン引数で受け取ったGitHubIDを用いて、ユーザを登録する。
 * GitHubのアカウントが存在するかの確認を行わない。
 */
fn main() {
    let args = env::args().collect::<Vec<String>>();
    let message = match (args.get(1).and_then(|user_id| { user_id.parse::<i32>().ok() }), args.get(2).and_then(|role_id| { role_id.parse::<i32>().ok() })) {
        (Some(user_id), Some(role_id)) => match db::connect().get() {
            Ok(connection) => match User::find(user_id, &connection) {
                Some(user) => if user.add_role(role_id, &connection) { format!("Roleの付与に成功しました。") } else { format!("Roleの付与に失敗しました。") },
                None => format!("ユーザが見つかりません。")
            },
            _ => format!("DBへの接続に失敗しました。")
        },
        _ => format!("user_idかrole_idが指定されていません。")
    };
    println!("{}", message);
}