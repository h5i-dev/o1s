//! PostgreSQL storage: the shared registry service (`o1s_pgwire`) over native
//! connections. Each command runs in one transaction under its repository's
//! transaction-scoped lock; see `o1s_pgwire` for why that is serializable and
//! `docs/storage.md` for operation.
pub use o1s_pgwire::service::{Failure, unavailable};
use o1s_pgwire::{DbError, native};

pub type PgBackend = o1s_pgwire::service::Service<native::Tcp, native::Tokio>;

/// Connect with tables in PostgreSQL schema `schema`, creating them, with at
/// most `connections` connections. `O1S_DATABASE_ATTEMPTS` bounds retries.
pub async fn connect(url: &str, schema: &str, connections: usize) -> Result<PgBackend, DbError> {
    let mut engine = native::connect(url, schema, connections).await?;
    if let Some(attempts) = std::env::var("O1S_DATABASE_ATTEMPTS")
        .ok()
        .and_then(|a| a.parse().ok())
    {
        engine.max_attempts = attempts;
    }
    // Commands waiting for one repository share its next transaction.
    engine.group_commit = std::env::var("O1S_GROUP_COMMIT").map_or(true, |v| v != "0");
    // Statements stay prepared on this server's own connections; set
    // O1S_PREPARED=0 behind a transaction-mode pooler.
    engine.prepared = std::env::var("O1S_PREPARED").map_or(true, |v| v != "0");
    Ok(o1s_pgwire::service::Service::new(engine))
}
