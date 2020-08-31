use diesel;
use diesel::prelude::*;
use crate::db::DBConnection;
use crate::types::DieselError;
use crate::schema::users;
use crate::schema::github_accounts;
use crate::models::{GitHubAccount, User, traits::*};

/// GitHubアカウントとの紐付けの実装
impl User {

    /// GitHubアカウントから作成する
    /// 注意: GitHubUserへの挿入が失敗した場合でも、そのレコードの削除を行わなっていない
    pub fn create_with_github_id(github_id: i32, conn: &DBConnection) -> Result<User, DieselError> {
        conn.transaction(|| {
            User::create((), conn).and_then(|user| {
                user.associate_to_github(github_id, conn).map(|_| user)
            })
        })
    }

    /// GitHubアカウントと紐付ける
    pub fn associate_to_github(&self, github_user_id: i32, connection: &DBConnection) -> Result<GitHubAccount, DieselError> {
        GitHubAccount::create((github_user_id, self.id), connection)
    }

    /// GitHubIdから検索する
    pub fn find_by_github_id(github_id: i32, connection: &DBConnection) -> Result<User, DieselError> {
        GitHubAccount::find(github_id, connection).and_then(|user: GitHubAccount| {
            user.to_user(connection)
        })
    }

    
    /// GitHubIdから検索し、存在しなければ作成する
    pub fn find_or_new_by_github_id(github_id: i32, connection: &DBConnection) -> Result<User, DieselError> {
        if let Ok(user) = User::find_by_github_id(github_id, connection) {
            Ok(user)
        } else {
            User::create_with_github_id(github_id, &connection)
        }
    }

    /// get all users with github
    pub fn all_with_github(connection: &DBConnection) -> Result<Vec<(User, Option<GitHubAccount>)>, DieselError> {
        users::table.left_join(github_accounts::table).load::<(User, Option<GitHubAccount>)>(connection)
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
        assert!(user.is_ok());
        // 指定したGitHubのIdでアカウントを作成できているか確認する
        assert_eq!(
            github_accounts::table.filter(github_accounts::user_id.eq(user.unwrap().id)).get_result::<GitHubAccount>(&connection).map(|user| user.github_id),
            Ok(github_id)
        );
        // GitHub ID の uniqueを確認する -> 同じIDで作成すると失敗する
        assert!(User::create_with_github_id(github_id, &connection).is_err());
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