use crate::bets::{Bet, Bets};
use crate::enums::get_animal_from_number;
use crate::scraper::get_once_results;
use crate::users::Users;

pub struct Game {
    pub users: Users,
    pub bets: Bets,
    finished: bool,
}

// work on the main loop of the game
impl Game {
    pub fn new() -> Game {
        Game {
            users: Users::load("users.bin"),
            bets: Bets::load("bets.bin"),
            finished: false,
        }
    }

    pub async fn check_lottery(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let lottery_results = get_once_results().await;
        match self.bets.lottery_num {
            Some(lottery_num) => {
                if !self.finished && lottery_num != lottery_results? {
                    let winner_animal = get_animal_from_number(lottery_num);
                    let results = self.bets.get_results(winner_animal);
                    for result in results {
                        self.users.update_amount(result.user, result.amount);
                    }
                    self.finished = true;
                    self.users.save("users.bin");
                    self.bets.save("bets.bin");
                }
                Ok(())
            }
            None => {
                self.bets.lottery_num = Some(lottery_results?);
                self.bets.save("bets.bin");
                Ok(())
            }
        }
    }

    pub fn add_bet(&mut self, bet: Bet) {
        self.users.update_amount(bet.user, -bet.amount);
        self.bets.add_bet(bet);

        // save bets and users
        self.bets.save("bets.bin");
        self.users.save("users.bin");
    }
}
