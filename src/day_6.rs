pub mod day_6
{
    use crate::utils::utils;

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_move_guard_up () {
            let mut vec:Vec<char> = vec! ['.', '.', '.', '\n',
                                          '.', '.', '.', '\n',
                                          '.', '^', '.', '\n'];

            let num_moves = move_guard(&mut vec);

            let res:Vec<char> = vec! ['.', 'X', '.', '\n',
                                          '.', 'X', '.', '\n',
                                          '.', 'X', '.', '\n'];
            assert_eq!(vec, res);
            assert_eq! (num_moves, 3);
        }

        #[test]
        fn test_move_guard_down () {
            let mut vec:Vec<char> = vec! ['#', '.', '.', '\n',
                                          '.', '.', '#', '\n',
                                          '.', '.', '.', '\n',
                                          '^', '#', '.', '\n'];

            let num_moves = move_guard(&mut vec);

            let res:Vec<char> = vec! ['#', '.', '.', '\n',
                                          'X', 'X', '#', '\n',
                                          'X', 'X', '.', '\n',
                                          'X', '#', '.', '\n'];
            assert_eq!(vec, res);
            assert_eq! (num_moves, 5);
        }

        #[test]
        fn test_move_guard_edge_case () {
            let mut vec:Vec<char> = vec! ['#', '.', '.', '\n',
                                          '.', '#', '.', '\n',
                                          '.', '.', '.', '\n',
                                          '^', '.', '.', '\n'];

            let num_moves = move_guard(&mut vec);

            let res: Vec<char> = vec! ['#', '.', '.', '\n',
                                          'X', '#', '.', '\n',
                                          'X', '.', '.', '\n',
                                          'X', '.', '.', '\n'];
            assert_eq!(vec, res);
            assert_eq! (num_moves, 3);
        }
    }

    fn is_obstacle (next_tile: char) -> bool {
        if next_tile == '#' {
            return true;
        }
        false
    }

    fn is_map_sides (next_tile: char) -> bool {
        if next_tile == '\n' {
            return true;
        }

        false
    }

    fn get_direction_movement (line_length: i32, current_position: char) -> i32 {
        let direction = match current_position {
            '^' => -(line_length + 1),
            'v' => line_length + 1,
            '<' => -1,
            '>' => 1,
            _ => 0
        };

        direction
    }

    fn turn_90_degrees (current_position: char) -> char {
        let direction = match current_position {
            '^' => '>',
            'v' => '<',
            '<' => '^',
            '>' => 'v',
            _ => current_position
        };

        direction
    }

    fn draw_direction (current_position: char, current_char: char) -> char {
        let direction = match current_position {
            '^' => '|',
            'v' => '|',
            '<' => '-',
            '>' => '-',
            _ => current_position
        };

        if current_char == '^' {
            return current_char;
        }

        if current_char == '-' && direction == '|' {
            return '+';
        }

        if current_char == '|' && direction == '-' {
            return '+';
        }

        if current_char == '+' {
            return '+';
        }

        direction
    }

    fn draw_direction_v2 (current_position: char, current_char: char) -> char {
        let direction = match current_position {
            '^' => 'u',
            'v' => 'd',
            '<' => 'l',
            '>' => 'r',
            _ => current_position
        };

        if current_char == '^' {
            return current_char;
        }

        if (current_char == 'l' || current_char == 'r') && (direction == 'u' || direction == 'd') {
            return '+';
        }

        if (current_char == 'u' || current_char == 'd') && (direction == 'l' || direction == 'r') {
            return '+';
        }

        if current_char == '+' {
            return '+';
        }

        if current_char != '.' {
            return current_char;
        }

        direction
    }

    fn detect_loop (current_position: char, current_char: char) -> bool {
        let direction = match current_position {
            '^' => 'u',
            'v' => 'd',
            '<' => 'l',
            '>' => 'r',
            _ => current_position
        };

        if current_char == 'r' && direction == 'r' {
            return true;
        }

        if current_char == 'u' && direction == 'u' {
            return true;
        }

        if current_char == 'd' && direction == 'd' {
            return true;
        }

        if current_char == 'l' && direction == 'l' {
            return true;
        }

        false
    }

    fn move_guard (content: &mut Vec<char>)->i32 {
        let mut guard_position: i32 = content.iter().position(|&c| c == '^').unwrap() as i32;
        let line_length: i32 = content.iter().position(|&c| c == '\n').unwrap() as i32;

        loop
        {
            let guard_char = content[guard_position as usize];
            let movement = get_direction_movement (line_length, guard_char);
            let new_position = guard_position + movement;

            if let Some(new_char) = content.get_mut(new_position as usize) {

                if is_obstacle(*new_char) {
                    content[guard_position as usize] = turn_90_degrees (guard_char);
                    continue;
                }

                if is_map_sides (*new_char) {
                    content[guard_position as usize] = 'X';
                    break;
                }

                *new_char = guard_char;
                content[guard_position as usize] = 'X';
                guard_position = new_position;
            }
            else
            {
                content[guard_position as usize] = 'X';
                break
            }
        }

        // count x's in vec
        let all_x: Vec<_> = content.iter().filter(|&c| *c == 'X').collect();

        all_x.len() as i32
    }

    // got to do it as it's happening, check if placing a block ahead of you causes you to meet up with some where you've already been
    fn draw_move_guard (content: &mut Vec<char>)->i32 {
        let mut guard_position: i32 = content.iter().position(|&c| c == '^').unwrap() as i32;
        let line_length: i32 = content.iter().position(|&c| c == '\n').unwrap() as i32;
        let mut guard_char = content[guard_position as usize];
        let mut prev_rotate = false;

        loop
        {
            let movement = get_direction_movement (line_length, guard_char);
            let new_position = guard_position + movement;
            let current_char = content[guard_position as usize];

            if let Some(new_char) = content.get_mut (new_position as usize) {
                // gone off map
                if is_map_sides (*new_char) {
                    content[guard_position as usize] = draw_direction (guard_char, current_char);
                    break;
                }

                if is_obstacle (*new_char) {
                    guard_char = turn_90_degrees(guard_char);
                    content[guard_position as usize] = '+';
                    prev_rotate = true;
                    continue;
                }

                //*new_char = guard_char;
                if ! prev_rotate {
                    content[guard_position as usize] = draw_direction (guard_char, current_char);
                }

                prev_rotate = false;
                guard_position = new_position;
            }
            else
            {
                // end - gone gone off map
                content[guard_position as usize] = draw_direction (guard_char, current_char);
                break
            }
        }

        // count x's in vec
        let all_x: Vec<_> = content.iter().filter(|&c| *c == 'X').collect();

        all_x.len() as i32
    }

    // fn step_through_move_guard (content: &mut Vec<char>) -> i32 {
    //     let mut guard_position: i32 = content.iter().position(|&c| c == '^').unwrap() as i32;
    //     let line_length: i32 = content.iter().position(|&c| c == '\n').unwrap() as i32;
    //     let mut guard_char = content[guard_position as usize];
    //     let mut prev_rotate = false;
    //
    //     let mut blockers_can_place = 0;
    //
    //     loop
    //     {
    //         let movement = get_direction_movement (line_length, guard_char);
    //         let new_position = guard_position + movement;
    //         let current_char = content[guard_position as usize];
    //
    //         if let Some(new_char) = content.get_mut (new_position as usize) {
    //             // gone off map
    //             if is_map_sides (*new_char) {
    //                 content[guard_position as usize] ='.';
    //                 break;
    //             }
    //
    //             if is_obstacle (*new_char) {
    //                 content[guard_position as usize] = guard_char;
    //                 guard_char = turn_90_degrees(guard_char);
    //                 prev_rotate = true;
    //                 continue;
    //             }
    //             else
    //             {
    //                 // if the position was blocked
    //                 let mut stuck_counter = 0;
    //                 if check_right_for_blockage (content, guard_char, guard_position, line_length, &mut stuck_counter) {
    //                     blockers_can_place += 1;
    //                     // let char_to_place = (blockers_can_place as u8 + b'0') as char;
    //                     content[guard_position as usize] = '0';
    //
    //                     // printVec (&content);
    //                     // print!("\n");
    //                 }
    //             }
    //
    //             // *new_char = guard_char;
    //             // if ! prev_rotate && content[guard_position as usize] != '0' {
    //             //     content[guard_position as usize] = '.';
    //             // }
    //
    //
    //             prev_rotate = false;
    //             guard_position = new_position;
    //         }
    //         else
    //         {
    //             // end - gone gone off map
    //             content[guard_position as usize] = '.';
    //             break
    //         }
    //     }
    //
    //   blockers_can_place
    // }

    fn find_loops_move_guard (content: &mut Vec<char>)->bool {
        let mut guard_position: i32 = content.iter().position(|&c| c == '^').unwrap() as i32;
        let line_length: i32 = content.iter().position(|&c| c == '\n').unwrap() as i32;
        let mut guard_char = content[guard_position as usize];
        let mut prev_rotate = false;

        let mut cnt = 0;

        loop
        {
            let movement = get_direction_movement (line_length, guard_char);
            let new_position = guard_position + movement;
            let current_char = content[guard_position as usize];

            if cnt > 100000 {
                return true;
            }

            if let Some(new_char) = content.get_mut (new_position as usize) {
                // gone off map
                if is_map_sides (*new_char) {
                    content[guard_position as usize] = draw_direction_v2 (guard_char, current_char);
                    break;
                }

                if is_obstacle (*new_char) {
                    guard_char = turn_90_degrees(guard_char);
                    content[guard_position as usize] = '+';
                    prev_rotate = true;
                    continue;
                }

                //*new_char = guard_char;
                if ! prev_rotate {

                    if detect_loop (guard_char, current_char) {
                        return true;
                    }

                    content[guard_position as usize] = draw_direction_v2 (guard_char, current_char);
                }

                prev_rotate = false;
                guard_position = new_position;
            }
            else
            {
                // end - gone gone off map
                content[guard_position as usize] = draw_direction_v2 (guard_char, current_char);
                break
            }

            cnt += 1;
        }

        println! ("count  {}", cnt);
        // count x's in vec
        false
    }

    fn step_through_attempt_two (content: &mut Vec<char>) -> i32 {
        let mut content_copy = content.clone();
        draw_move_guard (&mut content_copy);

        let indices: Vec<usize> = content_copy.iter()
            .enumerate()
            .filter(|&(_, &value)| value == '-' || value == '|' || value == '+')
            .map(|(index, _)| index)
            .collect();

        let mut counter = 0;

        for i in indices {
            let mut another_copy = content.clone();
            another_copy[i] = '#';
            if find_loops_move_guard (&mut another_copy) {
                counter += 1;
            }

            print_vec (&another_copy);
            print!("\n\n");
        }
        counter
    }

    fn print_vec (vec: &Vec<char>) {
        for char in vec {
            print!("{}", char);
        }
    }

    pub fn do_task()
    {
        let contents = utils::read_file ("/Users/reubenduckering/Documents/Personal Repo/Advent-Of-Code-2024/files/day_6.txt");
        let mut _content_vec: Vec<char> = contents.chars().collect();

        // let movements = move_guard (&mut content_vec);
        // println!("{:?}", movements);

        let contents2 = utils::read_file ("/Users/reubenduckering/Documents/Personal Repo/Advent-Of-Code-2024/files/day_6_mini.txt");
        let mut content_vec2: Vec<char> = contents2.chars().collect();

        let movements2 =  step_through_attempt_two (&mut content_vec2);
        println!("{:?}", movements2);
        print_vec (&content_vec2);
    }
}