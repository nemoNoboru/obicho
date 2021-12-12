mod test {
    use crate::bets::Bet;
    use crate::bicho::Game;
    use crate::enums::Animal;

    #[tokio::test]
    async fn test_users_lose_and_one_wins() {
        let mut game = Game::new(true);
        let bet: Bet = Bet {
            amount: 10.0,
            animal: Animal::Billy,
            user: 1,
        };

        let bet2: Bet = Bet {
            amount: 20.0,
            animal: Animal::Billy,
            user: 2,
        };

        let bet3: Bet = Bet {
            amount: 10.0,
            animal: Animal::Vand,
            user: 3,
        };

        let bet4: Bet = Bet {
            amount: 20.0,
            animal: Animal::Vand,
            user: 4,
        };

        game.add_bet(bet);
        game.add_bet(bet2);
        game.add_bet(bet3);
        game.add_bet(bet4);

        // the game is marked as finished when the day changes (the lotto number changes)
        let _ = game.check_lottery(Some(23)).await;
        assert_eq!(game.finished, false);

        // Animal for 4 is "Vand"
        let results = game.check_lottery(Some(4)).await;
        assert_eq!(game.finished, true);

        // assert_eq!(&results.unwrap().len(), 2);
        for result in results {
            println!("{:?}", result);
        }

        let user_1 = game.users.get_user(1);
        let user_2 = game.users.get_user(2);
        let user_3 = game.users.get_user(3);
        let user_4 = game.users.get_user(4);
        println!("{:?}", user_1);
        println!("{:?}", user_3);
        // users start with 100.0, he lost 10.0 so now he has 90.0
        assert_eq!(user_1.unwrap().amount, 90.0);
        // users start with 100.0, he lost 20.0 so now he has 85.0
        assert_eq!(user_2.unwrap().amount, 80.0);
        // users start with 100.0 he won, so now he has 120.0
        assert_eq!(user_3.unwrap().amount, 110.0);
        // users start with 100.0 he won, so now he has 120.0
        assert_eq!(user_4.unwrap().amount, 120.0);
    }
}
