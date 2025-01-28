pub mod day_1
{
    use std::fs;
    use std::path::Path;
    use std::collections::HashMap;

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_parse_locations() {
            let locations = "1   2\n3   4\n";

            let result = parse_locations(locations);
            assert_eq!(result[0], 1);
            assert_eq!(result[1], 2);
            assert_eq!(result[2], 3);
            assert_eq!(result[3], 4);
        }

        #[test]
        fn test_find_distances()
        {
            let nums1: Vec<i32> = vec![1, 2, 3, 4];
            let nums2: Vec<i32> = vec![2, 4, 8, 12];
            let result = find_differences(&nums1, &nums2);
            assert_eq!(result[0], 1);
            assert_eq!(result[1], 2);
            assert_eq!(result[2], 5);
            assert_eq!(result[3], 8);
        }

        #[test]
        fn test_find_similarity()
        {
            let nums1: Vec<i32> = vec![1, 2, 4, 1, 2];
            let nums2: Vec<i32> = vec![1, 2, 2, 1, 1];
            let result = find_similarity(&nums1, &nums2);
            assert_eq!(result[0], 6);
            assert_eq!(result[1], 8);
            assert_eq!(result[2], 0);
        }
    }

    fn read_file(filename: &str) -> String
    {
        let file_path = Path::new(filename);

        if file_path.exists() == false {
            panic! ("File does not exist");
        }

        let contents = fs::read_to_string(file_path)
            .expect ("Should have been able to read the file");

        contents
    }

    fn parse_locations (locations: &str) -> Vec<i32>
    {
        let mut vec: Vec<i32> = Vec::new();

        let lines = locations.lines();

        for line in lines
        {
            let v = line.split_terminator ("   ");

            for val in v {
                vec.push (val.parse::<i32>().unwrap());
            }
        }

        vec
    }

    fn find_differences (location_nums_1: &Vec<i32>, location_nums_2: &Vec<i32>) -> Vec<i32>
    {
        let mut count: i32 = 0;
        let mut diffs : Vec<i32> = Vec::new();

        loop
        {
            let d = location_nums_1[count as usize] - location_nums_2[count as usize];
            diffs.push (d.abs());

            count += 1;

            if count >= location_nums_1.len() as i32 {
                break;
            }
        }

        diffs
    }

    fn find_similarity (location_nums_1: &Vec<i32>, location_nums_2: &Vec<i32>) -> Vec<u32>
    {
        let mut map = HashMap::new();

        for item in location_nums_1
        {
            let number_to_find = item;
            let mut count: u32 = 0;

            for item2 in location_nums_2
            {
                if number_to_find == item2 {
                    count += 1;
                }
            }

            if let Some (value) = map.get_mut (number_to_find)  {
                *value += count;
            }
            else {
                map.insert (number_to_find, count);
            }
        }

        let mut new_list: Vec<u32> = Vec::new();

        for item in location_nums_1
        {
            if let Some (&value) = map.get (item)
            {
                let multiply = *item as u32 * value;
                new_list.push (multiply);
            }
        }

        new_list
    }

    fn split_list (list1: &mut Vec<i32>, list2: &mut Vec<i32>, location_num_array: &Vec<i32>)
    {
        let mut count: i32 = 0;

        loop
        {
            if (count % 2) == 1 {
                list2.push (location_num_array[count as usize]);
            }
            else {
                list1.push (location_num_array[count as usize]);
            }

            count += 1;

            if count >= location_num_array.len() as i32 {
                break;
            }
        }
    }

    pub fn do_task_1()
    {
        let contents = read_file ("/Users/reubenduckering/RustroverProjects/AdventOfCode/files/day_1.txt");
        let location_num_array = parse_locations (contents.as_str());

        let mut list1: Vec<i32> = Vec::new();
        let mut list2: Vec<i32> = Vec::new();

        split_list (&mut list1, &mut list2, &location_num_array);

        list1.sort();
        list2.sort();

        // part 1
        let diffs = find_differences(&list1, &list2);
        let _sum = diffs.iter().sum::<i32>();

        // part2
        let sim = find_similarity(&list1, &list2);
        let _sim_val = sim.iter().sum::<u32>();
        println!("The similarity between list1 and list2 is: {}", sim.iter().sum::<u32>());
    }
}