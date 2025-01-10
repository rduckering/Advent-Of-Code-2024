pub mod day_4
{
    use crate::utils::utils;

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_search_for_word()
        {
            let contents: String = String::from("abcksdljfkljdlkfabcvaksajdklajsdabc\naaabc");
            assert_eq! (search_for_word (&contents, "abc"), 4);
        }

        #[test]
        fn test_search_for_word_diagonally()
        {
            let contents: String = String::from("jXSXooo\n\
                                                    XkMAMoo\n\
                                                    lMlAMAo\n\
                                                    llAcSXS\n\
                                                    lllSSXS");

            let contents2: String = String::from("jffSXoo\n\
                                                     kkAMboo\n\
                                                     lMAffao\n\
                                                     XSlcffo");

            assert_eq! (search_for_word_diagonally (&contents, "abc"), 4);
            assert_eq! (search_for_word_diagonally (&contents2, "abc"), 2);
        }

        #[test]
        fn test_search_for_word_x_mas()
        {
            let contents: String = String::from("0M0S000\n\
                                                    00A0000\n\
                                                    0M0S0M0\n\
                                                    0000A00\n\
                                                    000S0M0");

            assert_eq! (search_for_word_x_mas (&contents, "abc"), 2);
        }

        #[test]
        fn test_whole_thing()
        {
            let contents = "MMMSXXMASM\n\
             MSAMXMSMSA\n\
             AMXSXMAAMM\n\
             MSAMASMSMX\n\
             XMASAMXAMM\n\
             XXAMMXXAMA\n\
             SMSMSASXSS\n\
             SAXAMASAAA\n\
             MAMMMXMMMM\n\
             MXMXAXMASX";

            let forward = search_for_word (&contents, "XMAS");
            let backword = search_for_word (&contents, "SAMX");
            let diagonally = search_for_word_diagonally (&contents, "XMAS");
            let total = forward + backword + diagonally;
            assert_eq! (total, 18);
        }
    }

    fn search_for_word (contents: &str, pattern: &str) -> usize
    {
        let vec = contents.matches (pattern).collect::<Vec<&str>>();
        vec.len()
    }

    fn search_for_word_diagonally (contents: &str, _pattern: &str) -> usize
    {
        let lines: Vec<&str> = contents.lines().collect();

        // tried to use VecDeque but wasn't working
        let mut xmas_count: usize = 0;

        // loop through lines
        for count in 0..lines.len() - 3
        {
            let line1: &str = &lines[count];
            let line2: &str = &lines[count + 1];
            let line3: &str = &lines[count + 2];
            let line4: &str = &lines[count + 3];

            // loop through letters
            for letter_count in 0..line1.len()
            {
                if letter_count + 3 < line1.len()
                {
                    if line1.as_bytes()[letter_count] == b'X'
                    && line2.as_bytes()[letter_count + 1] == b'M'
                    && line3.as_bytes()[letter_count + 2] == b'A'
                    && line4.as_bytes()[letter_count + 3] == b'S'
                    {
                        xmas_count += 1;
                    }

                    if line1.as_bytes()[letter_count] == b'S'
                        && line2.as_bytes()[letter_count + 1] == b'A'
                        && line3.as_bytes()[letter_count + 2] == b'M'
                        && line4.as_bytes()[letter_count + 3] == b'X'
                    {
                        xmas_count += 1;
                    }
                }

                if letter_count >= 3 && letter_count != line1.len()
                {
                    if line1.as_bytes()[letter_count] == b'X'
                        && line2.as_bytes()[letter_count - 1] == b'M'
                        && line3.as_bytes()[letter_count - 2] == b'A'
                        && line4.as_bytes()[letter_count - 3] == b'S'
                    {
                        xmas_count += 1;
                    }

                    if line1.as_bytes()[letter_count] == b'S'
                        && line2.as_bytes()[letter_count - 1] == b'A'
                        && line3.as_bytes()[letter_count - 2] == b'M'
                        && line4.as_bytes()[letter_count - 3] == b'X'
                    {
                        xmas_count += 1;
                    }
                }

                if letter_count != line1.len()
                {
                    if line1.as_bytes()[letter_count] == b'X'
                        && line2.as_bytes()[letter_count] == b'M'
                        && line3.as_bytes()[letter_count] == b'A'
                        && line4.as_bytes()[letter_count] == b'S'
                    {
                        xmas_count += 1;
                    }

                    if line1.as_bytes()[letter_count] == b'S'
                        && line2.as_bytes()[letter_count] == b'A'
                        && line3.as_bytes()[letter_count] == b'M'
                        && line4.as_bytes()[letter_count] == b'X'
                    {
                        xmas_count += 1;
                    }
                }

                if letter_count == line1.len() {
                    break;
                }
            }
        }

        xmas_count
    }

    fn search_for_word_x_mas (contents: &str, _pattern: &str) -> usize
    {
        let lines: Vec<&str> = contents.lines().collect();

        // tried to use VecDeque but wasn't working
        let mut xmas_count: usize = 0;

        // loop through lines
        for count in 0..lines.len() - 2
        {
            let line1: &str = &lines[count];
            let line2: &str = &lines[count + 1];
            let line3: &str = &lines[count + 2];

            // loop through letters
            for letter_count in 1..line1.len() - 1
            {
                let mut has_one = false;

                if letter_count + 1 < line1.len()
                {
                    if line1.as_bytes()[letter_count - 1] == b'M'
                        && line2.as_bytes()[letter_count] == b'A'
                        && line3.as_bytes()[letter_count + 1] == b'S'
                    {
                        has_one = true;
                    }
                    else if line1.as_bytes()[letter_count - 1] == b'S'
                        && line2.as_bytes()[letter_count] == b'A'
                        && line3.as_bytes()[letter_count + 1] == b'M'
                    {
                        has_one = true;
                    }
                }

                if letter_count >= 1 && letter_count + 1 < line1.len()
                {
                    if line1.as_bytes()[letter_count + 1] == b'M'
                        && line2.as_bytes()[letter_count] == b'A'
                        && line3.as_bytes()[letter_count - 1] == b'S'
                    {
                        if has_one {
                            xmas_count += 1;
                        }
                    }
                    else if line1.as_bytes()[letter_count + 1] == b'S'
                        && line2.as_bytes()[letter_count] == b'A'
                        && line3.as_bytes()[letter_count - 1] == b'M'
                    {
                        if has_one {
                            xmas_count += 1;
                        }
                    }
                }
            }
        }

        xmas_count
    }

    pub fn do_task()
    {
        let contents = utils::read_file("/Users/reubenduckering/Documents/Personal Repo/Advent-Of-Code-2024/files/day_4.txt");
        let forward = search_for_word (&contents, "XMAS");
        let backward = search_for_word (&contents, "SAMX");
        let diagonally = search_for_word_diagonally (&contents, "XMAS");

        let _x_mas = search_for_word_x_mas (&contents, "XMAS");

        let count = forward + backward + diagonally as usize;
        println!("word count is '{}'", count);
    }
}