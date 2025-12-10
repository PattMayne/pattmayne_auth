// I'm actually using MariaDB which is supposedly a drop-in replacement for MySQL

use sqlx::{MySqlPool };
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use rand_core::OsRng;
use password_hash::{SaltString, PasswordHash};
use time::{ OffsetDateTime, Duration };
use anyhow::{ Result, anyhow };
use serde;

use crate::utils;

/* 
 * 
 * 
 * 
 * 
 * =============================
 * =============================
 * =====                   =====
 * =====  STRUCTS & ENUMS  =====
 * =====                   =====
 * =============================
 * =============================
 * 
 * 
 * 
 * 
 */


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

#[derive(serde::Serialize)]
pub struct RefreshToken {
    id: i32,
    user_id: i32,
    client_id: String,
    token: String,
    created_timestamp: OffsetDateTime,
    expires_timestamp: OffsetDateTime
}

/**
 * When you UPDATE existing client site data
 */
pub struct UpdateClientData {
    pub site_domain: String,
    pub site_name: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub logo_url: String,
    pub description: String,
    pub category: String,
    pub client_type: String,
    pub is_active: bool,
}

pub struct UpdateClientSecret {
    pub hashed_client_secret: String,
}


/**
 * When you ENTER client site data for the first time
 */
pub struct NewClientData {
    pub site_domain: String,
    pub site_name: String,
    pub hashed_client_secret: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub logo_url: String,
    pub description: String,
    pub category: String,
    pub client_type: String,
    pub is_active: bool,
}

/**
 * When you GET the client site data to use
 */
pub struct ClientData {
    pub id: i32,
    pub domain: String,
    pub name: String,
    pub hashed_client_secret: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub logo_url: String,
    pub description: String,
    pub category: String,
    pub client_type: String,
    is_active: i8,
    is_internal: i8,
    pub created_timestamp: OffsetDateTime,
}


/* A container to satisfy sqlx's insatiable lust for structs.
 * For when we need to get a list of all the client_ids from
 * the client_sites table.
*/
pub struct ClientRef {
    pub client_id: String,
    pub name: String,
    pub logo_url: String,
}


impl ClientData {
    pub fn get_is_active(&self) -> bool { self.is_active == 1 }
    pub fn get_is_internal(&self) -> bool { self.is_internal == 1 }
}


impl RefreshToken {
    pub fn get_token(&self) -> &String { &self.token }
    pub fn get_client_id(&self) -> &String { &self.client_id }
    pub fn get_user_id(&self) -> i32 { self.user_id }
    pub fn get_created_timestamp(&self) -> &OffsetDateTime { &self.created_timestamp }
    pub fn get_expires_timestamp(&self) -> &OffsetDateTime { &self.expires_timestamp }

    pub fn is_expired(&self) -> bool {
        self.expires_timestamp < OffsetDateTime::now_utc()
    }
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





/* 
 * 
 * 
 * 
 * 
 * ==============================
 * ==============================
 * =====                    =====
 * =====  SELECT FUNCTIONS  =====
 * =====                    =====
 * ==============================
 * ==============================
 * 
 * retrieving data from the DB
 * 
 * 
 * 
 * 
 * 
 */


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


/**
 * Get a refresh token for specified user and client site.
 */
pub async fn get_refresh_token(user_id: i32, client_id: String) -> Result<Option<RefreshToken>> {
    let pool: MySqlPool = create_pool().await?;

    Ok(sqlx::query_as!(
        RefreshToken,
        "SELECT id, user_id, client_id,
            token, created_timestamp, expires_timestamp
            FROM refresh_tokens WHERE user_id = ? AND client_id = ?",
        user_id, client_id
    ).fetch_optional(&pool).await?)
}

/**
 * Get a collection of all the client_ids and names in the client_sites table.
 * These are references for the sake of lists, where the client_id can also
 * provide a handle for a link to an edit page (or whatever)
 */
pub async fn get_client_refs() -> Result<Vec<ClientRef>> {
    let pool: MySqlPool = create_pool().await?;
    let client_refs: Vec<ClientRef> = sqlx::query_as!(
        ClientRef,
        "SELECT client_id, name, logo_url FROM client_sites"
    ).fetch_all(&pool).await?;

    Ok(client_refs)
}


pub async fn get_client_by_client_id(client_id: &String) -> Result<Option<ClientData>> {
    let pool: MySqlPool = create_pool().await?;

    Ok(sqlx::query_as!(
        ClientData,
        "SELECT id, client_id, hashed_client_secret,
            name, domain, redirect_uri,
            description, category, logo_url, is_active,
            client_type, is_internal, created_timestamp
            FROM client_sites WHERE client_id = ?",
        client_id
    ).fetch_optional(&pool).await?)
}


/* 
 * 
 * 
 * 
 * 
 * 
 * 
 * ==============================
 * ==============================
 * =====                    =====
 * =====  INSERT FUNCTIONS  =====
 * =====                    =====
 * ==============================
 * ==============================
 * 
 * Adding new entries to the database
 * 
 * 
 * 
 * 
 */

 /**
  * Add a refresh token to the database.
  * for a particular user and particular client site.
  * Take ownership of token, because it should ONLY be given back
  * if it's saved successfully to the DB.
  */
 pub async fn add_refresh_token(
    user_id: i32,
    client_id: String,
    refresh_token: String
) -> Result<String, anyhow::Error> {
    let pool: MySqlPool = create_pool().await.map_err(|e| {
        eprintln!("Failed to create pool: {:?}", e);
        anyhow!("Could not create pool: {e}")
    })?;

    let expires_timestamp: OffsetDateTime =
        OffsetDateTime::now_utc() + Duration::days(14);
    let created_timestamp: OffsetDateTime = OffsetDateTime::now_utc();

    let _result: sqlx::mysql::MySqlQueryResult = sqlx::query(
        "INSERT INTO refresh_tokens (
            user_id,
            client_id,
            token,
            created_timestamp,
            expires_timestamp)
        VALUES (?, ?, ?, ?, ?)
        ON DUPLICATE KEY UPDATE
            token = VALUES(token),
            created_timestamp = VALUES(created_timestamp),
            expires_timestamp = VALUES(expires_timestamp);
            ")
    .bind(user_id)
    .bind(client_id)
    .bind(&refresh_token)
    .bind(created_timestamp)
    .bind(expires_timestamp)
    .execute(&pool).await.map_err(|e| {
        eprintln!("Failed to save refresh_token to database: {:?}", e);
        anyhow!("Could not save refresh_token to database: {e}")
    })?;

    Ok(refresh_token)
 }


// Add new user to database
pub async fn add_user(username: &String, email: &String, password: String) -> Result<i32, anyhow::Error> {
    // map_err changes a possible error into the return type of error I return in the closure
    // This is simpler and more idiomatic than doing a match
    let pool: MySqlPool = create_pool().await.map_err(|e| {
        eprintln!("Failed to create pool: {:?}", e);
        anyhow!("Could not create pool: {e}")
    })?;

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


/**
 * When the server starts up we make sure there is an admin.
 * Their default pre-hashed password is saved in an env variable.
 */
pub async fn create_primary_admin() -> Result<bool, anyhow::Error> {
    let pool: MySqlPool = create_pool().await.map_err(|e| {
        eprintln!("Failed to create pool: {:?}", e);
        anyhow!("Could not create pool: {e}")
    })?;

    // If admin already exists, print their name and return false.

    let count_option: Option<Count> = match sqlx::query_as!(
        Count,
        "SELECT COUNT(*) as count FROM users WHERE role = ?",
        "admin"
    ).fetch_optional(&pool).await {
        Ok(count) => count,
        Err(e) => {
            eprintln!("Failed to fetch admin user count from DB: {:?}", e);
            return Err(anyhow!("Could not fetch admin count: {e}"));
        }
    };

    let count: i64 = count_option.unwrap_or(Count{count: 0}).count;
    if count > 0 {
        println!("Admin already exists.");
        return Ok(false);
    }

    // Admin does NOT exist (if we reached this point in the function)
    // Time to create the admin
    let default_pw: String = std::env::var("ADMIN_PW")?;

    let username: &str = "pattmayne";
    let email: &str = "pattmayne@gmail.com";
    let role: &str = "admin";
    let result: sqlx::mysql::MySqlQueryResult = sqlx::query(
            "INSERT INTO users (
                username,
                email,
                role,
                password_hash)
            VALUES (?, ?, ?, ?)")
        .bind(username)
        .bind(email)
        .bind(role)
        .bind(&default_pw)
        .execute(&pool).await.map_err(|e| {
            eprintln!("Failed to save FIRST ADMIN user to database: {:?}", e);
            anyhow!("Could not save FIRST ADMIN user to database: {e}")
        })?;

    let new_rows_count: u64 = result.rows_affected();
    Ok(new_rows_count > 0)
}


/**
 * When the server starts up we make sure the auth site (this site)
 * exists as a client_site in the DB.
 */
pub async fn create_self_client() -> Result<bool, anyhow::Error> {
    let domain: String = std::env::var("AUTH_DOMAIN")?;

    let pool: MySqlPool = create_pool().await.map_err(|e| {
        eprintln!("Failed to create pool: {:?}", e);
        anyhow!("Could not create pool: {e}")
    })?;

    // If site already exists, print their name and return false.

    let count_option: Option<Count> = match sqlx::query_as!(
        Count,
        "SELECT COUNT(*) as count FROM client_sites WHERE domain = ?",
        &domain
    ).fetch_optional(&pool).await {
        Ok(count) => count,
        Err(e) => {
            eprintln!("Failed to fetch client_sites count from DB: {:?}", e);
            return Err(anyhow!("Could not fetch auth client_sites count: {e}"));
        }
    };

    let count: i64 = count_option.unwrap_or(Count{count: 0}).count;
    if count > 0 {
        println!("Auth client_site already exists.");
        return Ok(false);
    }

    // Auth site does NOT already exist (if we reached this far in the function)
    // Create auth site
    let client_id: String = utils::auth_client_id();
    let client_secret: &str = "CLIENT_SECRET_PLACEHOLDER";
    let name: &str = "Auth Site";
    let redirect_uri: &str = "127.0.0.1:8080/dashboard";
    let client_type: &str = "confidential";
    let category: &str = "service";
    let is_internal: bool = true;


    let result: sqlx::mysql::MySqlQueryResult = sqlx::query(
    "INSERT INTO client_sites (
            client_id,
            hashed_client_secret,
            name,
            domain,
            redirect_uri,
            client_type,
            category,
            is_internal
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(client_id)
        .bind(client_secret)
        .bind(name)
        .bind(domain)
        .bind(redirect_uri)
        .bind(client_type)
        .bind(category)
        .bind(is_internal)
        .execute(&pool).await.map_err(|e| {
            eprintln!("Failed to save FIRST AUTH client to database: {:?}", e);
            anyhow!("Could not save FIRST AUTH client to database: {e}")
        })?;

    Ok(result.rows_affected() > 0)
}



/**
 * When the server starts up we make sure the auth site (this site)
 * exists as a client_site in the DB.
 */
pub async fn add_external_client(new_client_data: NewClientData) -> Result<u64, anyhow::Error> {

    println!("In the DB to add a NEW CLIENT SITE!!!!!");
    let pool: MySqlPool = create_pool().await.map_err(|e| {
        eprintln!("Failed to create pool: {:?}", e);
        anyhow!("Could not create pool: {e}")
    })?;

    // We trust that the data has already been checked. We simply enter it like obedient robots now.
    // Except that we will turn the bool into an int.
    let result: sqlx::mysql::MySqlQueryResult = sqlx::query(
    "INSERT INTO client_sites (
            client_id,
            hashed_client_secret,
            name,
            domain,
            redirect_uri,
            logo_url,
            client_type,
            description,
            category,
            is_internal,
            is_active
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(new_client_data.client_id)
        .bind(new_client_data.hashed_client_secret)
        .bind(new_client_data.site_name)
        .bind(new_client_data.site_domain)
        .bind(new_client_data.redirect_uri)
        .bind(new_client_data.logo_url)
        .bind(new_client_data.client_type)
        .bind(new_client_data.description)
        .bind(new_client_data.category)
        .bind(0)
        .bind(new_client_data.is_active)
        .execute(&pool).await.map_err(|e| {
            eprintln!("Failed to save EXTERNAL CLIENT to database: {:?}", e);
            anyhow!("Could not save EXTERNAL CLIENT to database: {e}")
        })?;
    
    Ok(result.rows_affected())
}

/* 
 * 
 * 
 * 
 * 
 * 
 * 
 * ==============================
 * ==============================
 * =====                    =====
 * =====  UPDATE FUNCTIONS  =====
 * =====                    =====
 * ==============================
 * ==============================
 * 
 * 
 * 
 * update existing entries in the DB
 * 
 * 
 * 
 * 
 */


pub async fn update_external_client(update_client_data: UpdateClientData) -> Result<i32, anyhow::Error> {
    println!("Updating client in the database.");
        let pool: MySqlPool = create_pool().await.map_err(|e| {
        eprintln!("Failed to create pool: {:?}", e);
        anyhow!("Could not create pool: {e}")
    })?;

    let result: sqlx::mysql::MySqlQueryResult = sqlx::query(
    "UPDATE client_sites SET name = ?, domain = ?, redirect_uri = ?,
            description = ?, logo_url = ?, is_active = ?,
            client_type = ?, category = ? WHERE client_id = ?")
        .bind(update_client_data.site_name)
        .bind(update_client_data.site_domain)
        .bind(update_client_data.redirect_uri)
        .bind(update_client_data.description)
        .bind(update_client_data.logo_url)
        .bind(update_client_data.is_active)
        .bind(update_client_data.client_type)
        .bind(update_client_data.category)
        .bind(update_client_data.client_id)
        .execute(&pool)
        .await?;

    Ok(result.rows_affected() as i32)
}

pub async fn update_real_names(
    first_name: &String,
    last_name: &String,
    id: i32
)-> Result<i32, anyhow::Error> {
     // map_err changes a possible error into the return type of error I return in the closure
    // This is simpler and more idiomatic than doing a match
    let pool: MySqlPool = create_pool().await.map_err(|e| {
        eprintln!("Failed to create pool: {:?}", e);
        anyhow!("Could not create pool: {e}")
    })?;

    let result: sqlx::mysql::MySqlQueryResult = sqlx::query(
    "UPDATE users SET first_name = ?, last_name = ? WHERE id = ?")
            .bind(first_name)
            .bind(last_name)
            .bind(id)
            .execute(&pool)
            .await?;

    Ok(result.rows_affected() as i32)
}


pub async fn update_client_secret(
    client_id: &String,
    hashed_client_secret: &String
) -> Result<i32, anyhow::Error> {
    let pool: MySqlPool = create_pool().await.map_err(|e| {
        eprintln!("Failed to create pool: {:?}", e);
        anyhow!("Could not create pool: {e}")
    })?;

    let result = sqlx::query(
        "UPDATE client_sites SET hashed_client_secret = ? WHERE client_id = ?")
            .bind(hashed_client_secret)
            .bind(client_id)
            .execute(&pool)
            .await?;

    Ok(result.rows_affected() as i32)
}


/**
 * User is updating password.
 * Route has already confirmed that it's an acceptable password.
 * Hash it and save it to the database.
 */
pub async fn update_password(password: &String, id: i32)-> Result<i32, anyhow::Error> {
    let hashed_password: String = hash_password(password.to_owned());

    // save password to DB and return positive result
    let pool: MySqlPool = create_pool().await.map_err(|e| {
        eprintln!("Failed to create pool: {:?}", e);
        anyhow!("Could not create pool: {e}")
    })?;

    let result: sqlx::mysql::MySqlQueryResult = sqlx::query(
        "UPDATE users SET password_hash = ? WHERE id = ?")
            .bind(hashed_password)
            .bind(id)
            .execute(&pool)
            .await?;

    Ok(result.rows_affected() as i32)
}


/* 
 * 
 * 
 * 
 * 
 * 
 * 
 * ==============================
 * ==============================
 * =====                    =====
 * =====  DELETE FUNCTIONS  =====
 * =====                    =====
 * ==============================
 * ==============================
 * 
 * 
 * 
 * 
 * 
 * 
 */


/**
 * When a user logs out of a site, delete all their refresh tokens
 */
pub async fn delete_refresh_token(user_id: i32) -> Result<i32, anyhow::Error> {
    let pool: MySqlPool = create_pool().await.map_err(|e| {
        eprintln!("Failed to create pool: {:?}", e);
        anyhow!("Could not create pool: {e}")
    })?;

    let result: sqlx::mysql::MySqlQueryResult = sqlx::query(
        "DELETE FROM refresh_tokens WHERE user_id = ?")
            .bind(user_id)
            .execute(&pool)
            .await?;

    Ok(result.rows_affected() as i32)
}



/* 
 * 
 * 
 * 
 * 
 * 
 * ==============================
 * ==============================
 * =====                    =====
 * =====  HELPER FUNCTIONS  =====
 * =====                    =====
 * ==============================
 * ==============================
 * 
 * 
 * 
 * Functions which facilitate the processing of the above DB functions
 * 
 * 
 * 
 * 
 */



// Pre-check for duplicates
// TO DO: errors cannot return false. We haven't confirmed the values are unique.
// ACTUALLY don't bother. Instead, deal with broken pools when doing the actual insert.

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



pub async fn create_pool() -> Result<MySqlPool> {
     // Load environment variables from .env file.
    // CHECK: Fails if .env file not found, not readable or invalid.
    let database_url: String = std::env::var("DATABASE_URL")?;
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

    match PasswordHash::new(&stored_hash) {
        Ok(parsed_stored_hash) => {
            let argon2: Argon2<'_> = Argon2::default();

            // returns a bool
            argon2.verify_password(
                input_password.as_bytes(),
                &parsed_stored_hash
            ).is_ok()
        },
        Err(e) => {
            eprintln!("Password hash error: {:?}", e);
            false
        }
    }
}
