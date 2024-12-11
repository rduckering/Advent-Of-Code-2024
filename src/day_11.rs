pub mod day_11
{
    use std::collections::HashMap;
    use crate::utils::utils;
    use std::time::Instant;

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::collections::HashMap;

        #[test]
        fn test_is_even_number()
        {
            let val_1 = "1111";
            let val_2 = "213";

            assert_eq!(is_even_number_of_digits(val_1), true);
            assert_eq!(is_even_number_of_digits(val_2), false);
        }

        #[test]
        fn test_is_zero()
        {
            let val_1 = "0";
            let val_2 = "1";

            assert_eq!(is_zero (val_1), true);
            assert_eq!(is_zero (val_2), false);
        }

        #[test]
        fn test_discard_zeros()
        {
            let mut val_1 = String::from("0001");
            let mut val_2 = String::from("0200");
            let mut val_3 = String::from("0000");

            discard_leading_zeros (&mut val_1);
            discard_leading_zeros (&mut val_2);
            discard_leading_zeros (&mut val_3);

            assert_eq!(val_1, "1");
            assert_eq!(val_2, "200");
            assert_eq!(val_3, "0");
        }

        #[test]
        fn test_parse_stones_1()
        {
            // 0 1 10 99 999
            let mut val_1 = vec! [String::from ("0"), String::from ("1"), String::from ("10"), String::from ("99"), String::from ("999")];

            process_stones (&mut val_1, 1);

            let expected: Vec<String> = vec! [String::from ("1"), String::from ("2024"), String::from ("1"), String::from ("0"), String::from ("9"), String::from ("9"), String::from ("2021976")];
            assert_eq!(val_1, expected);
        }

        #[test]
        fn test_parse_stones_2()
        {
            // 125 17
            let mut val_1 = vec! [String::from ("125"), String::from ("17")];

            process_stones (&mut val_1, 1);

            // 253000 1 7
            let expected: Vec<String> = vec! [String::from ("253000"), String::from ("1"), String::from ("7")];
            assert_eq!(val_1, expected);
        }

        #[test]
        fn test_parse_stones_3()
        {
            // 125 17
            let mut val_1 = vec! [String::from ("125"), String::from ("17")];

            process_stones (&mut val_1, 2);

            // 253 0 2024 14168
            let expected: Vec<String> = vec! [String::from ("253"), String::from ("0"), String::from ("2024"), String::from ("14168")];
            assert_eq!(val_1, expected);
        }

        #[test]
        fn test_parse_stones_4()
        {
            // 125 17
            let mut val_1 = vec! [String::from ("125"), String::from ("17")];

            process_stones (&mut val_1, 3);

            // 512072 1 20 24 28676032
            let expected: Vec<String> = vec! [String::from ("512072"), String::from ("1"), String::from ("20"), String::from ("24"), String::from ("28676032")];
            assert_eq!(val_1, expected);
        }

        #[test]
        fn test_parse_stones_5()
        {
            // 125 17
            let mut val_1 = vec! [String::from ("125"), String::from ("17")];

            process_stones (&mut val_1, 6);

            // 2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2
            let expected: Vec<String> = vec! [String::from ("2097446912"), String::from ("14168"), String::from ("4048"),
                                              String::from ("2"), String::from ("0"), String::from ("2"), String::from ("4"),
                                              String::from ("40"), String::from ("48"), String::from ("2024"), String::from ("40"),
                                              String::from ("48"), String::from ("80"), String::from ("96"), String::from ("2"),
                                              String::from ("8"), String::from ("6"), String::from ("7"), String::from ("6"),
                                              String::from ("0"), String::from ("3"), String::from ("2")];
            assert_eq!(val_1, expected);
        }

        #[test]
        fn test_generate_new_list()
        {
            // 0 1 10 99 999
            let mut val_1 = 1;
            let mut val_2 = 2024;

            let list = get_new_list(val_1);
            let list2 = get_new_list(val_2);

            let expected = vec! [2024];
            let expected2 = vec! [20, 24];
            assert_eq! (list, expected);
            assert_eq! (list2, expected2);
        }

        #[test]
        fn test_parse_stones_v2_1()
        {
            let mut val_1 = vec! [String::from ("125"), String::from ("17")];
            let count = process_stones_v2 (&mut val_1, 1);
            assert_eq!(count, 3);
        }

        #[test]
        fn test_parse_stones_v2_2()
        {
            let mut val_1 = vec! [String::from ("125"), String::from ("17")];
            let count = process_stones_v2 (&mut val_1, 2);
            assert_eq!(count, 4);
        }

        #[test]
        fn test_parse_stones_v2_3()
        {
            let mut val_1 = vec! [String::from ("125"), String::from ("17")];
            let count = process_stones_v2 (&mut val_1, 4);
            assert_eq!(count, 9);
        }

        #[test]
        fn test_parse_stones_v2_4()
        {
            // 0 27 5409930 828979 4471 3 68524 170
            let mut val_1 = vec! [String::from ("0"), String::from ("27"), String::from ("5409930"), String::from ("828979"),
                                              String::from ("4471"), String::from ("3"),  String::from ("68524"), String::from ("170")];
            let count = process_stones_v2 (&mut val_1, 25);
            assert_eq! (count, 194482);
        }
    }

    fn is_even_number_of_digits (number: &str) -> bool {
        number.len() % 2 == 0
    }

    fn is_zero (number: &str) -> bool {
        number == "0"
    }

    fn discard_leading_zeros (number: &mut String) {
        let mut index = 0;

        loop
        {
            if number.len() == 1 {
                break;
            }

            if number[index..index + 1] == *"0" {
                number.remove (index);
                continue;
            }
            else
            {
                break;
            }
        }
    }

    fn process_stones (content: &mut Vec<String>, num_time_to_process: i32) {

        let mut holding : Vec<Vec<String>> = Vec::new();

        for loop_cnt in 0..num_time_to_process {
            let mut index = 0;
            loop // i in 0..content.len()
            {
                let word: &str = &content[index];

                if is_zero (word)
                {
                    content[index] = String::from ("1");
                }
                else if is_even_number_of_digits (word)
                {
                    let split_num = word.len() / 2;
                    let mut word_to_split = String::from (word);
                    let split_word = word_to_split.split_at_mut (split_num);
                    content[index] = String::from (split_word.0);

                    let mut s_part2 = String::from (split_word.1);
                    discard_leading_zeros (&mut s_part2);
                    content.insert (index + 1, s_part2);
                    index += 1;
                }
                else
                {
                    let mut number = word.parse::<u64>().unwrap();
                    number *= 2024;
                    content[index] = String::from (&number.to_string());
                }

                index += 1;

                if index >= content.len() { break; }
            }
            _print_vec (content);
            println!("blinks: {}", loop_cnt);
        }
    }

    //0->1->2024->20,24->2,0,2,4

    fn get_new_list (value :u64) -> Vec<u64> {
        let mut vec = Vec::new();
        let mut value_string = value.to_string();

        if value == 0
        {
            vec.push (1);
        }
        else if is_even_number_of_digits (&value_string)
        {
            let split_num = value_string.len() / 2;
            let split_word = value_string.split_at_mut (split_num);
            vec.push (split_word.0.parse::<u64>().unwrap());

            let mut s_part2 = String::from (split_word.1);
            discard_leading_zeros (&mut s_part2);
            vec.push (s_part2.parse::<u64>().unwrap());
        }
        else
        {
            vec.push (value * 2024);
        }

        vec
    }

    fn recursive_count (value: u64, remaining_blinks: u64, map: &mut HashMap<u64, HashMap<u64,u64>>) -> u64 {

        // check map for key
        if map.contains_key (&value) {
            if let Some(v) = map.get (&value) {
                if v.contains_key (&remaining_blinks) {
                    if let Some(result) = v.get (&remaining_blinks) {
                        return *result;
                    }
                }
            }
        }

        if remaining_blinks == 0 {
            return 1;
        }

        let vec = get_new_list (value);
        let mut count = 0;

        for i in vec {
            count += recursive_count (i, remaining_blinks - 1, map);
        }

        if ! map.contains_key (&value) {
            map.insert (value, HashMap::new ());
        }

        if let Some (v) = map.get_mut (&value) {
            if ! v.contains_key (&remaining_blinks) {
                v.insert (remaining_blinks, count);
            }
        }

        count
    }

    fn process_stones_v2 (content: &mut Vec<String>, num_time_to_process: u64) -> u64 {

       // recursive version
        let mut count = 0;
        let mut map : HashMap<u64,HashMap<u64,u64>> = HashMap::new();

        for i in content {
            println! ("starting char {}", i);
            let number = i.parse::<u64>().unwrap();
            count += recursive_count (number, num_time_to_process, &mut map);
        }

        count
    }

    fn _print_vec (vec: &Vec<String>) {
        for char in vec {
            print!("{} ", char);
        }

        println!();
        println!();
    }

    fn process_content (content: &str) -> Vec<String> {

        let mut vec = Vec::new();

        for word in content.split_whitespace() {
            vec.push (word.to_string());
        }

        vec
    }


    pub fn do_task()
    {
        let start = Instant::now();

        let contents = utils::read_file("/Users/reubenduckering/Documents/Personal Repo/Advent-Of-Code-2024/files/day_11.txt");
        // let contents = String::from("0");
        let mut vec = process_content (&contents);

        let count = process_stones_v2 (&mut vec, 75);

        let duration = start.elapsed();
        println!("Time elapsed: {:?}", duration);
        println! ("count {}", count);
    }
}