pub mod day_8
{
    use crate::utils::utils;
    use std::collections::HashSet;

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_find_indices_with_char() {
            let vec = "abcdefga";

            let result = find_indices_with_char (&vec, 'a');
            assert_eq!(result, vec![0, 7]);
        }

        #[test]
        fn test_lines_between ()
        {
            {
                let vec = "aaaa\nbbbb\ncccc\ndddd\neeee";
                let result = lines_between(&vec, 0, vec.len() - 1);
                assert_eq!(result, 4);
            }

            {
                let vec = "aaaa\nbbbb\ncccc\ndddd\neeee";
                let result = lines_between(&vec,vec.len() - 1, 0);
                assert_eq!(result, 4);
            }
        }

        #[test]
        fn test_validate_location()
        {
            {
                let vec = "aXaa\nbXbb\ncccc";
                let result = validate_location(&vec, 1, 11, 1);
                assert_eq!(result, true);
            }

            {
                let vec = "aXaa\nbbbX\ncccc\ndddd";
                let result = validate_location(&vec, 1, 17, 1);
                assert_eq!(result, false);
            }
        }

    }

    fn find_indices_with_char (array: &str, target_char: char) -> Vec<usize> {
        array.chars()
            .enumerate()
            .filter(|&(_, c)| c == target_char)
            .map(|(i, _)| i)
            .collect()
    }

    fn lines_between (contents: &str, index_0: usize, index_1: usize) -> usize {
        let start = if index_0 < index_1 { index_0 } else { index_1 };
        let end = if index_0 < index_1 { index_1 } else { index_0 };

        contents[start..end].chars().filter(|c| *c == '\n').count()
    }

    fn validate_location (contents: &str, index_0: usize, index_1: usize, orig_line_difference: usize) -> bool {
        let new_line_difference = lines_between (&contents, index_0, index_1);

        if new_line_difference == (orig_line_difference * 2) {
            return true;
        }

        false
    }

    fn _process (contents: &mut str) -> i32 {
        let mut frequencys: HashSet<char> = HashSet::new();

        let mut content_as_chars = contents.chars().collect::<Vec<char>>();

        for char in contents.chars() {
            if char != '\n' && char != '.' {
                frequencys.insert (char);
            }
        }

        let mut valid_count = 0;

        // process each frequency differently
        for f in &frequencys {
            let f_indexs = find_indices_with_char (&contents, *f);

            // compare each frequency point with all other. check line spacing
            for f_0 in f_indexs.clone() {

                for f_index in f_indexs.clone() {
                    if f_index == f_0 {
                        continue;
                    }

                    let line_difference = lines_between (&contents, f_0, f_index);
                    let diff_space = f_index as i32 - f_0 as i32;

                    let new_location_i32 = f_0 as i32 + (diff_space * 2);

                    if new_location_i32 < 0 || new_location_i32 as usize >= contents.len() {
                        continue;
                    }

                    let new_location = new_location_i32 as usize;

                    // validate new location
                    if content_as_chars[new_location] != '\n' && validate_location (&contents, f_0, new_location, line_difference)
                    {
                        if content_as_chars[new_location] != '#' {
                            valid_count += 1;
                        }

                        content_as_chars[new_location] = '#';
                    }
                }
            }
        }

        println!("{}", frequencys.len());
        valid_count
    }

    fn process_part2 (contents: &mut str) -> i32 {
        let mut frequencys: HashSet<char> = HashSet::new();

        let mut content_as_chars = contents.chars().collect::<Vec<char>>();

        for char in contents.chars()
        {
            if char != '\n' && char != '.' {
                frequencys.insert (char);
            }
        }

        // process each frequnecy differently
        for f in &frequencys
        {
            let f_indexs = find_indices_with_char (&contents, *f);

            // compare each frequency point with all other. check line spacing
            for f_0 in f_indexs.clone() {

                for f_index in f_indexs.clone() {
                    if f_index == f_0 {
                        continue;
                    }

                    let line_difference = lines_between (&contents, f_0, f_index);
                    let diff_space = f_index as i32 - f_0 as i32;

                    let mut placement_index = f_0;

                    // loop backwards
                    loop
                    {
                        let new_location_i32 = placement_index as i32 - (diff_space * 2);

                        if new_location_i32 < 0 || new_location_i32 as usize >= contents.len() {
                            break;
                        }

                        let new_location = new_location_i32 as usize;

                        // validate new location
                        if content_as_chars[new_location] != '\n' && validate_location(&contents, placement_index, new_location, line_difference)
                        {
                            content_as_chars[placement_index] = '#';
                            content_as_chars[new_location] = '#';
                        }
                        else
                        {
                            break;
                        }

                        placement_index = new_location;
                    }

                    let mut placement_index = f_0;

                    // loop forward
                    loop
                    {
                        let new_location_i32 = placement_index as i32 + (diff_space * 2);

                        if new_location_i32 < 0 || new_location_i32 as usize >= contents.len() {
                            break;
                        }

                        let new_location = new_location_i32 as usize;

                        // validate new location
                        if content_as_chars[new_location] != '\n' && validate_location(&contents, placement_index, new_location, line_difference)
                        {
                            content_as_chars[placement_index] = '#';
                            content_as_chars[new_location] = '#';
                        }
                        else
                        {
                            break;
                        }

                        placement_index = new_location;
                    }
                }
            }
        }

        print_vec (&content_as_chars);
        content_as_chars.iter().filter (|&c| *c != '.' && *c != '\n').count() as i32
    }


    fn print_vec (vec: &Vec<char>) {
        for char in vec {
            print!("{}", char);
        }
        println!();
        println!();
    }

    pub fn do_task()
    {
        // let mut contents = utils::read_file("/Users/reubenduckering/Documents/Personal Repo/Advent-Of-Code-2024/files/day_8_mini.txt");
        let mut contents = utils::read_file("/Users/reubenduckering/Documents/Personal Repo/Advent-Of-Code-2024/files/day_8.txt");

        let result = process_part2 (&mut contents);
        println!("{}", result);
    }
}