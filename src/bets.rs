use crate::enums::Animal;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Read;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct Bet {
    pub amount: f64,
    pub animal: Animal,
    pub user: i64,
}

pub struct BetResult {
    pub amount: f64,
    pub user: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Bets {
    pub bets: Vec<Bet>,
    pub lottery_num: Option<i32>,
}

impl Bets {
    pub fn new() -> Bets {
        Bets {
            bets: Vec::new(),
            lottery_num: None,
        }
    }

    pub fn add_bet(&mut self, bet: Bet) {
        self.bets.push(bet);
    }

    pub fn get_total_amount(&self) -> f64 {
        let mut total = 0.0;
        for bet in &self.bets {
            total += bet.amount;
        }
        total
    }

    pub fn get_all_animals(&self) -> Vec<Animal> {
        let mut animals = Vec::new();
        for bet in &self.bets {
            animals.push(bet.animal);
        }
        animals
    }

    pub fn get_amount_by_animal(&self, animal: Animal) -> f64 {
        let mut total = 0.0;
        for bet in &self.bets {
            if bet.animal == animal {
                total += bet.amount;
            }
        }
        total
    }

    pub fn get_animals_percentage(&self) -> HashMap<Animal, f64> {
        let mut animals = HashMap::new();
        for animal in self.get_all_animals() {
            let amount = self.get_amount_by_animal(animal);
            animals.insert(animal, self.get_total_amount() / amount);
        }
        animals
    }

    pub fn get_amount_by_user_and_animal(&self, user: i64, animal: Animal) -> f64 {
        let mut total = 0.0;
        for bet in &self.bets {
            if bet.user == user && bet.animal == animal {
                total += bet.amount;
            }
        }
        total
    }

    pub fn get_all_users(&self) -> Vec<i64> {
        let mut users = Vec::new();
        for bet in &self.bets {
            if !users.contains(&bet.user) {
                users.push(bet.user);
            }
        }
        users
    }

    pub fn get_results(&self, result: Animal) -> Vec<BetResult> {
        let mut results = Vec::new();
        let total = self.get_total_amount();
        let users = self.get_all_users();
        let animal_amount = self.get_amount_by_animal(result);

        for user in users {
            let bet_user_amount = self.get_amount_by_user_and_animal(user, result);
            let user_won = (total / animal_amount) * bet_user_amount;
            results.push(BetResult {
                user,
                amount: user_won,
            });
        }
        results
    }

    // load from disc using bincode
    pub fn load(path: &str) -> Bets {
        // if no file exists, create a new one
        let mut file = match std::fs::File::open(path) {
            Ok(file) => file,
            Err(_) => {
                let bets = Bets::new();
                bets.save(path);
                return bets;
            }
        };
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let bets = bincode::deserialize(&buffer).unwrap();
        bets
    }

    pub fn save(&self, path: &str) {
        let mut file = std::fs::File::create(path).unwrap();
        let buffer = bincode::serialize(&self, bincode::Infinite).unwrap();
        file.write_all(&buffer).unwrap();
    }
}
