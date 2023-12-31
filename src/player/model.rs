use diesel::prelude::*;
use serde::Serialize;
use serde::Deserialize;

use crate::error_handler::CustomError;
use crate::database::establish_connection;

use crate::schema::players::dsl::*;


#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::players)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Player {
    pub player_id: i32,
    pub username: String,
    pub email: Option<String>,
    pub password: Option<String>
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::players)]
pub struct NoIdPlayer {
    pub username: String,
    pub email: Option<String>,
    pub password: Option<String>
}

#[derive(Serialize, Queryable)]
#[diesel(table_name = crate::schema::players)]
pub struct PlayerSafe {
    pub player_id: i32,
    pub username: String,
    pub email: Option<String>,
}

impl Player {
    pub fn find_all() -> Result<Vec<PlayerSafe>, CustomError> {
        let conn = &mut establish_connection();
        let result = 
        players.select((player_id, username, email))
        .load::<PlayerSafe>(conn).expect("Failed to load all players");
        Ok(result)
    }

    pub fn find(id: i32) -> Result<PlayerSafe, CustomError> {
        let conn = &mut establish_connection();
        let player = players.find(id).first::<Player>(conn).expect("Failed to find player with the given ID");

        let safe_player = PlayerSafe {
            player_id: player.player_id,
            username: player.username,
            email: player.email,
        };

        Ok(safe_player)
    }

    pub fn create(player: NoIdPlayer) {
        let conn = &mut establish_connection();
        diesel::insert_into(players).values(&player).execute(conn).expect("Failed to create new player");
    }

    pub fn update(id: i32, player: NoIdPlayer) {
        let conn = &mut establish_connection();

        let _ = diesel::update(players)
            .filter(player_id.eq(id))
            .set(player)
            .execute(conn).expect("Failed to update player with the specified ID");
    }

    pub fn delete(id: i32) {
        let conn = &mut establish_connection();
        let _ = diesel::delete(players.filter(player_id.eq(id))).execute(conn).expect("Failed to delete player with the specified ID");
    }

    }