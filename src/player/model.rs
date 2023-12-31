use diesel::prelude::*;
use serde::Serialize;
use serde::Deserialize;

use crate::error_handler::CustomError;
use crate::database::establish_connection;

use crate::schema::players::dsl::*;


#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::players)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Player {
    #[serde(skip_deserializing)]
    pub player_id: i32,
    pub username: String,
    pub email: Option<String>,
    pub password: Option<String>
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::players)]
pub struct NoIdPlayer {
    pub username: String,
    pub email: Option<String>,
    pub password: Option<String>
}

impl Player {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = &mut establish_connection();
        let result = players.load::<Player>(conn).expect("error");
        Ok(result)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = &mut establish_connection();
        let player = players.find(id).first(conn).expect("error");
        Ok(player)
    }

    pub fn create(player: NoIdPlayer) {
        let conn = &mut establish_connection();
        diesel::insert_into(players).values(&player).execute(conn).expect("msg");
    }

    pub fn update(id: i32, player: NoIdPlayer) {
        let conn = &mut establish_connection();

        let _ = diesel::update(players)
            .filter(player_id.eq(id))
            .set(player)
            .execute(conn).expect("msg");
    }

    pub fn delete(id: i32) {
        let conn = &mut establish_connection();
        let _ = diesel::delete(players.filter(player_id.eq(id))).execute(conn);
    }

    }