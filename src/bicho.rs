use crate::bets::{Bet, BetResult, Bets};
use crate::enums::get_animal_from_number;
use crate::scraper::get_once_results;
use crate::users::Users;

pub struct Game {
    pub users: Users,
    pub bets: Bets,
    pub finished: bool,
}

// work on the main loop of the game
impl Game {
    pub fn new(test: bool) -> Game {
        if test {
            Game {
                users: Users::new(),
                bets: Bets::new(),
                finished: false,
            }
        } else {
            Game {
                users: Users::load("users.bin"),
                bets: Bets::load("bets.bin"),
                finished: false,
            }
        }
    }

    pub async fn check_lottery(
        &mut self,
        override_lottery_num: Option<i32>,
    ) -> Result<Vec<BetResult>, Box<dyn std::error::Error>> {
        let lottery_results: Result<i32, Box<dyn std::error::Error>> = match override_lottery_num {
            Some(num) => Ok(num),
            None => Ok(get_once_results().await?),
        };
        let copied_lotto_results = lottery_results?;
        match self.bets.lottery_num {
            Some(lottery_num) => {
                if !self.finished && lottery_num != copied_lotto_results {
                    let winner_animal = get_animal_from_number(copied_lotto_results);
                    let results = self.bets.get_results(winner_animal);
                    for result in &results {
                        self.users.update_amount(result.user, result.amount);
                    }
                    self.finished = true;
                    self.users.save("users.bin");
                    self.bets.save("bets.bin");
                    return Ok(results);
                } else {
                    return Ok(Vec::new());
                }
            }
            None => {
                self.bets.lottery_num = Some(copied_lotto_results);
                self.bets.save("bets.bin");
                Ok(Vec::new())
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
