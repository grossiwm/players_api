// @generated automatically by Diesel CLI.

diesel::table! {
    players (player_id) {
        player_id -> Integer,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 255]
        password -> Nullable<Varchar>,
    }
}
