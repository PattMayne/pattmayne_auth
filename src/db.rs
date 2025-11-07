// I'm actually using MariaDB which is supposedly a drop-in replacement for MySQL

use sqlx::{MySqlPool, Pool, mysql::MySqlPoolOptions, Connection, MySqlConnection};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use rand_core::OsRng;
use password_hash::{SaltString, PasswordHash};
use dotenvy;
use std::error::Error;

fn load_db_old() {
    // get the database URL from the .env file
    //let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create the connection pool
    //let pool = sqlx::MySqlPool::connect(&db_url).await?;

}


pub fn load_db() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file.
    // CHECK: Fails if .env file not found, not readable or invalid.
    dotenvy::dotenv()?;

    let database_url = dotenvy::var("DATABASE_URL")?;

    println!("DATABASE_URL: {}", database_url);

    Ok(())
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
pub fn verify_password(input_password: String, stored_hash: String) -> bool {
    let parsed_stored_hash: PasswordHash<'_> = PasswordHash::new(&stored_hash).unwrap();
    let argon2: Argon2<'_> = Argon2::default();

    // returns a bool
    argon2.verify_password(
        input_password.as_bytes(),
        &parsed_stored_hash
    ).is_ok()
}