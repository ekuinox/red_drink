extern crate red_drink;

use red_drink::models::user::{User, UserInsertable};
use red_drink::models::role::ADMIN_ROLE_ID;
use red_drink::db;
use std::env;

/**
 * コマンドライン引数でGitHubのIDを受け取り、admin権限を持ったユーザを新規追加する
 */
fn main() {
    if let Some(id) = env::args().collect::<Vec<String>>().get(1).map(|id| { id.parse::<i32>().ok() }).flatten() {
        let pool = db::connect();
        if let Ok(connection) = pool.get() {
            if let Some(user) = User::create(UserInsertable::new(), &connection) {
                // 作成したユーザにGitHubアカウントを紐付ける
                if let (user, true) = user.associate_to_github(id, &connection) {
                    /**
                     * Todo ADMIN_ROLE_IDを指定して紐付け
                     */
                    println!("新規追加成功");
                }
            }
        }

        println!("{}", id);
        return;
    }
    println!("GitHubのユーザIDを指定してください");
}