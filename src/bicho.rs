use crate::bets::{Bet, Bets};
use crate::users::Users;

pub struct Game {
    pub users: Users,
    pub bets: Bets,
}

// work on the main loop of the game
impl Game {
    pub fn new() -> Game {
        Game {
            users: Users::load_users("users.bin"),
            bets: Bets::load("bets.bin"),
        }
    }
}
