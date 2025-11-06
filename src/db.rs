mod db;

// I'm actually using MariaDB which is supposedly a drop-in replacement for MySQL

use sqlx::{MySqlPool, Pool, mysql::MySqlPoolOptions, Connection, MySqlConnection};


