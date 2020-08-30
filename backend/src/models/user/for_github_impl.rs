use diesel;
use diesel::prelude::*;
use crate::db::DBConnection;
use crate::schema::users;
use crate::schema::github_accounts;
use crate::models::github_account::*;
use crate::models::user::User;
use crate::models::user::UserInsertable;

/// GitHubアカウントとの紐付けの実装
impl User {

    /// GitHubアカウントから作成する
    /// 注意: GitHubUserへの挿入が失敗した場合でも、そのレコードの削除を行わなっていない
    pub fn create_with_github_id(github_id: i32, connection: &DBConnection) -> Option<User> {
        Self::create(UserInsertable::new(), connection).and_then(|user| {
            match user.associate_to_github(github_id, connection) {
                (user, true) => Some(user),
                _ => None
            }
        })
    }

    /// GitHubアカウントと紐付ける
    pub fn associate_to_github(self, github_user_id: i32, connection: &DBConnection) -> (User, bool) {
        (self, GitHubUserInsertable::new(self.id, github_user_id).create(connection))
    }

    /// GitHubIdから検索する
    pub fn find_by_github_id(github_id: i32, connection: &DBConnection) -> Option<User> {
        GitHubAccount::find(github_id, connection).and_then(|user: GitHubAccount| {
            user.to_user(connection)
        })
    }

    
    /// GitHubIdから検索し、存在しなければ作成する
    pub fn find_or_new_by_github_id(github_id: i32, connection: &DBConnection) -> Option<User> {
        Self::find_by_github_id(github_id, connection).or_else(|| {
            User::create_with_github_id(github_id, &connection)
        })
    }

    /// get all users with github
    pub fn all_with_github(connection: &DBConnection) -> Option<Vec<(User, Option<GitHubAccount>)>> {
        users::table.left_join(github_accounts::table).load::<(User, Option<GitHubAccount>)>(connection).ok()
    }
}

#[test]
fn test_create_with_github_id() {
    use crate::schema::github_accounts;
    use crate::db::connect;

    let github_id = 1;
    let connection = connect().get().expect("cannnot get connection");
    connection.test_transaction::<_, diesel::result::Error, _>(|| {
        let user = User::create_with_github_id(github_id, &connection);
        // 作成できていることを確認する
        assert_ne!(user, None);
        // 指定したGitHubのIdでアカウントを作成できているか確認する
        assert_eq!(
            github_accounts::table.filter(github_accounts::user_id.eq(user.unwrap().id)).get_result::<GitHubAccount>(&connection).ok().map(|user| { user.github_id }),
            Some(github_id)
        );
        // GitHub ID の uniqueを確認する -> 同じIDで作成すると失敗する
        assert_eq!(User::create_with_github_id(github_id, &connection), None);
        Ok(())
    });
}

#[test]
fn test_find_by_github_id() {
    use crate::db::connect;

    let github_id = 1;
    let connection = connect().get().expect("cannnot get connection");
    connection.test_transaction::<_, diesel::result::Error, _>(|| {
        // 指定したGitHub Idでユーザを引っ張ってこれるかチェックする
        assert_eq!(User::create_with_github_id(github_id, &connection), User::find_by_github_id(github_id, &connection));
        Ok(())
    });
}