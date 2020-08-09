use diesel;
use diesel::prelude::*;
use std::collections::HashSet;
use crate::db::DBConnection;
use chrono::{NaiveDateTime};
use crate::schema::users;
use crate::models::github_user::*;
use crate::models::users_roles::*;
use crate::models::role::Role;
use crate::models::permission::Permission;

/**
 * RedDrinkのユーザ
 */
#[table_name = "users"]
#[derive(AsChangeset, Serialize, Deserialize, Insertable, Identifiable, Associations, Queryable, PartialEq, Clone, Copy, Debug)]
#[primary_key(id)]
pub struct User {
    pub id: i32,
    pub created_at: NaiveDateTime
}

/**
 * Userの新規挿入用モデル
 */
#[table_name = "users"]
#[derive(Insertable, Debug)]
pub struct UserInsertable {
    pub id: Option<i32>
}

/** --- Implementations --- */

/**
 * 新規挿入用モデル
 */
impl UserInsertable {
    pub fn new() -> UserInsertable {
        UserInsertable { id: None }
    }
    pub fn new_with_id(id: i32) -> UserInsertable {
        UserInsertable { id: Some(id) }
    }
}

/**
 * ユーザの操作
 */
impl User {
    /**
     * 新規登録
     */
    pub fn create(user: UserInsertable, connection: &DBConnection) -> Option<User> {
        diesel::insert_into(users::table).values(&user).execute(connection)
            .map(|_| {
                users::table.order(users::id.desc()).first(connection).ok()
            })
            .ok()
            .flatten()
    }

    /**
     * GitHubアカウントから作成する
     * 注意: GitHubUserへの挿入が失敗した場合でも、そのレコードの削除を行わなっていない
     */
    pub fn create_with_github_id(github_id: i32, connection: &DBConnection) -> Option<User> {
        Self::create(UserInsertable::new(), connection).and_then(|user| {
            match user.associate_to_github(github_id, connection) {
                (user, true) => Some(user),
                _ => None
            }
        })
    }

    /**
     * GitHubアカウントと紐付ける
     */
    pub fn associate_to_github(self, github_user_id: i32, connection: &DBConnection) -> (User, bool) {
        (self, GitHubUserInsertable::new(self.id, github_user_id).create(connection))
    }

    /**
     * ユーザにRoleを付与する
     */
    pub fn add_role(&self, role_id: i32, connection: &DBConnection) -> bool {
        UsersRoleInsertable::new(self.id, role_id).create(connection)
    }

    /**
     * ユーザの持つRoleを取得する
     */
    pub fn get_roles(&self, connection: &DBConnection) -> Option<Vec<Role>> {
        UsersRole::belonging_to(self).get_results::<UsersRole>(connection).map(|users_roles| {
            users_roles.iter().map(|users_role| {
                Role::find(users_role.role_id, connection).unwrap()
            }).collect::<Vec<Role>>()
        }).ok()
    }

    /**
     * Userが持つPermissionを取得する
     */
    pub fn get_permissions(&self, connection: &DBConnection) -> Option<Vec<Permission>> {
        self.get_roles(connection).map(|roles| {
            roles.into_iter().fold(Vec::<Permission>::new(), |prev, role| {
                role.get_permissions(connection).map(|permissions| [&prev[..], &permissions[..]].concat()).unwrap_or(prev)
            })
        }).map(|permissions| {
            // 重複を取り除く
            permissions.into_iter().collect::<HashSet<Permission>>().into_iter().collect::<Vec<Permission>>()
        })
    }

    /**
     * ユーザIDから取得する
     */
    pub fn find(id: i32, connection: &DBConnection) -> Option<User> {
        users::table.find(id).get_result(connection).ok()
    }

    /**
     * GitHubIdから検索する
     */
    pub fn find_by_github_id(github_id: i32, connection: &DBConnection) -> Option<User> {
        GitHubUser::find(github_id, connection).and_then(|user: GitHubUser| {
            user.to_user(connection)
        })
    }

    /**
     * GitHubIdから検索し、存在しなければ作成する
     */
    pub fn find_or_new_by_github_id(github_id: i32, connection: &DBConnection) -> Option<User> {
        Self::find_by_github_id(github_id, connection).or_else(|| {
            User::create_with_github_id(github_id, &connection)
        })
    }
}

#[test]
fn test_create_with_github_id() {
    use crate::schema::github_users;
    use crate::db::connect;

    let github_id = 1;
    let connection = connect().get().expect("cannnot get connection");
    connection.test_transaction::<_, diesel::result::Error, _>(|| {
        let user = User::create_with_github_id(github_id, &connection);
        // 作成できていることを確認する
        assert_ne!(user, None);
        // 指定したGitHubのIdでアカウントを作成できているか確認する
        assert_eq!(
            github_users::table.filter(github_users::user_id.eq(user.unwrap().id)).get_result::<GitHubUser>(&connection).ok().map(|user| { user.github_id }),
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