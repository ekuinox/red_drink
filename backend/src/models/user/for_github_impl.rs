use diesel;
use diesel::prelude::*;
use crate::db::DBConnection;
use crate::types::DieselError;
use crate::schema::users;
use crate::schema::github_accounts;
use crate::models::{GitHubAccount, User, traits::*};
use crate::github::create_api_client;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubAccountDetail {
    pub login: String,
    pub id: i32,
    pub avatar_url: String,
    pub name: String,
    pub email: String
}

impl GitHubAccountDetail {
    fn resolve(token: &String) -> Result<Self, ()> {
        create_api_client(token)
            .get("https://api.github.com/user")
            .send()
            .and_then(|response| response.json::<Self>())
            .map_err(|_| ())
    }
}

/// GitHubアカウントとの紐付けの実装
impl User {

    /// GitHubアカウントから作成する
    /// 注意: GitHubUserへの挿入が失敗した場合でも、そのレコードの削除を行わなっていない
    pub fn create_with_github_detail(detail: GitHubAccountDetail, conn: &DBConnection) -> Result<User, ()> {
        let GitHubAccountDetail { login, id, avatar_url, email, .. } = detail;
        conn.transaction(|| {
            User::create((login, avatar_url, email), conn).and_then(|user| {
                user.associate_to_github(id, conn).map(|_| user)
            })
        }).map_err(|_| ())
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

    
    /// ユーザのトークンから検索し、存在しなければ作成する
    pub fn find_or_new_by_github_token(token: &String, connection: &DBConnection) -> Result<User, ()> {
        let detail = GitHubAccountDetail::resolve(token)?;
        if let Ok(user) = User::find_by_github_id(detail.id, connection) {
            Ok(user)
        } else {
            User::create_with_github_detail(detail, &connection)
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

    let detail = GitHubAccountDetail {
        id: 1,
        name: "Foo Taro".to_string(),
        avatar_url: "avatar_url".to_string(),
        email: "foo@example.com".to_string(),
        login: "foo".to_string()
    };
    let connection = connect().get().expect("cannnot get connection");
    connection.test_transaction::<_, diesel::result::Error, _>(|| {
        let user = User::create_with_github_detail(detail.clone(), &connection);
        // 作成できていることを確認する
        assert!(user.is_ok());
        // 指定したGitHubのIdでアカウントを作成できているか確認する
        assert_eq!(
            github_accounts::table.filter(github_accounts::user_id.eq(user.unwrap().id)).get_result::<GitHubAccount>(&connection).map(|user| user.github_id),
            Ok(detail.id)
        );
        // GitHub ID の uniqueを確認する -> 同じIDで作成すると失敗する
        assert!(User::create_with_github_detail(detail.clone(), &connection).is_err());
        Ok(())
    });
}

#[test]
fn test_find_by_github_id() {
    use crate::db::connect;

    let detail = GitHubAccountDetail {
        id: 1,
        name: "Foo Taro".to_string(),
        avatar_url: "avatar_url".to_string(),
        email: "foo@example.com".to_string(),
        login: "foo".to_string()
    };
    let connection = connect().get().expect("cannnot get connection");
    connection.test_transaction::<_, diesel::result::Error, _>(|| {
        // 指定したGitHub Idでユーザを引っ張ってこれるかチェックする
        assert_eq!(User::create_with_github_detail(detail.clone(), &connection), User::find_by_github_id(detail.id, &connection).map_err(|_| ()));
        Ok(())
    });
}
