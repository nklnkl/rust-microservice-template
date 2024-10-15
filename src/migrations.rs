use diesel::pg::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::{info, error};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn run_migrations(connection: &mut PgConnection) {
    println!("Running migrations");
    match connection.run_pending_migrations(MIGRATIONS) {
        Ok(applied_migrations) => {
            if applied_migrations.is_empty() {
                println!("No new migrations to apply.");
            } else {
                println!("Successfully applied {} migrations:", applied_migrations.len());
                for migration in applied_migrations {
                    println!("  - {}", migration);
                }
            }
        },
        Err(e) => {
            error!("Failed to run migrations: {}", e);
            panic!("Error running migrations: {}", e);
        }
    }
}
