use std::collections::HashSet;

pub mod day_15
{
    use crate::utils::utils;
    use std::ops::Add;
    use std::collections::HashSet;
    use std::cmp::Reverse;

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_move_robot() {

            let content = "#######\n\
                                 #.....#\n\
                                 #..@..#\n\
                                 #.....#\n\
                                 #######";
            let mut robot = Robot { current_position: Point { x: 0, y:0 }, starting_position: Point {x: 0, y: 0 }, id: 0 };
            let movement = Movement{ x: 0, y:1 };
            let mut cells: Vec<Cell> = Vec::new();

            let map_info = convert_to_map_points (&content, &mut cells, &mut robot);
            draw_map (&map_info, &robot, &cells);
            move_robot (&mut robot, &movement, &mut cells);

            assert_eq!(robot.current_position, Point { x: 3, y: 3 });
            assert_eq!(cells[24].obstacle, '.');
        }

        #[test]
        fn test_move_robot_2() {

            let content = "#######\n\
                                 #.....#\n\
                                 #..@..#\n\
                                 #..#..#\n\
                                 #######";
            let mut robot = Robot { current_position: Point { x: 0, y:0 }, starting_position: Point {x: 0, y: 0 }, id: 0 };
            let movement = Movement{ x: 0, y:1 };
            let mut cells: Vec<Cell> = Vec::new();

            let map_info = convert_to_map_points (&content, &mut cells, &mut robot);
            move_robot (&mut robot, &movement, &mut cells);

            assert_eq!(robot.current_position, Point { x: 3, y: 2 });
            assert_eq!(cells[24].obstacle, '#');
        }

        #[test]
        fn test_move_robot_3() {

            let content = "#######\n\
                                 #.....#\n\
                                 #..@O.#\n\
                                 #..O..#\n\
                                 #######";
            let mut robot = Robot { current_position: Point { x: 0, y:0 }, starting_position: Point {x: 0, y: 0 }, id: 0 };
            let mut cells: Vec<Cell> = Vec::new();

            let map_info = convert_to_map_points (&content, &mut cells, &mut robot);

            {
                let movement = Movement { x: 0, y: 1 };
                move_robot(&mut robot, &movement, &mut cells);

                assert_eq!(robot.current_position, Point { x: 3, y: 2 });
                assert_eq!(cells[24].obstacle, 'O');
            }

            {
                let movement = Movement { x: 1, y: 0 };

                assert_eq!(cells[18].obstacle, 'O');
                assert_eq!(cells[19].obstacle, '.');

                move_robot(&mut robot, &movement, &mut cells);

                assert_eq!(robot.current_position, Point { x: 4, y: 2 });
                assert_eq!(cells[18].obstacle, '.');
                assert_eq!(cells[19].obstacle, 'O');
            }
        }

        #[test]
        fn test_move_robot_4() {

            let content = "########\n\
                                 #......#\n\
                                 #..@OO.#\n\
                                 #......#\n\
                                 ########";
            let mut robot = Robot { current_position: Point { x: 0, y:0 }, starting_position: Point {x: 0, y: 0 }, id: 0 };
            let mut cells: Vec<Cell> = Vec::new();

            let map_info = convert_to_map_points (&content, &mut cells, &mut robot);

            {
                let movement = Movement { x: 1, y: 0 };
                move_robot (&mut robot, &movement, &mut cells);

                assert_eq!(robot.current_position, Point { x: 4, y: 2 });
                assert_eq!(cells[20].obstacle, '.');
                assert_eq!(cells[21].obstacle, 'O');
                assert_eq!(cells[22].obstacle, 'O');
            }
        }

        #[test]
        fn test_v2_parse_map() {
            let content = "########\n\
                                 #..#...#\n\
                                 #..@OO.#\n\
                                 #......#\n\
                                 ########";

            let mut robot = Robot { current_position: Point { x: 0, y:0 }, starting_position: Point {x: 0, y: 0 }, id: 0 };
            let mut cells: Vec<Cell> = Vec::new();

            let map_info = convert_to_map_points_v2 (&content, &mut cells, &mut robot);
            // draw_map (&map_info, &robot, &cells);
        }

        #[test]
        fn test_move_robot_v2_3() {

            let content = "#######\n\
                                 #.....#\n\
                                 #..@O.#\n\
                                 #..O..#\n\
                                 #######";
            let mut robot = Robot { current_position: Point { x: 0, y:0 }, starting_position: Point {x: 0, y: 0 }, id: 0 };
            let mut cells: Vec<Cell> = Vec::new();

            let map_info = convert_to_map_points_v2 (&content, &mut cells, &mut robot);

            {
                let movement = Movement { x: 0, y: 1 };
                move_robot(&mut robot, &movement, &mut cells);
                // draw_map (&map_info, &robot, &cells);

                assert_eq!(robot.current_position, Point { x: 6, y: 2 });
                assert_eq!(cells[48].obstacle, '[');
            }

            {
                let movement = Movement { x: 1, y: 0 };

                draw_map (&map_info, &robot, &cells);

                assert_eq!(cells[36].obstacle, '[');
                assert_eq!(cells[38].obstacle, '.');

                move_robot(&mut robot, &movement, &mut cells);
                move_robot(&mut robot, &movement, &mut cells); //  move twice
                draw_map (&map_info, &robot, &cells);

                assert_eq!(robot.current_position, Point { x: 8, y: 2 });
                assert_eq!(cells[36].obstacle, '.');
                assert_eq!(cells[37].obstacle, '[');
                assert_eq!(cells[38].obstacle, ']');
            }
        }

        #[test]
        fn test_move_robot_v2_4() {

            let content = "#######\n\
                                 #.....#\n\
                                 #.@O..#\n\
                                 #..O..#\n\
                                 #.....#\n\
                                 #######";
            let mut robot = Robot { current_position: Point { x: 0, y:0 }, starting_position: Point {x: 0, y: 0 }, id: 0 };
            let mut cells: Vec<Cell> = Vec::new();

            let map_info = convert_to_map_points_v2 (&content, &mut cells, &mut robot);

            {
                let movement_right = Movement { x: 1, y: 0 };
                let movement_up = Movement { x: 0, y: -1 };
                let movement_down = Movement { x: 0, y: 1 };

                // draw_map (&map_info, &robot, &cells);

                move_robot_v2 (&mut robot, &movement_right, &mut cells);
                move_robot_v2 (&mut robot, &movement_right, &mut cells);
                move_robot_v2 (&mut robot, &movement_up, &mut cells);
                move_robot_v2 (&mut robot, &movement_right, &mut cells);

                // draw_map (&map_info, &robot, &cells);
                move_robot_v2 (&mut robot, &movement_down, &mut cells);

                // draw_map (&map_info, &robot, &cells);

                assert_eq!(robot.current_position, Point { x: 7, y: 2 });
                assert_eq!(cells[48].obstacle, '.');
                assert_eq!(cells[62].obstacle, '[');
                assert_eq!(cells[63].obstacle, ']');
            }
        }

        #[test]
        fn test_move_robot_v2_5() {

            let content = "#######\n\
                                 #.....#\n\
                                 #.@O..#\n\
                                 #..O..#\n\
                                 #.....#\n\
                                 #..#..#\n\
                                 #######";
            let mut robot = Robot { current_position: Point { x: 0, y:0 }, starting_position: Point {x: 0, y: 0 }, id: 0 };
            let mut cells: Vec<Cell> = Vec::new();

            let map_info = convert_to_map_points_v2 (&content, &mut cells, &mut robot);

            {
                let movement_right = Movement { x: 1, y: 0 };
                let movement_up = Movement { x: 0, y: -1 };
                let movement_down = Movement { x: 0, y: 1 };

                cells[77].obstacle = '.';
                // draw_map (&map_info, &robot, &cells);

                move_robot_v2 (&mut robot, &movement_right, &mut cells);
                move_robot_v2 (&mut robot, &movement_right, &mut cells);
                move_robot_v2 (&mut robot, &movement_up, &mut cells);
                move_robot_v2 (&mut robot, &movement_right, &mut cells);

                // draw_map (&map_info, &robot, &cells);
                move_robot_v2 (&mut robot, &movement_down, &mut cells);

                // draw_map (&map_info, &robot, &cells);

                assert_eq!(robot.current_position, Point { x: 7, y: 2 });
                assert_eq!(cells[48].obstacle, '.');
                assert_eq!(cells[62].obstacle, '[');
                assert_eq!(cells[63].obstacle, ']');
            }
        }

        #[test]
        fn test_move_robot_v2_6() {

            let content = "#######\n\
                                 #..#..#\n\
                                 #.....#\n\
                                 #..O..#\n\
                                 #.@O..#\n\
                                 #.....#\n\
                                 #######";
            let mut robot = Robot { current_position: Point { x: 0, y:0 }, starting_position: Point {x: 0, y: 0 }, id: 0 };
            let mut cells: Vec<Cell> = Vec::new();

            let map_info = convert_to_map_points_v2 (&content, &mut cells, &mut robot);

            {
                let movement_right = Movement { x: 1, y: 0 };
                let movement_up = Movement { x: 0, y: -1 };
                let movement_down = Movement { x: 0, y: 1 };

                cells[77].obstacle = '.';
                // draw_map (&map_info, &robot, &cells);

                move_robot_v2(&mut robot, &movement_right, &mut cells);
                move_robot_v2(&mut robot, &movement_right, &mut cells);
                move_robot_v2(&mut robot, &movement_down, &mut cells);
                move_robot_v2(&mut robot, &movement_right, &mut cells);

                // draw_map (&map_info, &robot, &cells);
                move_robot_v2(&mut robot, &movement_up, &mut cells);

                // draw_map (&map_info, &robot, &cells);

                assert_eq!(robot.current_position, Point { x: 7, y: 4 });
                assert_eq!(cells[48].obstacle, '.');
                assert_eq!(cells[34].obstacle, '[');
                assert_eq!(cells[35].obstacle, ']');
                assert_eq!(cells[49].obstacle, '[');
                assert_eq!(cells[50].obstacle, ']');
            }
        }

        #[test]
        fn test_move_robot_v2_7() {

            let content = "#######\n\
                                 #.....#\n\
                                 #.....#\n\
                                 #.....#\n\
                                 #OO@..#\n\
                                 #.....#\n\
                                 #######";
            let mut robot = Robot { current_position: Point { x: 0, y:0 }, starting_position: Point {x: 0, y: 0 }, id: 0 };
            let mut cells: Vec<Cell> = Vec::new();

            let map_info = convert_to_map_points_v2 (&content, &mut cells, &mut robot);

            {
                let movement_left = Movement { x: -1, y: 0 };

                // draw_map (&map_info, &robot, &cells);

                move_robot_v2 (&mut robot, &movement_left, &mut cells);

                // draw_map (&map_info, &robot, &cells);

                assert_eq!(robot.current_position, Point { x: 6, y: 4 });
                assert_eq!(cells[58].obstacle, '[');
                assert_eq!(cells[59].obstacle, ']');
                assert_eq!(cells[60].obstacle, '[');
                assert_eq!(cells[61].obstacle, ']');
            }
        }

        #[test]
        fn test_move_robot_v2_8() {

            let content = "#######\n\
                                 #..@..#\n\
                                 #..O..#\n\
                                 #.....#\n\
                                 #.....#\n\
                                 #.....#\n\
                                 #######";
            let mut robot = Robot { current_position: Point { x: 0, y:0 }, starting_position: Point {x: 0, y: 0 }, id: 0 };
            let mut cells: Vec<Cell> = Vec::new();

            let map_info = convert_to_map_points_v2 (&content, &mut cells, &mut robot);

            {
                let movement_down = Movement { x: 0, y: 1 };

                // draw_map (&map_info, &robot, &cells);

                move_robot_v2(&mut robot, &movement_down, &mut cells);

                // draw_map (&map_info, &robot, &cells);

                assert_eq!(robot.current_position, Point { x: 6, y: 2 });
                assert_eq!(cells[48].obstacle, '[');
                assert_eq!(cells[49].obstacle, ']');
            }
        }

        #[test]
        fn test_move_robot_v2_9() {

            let content = "#######\n\
                                 #..@..#\n\
                                 #..O..#\n\
                                 #.....#\n\
                                 #..OO.#\n\
                                 #.....#\n\
                                 #######";
            let mut robot = Robot { current_position: Point { x: 0, y:0 }, starting_position: Point {x: 0, y: 0 }, id: 0 };
            let mut cells: Vec<Cell> = Vec::new();

            let map_info = convert_to_map_points_v2 (&content, &mut cells, &mut robot);

            {
                let movement_down = Movement { x: 0, y: 1 };

                cells[48].obstacle = '.';
                cells[49].obstacle = '[';
                cells[50].obstacle = ']';

                // draw_map (&map_info, &robot, &cells);

                move_robot_v2(&mut robot, &movement_down, &mut cells);

                // draw_map (&map_info, &robot, &cells);

                assert_eq!(robot.current_position, Point { x: 6, y: 2 });
                assert_eq!(cells[63].obstacle, '[');
                assert_eq!(cells[64].obstacle, ']');
                assert_eq!(cells[78].obstacle, '[');
                assert_eq!(cells[79].obstacle, ']');
            }
        }

        #[test]
        fn test_move_robot_v2_10() {

            let content = "#######\n\
                                 #..@..#\n\
                                 #..O..#\n\
                                 #..O..#\n\
                                 #..#..#\n\
                                 #.....#\n\
                                 #######";
            let mut robot = Robot { current_position: Point { x: 0, y:0 }, starting_position: Point {x: 0, y: 0 }, id: 0 };
            let mut cells: Vec<Cell> = Vec::new();

            let map_info = convert_to_map_points_v2 (&content, &mut cells, &mut robot);

            {
                let movement_down = Movement { x: 0, y: 1 };

                cells[48].obstacle = '.';
                cells[49].obstacle = '[';
                cells[50].obstacle = ']';

                // draw_map (&map_info, &robot, &cells);

                move_robot_v2(&mut robot, &movement_down, &mut cells);

                // draw_map (&map_info, &robot, &cells);

                assert_eq!(robot.current_position, Point { x: 6, y: 1 });
                assert_eq!(cells[49].obstacle, '[');
                assert_eq!(cells[50].obstacle, ']');
            }
        }

        #[test]
        fn test_move_robot_v2_11() {

            let content = "#######\n\
                                 #..@..#\n\
                                 #.#O..#\n\
                                 #.....#\n\
                                 #.....#\n\
                                 #.....#\n\
                                 #######";
            let mut robot = Robot { current_position: Point { x: 0, y:0 }, starting_position: Point {x: 0, y: 0 }, id: 0 };
            let mut cells: Vec<Cell> = Vec::new();

            let map_info = convert_to_map_points_v2 (&content, &mut cells, &mut robot);

            {
                let movement_down = Movement { x: 0, y: 1 };

                cells[47].obstacle = '[';
                cells[48].obstacle = ']';
                cells[61].obstacle = '[';
                cells[62].obstacle = ']';

                draw_map (&map_info, &robot, &cells);

                move_robot_v2(&mut robot, &movement_down, &mut cells);

                draw_map (&map_info, &robot, &cells);

                assert_eq!(robot.current_position, Point { x: 6, y: 2 });
                assert_eq!(cells[48].obstacle, '[');
                assert_eq!(cells[49].obstacle, ']');
            }
        }

        #[test]
        fn test_move_robot_v2_12() {

            let content = "#######\n\
                                 #..@..#\n\
                                 #.....#\n\
                                 #.OO..#\n\
                                 #.....#\n\
                                 #.....#\n\
                                 #######";
            let mut robot = Robot { current_position: Point { x: 0, y:0 }, starting_position: Point {x: 0, y: 0 }, id: 0 };
            let mut cells: Vec<Cell> = Vec::new();

            let map_info = convert_to_map_points_v2 (&content, &mut cells, &mut robot);

            {
                let movement_down = Movement { x: 0, y: 1 };

                cells[33].obstacle = '[';
                cells[34].obstacle = ']';

                // cells[59].obstacle = '[';
                // cells[60].obstacle = ']';
                cells[61].obstacle = '[';
                cells[62].obstacle = ']';
                // cells[63].obstacle = '[';
                // cells[64].obstacle = ']';

                draw_map (&map_info, &robot, &cells);

                move_robot_v2(&mut robot, &movement_down, &mut cells);

                draw_map (&map_info, &robot, &cells);

                assert_eq!(robot.current_position, Point { x: 6, y: 2 });
                assert_eq!(cells[75].obstacle, '[');
                assert_eq!(cells[76].obstacle, ']');
                assert_eq!(cells[60].obstacle, '[');
                assert_eq!(cells[61].obstacle, ']');
                assert_eq!(cells[62].obstacle, '[');
                assert_eq!(cells[63].obstacle, ']');
            }
        }
    }

    #[derive(PartialEq, Eq, Clone, Debug)]
    struct Cell {
        position: Point,
        obstacle: char,
    }

    #[derive(PartialEq, Eq, Clone, Debug, Hash)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[derive(PartialEq, Eq, Clone, Debug, Hash)]
    struct Movement {
        x: i32,
        y: i32,
    }

    #[derive(PartialEq, Eq, Clone, Debug)]
    struct Robot {
        starting_position: Point,
        current_position: Point,
        id: u32,
    }

    struct MapInfo {
        max_rows: usize,
        max_cols: usize,
    }

    impl Add for Point {
        type Output = Point;

        fn add (self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    impl Add<Movement> for Point {
        type Output = Point;

        fn add (self, other: Movement) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    fn convert_to_map_points (content: &str, points: &mut Vec<Cell>, robot: &mut Robot) -> MapInfo {

        let mut row: usize = 0;
        let mut col: usize = 0;

        let mut max_rows: usize = 0;
        let mut max_cols: usize = 0;

        let mut id = 0;

        for line in content.lines()
        {
            for c in line.chars()
            {
                let pos =  Point { x: col as i32, y: row as i32 };
                let mut insert_c = c;

                if c == '@' {
                    robot.current_position = pos.clone();
                    robot.starting_position = pos.clone();
                    robot.id = id;
                    id += 1;

                    insert_c = '.'
                }

                points.push (Cell { position: pos.clone(), obstacle: insert_c });
                col += 1;
            }

            max_cols = max_cols.max (col);
            max_rows = max_rows.max (row);

            col = 0;
            row += 1;
        }

        max_rows += 1;

        MapInfo { max_rows, max_cols }
    }

    fn parse_instructions (content: &str, instructions: &mut Vec<Movement>) {

        for line in content.lines() {
            for c in line.chars() {
                let movement =  match c {
                    '<' => Movement { x:-1, y:0 },
                    '>' => Movement { x:1, y:0 },
                    '^' => Movement { x:0, y:-1 },
                    'v' => Movement { x:0, y:1 },
                    _ => panic!("how?"),
                };

                instructions.push (movement);
            }
        }
    }

    fn get_map_point<'a> (point: &Point, cells: &'a mut [Cell]) -> Option<&'a mut Cell> {

        for c in cells.iter_mut() {
            if c.position == *point {
                return Some (c);
            }
        }

        None
    }

    fn get_map_point_copy (point: &Point, cells: &[Cell]) -> Option <Cell> {

        for c in cells.iter() {
            if c.position == *point {
                return Some (c.clone());
            }
        }
        None
    }

    fn get_map_point_from_index (index: usize, cells: Vec<Cell>) -> Option<Cell> {
        Some (cells[index].clone())
    }

    fn get_cell_at_sides (cell_to_move: Cell, cells: &Vec<Cell>) -> (Cell, Cell) {
        let cell_left = cell_to_move.position.clone() + Point {x: -1, y:0 };
        let cell_right = cell_to_move.position.clone() + Point {x: 1, y:0 };

        (get_map_point_copy (&cell_left, cells).unwrap(), get_map_point_copy (&cell_right, cells).unwrap())
    }

    fn get_map_point_char (point: &Point, cells: &[Cell]) -> Option<char>  {
        for c in cells.iter() {
            if c.position == *point {
                return Some (c.obstacle);
            }
        }

        None
    }

    fn move_obstacle (obstacle: &Cell, movement: Movement, cells: &Vec<Cell>) -> Vec<(Cell, bool)> {
        let new_position = obstacle.clone().position.add (movement.clone());

         let mut mark_to_move : Vec<(Cell, bool)> = Vec::new();

        // Find the index of the valid point to avoid multiple mutable borrows
        if let Some(valid_point_index) = cells.iter().position(|c| c.position == new_position) {
            let valid_point = &cells[valid_point_index];

            if valid_point.obstacle == '#'
            {
                mark_to_move.push ((valid_point.clone(), false));
            }
            else if valid_point.obstacle == 'O'
            {
                let moves = move_obstacle (valid_point, movement, cells);

                mark_to_move.push ((obstacle.clone(), true));
                mark_to_move.append(&mut moves.clone());
            }
            else
            {
                mark_to_move.push((valid_point.clone(), true));
            }
        }

         mark_to_move
    }

    fn move_robot (robot: &mut Robot, movement: &Movement, cells: &mut Vec<Cell>) {

        let new_position = Point { x: robot.current_position.x + movement.x, y: robot.current_position.y + movement.y };

        if let Some(valid_point_index) = cells.iter().position(|c| c.position == new_position) {
            let valid_point = &cells[valid_point_index];

            if valid_point.obstacle == '#' {
                // print!("Way blocked");
            }
            else if valid_point.obstacle == 'O' || valid_point.obstacle == '[' || valid_point.obstacle == ']'
            {
                // move obstacle and robot
                let mut obstacle_moves:  Vec<(Cell, bool)> = move_obstacle_v2 (valid_point, movement.clone(), cells);

                if movement.y != 0 {
                    if valid_point.obstacle == '[' {
                        let cell_right = &cells[valid_point_index + 1];
                        let mut obstacle_moves_2: Vec<(Cell, bool)> = move_obstacle_v2(cell_right, movement.clone(), cells);
                        obstacle_moves.clear();
                        obstacle_moves.append(&mut obstacle_moves_2.clone());
                    }

                    if valid_point.obstacle == ']' {
                        let cell_left = &cells[valid_point_index - 1];
                        let mut obstacle_moves_2: Vec<(Cell, bool)> = move_obstacle_v2(cell_left, movement.clone(), cells);
                        obstacle_moves.clear();
                        obstacle_moves.append(&mut obstacle_moves_2.clone());
                    }
                }

                let mut move_everything = true;

                obstacle_moves.insert (0,(valid_point.clone(), true));

                for i in (1..obstacle_moves.len()).rev() {
                    let icon = obstacle_moves[i - 1].0.obstacle;

                    if obstacle_moves[i].1 == false {
                        move_everything = false;
                        break;
                    }

                    if (icon == '[' || icon == ']') && (movement.y == 1 || movement.y == -1)
                    {
                        if icon == '[' {
                            {
                                let new_obstacle_left = '[';
                                let cell_left = get_map_point(&obstacle_moves[i].0.position, cells).unwrap();
                                cell_left.obstacle = new_obstacle_left;
                            }

                            {
                                let new_obstacle_right = ']';
                                let right_position = obstacle_moves[i].clone().0.position + Point { x: 1, y: 0 };
                                let cell_right = get_map_point(&right_position, cells).unwrap();
                                cell_right.obstacle = new_obstacle_right;
                            }

                            // clear old cells
                            {
                                let clear_move = if movement.y > 0 { -1 } else { 1 };
                                let clear_position = obstacle_moves[i].clone().0.position + Point { x: 1, y: clear_move };
                                let cell_clear = get_map_point(&clear_position, cells).unwrap();
                                cell_clear.obstacle = '.';
                            }

                            {
                                let clear_move = if movement.y > 0 { -1 } else { 1 };
                                let clear_position = obstacle_moves[i].clone().0.position + Point { x: 0, y: clear_move };
                                let cell_clear = get_map_point(&clear_position, cells).unwrap();
                                cell_clear.obstacle = '.';
                            }
                        }
                        else
                        {
                            {
                                let new_obstacle_left = '[';
                                let left_position = obstacle_moves[i].clone().0.position + Point { x: -1, y: 0 };
                                let cell_left = get_map_point(&left_position, cells).unwrap();
                                cell_left.obstacle = new_obstacle_left;
                            }

                            {
                                let new_obstacle_right = ']';
                                let cell_right = get_map_point(&obstacle_moves[i].0.position, cells).unwrap();
                                cell_right.obstacle = new_obstacle_right;
                            }

                            // clear old cells
                            {
                                let clear_move = if movement.y > 0 { -1 } else { 1 };
                                let clear_position = obstacle_moves[i].clone().0.position + Point { x: -1, y: clear_move };
                                let cell_clear = get_map_point(&clear_position, cells).unwrap();
                                cell_clear.obstacle = '.';
                            }

                            {
                                let clear_move = if movement.y > 0 { -1 } else { 1 };
                                let clear_position = obstacle_moves[i].clone().0.position + Point { x: 0, y: clear_move };
                                let cell_clear = get_map_point(&clear_position, cells).unwrap();
                                cell_clear.obstacle = '.';
                            }
                        }
                    }
                    else {
                        let new_obstacle = obstacle_moves[i - 1].0.obstacle;

                        let cell = get_map_point(&obstacle_moves[i].0.position, cells).unwrap();
                        cell.obstacle = new_obstacle;
                    }
                }

                if move_everything {
                    {
                        robot.current_position = new_position.clone();
                        let cell = get_map_point (&robot.current_position, cells).unwrap();
                        cell.obstacle = '.';
                    }

                    if obstacle_moves[0].0.obstacle == '[' && (movement.y == 1 || movement.y == -1)
                    {
                        let delete_position = new_position.clone() + Point { x: 1, y: 0 };
                        let cell = get_map_point (&delete_position, cells).unwrap();
                        cell.obstacle = '.';
                    }
                    else if obstacle_moves[0].0.obstacle == ']' && (movement.y == 1 || movement.y == -1)
                    {
                        let delete_position = new_position.clone() + Point { x: -1, y: 0 };
                        let cell = get_map_point (&delete_position, cells).unwrap();
                        cell.obstacle = '.';
                    }
                }
            }
            else
            {
                // just move
                robot.current_position = new_position;
            }
        }
    }

    // Implement Ord and PartialOrd for Point to allow sorting
    impl Ord for Point {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.x.cmp(&other.x).then_with(|| self.y.cmp(&other.y))
        }
    }

    impl PartialOrd for Point {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    #[derive(PartialEq, Eq, Clone, Debug, Hash)]
    struct MoveOrder {
        location: Point,
        target: Point,
        allow_move: bool,
    }

    fn remove_duplicates<T: Eq + std::hash::Hash + Clone>(arr: Vec<T>) -> Vec<T> {
        let mut seen = HashSet::new();
        let mut result = Vec::new();

        for item in arr {
            if seen.insert(item.clone()) {
                result.push(item);
            }
        }

        result
    }

    fn recursive_movement (movement: Movement, cell_to_move: Cell, cells: Vec<Cell>) -> Vec<MoveOrder> {
        let (left_cell, right_cell) = get_cell_at_sides (cell_to_move.clone(), &cells);
        let obstacle = cell_to_move.obstacle;
        let mut move_orders: Vec<MoveOrder> = Vec::new();
        let target_location = cell_to_move.position.clone() + movement.clone();

        let alternate_side = if obstacle == '[' { right_cell.clone() } else { left_cell.clone() };

        if obstacle == '[' || obstacle == ']'
        {
            let next_left_point = cell_to_move.position.clone() + movement.clone();
            let next_right_point = alternate_side.position.clone() + movement.clone();

            let next_cell_left = get_map_point_copy (&next_left_point, &cells).unwrap();
            let next_cell_right = get_map_point_copy (&next_right_point, &cells).unwrap();

            let mut left_move_order = recursive_movement (movement.clone(), next_cell_left, cells.clone());
            let mut right_move_order = recursive_movement (movement.clone(), next_cell_right, cells.clone());

            let left_any_fails = if left_move_order.iter().any(|order| !order.allow_move) {true} else {false};
            let right_any_fails = if right_move_order.iter().any(|order| !order.allow_move) {true} else {false};

            if left_any_fails && right_any_fails {
                move_orders.push (MoveOrder { location: cell_to_move.position.clone(), target: target_location.clone(), allow_move: false })
            }
            else {
                move_orders.push (MoveOrder { location: cell_to_move.position.clone(), target: target_location, allow_move: true });
                move_orders.push (MoveOrder { location: alternate_side.position.clone(), target:  alternate_side.position.clone() + movement.clone(), allow_move: true });
                move_orders.append (&mut left_move_order);
                move_orders.append (&mut right_move_order);
            }
        }
        else if obstacle == '#'
        {
            move_orders.push (MoveOrder {location: cell_to_move.position.clone(), target: cell_to_move.position.clone(), allow_move: false })
        }
        else
        {
            move_orders.push (MoveOrder {location: cell_to_move.position.clone(), target: cell_to_move.position.clone(), allow_move: true })
        }

        move_orders
    }

    fn move_robot_v2 (robot: &mut Robot, movement: &Movement, cells: &mut Vec<Cell>) {
        let new_position = Point { x: robot.current_position.x + movement.x, y: robot.current_position.y + movement.y };

        if let Some(valid_point_index) = cells.iter().position(|c| c.position == new_position) {
            let valid_point = &cells[valid_point_index];
            let current_icon = valid_point.obstacle;

            if (current_icon == '[' || current_icon == ']') && movement.y != 0
            {
                let mut move_orders = recursive_movement (movement.clone(), valid_point.clone(), cells.clone());
                print!("got move orders");

                let any_fails = if move_orders.iter().any(|order| !order.allow_move) {true} else {false};

                if any_fails {
                    return;
                }

                let mut unique_moves= remove_duplicates (move_orders);

                if movement.y == 1 {
                    unique_moves.sort_by_key(|mo| mo.target.clone());
                }
                else {
                    unique_moves.sort_by_key(|mo| Reverse(mo.target.clone()));
                }

                for order in unique_moves.iter().rev() {
                    if (order.location == order.target) {
                        continue;
                    }

                    let location_index = cells.iter().position(|c| c.position == order.location).unwrap();
                    let target_index = cells.iter().position(|c| c.position == order.target,).unwrap();

                    let location_char = get_map_point_char (&order.location, cells).unwrap();
                    let target_char = get_map_point_char (&order.target, cells).unwrap();

                    {
                        let target = &mut cells[target_index];
                        target.obstacle = location_char;
                    }

                    {
                        let location = &mut cells[location_index];
                        location.obstacle = target_char;
                    }
                }

                robot.current_position = robot.current_position.clone() + movement.clone();
            }
            else if (current_icon == '[' || current_icon == ']') && movement.y == 0
            {
                let mut obstacle_moves:  Vec<(Cell, bool)> = move_obstacle_v2 (valid_point, movement.clone(), cells);

                let mut move_everything = true;

                obstacle_moves.insert (0,(valid_point.clone(), true));

                for i in (1..obstacle_moves.len()).rev() {
                    let icon = obstacle_moves[i - 1].0.obstacle;

                    if obstacle_moves[i].1 == false {
                        move_everything = false;
                        break;
                    }

                    {
                        let new_obstacle = obstacle_moves[i - 1].0.obstacle;

                        let cell = get_map_point(&obstacle_moves[i].0.position, cells).unwrap();
                        cell.obstacle = new_obstacle;
                    }
                }

                if move_everything {
                    robot.current_position = new_position.clone();
                    let cell = get_map_point(&robot.current_position, cells).unwrap();
                    cell.obstacle = '.';
                }
            }
            else if current_icon == '.'
            {
                robot.current_position = robot.current_position.clone() + movement.clone();
            }
        }
    }

    fn process_robot_orders (robot: &mut Robot, cells: &mut Vec<Cell>, moves: &Vec<Movement>, map_info: &MapInfo) {
        draw_map (&map_info, robot, cells);

        let mut cnt = 0;

        for m in moves.iter() {
            move_robot_v2 (robot, m, cells);

            // draw map
            // if cnt > 7363 {
            //     draw_map(&map_info, robot, cells);
            // }
            cnt += 1;
        }
        draw_map (&map_info, robot, cells);
    }

    fn draw_map (map_size: &MapInfo, robot: &Robot, cells: &Vec<Cell>) {
        let mut map: String = String::from("");

        // place map
        for h in 0..map_size.max_rows {
            for w in 0..map_size.max_cols {

                let point = Point { x: w as i32, y: h as i32};

                if robot.current_position == point {
                    map.push ('@');
                }
                else
                {
                    if let Some(c) = get_map_point_char (&point, cells) {
                        map.push(c);
                    } else {
                        panic!("nope");
                    }
                }
            }

            map.push('\n');
        }

        println!("{}", map);
    }

    fn sum_all_boxes (cells: &Vec<Cell>) ->u64 {

        let mut sum: u64 = 0;

        for cell in cells.iter() {
            if cell.obstacle != 'O' {
                continue;
            }

            let value:u64 = 100 * cell.position.y as u64 + cell.position.x as u64;
            sum += value;
        }

        sum
    }

    // v2
    fn convert_to_map_points_v2 (content: &str, points: &mut Vec<Cell>, robot: &mut Robot) -> MapInfo {

        let mut row: usize = 0;
        let mut col: usize = 0;

        let mut max_rows: usize = 0;
        let mut max_cols: usize = 0;

        let mut id = 0;

        for line in content.lines()
        {
            for c in line.chars()
            {
                let pos_1 =  Point { x: col as i32, y: row as i32 };
                let pos_2 =  Point { x: (col + 1) as i32, y: row as i32 };
                let mut insert_c_1 = c;
                let mut insert_c_2 = c;

                if c == '@' {
                    robot.current_position = pos_1.clone();
                    robot.starting_position = pos_1.clone();
                    robot.id = id;
                    id += 1;

                    insert_c_1 = '.';
                    insert_c_2 = '.';
                }

                if c == 'O' {
                    insert_c_1 = '[';
                    insert_c_2 = ']';
                }

                points.push (Cell { position: pos_1.clone(), obstacle: insert_c_1 });
                points.push (Cell { position: pos_2.clone(), obstacle: insert_c_2 });
                col += 2;
            }

            max_cols = max_cols.max (col);
            max_rows = max_rows.max (row);

            col = 0;
            row += 1;
        }

        max_rows += 1;

        MapInfo { max_rows, max_cols }
    }

    fn move_obstacle_v2 (obstacle: &Cell, movement: Movement, cells: &Vec<Cell>) -> Vec<(Cell, bool)> {
        let new_position = obstacle.clone().position.add (movement.clone());

        let mut mark_to_move : Vec<(Cell, bool)> = Vec::new();

        // Find the index of the valid point to avoid multiple mutable borrows
        if let Some(valid_point_index) = cells.iter().position(|c| c.position == new_position) {
            let valid_point = &cells[valid_point_index];

            if valid_point.obstacle == '#'
            {
                mark_to_move.push ((valid_point.clone(), false));
            }
            else if valid_point.obstacle == 'O' || valid_point.obstacle == '[' || valid_point.obstacle == ']'
            {
                let moves = move_obstacle_v2 (valid_point, movement.clone(), cells);

                let mut box_can_move = false;

                if valid_point.obstacle == '[' && (movement.y == 1 || movement.y == -1) {
                    let valid_point_right = &cells[valid_point_index + 1];
                    let move_right_bracket = move_obstacle (valid_point_right, movement.clone(), cells);
                    box_can_move = move_right_bracket[0].1;
                }
                else if valid_point.obstacle == ']' && (movement.y == 1 || movement.y == -1) {
                    let valid_point_left = &cells[valid_point_index - 1];
                    let move_left_bracket = move_obstacle (valid_point_left, movement.clone(), cells);
                    box_can_move = move_left_bracket[0].1;
                }
                else
                {
                    box_can_move = true;
                }

                mark_to_move.push ((valid_point.clone(), box_can_move));
                mark_to_move.append(&mut moves.clone());
            }
            else
            {
                mark_to_move.push((valid_point.clone(), true));
            }
        }

        mark_to_move
    }

    fn sum_all_boxes_v2 (cells: &Vec<Cell>) -> u64 {

        let mut sum: u64 = 0;

        for cell in cells.iter() {
            if cell.obstacle != '[' {
                continue;
            }

            let value:u64 = 100 * cell.position.y as u64 + cell.position.x as u64;
            sum += value;
        }

        sum
    }


    // this ones a mess! two me too long, I'm moving on!
    pub fn do_task()
    {
        // mini
        // let contents = utils::read_file("/Users/reubenduckering/Documents/Personal Repo/Advent-Of-Code-2024/files/day_15_mini.txt");
        // let instructions = utils::read_file("/Users/reubenduckering/Documents/Personal Repo/Advent-Of-Code-2024/files/day_15_mini_instructions.txt");

        // full
        let contents = utils::read_file("/Users/reubenduckering/Documents/Personal Repo/Advent-Of-Code-2024/files/day_15.txt");
        let instructions = utils::read_file("/Users/reubenduckering/Documents/Personal Repo/Advent-Of-Code-2024/files/day_15_instructions.txt");

        // map
        let mut cells: Vec<Cell> = Vec::new();
        let mut robot = Robot { current_position: Point { x: 0, y:0 }, starting_position: Point {x: 0, y: 0 }, id: 0 };
        let map_info = convert_to_map_points_v2 (&contents, &mut cells, &mut robot);

        // instructions
        let mut movements: Vec<Movement> = Vec::new();
        parse_instructions (&instructions, &mut movements);

        process_robot_orders (&mut robot, &mut cells, &mut movements, &map_info);

        let sum = sum_all_boxes_v2 (&cells);
        println!("sum:{}", sum);
    }
}