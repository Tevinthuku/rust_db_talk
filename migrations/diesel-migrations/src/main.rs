use anyhow::{anyhow, Context, Ok};
use connection_pool::create_diesel_pool;
use diesel::pg::Pg;
use diesel_migrations::{
    embed_migrations, EmbeddedMigrations, HarnessWithOutput, MigrationHarness,
};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn main() -> anyhow::Result<()> {
    let pool = create_diesel_pool()?;

    let mut conn = pool.get().context("Failed to get connection")?;

    run_migrations(&mut conn)
}

fn run_migrations(connection: &mut impl MigrationHarness<Pg>) -> anyhow::Result<()> {
    let _ = HarnessWithOutput::write_to_stdout(&mut *connection)
        .run_pending_migrations(MIGRATIONS)
        .map_err(|err| anyhow!(err))?;

    Ok(())
}
