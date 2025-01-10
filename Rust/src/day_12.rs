pub mod day_12
{
    use std::collections::HashSet;
    use crate::utils::utils;

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_convert_to_map_points() {
            let content = "rd\nfd";
            let mut vec: Vec<Point> = Vec::new();

            convert_to_map_points (content, &mut vec);
            let expected: Vec<Point> = vec![Point{ row: 0, col: 0, letter: 'r', unique_id: 0}, Point{ row: 0, col: 1, letter: 'd', unique_id: 1},
                                            Point{ row: 1, col: 0, letter: 'f', unique_id: 2}, Point{ row: 1, col: 1, letter: 'd', unique_id: 3}];

            assert_eq!(vec[0], expected[0]);
            assert_eq!(vec[1], expected[1]);
            assert_eq!(vec[2], expected[2]);
            assert_eq!(vec[3], expected[3]);
        }

        #[test]
        fn test_get_map_info() {
            let content = "rde\nfdh";
            let mut vec: Vec<Point> = Vec::new();

            convert_to_map_points (content, &mut vec);
            let map_info = get_map_info (&vec);
            let mut expected: HashSet<char> = HashSet::new();
            expected.insert('r');
            expected.insert('d');
            expected.insert('e');
            expected.insert('f');
            expected.insert('h');


            assert_eq!(map_info.max_rows, 2);
            assert_eq!(map_info.max_cols, 3);
            assert_eq!(map_info.flower_types, expected);
        }

        #[test]
        fn test_pair_horizontal() {
            let content = "rdee\nfddh";
            let mut vec: Vec<Point> = Vec::new();

            convert_to_map_points (content, &mut vec);
            let map_info = get_map_info (&vec);

            let mut flower_points: Vec<FlowerGroup> = Vec::new();
            pair_horizontal (&vec, &mut flower_points, &map_info);

            assert_eq!(flower_points[0].flower_type, 'r');
            assert_eq!(flower_points[0].grouped_flowers.len(), 1);
            assert_eq!(flower_points[0].grouped_flowers[0], Point { row:0, col:0, letter: 'r', unique_id: 0 });

            assert_eq!(flower_points[1].flower_type, 'd');
            assert_eq!(flower_points[1].grouped_flowers.len(), 1);
            assert_eq!(flower_points[1].grouped_flowers[0], Point { row:0, col:1, letter: 'd', unique_id: 1 });

            assert_eq!(flower_points[2].flower_type, 'e');
            assert_eq!(flower_points[2].grouped_flowers.len(), 2);
            assert_eq!(flower_points[2].grouped_flowers[0], Point { row:0, col:2, letter: 'e', unique_id: 2 });

            assert_eq!(flower_points[3].flower_type, 'f');
            assert_eq!(flower_points[3].grouped_flowers.len(), 1);
            assert_eq!(flower_points[3].grouped_flowers[0], Point { row:1, col:0, letter: 'f', unique_id: 4 });

            assert_eq!(flower_points[4].flower_type, 'd');
            assert_eq!(flower_points[4].grouped_flowers.len(), 2);
            assert_eq!(flower_points[4].grouped_flowers[0], Point { row:1, col:1, letter: 'd', unique_id: 5 });

            assert_eq!(flower_points[5].flower_type, 'h');
            assert_eq!(flower_points[5].grouped_flowers.len(), 1);
            assert_eq!(flower_points[5].grouped_flowers[0], Point { row:1, col:3, letter: 'h', unique_id: 7 });
        }

        #[test]
        fn test_get_area()
        {
            let vec: Vec<FlowerGroup> = vec![FlowerGroup {flower_type: 'c', grouped_flowers: vec![Point{ row:0, col:0, letter: 'c', unique_id: 0 }], ranged_flowers: Vec::new(), unique_id: 0 },
                                             FlowerGroup {flower_type: 'd', grouped_flowers: vec![Point{ row:0, col:0, letter: 'd', unique_id: 1 },
                                                                                                  Point{ row:0, col:1, letter: 'd', unique_id: 2 },], ranged_flowers: Vec::new(), unique_id: 1 }];

            assert_eq!(get_area('c', &vec), 1);
            assert_eq!(get_area('d', &vec), 2);
        }

        #[test]
        fn test_group_flowers() {
            let content = "rdde\nfddh\nfeej";
            let mut vec: Vec<Point> = Vec::new();

            convert_to_map_points (content, &mut vec);
            let map_info = get_map_info (&vec);

            let mut flower_points: Vec<FlowerGroup> = Vec::new();
            pair_horizontal (&vec, &mut flower_points, &map_info);
            group_flower_groups (&mut flower_points);

            assert_eq!(flower_points.len(), 7);

            assert_eq!(flower_points[0].grouped_flowers.len(), 1); // r
            assert_eq!(flower_points[1].grouped_flowers.len(), 1); // e
            assert_eq!(flower_points[2].grouped_flowers.len(), 4); // d
            assert_eq!(flower_points[3].grouped_flowers.len(), 1); // h
            assert_eq!(flower_points[4].grouped_flowers.len(), 2); // f
            assert_eq!(flower_points[5].grouped_flowers.len(), 2); // e
            assert_eq!(flower_points[6].grouped_flowers.len(), 1); // j
        }

        #[test]
        fn test_group_flowers_2() {
            let content = "rrrr\nrddr";
            let mut vec: Vec<Point> = Vec::new();

            convert_to_map_points (content, &mut vec);
            let map_info = get_map_info (&vec);

            let mut flower_points: Vec<FlowerGroup> = Vec::new();
            pair_horizontal (&vec, &mut flower_points, &map_info);
            group_flower_groups (&mut flower_points);

            assert_eq!(flower_points.len(), 2);

            assert_eq!(flower_points[0].grouped_flowers.len(), 2); // d
            assert_eq!(flower_points[1].grouped_flowers.len(), 6); // r
        }

        #[test]
        fn test_surrounding_flowers() {
            let content = "rrrr\nrddr\neegg";
            let mut vec: Vec<Point> = Vec::new();

            convert_to_map_points(content, &mut vec);
            let map_info = get_map_info(&vec);

            let mut flower_points: Vec<FlowerGroup> = Vec::new();
            pair_horizontal(&vec, &mut flower_points, &map_info);
            group_flower_groups(&mut flower_points);

            {
                let border = surrounding_letter(&vec, &vec[0]);
                assert_eq!(border, (2, 1));
            }

            {
                let border = surrounding_letter(&vec, &vec[1]);
                assert_eq!(border, (2, 0));
            }

            {
                let border = surrounding_letter(&vec, &vec[3]);
                assert_eq!(border, (2,1));
            }

            {
                let border = surrounding_letter(&vec, &vec[7]);
                assert_eq!(border, (3,3));
            }

            {
                let border = surrounding_letter(&vec, &vec[4]);
                assert_eq!(border, (3,3));
            }
        }

        #[test]
        fn test_calculate_fence() {
            let content = "rrrr\n\
                                 rddr\n\
                                 eegg";
            let mut vec: Vec<Point> = Vec::new();

            convert_to_map_points(content, &mut vec);
            let map_info = get_map_info(&vec);

            let mut flower_points: Vec<FlowerGroup> = Vec::new();
            pair_horizontal(&vec, &mut flower_points, &map_info);
            group_flower_groups(&mut flower_points);

            {
                let border = calculate_fence (&vec, &flower_points[0]);
                assert_eq!(border, (6, 4));
            }

            {
                let border = calculate_fence (&vec, &flower_points[1]);
                assert_eq!(border, (14, 8));
            }
        }

        #[test]
        fn test_calculate_fence_1() {
            let content = "abcd\n\
                                 ebbr\n\
                                 uibp";
            let mut vec: Vec<Point> = Vec::new();

            convert_to_map_points(content, &mut vec);
            let map_info = get_map_info(&vec);

            let mut flower_points: Vec<FlowerGroup> = Vec::new();
            pair_horizontal(&vec, &mut flower_points, &map_info);
            group_flower_groups(&mut flower_points);

            {
                let border = calculate_fence (&vec, &flower_points[7]);
                assert_eq!(border, (10, 8));
            }
        }

        #[test]
        fn test_calculate_example_1() {
            let content = "AAAA\nBBCD\nBBCC\nEEEC";
            let mut vec: Vec<Point> = Vec::new();

            convert_to_map_points(content, &mut vec);
            let map_info = get_map_info(&vec);

            let mut flower_points: Vec<FlowerGroup> = Vec::new();
            pair_horizontal(&vec, &mut flower_points, &map_info);
            group_flower_groups(&mut flower_points);

            let mut cost1  = 0;
            let mut cost2  = 0;

            for flower_group in flower_points {
                let cost = calculate_cost(&vec, &flower_group);
                cost1 += cost.0;
                cost2 += cost.1;
            }

            assert_eq!(cost1, 140);
            assert_eq!(cost2, 80);
        }

        #[test]
        fn test_calculate_example_2() {
            let content = "EEEEE\nEXXXX\nEEEEE\nEXXXX\nEEEEE";
            let mut vec: Vec<Point> = Vec::new();

            convert_to_map_points(content, &mut vec);
            let map_info = get_map_info(&vec);

            let mut flower_points: Vec<FlowerGroup> = Vec::new();
            pair_horizontal(&vec, &mut flower_points, &map_info);
            group_flower_groups(&mut flower_points);

            let mut _cost1  = 0;
            let mut cost2  = 0;

            for flower_group in flower_points {
                let cost = calculate_cost(&vec, &flower_group);
                _cost1 += cost.0;
                cost2 += cost.1;
            }


            assert_eq!(cost2, 236);
        }

        #[test]
        fn test_calculate_example_3() {
            let content = "AAAAAA\nAAABBA\nAAABBA\nABBAAA\nABBAAA\nAAAAAA";

            let mut vec: Vec<Point> = Vec::new();

            convert_to_map_points(content, &mut vec);
            let map_info = get_map_info(&vec);

            let mut flower_points: Vec<FlowerGroup> = Vec::new();
            pair_horizontal(&vec, &mut flower_points, &map_info);
            group_flower_groups(&mut flower_points);

            let mut _cost1  = 0;
            let mut cost2  = 0;

            for flower_group in flower_points {
                let cost = calculate_cost(&vec, &flower_group);
                _cost1 += cost.0;
                cost2 += cost.1;
            }

            assert_eq!(cost2, 368);
        }
    }

    #[derive(PartialEq, Eq, Clone, Debug)]
    struct Point {
        row: usize,
        col: usize,
        letter: char,
        unique_id: usize,
    }

    #[derive(PartialEq, Eq, Clone, Debug)]
    struct RangePoints {
        min: Point,
        max: Point,
        row: usize,
        all_points: Vec<Point>
    }

    #[derive(PartialEq, Eq, Clone, Debug)]
    struct FlowerGroup {
        flower_type: char,
        grouped_flowers: Vec<Point>,
        ranged_flowers: Vec<RangePoints>,
        unique_id: usize,
    }

    struct MapInfo {
        max_rows: usize,
        max_cols: usize,
        flower_types: HashSet<char>,
    }

    fn convert_to_map_points (content: &str, points: &mut Vec<Point>) {

        let mut row: usize = 0;
        let mut col: usize = 0;

        let mut id = 0;

        for line in content.lines()
        {
            for c in line.chars()
            {
                points.push (Point { row, col, letter: c, unique_id: id });

                id += 1;
                col += 1;
            }
            col = 0;
            row += 1;
        }
    }

    fn get_map_info (points: &Vec<Point>) -> MapInfo {

        let (mut max_rows, mut max_cols) = points.iter().fold((0,0), |(max_r, max_c), p| { (
            max_r.max (p.row),
            max_c.max (p.col)
        ) });

        max_rows += 1;
        max_cols += 1;

        let mut flower_types = HashSet::new();
        for p in points {
            flower_types.insert (p.letter);
        }

        MapInfo { max_rows, max_cols, flower_types }
    }

    fn get_point (points: &Vec<Point>, row: usize, col: usize) ->  Option<&Point> {

        for p in points {
            if p.row == row && p.col == col {
                return Some (p);
            }
        }

        None
    }

    fn make_range (points: &Vec<Point>) -> RangePoints {

        let mut min_col = 10000;
        let mut max_col = 0;

        let mut min_point = points[0].clone();
        let mut max_point = points[0].clone();

        for p in points {
            if p.col < min_col {
                min_col = p.col;
                min_point = p.clone();
            };

            if p.col > max_col {
                max_col = p.col;
                max_point = p.clone();
            }
        }

        let row = min_point.row;

        // let mut ranged_points = RangePoints {};
        RangePoints {min: min_point, max: max_point, row, all_points: points.clone()}
    }

    fn pair_horizontal (points: &Vec<Point>, flower_groups: &mut Vec<FlowerGroup>, map_info: &MapInfo) {

        let mut id = 0;

        for row in 0..map_info.max_rows
        {
            let mut prev_flower = ' ';
            let mut f_groups = FlowerGroup {flower_type: '1', grouped_flowers: Vec::new(), ranged_flowers: Vec::new(), unique_id: id };

            for col in 0..map_info.max_cols
            {
                if let Some (p) = get_point(&points, row, col)
                {
                    if prev_flower != p.letter
                    {
                        if f_groups.grouped_flowers.len() > 0 {
                            let range = make_range (&f_groups.grouped_flowers);
                            f_groups.ranged_flowers.push (range);
                            flower_groups.push(f_groups.clone());
                            f_groups.grouped_flowers.clear();
                            f_groups.ranged_flowers.clear();
                            id += 1;
                            f_groups.unique_id = id;
                        }

                        f_groups.flower_type = p.letter;
                        f_groups.grouped_flowers.push (p.clone());

                    }
                    else
                    {
                        f_groups.grouped_flowers.push (p.clone());
                    }

                    prev_flower = p.letter;
                }
            }

            let range = make_range (&f_groups.grouped_flowers);
            f_groups.ranged_flowers.push (range);
            flower_groups.push (f_groups.clone());
            id += 1;
        }
    }

    fn merge_group (group1: &FlowerGroup, group2: &FlowerGroup) -> FlowerGroup {
        let mut new_group = FlowerGroup { flower_type: group1.flower_type, grouped_flowers: group1.grouped_flowers.clone(),
                                          ranged_flowers: group1.ranged_flowers.clone(), unique_id: group2.unique_id };

        for p in group2.grouped_flowers.iter() {
            new_group.grouped_flowers.push (p.clone());
        }

        for p in group2.ranged_flowers.iter() {
            new_group.ranged_flowers.push (p.clone());
        }

        new_group
    }

    fn group_flower_groups (flower_groups: &mut Vec<FlowerGroup>) {

        let mut groups_to_remove: HashSet<usize> = HashSet::new();
        let mut i: usize = 0;

        // for i in 1..flower_groups.len()
        loop
        {
            if i >= flower_groups.len() { break; }

            for p in flower_groups[i].grouped_flowers.clone()
            {
                let mut j: usize = i + 1;

                // for j in i..flower_groups.len()
                loop
                {
                    if j >= flower_groups.len() { break; }

                    for p2 in flower_groups[j].grouped_flowers.clone()
                    {
                        if p.row + 1 == p2.row && p.col == p2.col && p.letter == p2.letter
                        {
                            if groups_to_remove.contains (&flower_groups[i].unique_id) {
                                continue;
                            }

                            let new_group = merge_group(&flower_groups[i], &flower_groups[j]);
                            flower_groups[j] = new_group;
                            groups_to_remove.insert (flower_groups[i].unique_id);
                            break;
                        }
                    }

                    j += 1;
                }
            }
            i += 1;
        }

        // delete old groups

        for index in (0..flower_groups.len()).rev() {
            if groups_to_remove.contains (&flower_groups[index].unique_id) {
                flower_groups.remove(index);
            }
        }
    }

    fn get_area (flower_letter: char, flowers: &Vec<FlowerGroup>) -> usize {

        for flower in flowers {
            if flower.flower_type == flower_letter {
                return flower.grouped_flowers.len()
            }
        }

        panic!("Flower {} not found", flower_letter);
    }

    struct EmptySpace {
        up: bool,
        down: bool,
        left: bool,
        right: bool,

        left_up: bool,
        left_down: bool,
        right_up: bool,
        right_down: bool,
    }

    fn empty_space_to_corner (grid: &EmptySpace) -> i32 {

        let mut corner = 0;

        // o - o
        // | x |
        // t f f
         if grid.left && grid.up && grid.right && ! grid.down
             && grid.left_down && ! grid.right_down {
             corner = 3
         }

        // o - o
        // | x |
        // f f t
        else if grid.left && grid.up && grid.right && ! grid.down
            && ! grid.left_down && grid.right_down {
            corner = 3
        }

        // f f t
        // | x |
        // o - o
        else if grid.left && grid.down && grid.right && ! grid.up
            && ! grid.left_up && grid.right_up {
            corner = 3
        }

        // t t f
        // | x |
        // o - o
        else if grid.left && grid.down && grid.right && ! grid.up
            && grid.left_up && ! grid.right_up {
            corner = 3
        }

        // f - f
        // f x f
        // o o o
        else if ! grid.left && ! grid.right && grid.up
            && ! grid.left_up && ! grid.right_up {
            corner = 2
        }

        // o f f
        // o x |
        // o f f
        else if ! grid.up && grid.right && ! grid.down
            && ! grid.right_up && ! grid.right_down {
            corner = 2
        }

        // o o o
        // f x f
        // f - f
        else if ! grid.left && ! grid.right && grid.down
            && ! grid.left_down && ! grid.right_down {
            corner = 2
        }

        // f f o
        // | x o
        // f f o
        else if grid.left && ! grid.up && !grid.down
            && ! grid.left_down && ! grid.left_up {
            corner = 2
        }


        // o f t
        // o x f
        // o o o
        else if ! grid.right && ! grid.up &&  grid.right_up {
            corner = 1;
        }

        // t f o
        // f x o
        // o o o
        else if ! grid.left && ! grid.up && grid.left_up {
            corner = 1;
        }

        // o o o
        // o x f
        // o f t
        else if ! grid.right && ! grid.down && grid.right_down {
            corner = 1;
        }

        // o o o
        // f x o
        // t f o
        else if ! grid.left && ! grid.down && grid.left_down {
            corner = 1;
        }

            // ==================

        // o f o
        // f x t
        // o t f
        else if ! grid.left && ! grid.up
            && grid.down && grid.right
            && ! grid.right_down {
            corner = 2;
        }

        // o t f
        // f x t
        // o f o
        else if ! grid.left && grid.up
            && ! grid.down && grid.right
            && ! grid.right_up {
            corner = 2;
        }

        // f t o
        // t x f
        // o f o
        else if grid.left && grid.up
            && ! grid.down && ! grid.right
            && ! grid.left_up {
            corner = 2;
        }


        // o f o
        // t x f
        // f t o
        else if grid.left && ! grid.up
            && grid.down && ! grid.right
            && ! grid.left_down {
            corner = 2;
        }

        // =====================


        // o f o
        // f x t
        // o t o
        else if ! grid.left && ! grid.up
               && grid.down && grid.right {
            corner = 1;
        }

        // o t o
        // f x t
        // o f o
        else if ! grid.left && grid.up
            && ! grid.down && grid.right {
            corner = 1;
        }

        // o t o
        // t x f
        // o f o
        else if grid.left && grid.up
            && ! grid.down && ! grid.right {
            corner = 1;
        }

        // o f o
        // t x f
        // o t o
        else if grid.left && ! grid.up
            && grid.down && ! grid.right {
            corner = 1;
        }

        //==============

        // o - o
        // | x o
        // o o o
        else if grid.left && grid.up && ! grid.down && ! grid.right {
            corner = 1;
        }

        // o - o
        // o x |
        // o o o
        else if grid.right && grid.up && ! grid.down && ! grid.left {
            corner = 1;
        }

        // o o o
        // o x |
        // o - o
        else if grid.right && grid.down && ! grid.up && ! grid.left {
            corner = 1;
        }

        // o o o
        // | x o
        // o - o
        else if grid.left && grid.down && ! grid.up && ! grid.right {
            corner = 1;
        }

        // o - o
        // | x o
        // o - o
        else if grid.left && grid.down && grid.up && ! grid.right {
            corner = 2;
        }

        // o - o
        // o x |
        // o - o
        else if grid.right && grid.down && grid.up && ! grid.left {
            corner = 2;
        }

        // o o o
        // | x |
        // o - o
        else if grid.right && grid.down && grid.left && ! grid.up {
            corner = 2;
        }

        // o - o
        // | x |
        // o o o
        else if grid.right && grid.up && grid.left && ! grid.down {
            corner = 2;
        }

        // o - o
        // | x |
        // o - o
        else if grid.right && grid.up && grid.left && grid.down {
            corner = 4;
        }



        corner
    }

    fn surrounding_letter (points: &Vec<Point>, point_to_test: &Point) -> (i32, i32) {
        // check surrounding points

        let letter = point_to_test.letter;
        let mut border = 0;
        let mut border_direction = EmptySpace { up:false, down:false, left:false, right:false, left_down:false, left_up:false, right_up:false, right_down:false };

        let up = point_to_test.row as i32 - 1;

        if up >= 0
        {
            if let Some (up_p) = get_point (points, up as usize, point_to_test.col) {
                if up_p.letter != letter {
                    border += 1;
                    border_direction.up = true;
                }
            } else { border += 1; border_direction.up = true; }
        }
        else { border += 1; border_direction.up = true; }

        let down = point_to_test.row as i32 + 1;

        if down >= 0
        {
            if let Some (down_p) = get_point (points, down as usize, point_to_test.col) {
                if down_p.letter != letter {
                    border += 1;
                    border_direction.down = true;
                }
            } else { border += 1; border_direction.down = true; }
        }
        else { border += 1; border_direction.down = true; }

        let left = point_to_test.col as i32 - 1;

        if left >= 0
        {
            if let Some (left_p) = get_point (points, point_to_test.row, left as usize) {
                if left_p.letter != letter {
                    border += 1;
                    border_direction.left = true;
                }
            } else { border += 1; border_direction.left = true; }
        }
        else { border += 1; border_direction.left = true; }

        let right = point_to_test.col as i32 + 1;

        if right >= 0
        {
            if let Some (right_p) = get_point (points, point_to_test.row, right as usize) {
                if right_p.letter != letter {
                    border += 1;
                    border_direction.right = true;
                }
            }
            else { border += 1; border_direction.right = true; }
        }
        else { border += 1; border_direction.right = true; }

        // figure out corners

        // left up
        if point_to_test.col as i32 - 1 >= 0 && point_to_test.row as i32 - 1 >= 0
        {
            if let Some (p) = get_point (points, point_to_test.row - 1, point_to_test.col - 1) {
                if p.letter != letter {
                    border_direction.left_up = true;
                }
            } else { border_direction.left_up = true; }
        } else { border_direction.left_up = true; }

        // right up
        if point_to_test.col as i32 + 1 >= 0 && point_to_test.row as i32 - 1 >= 0
        {
            if let Some (p) = get_point (points, point_to_test.row - 1, point_to_test.col + 1) {
                if p.letter != letter {
                    border_direction.right_up = true;
                }
            } else { border_direction.right_up = true; }
        }  else { border_direction.right_up = true; }

        // left down
        if point_to_test.col as i32 - 1 >= 0 && point_to_test.row as i32 + 1 >= 0
        {
            if let Some (p) = get_point (points, point_to_test.row + 1, point_to_test.col - 1) {
                if p.letter != letter {
                    border_direction.left_down = true;
                }
            } else { border_direction.left_down = true; }
        } else { border_direction.left_down = true; }

        // right down
        if point_to_test.col as i32 + 1 >= 0 && point_to_test.row as i32 + 1 >= 0
        {
            if let Some (p) = get_point (points, point_to_test.row + 1, point_to_test.col + 1) {
                if p.letter != letter {
                    border_direction.right_down = true;
                }
            } else { border_direction.right_down = true; }
        }
        else { border_direction.right_down = true; }

        // this never worked
        let corner = empty_space_to_corner (&border_direction);

        (border, corner)
    }

    fn calculate_fence (points: &Vec<Point>, flower_group: &FlowerGroup) -> (i32,i32) {

        let mut border = 0;
        let mut corner = 0;

        for point in flower_group.grouped_flowers.iter() {
            let result = surrounding_letter (points, point);
            border += result.0;
            corner += result.1;
        }

        (border, corner)
    }

    fn calculate_cost (points: &Vec<Point>, flower_group: &FlowerGroup) -> (u32, u32) {

        let area = flower_group.grouped_flowers.len() as u32;

        let border_size = calculate_fence (points, flower_group);

        let corners = calculate_corners (&flower_group);

        (area * (border_size.0 as u32), area * (corners as u32))
    }


    fn find_connected_range (flower_range: &RangePoints, point: &Point, flower_group: &FlowerGroup, row: i32) -> Option<RangePoints> {

        let row_to_find = flower_range.row as i32 + row;

        if row_to_find < 0 {
            return None;
        }

        for r in flower_group.ranged_flowers.iter() {
            if r == flower_range || r.row != row_to_find as usize {
                continue;
            }

            let mut matching_point = false;

            for p in r.all_points.iter() {
                if p.col == point.col {
                    matching_point = true;
                    break;
                }
            }

            if matching_point {
                return Some (r.clone());
            }
        }

        None
    }


    fn calculate_corners (flower_group: &FlowerGroup) -> i32 {

        let mut corner = 0;

        for group_i in 0..flower_group.ranged_flowers.len() {

            if let Some (group) = flower_group.ranged_flowers.get (group_i)
            {
                //find row - 1
                let i_h = group_i as i32 - 1;
                if i_h >= 0
                {
                    if let Some (group_h_min) = find_connected_range (group, &group.min, flower_group, -1) {

                        if group_h_min.min.col != group.min.col {
                            corner += 1;
                        }
                    }
                    else
                    {
                        corner += 1;
                    }

                    if let Some (group_h_max) = find_connected_range (group, &group.max, flower_group, -1) {

                        if group_h_max.max.col != group.max.col {
                            corner += 1;
                        }
                    }
                    else
                    {
                        corner += 1;
                    }
                }
                else
                {
                    corner += 2;
                }

                // find row + 1
                if let Some (group_l_min) = find_connected_range (group, &group.min, flower_group, 1) {

                    if group_l_min.min.col != group.min.col {
                        corner += 1;
                    }
                }
                else
                {
                    corner += 1;
                }

                if let Some (group_l_max) = find_connected_range (group, &group.max, flower_group, 1) {

                    if group_l_max.max.col != group.max.col {
                        corner += 1;
                    }
                }
                else
                {
                    corner += 1;
                }
            }
        }

        corner
    }

    pub fn do_task()
    {
        // let contents = "AAAA\nBBCD\nBBCC\nEEEC";
        // let contents = utils::read_file("/Users/reubenduckering/Documents/Personal Repo/Advent-Of-Code-2024/files/day_12_mini.txt");
        // let contents = "AAAAAA\nAAABBA\nAAABBA\nABBAAA\nABBAAA\nAAAAAA";;
        let contents = utils::read_file("/Users/reubenduckering/Documents/Personal Repo/Advent-Of-Code-2024/files/day_12.txt");
        let mut vec: Vec<Point> = Vec::new();
        let mut flower_groups: Vec<FlowerGroup> = Vec::new();

        println! ("Converting map points");
        convert_to_map_points (&contents, &mut vec);
        let map_info = get_map_info (&vec);

        println! ("Pair Horizontal");
        pair_horizontal (&vec, &mut flower_groups, &map_info);

        println! ("Group Flowers");
        group_flower_groups (&mut flower_groups);

        let mut cost  = 0;

        for flower_group in flower_groups {
            println! ("Calculating cost for {}", &flower_group.flower_type);
            cost += calculate_cost (&vec, &flower_group).1;
        }

        println! ("Cost: {}", cost);
    }
}