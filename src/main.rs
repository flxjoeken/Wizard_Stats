use rusqlite::{Connection, Result};
use rusqlite::Params;


fn main() -> Result<()> {
    let conn = Connection::open("wiz.db")?;

    conn.execute(
        "
        create table if not exists wiz_player(
            player_id integer primary key,
            name text non null unique
        )",
        ());

    conn.execute(
        "create table if not exists wiz_game (
            game_id integer primary key,

        )", ());

    Ok(())
}
