pub mod day_10
{
    use crate::utils::utils;
    use std::collections::VecDeque;
    use std::collections::HashSet;

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_convert_to_map_points()
        {
            let result = "01\n12";
            let mut points: Vec<PointInfo> = Vec::new();
            convert_to_map_points (&result, &mut points);

            assert_eq!(4, points.len());
            assert_eq!(points[0].position.row, 0);
            assert_eq!(points[1].position.row, 0);
            assert_eq!(points[2].position.row, 1);
            assert_eq!(points[3].position.row, 1);
            assert_eq!(points[0].position.col, 0);
            assert_eq!(points[1].position.col, 1);
            assert_eq!(points[2].position.col, 0);
            assert_eq!(points[3].position.col, 1);

            assert_eq!(points[0].level, 0);
            assert_eq!(points[1].level, 1);
            assert_eq!(points[2].level, 1);
            assert_eq!(points[3].level, 2);
        }

        #[test]
        fn test_get_map_info()
        {
            let result = "0123\n1287\n9873";
            let mut points: Vec<PointInfo> = Vec::new();
            convert_to_map_points (&result, &mut points);
            let map_info = get_map_info (&points);

            assert_eq!(map_info.max_rows, 3);
            assert_eq!(map_info.max_cols, 4);
        }

        #[test]
        fn test_map_surrounding_points() {
            let result = "0123\n1287";
            let mut points: Vec<PointInfo> = Vec::new();
            convert_to_map_points (&result, &mut points);
            let mut map_info = get_map_info (&points);
            map_surrounding_points (&mut points, &mut map_info);

            assert_eq!(points[0].linked_nodes.len(), 2);
            assert_eq!(points[1].linked_nodes.len(), 2);
            assert_eq!(points[2].linked_nodes.len(), 1);
            assert_eq!(points[3].linked_nodes.len(), 0);

            assert_eq!(points[4].linked_nodes.len(), 1);
            assert_eq!(points[5].linked_nodes.len(), 0);
            assert_eq!(points[6].linked_nodes.len(), 0);
            assert_eq!(points[7].linked_nodes.len(), 1);

            assert_eq!(map_info.trail_heads.len(), 1);
        }

        #[test]
        fn test_find_different_paths_1()
        {
            let result = "0123\n\
                                1284\n\
                                2265\n\
                                1178\n\
                                2229";

            let mut points: Vec<PointInfo> = Vec::new();
            convert_to_map_points (&result, &mut points);
            let mut map_info = get_map_info (&points);
            map_surrounding_points (&mut points, &mut map_info);
            let paths = find_different_paths (&points, &map_info);
            assert_eq!(paths, (1,1));
        }

        #[test]
        fn test_find_different_paths_2()
        {
            let result = "0123\n\
                                1284\n\
                                2265\n\
                                9878\n\
                                2229";

            let mut points: Vec<PointInfo> = Vec::new();
            convert_to_map_points (&result, &mut points);
            let mut map_info = get_map_info (&points);
            map_surrounding_points (&mut points, &mut map_info);
            let paths = find_different_paths (&points, &map_info);
            assert_eq!(paths, (2,2));
        }

        #[test]
        fn test_find_different_paths_2_twisted()
        {
            let result = "0123\n\
                                1284\n\
                                2265\n\
                                1178\n\
                                2289";

            let mut points: Vec<PointInfo> = Vec::new();
            convert_to_map_points(&result, &mut points);
            let mut map_info = get_map_info (&points);
            map_surrounding_points (&mut points, &mut map_info);
            let paths = find_different_paths(&points, &map_info);
            assert_eq!(paths, (2,1));
        }
    }

    #[derive(PartialEq, Eq, Clone)]
    struct Point {
        row: usize,
        col: usize,
    }

    #[derive(PartialEq, Eq, Clone)]
    struct PointInfo {
        position: Point,
        level: i32,
        trail_head: bool,
        linked_trail_head: Vec<Point>,
        linked_nodes: Vec<Point>,
        unique_id: i32,
    }

    struct MapInfo {
        max_rows: usize,
        max_cols: usize,
        trail_heads: Vec<Point>,
    }

    fn convert_to_map_points (content: &str, points: &mut Vec<PointInfo>) {

        let mut row: usize = 0;
        let mut col: usize = 0;

        let mut id = 0;

        for line in content.lines()
        {
            for c in line.chars()
            {
                let level = c as i32 - '0' as i32;

                points.push (PointInfo { position: Point { row, col },
                    level,
                    trail_head: false,
                    linked_trail_head: Vec::new(),
                    linked_nodes: Vec::new(),
                    unique_id: id});

                id += 1;
                col += 1;
            }
            col = 0;
            row += 1;
        }
    }

    fn get_point_info (points: &mut Vec<PointInfo>, point: Point) ->  Option<&mut PointInfo> {

        for p in points {
            if p.position == point {
                return Some (p);
            }
        }

        None
    }

    fn get_point_info_copy (points: &Vec<PointInfo>, point: &Point) -> Option<PointInfo> {

        for p in points {
            if p.position == *point {
                return Some(p.clone());
            }
        }

        None
    }

    fn map_surrounding_points (points: &mut Vec<PointInfo>, map_info: &mut MapInfo) {

        for row in 0..map_info.max_rows
        {
            for col in 0..map_info.max_cols
            {
                let mut linked_nodes = Vec::new();
                let current_level;

                if let Some (current_p) = get_point_info (points, Point {row, col}) {
                    current_level = current_p.level;
                }
                else {
                    panic! ("this should never happen");
                }

                // check surrounding points
                let up = row as i32 - 1;

                if up >= 0
                {
                    if let Some (up_p) = get_point_info (points, Point {row: up as usize, col })
                    {
                        if up_p.level == current_level + 1 {
                            linked_nodes.push (up_p.position.clone());
                        }
                    }
                }

                let down = row as i32 + 1;

                if down >= 0
                {
                    if let Some (down_p) = get_point_info(points, Point {row: down as usize, col})
                    {
                        if down_p.level == current_level + 1 {
                            linked_nodes.push (down_p.position.clone());
                        }
                    }
                }

                let left = col as i32 - 1;

                if left >= 0
                {
                    if let Some (left_p) = get_point_info(points, Point {row, col: left as usize })
                    {
                        if left_p.level == current_level + 1 {
                            linked_nodes.push (left_p.position.clone());
                        }
                    }
                }

                let right = col as i32 + 1;

                if right >= 0
                {
                    if let Some (right_p) = get_point_info(points, Point {row, col: right as usize })
                    {
                        if right_p.level == current_level + 1 {
                            linked_nodes.push (right_p.position.clone());
                        }
                    }
                }

                if let Some (current_p) = get_point_info (points, Point {row, col})
                {
                    current_p.linked_nodes.append (&mut linked_nodes);

                    if current_p.level == 0 && current_p.linked_nodes.len() > 0
                    {
                        current_p.trail_head = true;
                        map_info.trail_heads.push (current_p.position.clone());
                    }
                }
                else {
                    panic! ("this should never happen");
                }
            }
        }
    }

    fn get_map_info (points: &Vec<PointInfo>) -> MapInfo {

        let (mut max_rows, mut max_cols) = points.iter().fold((0,0), |(max_r, max_c), p| { (
            max_r.max (p.position.row),
            max_c.max (p.position.col)
        ) });

        max_rows += 1;
        max_cols += 1;

        MapInfo { max_rows, max_cols, trail_heads: Vec::new() }
    }

    fn find_different_paths (points: &Vec<PointInfo>, map_info: &MapInfo) -> (i32, i32)
    {
        let mut sum: Vec<i32> = Vec::new();
        let mut unique_sum: Vec<i32> = Vec::new();

        for trail_head in map_info.trail_heads.iter()
        {
            let mut queue: VecDeque <Point> = VecDeque::new();
            let mut paths = 0;
            let mut unique_trail_ends: HashSet<i32> = HashSet::new();

            queue.push_back (trail_head.clone());

            loop
            {
                if let Some (p) = queue.front()
                {
                    if let Some (p_info) = get_point_info_copy (points, &p)
                    {
                        if p_info.level == 9
                        {
                            paths += 1;
                            unique_trail_ends.insert(p_info.unique_id);
                        }

                        if p_info.linked_nodes.len() > 0
                        {
                            for node in p_info.linked_nodes {
                                queue.push_back(node);
                            }
                        }
                    }

                    queue.pop_front();
                }

                if queue.len() == 0
                {
                    sum.push (paths);
                    unique_sum.push (unique_trail_ends.len() as i32);
                    break;
                }
            }
        }

        (sum.iter().sum::<i32>(), unique_sum.iter().sum::<i32>())
    }

    pub fn do_task()
    {
        let contents = utils::read_file("/Users/reubenduckering/Documents/Personal Repo/Advent-Of-Code-2024/files/day_10_mini1.txt");
        let mut points: Vec<PointInfo> = Vec::new();
        convert_to_map_points (&contents, &mut points);
        let mut map_info = get_map_info (&points);
        map_surrounding_points (&mut points, &mut map_info);
        let paths = find_different_paths(&points, &map_info);
        println!("Paths varients: {} - Unique path ends: {}", paths.0, paths.1);
    }
}