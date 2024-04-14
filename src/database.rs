use rocket_db_pools::sqlx;
use rocket_db_pools::Database;

#[derive(Database)]
#[database("sqlite_db")]
pub struct SqliteDb(sqlx::SqlitePool);
