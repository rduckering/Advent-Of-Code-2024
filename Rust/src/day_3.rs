pub mod day_3
{
    use crate::utils::utils;

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_is_valid_char() {
            assert_eq!(is_valid_char('c'), false);
            assert_eq!(is_valid_char('£'), false);
            assert_eq!(is_valid_char('c'), false);
            assert_eq!(is_valid_char('m'), true);
            assert_eq!(is_valid_char('4'), true);
            assert_eq!(is_valid_char(')'), true);
        }

        #[test]
        fn test_process_phrase() {
            assert_eq!(process_phrase(&['m','u','l','(','1',',','2',')',' ',' ',' ',' ']), 2);
            assert_eq!(process_phrase(&['m','u','l','(','1','0',',','2',')',' ',' ',' ']), 20);
            assert_eq!(process_phrase(&['m','u','l','(','1',',','2','0',')',' ',' ',' ']), 20);
        }

        #[test]
        fn test_is_valid_instruction_letter() {
            assert_eq!(is_valid_instruction_letter('c'), false);
            assert_eq!(is_valid_instruction_letter('£'), false);
            assert_eq!(is_valid_instruction_letter('c'), false);
            assert_eq!(is_valid_instruction_letter('d'), true);
            assert_eq!(is_valid_instruction_letter('n'), true);
            assert_eq!(is_valid_instruction_letter(')'), true);
        }

        #[test]
        fn test_process_instruction_letter() {
            assert_eq!(process_instruction_phrase(&['d', 'o', '(', ')' ,' ',' ', ' ']), true);
            assert_eq!(process_instruction_phrase(&['d', 'o', 'n', '\'' ,'t','(', ')']), false);
        }
    }

    fn is_valid_char(c: char) -> bool {
        if c >= '0' && c <= '9' {
            return true
        }

        if c == 'm' || c == 'u' || c == 'l' || c == '(' || c == ',' || c == ')'  {
            return true
        }

        return false
    }

    fn is_valid_instruction_letter (c: char) -> bool {
        if c == 'd' || c == 'o' || c == 'n' || c == '\'' || c == 't' || c == '(' || c == ')'{
            return true
        }

        false
    }

    fn process_phrase (buff: &[char; 12]) -> i32
    {
        let slice_as_string: String = buff.iter().collect();

        if slice_as_string.contains('(') && slice_as_string.contains(')') && slice_as_string.contains(',') {
            if &slice_as_string[..4] == "mul("
            {
                // starts with mull
                let start_index = 4;
                let comma_index= slice_as_string.find(',').unwrap_or(0);
                let closing_index = slice_as_string.find(')').unwrap_or(0);

                let first_num = &buff[start_index.. comma_index].iter().collect::<String>();
                let second_num = &buff[comma_index + 1..closing_index].iter().collect::<String>();

                let result = first_num.parse::<i32>().unwrap_or(0) * second_num.parse::<i32>().unwrap_or(0);
                return result;
            }
        }

        0
    }

    fn process_instruction_phrase (buff: &[char; 7]) -> bool
    {
        let slice_as_string: String = buff.iter().collect();

        if &slice_as_string[..4] == "do()" {
            return true;
        } else if &slice_as_string == "don't()" {
            return false;
        }

        panic!("help");
    }

    fn parse_content (content: &str) -> Vec<i32> {

        let mut vec: Vec<i32> = Vec::new();

        // scratch buffers
        let mut buff: [char; 12] = ['\0'; 12];
        let mut instruction_buff: [char; 7] = ['\0'; 7];

        let mut good_letters = 0;
        let mut good_letters_instructions = 0;

        let mut allow_multiplication = true;

        for index in 0..content.chars().count()
        {
            let letter: char = content.chars().nth(index).unwrap();

            // capture do or dont
            if is_valid_instruction_letter (letter)
            {
                if good_letters_instructions == 0 && letter != 'd' {
                    good_letters_instructions = 0;
                }
                else
                {
                    instruction_buff [good_letters_instructions] = letter;
                    good_letters_instructions += 1;

                    if letter == ')'
                    {
                        allow_multiplication = process_instruction_phrase (&instruction_buff);
                        good_letters_instructions = 0;
                    }
                }
            }

            // capture multiple logic
            if is_valid_char (letter)
            {
                if good_letters == 0 && letter != 'm' {
                    continue;
                }

                buff[good_letters] = letter;
                good_letters += 1;

                if letter == ')'
                {
                    if good_letters >= 8 && good_letters <= 12 && allow_multiplication
                    {
                        let result = process_phrase (&buff);
                        vec.push (result);
                    }

                    good_letters = 0;
                }
            }
            else {
                good_letters = 0;
            }

        }

        vec
    }

    pub fn _do_task()
    {
        let contents = utils::read_file("/Users/reubenduckering/Documents/Personal Repo/Advent-Of-Code-2024/files/day_3.txt");
        let results_array = parse_content (&contents);
        let sum = results_array.iter().sum::<i32>();
        println!("Day 3 Part 1: {}", sum);
    }
}