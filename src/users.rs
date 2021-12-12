use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Write;

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
pub struct User {
    pub user: i64,
    pub amount: f64,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
pub struct Users {
    pub users: HashMap<i64, User>,
}

impl Users {
    pub fn new() -> Users {
        Users {
            users: HashMap::new(),
        }
    }
    // load users from file using bincode
    pub fn load(file: &str) -> Users {
        let mut users = Users {
            users: HashMap::new(),
        };
        // check if file exists, create it if not
        if !std::path::Path::new(file).exists() {
            let mut file = File::create(file).unwrap();
            file.write_all(b"{}").unwrap();
        }
        let mut file = File::open(file).unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();
        users = bincode::deserialize(&data).unwrap();
        users
    }

    // save users to file using bincode
    pub fn save(&self, file: &str) {
        let mut f = File::create(file).expect("file not found");
        let encoded: Vec<u8> = bincode::serialize(&self.users, bincode::Infinite)
            .expect("something went wrong encoding the file");
        f.write_all(&encoded)
            .expect("something went wrong writing the file");
    }

    // add user to users
    pub fn add_user(&mut self, user: i64) {
        if self.users.contains_key(&user) {
            return;
        }
        self.users.insert(
            user,
            User {
                user,
                amount: 100.0,
            },
        );
    }

    // get user from users
    pub fn get_user(&self, user: i64) -> Option<&User> {
        self.users.get(&user)
    }

    // update amount of user
    pub fn update_amount(&mut self, user: i64, amount: f64) {
        if !self.users.contains_key(&user) {
            self.add_user(user)
        }
        // check if user has enough amount
        if self.users.get(&user).unwrap().amount + amount < 0.0 {
            return;
        }
        self.users.get_mut(&user).unwrap().amount += amount;
    }
}
