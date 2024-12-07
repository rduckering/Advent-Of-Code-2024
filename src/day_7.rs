pub mod day_7
{
    use crate::utils::utils;

    #[cfg(test)]
    mod tests {
        use super::*;
        // 190: 10 19
        // 3267: 81 40 27
        // 83: 17 5
        // 156: 15 6
        // 7290: 6 8 6 15
        // 161011: 16 10 13
        // 192: 17 8 14
        // 21037: 9 7 18 13
        // 292: 11 6 16 20
        #[test]
        fn test_valid_values() {
            assert_eq!(is_valid_input (190, vec ! [10, 19]), true);
            assert_eq!(is_valid_input (3267, vec ! [81, 40, 27]), true);
            assert_eq!(is_valid_input (83, vec ! [17, 5]), false);
            assert_eq!(is_valid_input (156, vec ! [15, 6]), false);
            assert_eq!(is_valid_input (7290, vec ! [6, 8, 6, 15]), false);
            assert_eq!(is_valid_input (292, vec ! [11, 6, 16, 20]), true);
        }

        #[test]
        fn test_run_test_groups_valid() {
            let mut vec: Vec<TestGroup> = Vec::new();

            vec.push (make_test_group (190, vec ! [10, 19]));
            vec.push (make_test_group (3267, vec ! [81, 40, 27]));
            vec.push (make_test_group (83, vec ! [17, 5]));
            vec.push (make_test_group (156, vec ! [15, 6]));
            vec.push (make_test_group (7290, vec ! [6, 8, 6, 15]));
            vec.push (make_test_group (292, vec ! [11, 6, 16, 20]));

            let sum =  run_test_groups_valid (&vec);
            assert_eq! (sum, 3749);
        }

        #[test]
        fn test_is_valid_input_complex() {
            assert_eq!(is_valid_input_complex (190, vec ! [10, 19]), true);
            assert_eq!(is_valid_input_complex (3267, vec ! [81, 40, 27]), true);
            assert_eq!(is_valid_input_complex (83, vec ! [17, 5]), false);
            assert_eq!(is_valid_input_complex (156, vec ! [15, 6]), true);
            assert_eq!(is_valid_input_complex (7290, vec ! [6, 8, 6, 15]), true);
            assert_eq!(is_valid_input_complex (292, vec ! [11, 6, 16, 20]), true);
            assert_eq!(is_valid_input_complex (192, vec ! [17, 8, 14]), true);
        }

        #[test]
        fn test_run_test_groups_valid_complex() {
            let mut vec: Vec<TestGroup> = Vec::new();

            vec.push (make_test_group (190, vec ! [10, 19]));
            vec.push (make_test_group (3267, vec ! [81, 40, 27]));
            vec.push (make_test_group (83, vec ! [17, 5]));
            vec.push (make_test_group (156, vec ! [15, 6]));
            vec.push (make_test_group (7290, vec ! [6, 8, 6, 15]));
            vec.push (make_test_group (292, vec ! [11, 6, 16, 20]));
            vec.push (make_test_group (192, vec ! [17, 8, 14]));

            let sum =  run_test_groups_valid_complex (&vec);
            assert_eq! (sum, 11387);
        }

        #[test]
        fn test_concatenate () {
            assert_eq! (concatenate (11,22), 1122);
            assert_eq! (concatenate (809,100), 809100);
        }
    }

    pub struct TestGroup {
        number_to_achieve: i64,
        test_numbers: Vec<i64>
    }

    fn make_test_group(key: i64, value: Vec<i64>) -> TestGroup
    {
        TestGroup {
            number_to_achieve: key,
            test_numbers: value
        }
    }

    fn parse_values (contents: &str) -> Vec<TestGroup> {

        let mut result: Vec<TestGroup> = Vec::new();

        for line in contents.lines() {
            let mut parts = line.split(':');

            // Parse the first part as the leading number
            let key: i64 = parts
                .next()
                .unwrap()
                .trim()
                .parse()
                .expect("Failed to parse the key");

            // Parse the numbers after the colon
            let values: Vec<i64> = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse().expect("Failed to parse a number"))
                .collect();

            result.push(make_test_group (key, values));
        }

        result
    }

    fn max_number(bits: u32) -> u64 {
        (1_u64 << bits) - 1
    }

    fn is_valid_input (key: i64, values: &Vec<i64>) -> bool {

        let number_of_operations = values.len() - 1;

        let max_count = max_number (number_of_operations as u32);

        for count in 0..max_count + 1
        {
            let mut sum : i64 = values[0] as i64;

            for v in 1..values.len() {
                if (count >> v - 1) & 1 == 0 {
                    sum *= values[v] as i64;
                }

                if (count >> v - 1) & 1 == 1 {
                    sum += values[v] as i64;
                }
            }

            if sum == key {
                return true;
            }
        }

        false
    }

    fn concatenate (x: i64, y: i64) -> i64 {
    let mut pow = 10;
        while y >= pow {
            pow *= 10;
        }

        x * pow + y
    }

    fn is_valid_input_complex (key: i64, values: &Vec<i64>) -> bool {

        let number_of_operations = values.len() - 1;

        let mut max_count = max_number (number_of_operations as u32);
        let orig_cnt = max_count;
        max_count = max_number (number_of_operations as u32 * 2);

        let mut sum : i64 = 0;

        for count in 0..max_count + 1
        {
            sum = values[0];

            for v in 1..values.len() {

                if ((count >> number_of_operations) >> v - 1) & 1 == 1 {
                    sum = concatenate (sum, values[v]);
                    continue;
                }

                if ((count & orig_cnt) >> v - 1) & 1 == 0 {
                    sum *= values[v];
                    continue;
                }

                if ((count & orig_cnt) >> v - 1) & 1 == 1 {
                    sum += values[v];
                    continue;
                }
            }

            if sum == key {
                return true;
            }
        }

        false
    }

    fn run_test_groups_valid (test_groups: &Vec<TestGroup>) -> i64 {

        let mut sum = 0;
        for test_group in test_groups {
            if is_valid_input (test_group.number_to_achieve, &test_group.test_numbers) {
                sum += test_group.number_to_achieve;
            }
        }

        sum
    }

    fn run_test_groups_valid_complex (test_groups: &Vec<TestGroup>) -> i64 {

        let mut sum = 0;
        let mut count = 0;
        let max_num = test_groups.len() - 1;

        for test_group in test_groups {
            println!("test_group: {:?}/{:?} {:?}", count, max_num, test_group.number_to_achieve);
            if is_valid_input_complex (test_group.number_to_achieve, &test_group.test_numbers) {
                sum += test_group.number_to_achieve;
            }
            count += 1;
        }

        sum
    }

    pub fn do_task()
    {
        let contents = utils::read_file("/Users/reubenduckering/Documents/Personal Repo/Advent-Of-Code-2024/files/day_7.txt");
        let lines = parse_values (&contents);

        let sum = run_test_groups_valid_complex (&lines);

        println!("Sum: {}", sum);
    }
}