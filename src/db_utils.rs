use rusqlite::{Connection, Result};

pub fn create_connection() -> Result<Connection> {
    Connection::open("wiz.db")
}

pub fn create_tables(connection: &Connection) {
    match connection.execute_batch(
        "
            BEGIN TRANSACTION;
            create table wiz_player
                    (
                        id         INTEGER primary key AUTOINCREMENT,
                        name       varchar(255) not null,
                        email      varchar(255) not null,
                        password   varchar(255) not null,
                        created_at timestamp default current_timestamp
                    );

            create table wiz_game
                (
                    id         INTEGER primary key AUTOINCREMENT,
                    name       varchar(255) not null,
                    player_1   Integer      not null,
                    player_2   Integer      not null,
                    player_3   Integer      not null,
                    player_4   Integer,
                    player_5   Integer,
                    created_at timestamp default current_timestamp,
                    foreign key (player_1) references wiz_player (id),
                    foreign key (player_2) references wiz_player (id),
                    foreign key (player_3) references wiz_player (id),
                    foreign key (player_4) references wiz_player (id),
                    foreign key (player_5) references wiz_player (id)
                );

            create table wiz_round
                (
                    id             Integer primary key AUTOINCREMENT,
                    game_id        Integer not null,
                    score_player_1 Integer not null,
                    score_player_2 Integer not null,
                    score_player_3 Integer not null,
                    score_player_4 Integer,
                    score_player_5 Integer,
                    created_at     timestamp default current_timestamp,
                    foreign key (game_id) references wiz_game (id)
                );
                END TRANSACTION;
                ", ) {
        Ok(_) => {
            println!("Table created successfully");
        }
        Err(_) => {
            println!("An error occurred creating the table");
        }
    }
}