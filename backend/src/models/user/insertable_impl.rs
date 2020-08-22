use crate::models::user::UserInsertable;

/// 新規挿入用モデルの実装
impl UserInsertable {
    pub fn new() -> UserInsertable {
        UserInsertable { id: None }
    }
    pub fn new_with_id(id: i32) -> UserInsertable {
        UserInsertable { id: Some(id) }
    }
}
