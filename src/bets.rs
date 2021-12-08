use crate::enums::Animal;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Read;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct Bet {
    pub amount: f64,
    pub animal: Animal,
    pub user: f64,
}

pub struct BetResult {
    pub amount: f64,
    pub user: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Bets {
    pub bets: Vec<Bet>,
    pub day: usize,
}

#[derive(Serialize, Deserialize)]
pub struct AllBets {
    pub bets: HashMap<usize, Bets>,
}

impl Bets {
    pub fn new() -> Bets {
        Bets {
            bets: Vec::new(),
            day: get_day_id(),
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

    pub fn get_amount_by_user_and_animal(&self, user: f64, animal: Animal) -> f64 {
        let mut total = 0.0;
        for bet in &self.bets {
            if bet.user == user && bet.animal == animal {
                total += bet.amount;
            }
        }
        total
    }

    pub fn get_all_users(&self) -> Vec<f64> {
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
                amount: user_won,
                user,
            });
        }
        results
    }
}

// returns a unique  id for each day
pub fn get_day_id() -> usize {
    let now = chrono::Utc::now();
    let day = now.date().format("%Y%m%d").to_string();
    day.parse::<usize>().unwrap()
}

impl AllBets {
    // save struct to file using bincode
    pub fn save(&self, path: &str) {
        let encoded = bincode::serialize(&self, bincode::Infinite).unwrap();
        let mut file = std::fs::File::create(path).unwrap();
        file.write_all(&encoded).unwrap();
    }

    // load struct from file using bincode
    pub fn load(path: &str) -> AllBets {
        let mut file = std::fs::File::open(path).unwrap();
        let mut encoded = Vec::new();
        file.read_to_end(&mut encoded).unwrap();
        let bets = bincode::deserialize(&encoded).unwrap();
        bets
    }

    // get a reference for active bets
    pub fn get_active_bets(&self) -> &Bets {
        &self.bets.get(&get_day_id()).unwrap()
    }

    // add a bet for the current day
    pub fn add_bet(&mut self, bet: Bet) {
        let day = get_day_id();
        if self.bets.contains_key(&day) {
            self.bets.get_mut(&day).unwrap().add_bet(bet);
        } else {
            let mut bets = Bets::new();
            bets.add_bet(bet);
            self.bets.insert(day, bets);
        }
    }
}
