// I'm actually using MariaDB which is supposedly a drop-in replacement for MySQL

use sqlx::{MySqlPool, Pool, mysql::MySqlPoolOptions, Connection, MySqlConnection};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use rand_core::OsRng;
use password_hash::{SaltString, PasswordHash};
use dotenvy;
use std::error::Error;
use time::OffsetDateTime;
use anyhow::Result;
use serde;

// use pattern matching in an impl function to get a String to store to DB
pub enum UserRole {
    Admin,
    Player,
}

#[derive(serde::Serialize)]
pub struct User {
    id: i64,
    username: String,
    email: String,
    first_name: Option<String>,
    last_name: Option<String>,
    role: String,
    pub password_hash: String,
    created_timestamp: OffsetDateTime,
}


fn load_db_old() {
    // get the database URL from the .env file
    //let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create the connection pool
    //let pool = sqlx::MySqlPool::connect(&db_url).await?;

}


// I WILL NOT USE THIS. DELETE later
pub fn load_db() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file.
    // CHECK: Fails if .env file not found, not readable or invalid.
    dotenvy::dotenv()?;

    let database_url = dotenvy::var("DATABASE_URL")?;

    println!("DATABASE_URL: {}", database_url);

    Ok(())
}


pub async fn create_pool() -> Result<MySqlPool> {
     // Load environment variables from .env file.
    // CHECK: Fails if .env file not found, not readable or invalid.
    dotenvy::dotenv()?;
    let database_url = dotenvy::var("DATABASE_URL")?;
    
    Ok(MySqlPool::connect(database_url.as_str()).await?)
}



/**
 * Used when a user registers. We must hash their password so that the raw
 * password is never stored in the DB.
 * We take ownership of the input String so it's annihilated after fn runs.
 * @return String (hashed password)
 */
pub fn hash_password(input_password: String) -> String {
    let salt: SaltString = SaltString::generate(&mut OsRng);

    // Hash the password and return
    Argon2::default().hash_password(
        input_password.as_bytes(),
        &salt
    ).unwrap().to_string()
}

/**
 * When a user logs in we take their raw password string and verify it against
 * the stored hashed password.
 * @return bool (matches or does not match)
 */
pub fn verify_password(input_password: &String, stored_hash: &String) -> bool {
    let parsed_stored_hash: PasswordHash<'_> = PasswordHash::new(&stored_hash).unwrap();
    let argon2: Argon2<'_> = Argon2::default();

    // returns a bool
    argon2.verify_password(
        input_password.as_bytes(),
        &parsed_stored_hash
    ).is_ok()
}


// RETRIEVE USER FUNCTIONS

pub async fn get_user_by_username(username: &String) -> Result<Option<User>> {
    let pool: MySqlPool = create_pool().await?;
    let query: &str = "SELECT * from users WHERE username = ?";

    Ok(sqlx::query_as!(
            User,
            "SELECT id, username, email, first_name, last_name, role, password_hash, created_timestamp FROM users WHERE username = ?",
            username
        ).fetch_optional(&pool).await?)
}
