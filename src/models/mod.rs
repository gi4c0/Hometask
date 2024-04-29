use sqlx::FromRow;

pub mod profile;

#[derive(FromRow)]
pub struct Total {
    pub total: i32,
}
