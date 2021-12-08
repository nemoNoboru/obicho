use crate::bets::{AllBets, Bet};
use crate::users::Users;

pub struct Game {
    pub users: Users,
    pub bets: AllBets,
}

// work on the main loop of the game
impl Game {
    pub fn new() -> Game {
        Game {
            users: Users::load_users("users.bin"),
            bets: AllBets::load("bets.bin"),
        }
    }

    pub fn add_bet(&mut self, user: i32, bet: Bet) {
        self.bets.get_active_bets().bets.push(bet);
    }
}
