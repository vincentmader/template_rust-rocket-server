use crate::database::SqliteDb;
use rocket_db_pools::Connection;
use sqlx::{Row, Value, ValueRef};

pub async fn is_existing_user_name(conn: &mut Connection<SqliteDb>, user_name: &str) -> bool {
    let rows = sqlx::query("SELECT user_name FROM users where user_name = ?")
        .bind(user_name) // TODO Make case-insensitive.
        .fetch_all(&mut ***conn)
        .await
        .unwrap();
    !rows.is_empty()
}

pub async fn is_existing_mail_addr(conn: &mut Connection<SqliteDb>, mail_addr: &str) -> bool {
    let rows = sqlx::query("SELECT user_name FROM users where mail_addr = ?")
        .bind(mail_addr) // TODO Make case-insensitive.
        .fetch_all(&mut ***conn)
        .await
        .unwrap();
    !rows.is_empty()
}

pub async fn is_valid_login(
    conn: &mut Connection<SqliteDb>,
    user_info: &str,
    pass_word: &str, // NOTE: Not actually the password, but a SHA256 encrypted version of it.
) -> bool {
    let rows = sqlx::query(
        "SELECT user_name,pass_hash FROM users 
          WHERE user_name = ? OR mail_addr = ? ",
    )
    .bind(user_info)
    .bind(user_info)
    .fetch_all(&mut ***conn)
    .await
    .unwrap();
    if rows.is_empty() {
        return false; // Here: No account with this user_name/mail_addr found.
    }
    if rows.len() > 1 {
        panic!("This should never happen. Each username must exist only once.");
        // TODO Return new `ApiMessage` variant for this.
    }

    let row = rows.get(0).unwrap();
    let pass_hash: String = row.try_get_raw(1).unwrap().to_owned().decode();

    bcrypt::verify(pass_word, &pass_hash).unwrap()
}
