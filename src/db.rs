// I'm actually using MariaDB which is supposedly a drop-in replacement for MySQL

use sqlx::{MySqlPool };
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use rand_core::OsRng;
use password_hash::{SaltString, PasswordHash};
use time::OffsetDateTime;
use anyhow::{ Result, anyhow };
use serde;

// use pattern matching in an impl function to get a String to store to DB
pub enum UserRole {
    Admin,
    Player,
}

#[derive(Debug)]
struct Count {
    count: i64,
}

#[derive(serde::Serialize)]
pub struct User {
    id: i32,
    username: String,
    email: String,
    first_name: Option<String>,
    last_name: Option<String>,
    role: String,
    password_hash: String,
    created_timestamp: OffsetDateTime,
    email_verified: i8 // actually a bool but mysql doesn't do "real" bools
}


impl User {

    pub fn get_email_verified(&self) -> bool {
        self.email_verified != 0
    }

    pub fn new(
        id: i32,
        username: String,
        email: String,
        first_name: Option<String>,
        last_name: Option<String>,
        role: String,
        password_hash: String,
        created_timestamp: OffsetDateTime,
        email_verified: bool
    ) -> Self {
        User {
            id, username, email, first_name, last_name, role,
            password_hash, created_timestamp,
            email_verified: if email_verified { 1 } else { 0 }
        }
    }

    pub fn get_password_hash(&self) -> &String {
        &self.password_hash
    }

    pub fn get_id(&self) -> i32 { self.id }
    pub fn get_role(&self) -> &String { &self.role }
    pub fn get_username(&self) -> &String { &self.username }

    pub fn get_first_name(&self) -> String {
        match self.first_name.clone() {
            Some(first_name) => first_name.to_owned(),
            None => String::new()
        }
    }

        pub fn get_last_name(&self) -> String {
        match self.last_name.clone() {
            Some(last_name) => last_name.to_owned(),
            None => String::new()
        }
    }


}



pub async fn create_pool() -> Result<MySqlPool> {
     // Load environment variables from .env file.
    // CHECK: Fails if .env file not found, not readable or invalid.
    let database_url = std::env::var("DATABASE_URL")?;
    
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

    Ok(sqlx::query_as!(
            User,
            "SELECT id, username, email, first_name,
                last_name, role, password_hash, created_timestamp,
                email_verified FROM users WHERE username = ?",
            username
        ).fetch_optional(&pool).await?)
}


pub async fn get_user_by_email(email: &String) -> Result<Option<User>> {
    let pool: MySqlPool = create_pool().await?;

    Ok(sqlx::query_as!(
        User,
        "SELECT id, username, email,
            first_name, last_name, role,
            password_hash, created_timestamp,
            email_verified FROM users WHERE email = ?",
        email
    ).fetch_optional(&pool).await?)
}


pub async fn get_user_by_id(id: i32) -> Result<Option<User>> {
    let pool: MySqlPool = create_pool().await?;

    Ok(sqlx::query_as!(
        User,
        "SELECT id, username, email,
            first_name, last_name, role,
            password_hash, created_timestamp,
            email_verified FROM users WHERE id = ?",
        id
    ).fetch_optional(&pool).await?)
}


// Pre-check for duplicates

// check if username already exists in DB
pub async fn username_taken(username: &String) -> bool {
    let pool: MySqlPool = match create_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to create pool: {:?}", e);
            return false;
        }
    };

    let count_option: Option<Count> = match sqlx::query_as!(
        Count,
        "SELECT COUNT(*) as count FROM users WHERE username = ?",
        username
    ).fetch_optional(&pool).await {
        Ok(count) => count,
        Err(e) => {
            eprintln!("Failed to fetch count from DB: {:?}", e);
            return false;
        }
    };

    let count: i64 = count_option.unwrap_or(Count{count: 0}).count;
    count > 0
}

// Check if email address already exists in DB
pub async fn email_taken(email: &String) -> bool {
    let pool: MySqlPool = match create_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to create pool: {:?}", e);
            return false;
        }
    };

    let count_option: Option<Count> = match sqlx::query_as!(
        Count,
        "SELECT COUNT(*) as count FROM users WHERE email = ?",
        email
    ).fetch_optional(&pool).await {
        Ok(count) => count,
        Err(e) => {
            eprintln!("Failed to fetch count from DB: {:?}", e);
            return false;
        }
    };

    let count: i64 = count_option.unwrap_or(Count{count: 0}).count;
    count > 0

}


// Add new user to database

pub async fn add_user(username: &String, email: &String, password: String) -> Result<i32, anyhow::Error> {

    // map_err changes a possible error into the return type of error I return in the closure
    // This is simpler and more idiomatic than doing a match
    let pool: MySqlPool = create_pool().await.map_err(|e| {
        eprintln!("Failed to create pool: {:?}", e);
        anyhow!("Could not create pool: {e}")
    })?;

    // hash the password
    let password_hash: String = hash_password(password);

    let result: sqlx::mysql::MySqlQueryResult = sqlx::query(
            "INSERT INTO users (
                username,
                email,
                password_hash)
            VALUES (?, ?, ?)")
        .bind(username)
        .bind(email)
        .bind(&password_hash)
        .execute(&pool).await.map_err(|e| {
            eprintln!("Failed to save user to database: {:?}", e);
            anyhow!("Could not save user to database: {e}")
        })?;

    Ok(result.last_insert_id() as i32)
}