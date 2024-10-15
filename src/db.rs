use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        let db_host = env::var("DB_HOST").expect("DB_HOST must be set");
        let db_name = env::var("DB_NAME").expect("DB_NAME must be set");
        let db_user = env::var("DB_USER").expect("DB_USER must be set");
        let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
        format!("postgres://{}:{}@{}/{}", db_user, db_password, db_host, db_name)
    });

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
