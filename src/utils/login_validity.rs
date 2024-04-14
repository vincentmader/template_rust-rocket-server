use crate::database::SqliteDb;
use regex::Regex;
use rocket_db_pools::Connection;

pub async fn is_existing_user_name(conn: &mut Connection<SqliteDb>, user_name: &str) -> bool {
    let rows = sqlx::query("SELECT user_name,mail_addr,pass_hash FROM users where user_name = ?")
        .bind(user_name)
        .fetch_all(&mut ***conn)
        .await
        .unwrap();

    !rows.is_empty()
}

pub async fn is_existing_mail_addr(conn: &mut Connection<SqliteDb>, mail_addr: &str) -> bool {
    let rows = sqlx::query("SELECT user_name,mail_addr,pass_hash FROM users where mail_addr = ?")
        .bind(mail_addr)
        .fetch_all(&mut ***conn)
        .await
        .unwrap();

    !rows.is_empty()
}

pub fn is_valid_mail_addr(mail_addr: &str) -> bool {
    let re = Regex::new(r"^([a-zA-Z0-9_\-\.]+)@([a-zA-Z0-9_\-\.]+)\.([a-zA-Z]{2,5})$").unwrap();
    re.is_match(mail_addr)
}

pub async fn is_valid_login(
    conn: &mut Connection<SqliteDb>,
    user_info: &str,
    pass_hash: &str,
) -> bool {
    let rows = sqlx::query(
        "SELECT user_name,mail_addr,pass_hash FROM users WHERE 
        (user_name = ? AND pass_hash = ?) OR
        (mail_addr = ? AND pass_hash = ?)",
    )
    .bind(user_info)
    .bind(pass_hash)
    .bind(user_info)
    .bind(pass_hash)
    .fetch_all(&mut ***conn)
    .await
    .unwrap();

    !rows.is_empty()
}
