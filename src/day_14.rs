pub mod day_14
{
    use crate::utils::utils;

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_move_robot() {
            let mut inputs = vec![Robot { starting_position: (5, 6), movement: (6, 4), current_position: (5, 7), id: 0 }];
            let map_size = MapSize { width: 11, height: 7 };

            manually_move_robots (&mut inputs, &map_size);

            assert_eq!(inputs[0].current_position.0, 0);
            assert_eq!(inputs[0].current_position.1, 4);
        }

        #[test]
        fn test_move_robot_2() {
            let mut inputs = vec![Robot { starting_position: (1, 1), movement: (-3, -3), current_position: (1, 1), id: 0 }];
            let map_size = MapSize { width: 11, height: 7 };

            manually_move_robots (&mut inputs, &map_size);

            assert_eq!(inputs[0].current_position.0, 9);
            assert_eq!(inputs[0].current_position.1, 5);
        }
    }

    #[derive(PartialEq, Eq, Clone, Debug)]
    struct Robot {
        starting_position: (i32, i32),
        movement: (i32, i32),
        current_position: (i32, i32),
        id: u32,
    }

    #[derive(PartialEq, Eq, Clone, Debug)]
    struct MapSize {
        width:i32,
        height:i32,
    }

    fn parse_robots (content : &str) -> Vec<Robot> {

        // p=0,4 v=3,-3
        let mut robots : Vec<Robot> = Vec::new();
        let mut id = 0;

        for line in content.split('\n') {
            let commands = line.split_whitespace().collect::<Vec<&str>>();


            let p = commands[0].strip_prefix("p=").unwrap().split_once(",").unwrap();
            let v = commands[1].strip_prefix("v=").unwrap().split_once(",").unwrap();

            let starting_position =  (p.0.parse::<i32>().unwrap(), p.1.parse::<i32>().unwrap());
            let movement = (v.0.parse::<i32>().unwrap(), v.1.parse::<i32>().unwrap());

            robots.push(Robot { starting_position, movement, current_position: starting_position, id });
            id += 1;
        }

        robots
    }

    fn get_robot_in_position (robots: &Vec<Robot>, position: (i32, i32)) -> Option<u32> {

        for robot in robots {
            if robot.current_position == position {
                return Some(robot.id);
            }
        }

        None
    }

    fn draw_map (map_size: &MapSize, robots: &Vec<Robot>, index: i32) {
        let mut map: String = String::from("");

        // place map
        for h in 0..map_size.height {
            for w in 0..map_size.width {

                if let Some (_) = get_robot_in_position (robots, (w, h)) {
                    map.push ('x');
                }
                else
                {
                    map.push ('.');
                }
            }
            map.push('\n');
        }

        for line in map.to_string().lines() {
            if line.contains ("xxxxxxxxxxxx") {
                println! ("pattern ? ");
            }
        }

        println!("{}", index);
        println!("{}", map);
    }

    fn manually_move_robots (robots: &mut Vec<Robot>, map_size: &MapSize) {

        for robot in robots {
            robot.current_position.0 += robot.movement.0;
            robot.current_position.1 += robot.movement.1;

            let mut x = robot.current_position.0 % map_size.width;
            let mut y = robot.current_position.1 % map_size.height;

            if x < 0 {
                x += map_size.width;
            }

            if y < 0 {
                y += map_size.height;
            }

            robot.current_position.0 = x;
            robot.current_position.1 = y;
        }
    }

    fn count_quadrants (robots: &Vec<Robot>, map_size: &MapSize) -> i32 {

        let row_to_delete = map_size.height / 2;
        let column_to_delete = map_size.width / 2;

        let mut count_robot: Vec<i32> = vec! [0, 0, 0, 0];

        for robot in robots {
            if robot.current_position.0 == column_to_delete {
                continue;
            }

            if robot.current_position.1 == row_to_delete {
                continue;
            }

            if robot.current_position.0 < column_to_delete && robot.current_position.1 < row_to_delete{
                count_robot[0] += 1;
            }
            else if robot.current_position.0 > column_to_delete && robot.current_position.1 < row_to_delete {
                count_robot[1] += 1;
            }
            else if robot.current_position.0 < column_to_delete && robot.current_position.1 > row_to_delete {
                count_robot[2] += 1;
            }
            else if robot.current_position.0 > column_to_delete && robot.current_position.1 > row_to_delete {
                count_robot[3] += 1;
            }
        }

        count_robot.iter().fold(1, |acc, &x| acc * x)
    }

    fn look_for_tree (robots: &Vec<Robot>) -> (i64, i64) {

        let variance_x:i64 =  robots.iter().fold (1, |acc, x| acc + (x.current_position.0 as i64 * x.current_position.0 as i64)) / robots.len() as i64;
        let variance_y:i64 =  robots.iter().fold (1, |acc, x| acc + (x.current_position.1 as i64 * x.current_position.1 as i64)) / robots.len() as i64;

        (variance_x, variance_y)
    }

    pub fn do_task()
    {
        // let contents = utils::read_file("/Users/reubenduckering/Documents/Personal Repo/Advent-Of-Code-2024/files/day_14_mini.txt");
        let contents = utils::read_file("/Users/reubenduckering/Documents/Personal Repo/Advent-Of-Code-2024/files/day_14.txt");

        let map_size = MapSize { width: 101, height: 103 };
        // let map_size = MapSize { width: 11, height: 7 };
        let mut robots = parse_robots (&contents);

        let mut variance: Vec<(i64, i64)> = Vec::new();

        for i in 6300..6400 {
            draw_map (&map_size, &robots, i);
            variance.push (look_for_tree (&robots));
            manually_move_robots (&mut robots, &map_size);
        }

        let robot_count = count_quadrants (&robots, &map_size);
        draw_map (&map_size, &robots, 200);

        // print variance
        let mut index= 0;
        for v in variance {
            println!("variance - index{}: {},{}", index, v.0, v.1);
            index += 1;
        }

        println!("robots: {}", robot_count);
    }
}