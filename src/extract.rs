use crate::bets::Bet;
use crate::enums::get_animal_from_number;
use crate::enums::Animal;
use std::collections::HashMap;

pub fn get_bet_from_text(text: String, user: i64) -> Option<Bet> {
    let mut words = text.split_whitespace();
    let amount = words.next()?.parse::<f64>().ok()?;
    let animal = words.next()?.parse::<Animal>().ok()?;
    Some(Bet {
        amount,
        animal,
        user,
    })
}

lazy_static! {
    static ref ANIMALS_HASH: HashMap<Animal, Vec<i32>> = {
        let mut hash = HashMap::new();
        for i in 0..100 {
            let animal = get_animal_from_number(i);
            let vec = hash.entry(animal).or_insert(vec![]);
            vec.push(i);
        }
        hash
    };
}

pub fn display_numbers_from_animals() -> String {
    ANIMALS_HASH.keys().fold(String::new(), |mut acc, animal| {
        acc.push_str(&format!(
            "{:?}:{:?}\n",
            animal,
            ANIMALS_HASH.get(animal).unwrap()
        ));
        acc
    })
}

pub fn display_numbers_from_animal(animal: Animal) -> String {
    ANIMALS_HASH
        .get(&animal)
        .ok_or("Animal not found")
        .unwrap()
        .iter()
        .fold(String::new(), |mut acc, &number| {
            acc = acc + &format!("{:?},", number);
            acc
        })
}
