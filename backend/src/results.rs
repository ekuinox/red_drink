use crate::diesel::result::Error as DieselError;

/// red_drinkで返すエラー
/// diesel::result::Errorをラップしちゃおう
#[derive(PartialEq, Debug)]
pub enum RedDrinkError {
    Diesel(DieselError),
    Any(String)
}

impl RedDrinkError {
    pub fn from_diesel_error(error: DieselError) -> RedDrinkError {
        RedDrinkError::Diesel(error)
    }
    pub fn from_string(error: String) -> RedDrinkError {
        RedDrinkError::Any(error)
    }
}

pub type RedDrinkResult<T> = Result<T, RedDrinkError>;
