use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::auth::Authenticable;
use crate::database::establish_connection;
use crate::error_handler::CustomError;

use crate::schema::players::dsl::*;

use bcrypt::{hash, verify, DEFAULT_COST};

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::players)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Player {
    pub player_id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::players)]
pub struct NoIdPlayer {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct AuthenticablePlayer {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Queryable)]
#[diesel(table_name = crate::schema::players)]
pub struct PlayerSafe {
    pub player_id: i32,
    pub username: String,
    pub email: String,
}

impl Authenticable for Player {
    fn get_username(&self) -> String {
        self.username.clone()
    }
}

impl Player {
    pub fn find_all() -> Result<Vec<PlayerSafe>, CustomError> {
        let conn = &mut establish_connection();
        let result = players
            .select((player_id, username, email))
            .load::<PlayerSafe>(conn)
            .expect("Failed to load all players");
        Ok(result)
    }

    pub fn find(id: i32) -> Result<PlayerSafe, CustomError> {
        let conn = &mut establish_connection();
        let player = players
            .find(id)
            .first::<Player>(conn)
            .expect("Failed to find player with the given ID");

        let safe_player = PlayerSafe {
            player_id: player.player_id,
            username: player.username,
            email: player.email,
        };

        Ok(safe_player)
    }

    pub fn create(mut player: NoIdPlayer) {
        let conn = &mut establish_connection();
        Self::set_password(&mut player);
        diesel::insert_into(players)
            .values(player)
            .execute(conn)
            .expect("Failed to create new player");
    }

    pub fn update(id: i32, mut player: NoIdPlayer) {
        let conn = &mut establish_connection();
        Self::set_password(&mut player);
        let _ = diesel::update(players)
            .filter(player_id.eq(id))
            .set(player)
            .execute(conn)
            .expect("Failed to update player with the specified ID");
    }

    pub fn delete(id: i32) {
        let conn = &mut establish_connection();
        let _ = diesel::delete(players.filter(player_id.eq(id)))
            .execute(conn)
            .expect("Failed to delete player with the specified ID");
    }

    pub fn authenticate(auth_player: AuthenticablePlayer) -> Result<Player, &'static str> {
        let conn = &mut establish_connection();
        let auth_player_username = auth_player.username.clone();
        let player_from_db: Player = players
            .filter(username.eq(auth_player_username))
            .first::<Player>(conn)
            .expect("Failed to find player with the given username");

        if player_from_db.username == auth_player.username
            && verify(&auth_player.password, &player_from_db.password).unwrap()
        {
            Ok(player_from_db)
        } else {
            Err("Invalid Credentials")
        }
    }

    fn set_password(player: &mut NoIdPlayer) {
        player.password = hash(player.password.clone(), DEFAULT_COST).unwrap();
    }
}
