pub mod day_2
{
    use crate::utils::utils;

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_parse_codes() {
            let codes = "1 2 3 4\n5 6 7 8";

            let result = parse_codes (codes);
            assert_eq!(result[0], [1, 2, 3, 4]);
            assert_eq!(result[1], [5, 6, 7, 8]);
        }

        #[test]
        fn test_sequence_is_safe() {
            let codes_1 = vec![7, 6, 4, 2, 1]; // safe
            let codes_2 = vec![1, 2, 7, 8, 9]; // unsafe
            let codes_3 = vec![9, 7, 6, 2, 1]; // unsafe
            let codes_4 = vec![1, 3, 2, 4, 5]; // unsafe
            let codes_5 = vec![8, 6, 4, 4, 1]; // unsafe
            let codes_6 = vec![1, 3, 6, 7, 9]; // safe

            assert_eq!(sequence_is_safe(&codes_1), true);
            assert_eq!(sequence_is_safe(&codes_2), false);
            assert_eq!(sequence_is_safe(&codes_3), false);
            assert_eq!(sequence_is_safe(&codes_4), false);
            assert_eq!(sequence_is_safe(&codes_5), false);
            assert_eq!(sequence_is_safe(&codes_6), true);
        }

        #[test]
        fn test_complex_sequence_is_safe() {
            let codes_1 = vec![7, 6, 4, 2, 1]; // safe
            let codes_2 = vec![1, 2, 7, 8, 9]; // unsafe
            let codes_3 = vec![9, 7, 6, 2, 1]; // unsafe
            let codes_4 = vec![1, 3, 2, 4, 5]; // safe
            let codes_5 = vec![8, 6, 4, 4, 1]; // safe
            let codes_6 = vec![1, 3, 6, 7, 9]; // safe
            let codes_7 = vec![86, 85, 89, 92, 94, 97]; // safe

            assert_eq!(complex_sequence_is_safe(&codes_1), true);
            assert_eq!(complex_sequence_is_safe(&codes_2), false);
            assert_eq!(complex_sequence_is_safe(&codes_3), false);
            assert_eq!(complex_sequence_is_safe(&codes_4), true);
            assert_eq!(complex_sequence_is_safe(&codes_5), true);
            assert_eq!(complex_sequence_is_safe(&codes_6), true);
            assert_eq!(complex_sequence_is_safe(&codes_7), true);
        }
    }

    fn parse_codes (locations: &str) -> Vec<Vec<i32>>
    {
        let mut vec: Vec<Vec<i32>> = Vec::new();
        let lines = locations.lines();

        for line in lines
        {
            let mut codes: Vec<i32> = Vec::new();

            let v = line.split_terminator(" ");

            for val in v {
                codes.push (val.parse::<i32>().unwrap());
            }

            vec.push (codes);
        }

        vec
    }

    fn sequence_is_safe (sequence: &Vec<i32>) -> bool
    {
        // rules: all increasing/decreasing
        // rules: at least one at most three
        let mut prev_value = sequence[0];
        let decending: bool = if sequence[0] > sequence[1] { true } else { false };

        for i in &sequence[1..]
        {
            if decending && *i > prev_value {
                return false;
            }

            if ! decending && *i < prev_value {
                return false;
            }

            let max: i32 = if *i > prev_value { *i } else { prev_value };
            let min: i32 = if *i > prev_value { prev_value } else { *i };

            if max > min + 3 || max == min {
                return false;
            }

            prev_value = *i;
        }

        true
    }

    fn complex_sequence_is_safe (sequence: &Vec<i32>) -> bool
    {
        // rules: all increasing/decreasing
        // rules: at least one at most three
        // rules: can remove one index and be safe
        if sequence_is_safe (sequence) {
            return true;
        }

        // brute force!
        let size = sequence.len();

        for i in 0..size
        {
            let mut vec_copy = sequence.clone();
            vec_copy.remove (i);

            if sequence_is_safe(&vec_copy) {
                return true;
            }
        }

        false
    }

    fn check_codes(code: &Vec<Vec<i32>>) -> u32
    {
        let mut safe_codes: u32 = 0;

        for line in code
        {
            if sequence_is_safe(&line) {
                safe_codes += 1
            }
        }

        safe_codes
    }


    fn complex_check_codes(code: &Vec<Vec<i32>>) -> u32
    {
        let mut safe_codes: u32 = 0;

        for line in code
        {
            if complex_sequence_is_safe(&line) {
                safe_codes += 1
            }
        }

        safe_codes
    }

    pub fn do_task()
    {
        let contents = utils::read_file("/Users/reubenduckering/RustroverProjects/AdventOfCode/files/day_2.txt");
        let codes= parse_codes (&contents.as_str());
        let _safe_codes = check_codes (&codes);
        let _more_safe_codes = complex_check_codes (&codes);
        println!("Day 2 - Task 1: {}", _safe_codes);
        println!("Day 2 - Task 1: {}", _more_safe_codes);
    }
}