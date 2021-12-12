use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Animal {
    Corno,
    Dellette,
    Vagabundo,
    Gado,
    Vand,
    COVID,
    Viado,
    Jorel,
    Macaco,
    Traveco,
    TioTesla,
    Pinguim,
    AnaoX,
    BebeCabeludo,
    Porco,
    Leao,
    Cachorro,
    MGTOW,
    Cabra,
    Billy,
    Kibador,
    Dolarhyde,
    Pavao,
    Peru,
    Travequeiro,
    Bundeiro,
    NotFound,
}

pub fn get_animal_from_number(raw_number: i32) -> Animal {
    let number = raw_number % 25;
    match number {
        0 => Animal::Corno,
        1 => Animal::Dellette,
        2 => Animal::Vagabundo,
        3 => Animal::Gado,
        4 => Animal::Vand,
        5 => Animal::COVID,
        6 => Animal::Viado,
        7 => Animal::Jorel,
        8 => Animal::Macaco,
        9 => Animal::Traveco,
        10 => Animal::TioTesla,
        11 => Animal::Pinguim,
        12 => Animal::AnaoX,
        13 => Animal::BebeCabeludo,
        14 => Animal::Porco,
        15 => Animal::Leao,
        16 => Animal::Cachorro,
        17 => Animal::MGTOW,
        18 => Animal::Cabra,
        19 => Animal::Billy,
        20 => Animal::Kibador,
        21 => Animal::Dolarhyde,
        22 => Animal::Pavao,
        23 => Animal::Peru,
        24 => Animal::Travequeiro,
        25 => Animal::Bundeiro,
        _ => Animal::NotFound,
    }
}

impl FromStr for Animal {
    type Err = ();

    fn from_str(animal: &str) -> Result<Animal, Self::Err> {
        match get_animal_from_text(animal) {
            Animal::NotFound => Err(()),
            animal => Ok(animal),
        }
    }
}

pub fn get_animal_from_text(text: &str) -> Animal {
    let text = text.to_lowercase();
    match text.as_str() {
        "corno" => Animal::Corno,
        "dellette" => Animal::Dellette,
        "vagabundo" => Animal::Vagabundo,
        "gado" => Animal::Gado,
        "vand" => Animal::Vand,
        "covid" => Animal::COVID,
        "viado" => Animal::Viado,
        "jorel" => Animal::Jorel,
        "macaco" => Animal::Macaco,
        "traveco" => Animal::Traveco,
        "tiotele" => Animal::TioTesla,
        "pinguim" => Animal::Pinguim,
        "anaox" => Animal::AnaoX,
        "bebecabeludo" => Animal::BebeCabeludo,
        "porco" => Animal::Porco,
        "leao" => Animal::Leao,
        "cachorro" => Animal::Cachorro,
        "mgtow" => Animal::MGTOW,
        "cabra" => Animal::Cabra,
        "billy" => Animal::Billy,
        "kibador" => Animal::Kibador,
        "dolarhyde" => Animal::Dolarhyde,
        "pavao" => Animal::Pavao,
        "peru" => Animal::Peru,
        "travequeiro" => Animal::Travequeiro,
        "bundeiro" => Animal::Bundeiro,
        _ => Animal::NotFound,
    }
}
