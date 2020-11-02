use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;

pub fn db_init() -> Result<()> {
    let conn = Connection::open("sessions.db")?;

    conn.execute(
        "create table if not exists sessions (
            id integer primary key,
            name text not null unique
        )",
        NO_PARAMS,
    )?;

    Ok(())
}