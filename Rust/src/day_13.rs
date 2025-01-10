pub mod day_13
{
    use std::collections::HashMap;
    use crate::utils::utils;
    use regex::Regex;
    use nalgebra::{Matrix2, Vector2};

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn part_1_test(){
            let contents = utils::read_file("/Users/reubenduckering/Documents/Personal Repo/Advent-Of-Code-2024/files/day_13_mini.txt");

            let games = parse_input (&contents);
            let cost_to_win_all = process_game (&games);

            assert_eq!(cost_to_win_all, 480);
        }

        // #[test]
        // fn part_2_test(){
        //     let contents = utils::read_file("/Users/reubenduckering/Documents/Personal Repo/Advent-Of-Code-2024/files/day_13.txt");
        //
        //     let games = parse_input (&contents);
        //     let cost_to_win_all = process_game (&games);
        //
        //     assert_eq!(cost_to_win_all, 26005);
        // }

        #[test]
        fn part_2_test(){
            let games = vec! [Game {button_a: (37, 17), button_b: (24, 58), prize: (5634, 7098)}];
            let cost_to_win_all = process_game_v2 (&games);

            assert_eq!(cost_to_win_all, 366);
        }
    }

    #[derive(PartialEq, Eq, Clone, Debug)]
    struct Game {
        button_a: (u64, u64),
        button_b: (u64, u64),
        prize: (u64, u64),
    }

    fn fill_cache_for_value (direction: (u64,u64), times_min: u64, time_max: u64) -> HashMap<u64, (u64,u64)> {
        let mut cache: HashMap<u64, (u64, u64)> = HashMap::new();

        for i in times_min..time_max {
            let x = direction.0 * i;
            let y = direction.1 * i;
            cache.insert(i, (x, y));
        }

        cache
    }

    fn parse_coordinates(line: &str) -> Option<(u64, u64)> {
        let re = Regex::new(r"X[=+]?(-?\d+), Y[=+]?(-?\d+)").unwrap();
        if let Some(captures) = re.captures(line) {
            let x: u64 = captures.get(1)?.as_str().parse().ok()?;
            let y: u64 = captures.get(2)?.as_str().parse().ok()?;
            return Some((x, y));
        }
        None
    }

    fn parse_input(s: &str) -> Vec<Game> {
        let mut games: Vec<Game> = Vec::new();

        let mut temp_game = Game { button_a: (0,0), button_b: (0,0), prize: (0,0) };

        for line in s.split("\n") {

            if line.contains("Button A")
            {
                if let Some(values) = parse_coordinates (line) {
                    temp_game.button_a = values.clone();
                } else {panic! ("help")}
            }
            else if line.contains("Button B")
            {
                if let Some(values) = parse_coordinates (line) {
                    temp_game.button_b = values.clone();
                } else {panic! ("help")}
            }
            else if line.contains("Prize")
            {
                if let Some(values) = parse_coordinates (line) {
                    let p_v = values.clone();
                    // p_v.0 += 10000000000000;
                    // p_v.1 += 10000000000000;

                    temp_game.prize = p_v;
                    games.push (temp_game.clone());
                } else {panic! ("help")}
            }
        }

        games
    }

    fn cost_for_combination (moves: &(u64, u64)) -> u64 {
        let a_cost = 3;
        let b_cost = 1;

        let mut overall_cost: u64 = 0;

        overall_cost += moves.0 * a_cost;
        overall_cost += moves.1 * b_cost;

        overall_cost
    }

    fn do_algebra (game: &Game) -> (f64, f64) {

        let a = game.button_a.0 as f64;
        let b = game.button_b.0 as f64;
        let c = game.button_a.1 as f64;
        let d = game.button_b.1 as f64;

        let p_x = game.prize.0 as f64;
        let p_y = game.prize.1 as f64;

        let a = Matrix2::new(a, b, c, d);

        // Define the vector b
        let b = Vector2::new(p_x, p_y);

        // Solve the system A * x = b
        let x = a.try_inverse()
            .expect("Matrix is not invertible")
            * b;

        (x[0], x[1])
    }

    fn process_game(games: &Vec<Game>) -> u64 {
        let mut cost_to_win_all = 0;

        for game in games {
            let button_a_cache = fill_cache_for_value (game.button_a, 0, 100);
            let button_b_cache = fill_cache_for_value (game.button_b, 0, 100);

            let mut combinations: Vec<(u64,u64)> = Vec::new();

            for i_a in 0..100 {
                let a = button_a_cache.get(&i_a).unwrap();

                for i_b in 0..100 {
                    let b = button_b_cache.get(&i_b).unwrap();

                    let result = (a.0 + b.0, a.1 + b.1);

                    if result == game.prize {
                        combinations.push((i_a, i_b));
                    }
                }
            }

            // process combinations

            if combinations.len() == 0 {
                continue;
            }
            else if combinations.len() == 1 {
                println! ("game has a result ({}, {}) ,({}, {}), ({}, {})", game.button_a.0, game.button_a.1, game.button_b.0, game.button_b.1, game.prize.0, game.prize.1);
                cost_to_win_all += cost_for_combination (&combinations[0]);
            }
            else {
                // never triggered
                let mut lowest_cost_to_win: u64 = 18446744073709551615;

                for combination in combinations.iter() {
                    let cost_to_win = cost_for_combination (combination);

                    if cost_to_win < lowest_cost_to_win {
                        lowest_cost_to_win = cost_to_win;
                    }
                }

                cost_to_win_all += lowest_cost_to_win;
            }
        }
        print!("stop");
        cost_to_win_all
    }

    fn process_game_v2 (games: &Vec<Game>) -> u64{
        let mut cost_to_win_all = 0;

        for game in games {

            let answer = do_algebra (&game);

            let converted_answer = (answer.0.round() as u64, answer.1.round() as u64);

            let v1 = (converted_answer.0 * game.button_a.0) + (converted_answer.1 * game.button_b.0);
            let v2 =  (converted_answer.0 * game.button_a.1) + (converted_answer.1 * game.button_b.1);

            if v1 == game.prize.0 && v2 == game.prize.1 {
                cost_to_win_all += cost_for_combination (&converted_answer);
                println! ("game has a result ({}, {}) ,({}, {}), ({}, {})", game.button_a.0, game.button_a.1, game.button_b.0, game.button_b.1, game.prize.0, game.prize.1);
            }
        }

        cost_to_win_all
    }

    pub fn do_task()
    {
        // let contents = utils::read_file ("/Users/reubenduckering/Documents/Personal Repo/Advent-Of-Code-2024/files/day_13_mini.txt");
        let contents = utils::read_file ("/Users/reubenduckering/Documents/Personal Repo/Advent-Of-Code-2024/files/day_13.txt");

        let games = parse_input (&contents);
        let cost_to_win_all = process_game_v2 (&games);
        println! ("cost to win all {}", cost_to_win_all);
    }
}