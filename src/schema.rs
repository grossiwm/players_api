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

diesel::table! {
    wallets (wallet_id) {
        wallet_id -> Integer,
        player_id -> Integer,
        #[max_length = 255]
        public_key -> Varchar,
        #[max_length = 255]
        private_key -> Varchar,
    }
}

diesel::joinable!(wallets -> players (player_id));

diesel::allow_tables_to_appear_in_same_query!(
    players,
    wallets,
);
