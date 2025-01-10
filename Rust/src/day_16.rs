pub mod day_16
{
    use crate::utils::utils;
    use std::collections::BinaryHeap;
    use std::collections::HashSet;
    use std::collections::HashMap;
    use std::time::Instant;

    use petgraph::graph::{NodeIndex, UnGraph};
    use petgraph::algo::{dijkstra, min_spanning_tree};
    use petgraph::data::FromElements;
    use petgraph::dot::{Dot, Config};

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_get_path_options() {
            let linked_nodes = PathLinks { north: None, east: Some (Point { row: 2, col: 5}), south: Some (Point { row: 1, col: 2}), west: Some (Point { row: 5, col: 6}), active_num:  0 };
            let valid_paths = get_path_options ('E', linked_nodes);

            assert_eq! (valid_paths[0], Point { row: 2, col: 5 });
            assert_eq! (valid_paths[1], Point { row: 1, col: 2 });
            assert_eq! (valid_paths.len(), 2);
        }

        #[test]
        fn test_did_turn() {
            let linked_nodes = PathLinks { north: None, east: Some (Point { row: 2, col: 5}), south: Some (Point { row: 1, col: 2}), west: Some (Point { row: 3, col: 4}), active_num:  0 };
            let turned_1 = did_turn('W', Point { row: 1, col: 2}, linked_nodes.clone());
            let turned_2 = did_turn('W', Point { row: 3, col: 4}, linked_nodes.clone());

            assert_eq! (turned_1.0, true);
            assert_eq! (turned_1.1, 'S');
            assert_eq! (turned_2.0, false);
            assert_eq! (turned_2.1, 'W');
        }

        #[test]
        fn test_dikstra() {
            // Create an undirected graph with `i32` nodes and edges with `()` associated data.
            let g = UnGraph::<i32, ()>::from_edges(&[
                (1, 2), (2, 3), (3, 4),
                (1, 4)]);
            let node_map = dijkstra(&g, 1.into(), Some(4.into()), |_| 1);
            assert_eq!(&1i32, node_map.get(&NodeIndex::new(4)).unwrap());

            let mst = UnGraph::<_, _>::from_elements(min_spanning_tree(&g));
            assert_eq!(g.raw_edges().len() - 1, mst.raw_edges().len());

            println!("{:?}", Dot::with_config(&mst, &[Config::EdgeNoLabel]));
        }
    }

    #[derive(PartialEq, Eq, Clone, Hash, Debug)]
    struct Point {
        row: usize,
        col: usize,
    }

    #[derive(PartialEq, Eq, Clone, Hash, Debug)]
    struct PointInfo {
        position: Point,
        tile_type: char,
        linked_nodes: PathLinks,
        unique_id: i32,
    }

    #[derive(PartialEq, Eq, Clone, Hash, Debug)]
    struct PathLinks {
        north: Option <Point>,
        east: Option<Point>,
        south: Option<Point>,
        west: Option<Point>,
        active_num: i32
    }

    #[derive(PartialEq, Eq, Clone, Debug)]
    struct Player {
        orientation: char,
        score : u64,
        location: Point,
        travelled_points: HashSet<Point>,
    }

    #[derive(PartialEq, Eq, Clone, Hash, Debug)]
    struct PlayerBFS {
        orientation: char,
        score : u64,
        location: Point,
        travelled_points: Vec<(Point, char)>,
    }

    impl PartialOrd for PlayerBFS {
        fn partial_cmp (&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some (other.score.cmp (&self.score))
        }
    }

    impl Ord for PlayerBFS {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.score.cmp(&self.score)
        }
    }

    #[derive(PartialEq, Eq, Clone, Debug)]
    struct MapInfo {
        max_rows: usize,
        max_cols: usize,
        start: Point,
        end: Point,
        dead_end: HashMap<FootPrint, bool>,
    }

    #[derive(PartialEq, Eq, Clone, Hash, Debug)]
    struct FootPrint {
        point: Point,
        orientation: char,
    }

    fn convert_to_map_points (content: &str, points: &mut Vec<PointInfo>) {

        let mut row: usize = 0;
        let mut col: usize = 0;

        let mut id = 0;

        for line in content.lines()
        {
            for c in line.chars()
            {
                points.push (PointInfo { position: Point { row, col },
                    tile_type: c,
                    linked_nodes: PathLinks { north:None, east: None, south: None, west: None, active_num: 0},
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

    fn get_point_info_copy (points: &HashMap<Point, PointInfo>, point: &Point) -> Option<PointInfo> {

        if let Some (value) = points.get (point) {
            return Some (value.clone());
        }

        None
    }

    fn map_surrounding_points (points: &mut Vec<PointInfo>, map_info: &mut MapInfo) {

        for row in 0..map_info.max_rows
        {
            for col in 0..map_info.max_cols
            {
                let mut linked_nodes = PathLinks { north: None, east: None, south: None, west: None, active_num: 0};
                let current_type;

                if let Some (current_p) = get_point_info(points, Point { row, col }) {
                    current_type = current_p.tile_type;
                }
                else {
                    panic! ("this should never happen");
                }

                if current_type == '#' {
                    continue;
                }

                // check surrounding points
                let up = row as i32 - 1;

                if up >= 0
                {
                    if let Some (up_p) = get_point_info (points, Point { row: up as usize, col })
                    {
                        if matches! (up_p.tile_type, '.' | 'E' | 'S') {
                            linked_nodes.north = Some(up_p.position.clone());
                            linked_nodes.active_num += 1;
                        }
                    }
                }

                let down = row as i32 + 1;

                if down >= 0
                {
                    if let Some (down_p) = get_point_info(points, Point {row: down as usize, col})
                    {
                        if matches! (down_p.tile_type, '.' | 'E' | 'S') {
                            linked_nodes.south  = Some(down_p.position.clone());
                            linked_nodes.active_num += 1;
                        }
                    }
                }

                let left = col as i32 - 1;

                if left >= 0
                {
                    if let Some (left_p) = get_point_info(points, Point {row, col: left as usize })
                    {
                        if matches! (left_p.tile_type, '.' | 'E' | 'S') {
                            linked_nodes.west = Some(left_p.position.clone());
                            linked_nodes.active_num += 1;
                        }
                    }
                }

                let right = col as i32 + 1;

                if right >= 0
                {
                    if let Some (right_p) = get_point_info(points, Point {row, col: right as usize })
                    {
                        if matches! (right_p.tile_type, '.' | 'E' | 'S') {
                            linked_nodes.east  = Some (right_p.position.clone());
                            linked_nodes.active_num += 1;
                        }
                    }
                }

                if let Some (current_p) = get_point_info (points, Point { row, col })
                {
                    current_p.linked_nodes = linked_nodes;

                    if current_p.tile_type == 'S' {
                        map_info.start = current_p.position.clone();
                    }

                    if current_p.tile_type == 'E' {
                        map_info.end = current_p.position.clone();
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

        MapInfo { max_rows, max_cols, start: Point { row: 0, col: 0 }, end: Point { row: 0, col: 0 }, dead_end: HashMap::new() }
    }

    fn get_path_options (p_orientation : char, linked_nodes: PathLinks) -> Vec<Point> {
        // avoid coming from the way we already came from
        let mut points = Vec::new();

        if p_orientation != 'S' {
            if let Some (north) = linked_nodes.north {
                points.push (north);
            }
        }

        if p_orientation != 'W' {
            if let Some (east) = linked_nodes.east {
                points.push(east);
            }
        }

        if p_orientation != 'N' {
            if let Some (south) = linked_nodes.south {
                points.push (south.clone());
            }
        }

        if p_orientation != 'E' {
            if let Some (west) = linked_nodes.west {
                points.push(west);
            }
        }

        points
    }

    fn did_turn (p_orientation : char, new_path: Point, linked_nodes: PathLinks) -> (bool, char) {

        let mut new_path_orientation= 'A';

        if let Some(north) = linked_nodes.north {
            if new_path == north {
                new_path_orientation = 'N';
            }
        }

        if let Some(east) = linked_nodes.east {
            if new_path == east {
                new_path_orientation = 'E';
            }
        }

        if let Some(south) = linked_nodes.south {
            if new_path == south {
                new_path_orientation = 'S';
            }
        }

        if let Some(west) = linked_nodes.west {
            if new_path == west {
                new_path_orientation = 'W';
            }
        }

        if (new_path_orientation == 'N' && p_orientation == 'N')
            || (new_path_orientation == 'S' && p_orientation == 'S')
            || (new_path_orientation == 'E' && p_orientation == 'E')
            || (new_path_orientation == 'W' && p_orientation == 'W') {
            return (false, new_path_orientation);
        }

        (true, new_path_orientation)
    }


    fn contains_point (points: &HashSet<Point>, target: &FootPrint, _ignore_direction: bool) -> bool {

        points.contains (&target.point)
    }

    fn all_points_used (used_points: &HashSet<Point>, links: PathLinks) -> bool {

        let num_links = links.active_num;
        let mut cnt = 0;

        if let Some (n) = links.north {
            let foot_print = FootPrint { point: n, orientation: 'c'};
            if contains_point (used_points, &foot_print, true) {
                cnt += 1;
            }
        }

        if let Some (e) = links.east {
            let foot_print = FootPrint { point: e, orientation: 'c'};
            if  contains_point (used_points, &foot_print, true) {
                cnt += 1;
            }
        }

        if let Some (s) = links.south {
            let foot_print = FootPrint { point: s, orientation: 'c'};
            if  contains_point (used_points, &foot_print, true) {
                cnt += 1;
            }
        }

        if let Some (w) = links.west {
            let foot_print = FootPrint { point: w, orientation: 'c'};
            if contains_point (used_points, &foot_print, true) {
                cnt += 1;
            }
        }

        num_links == cnt
    }

    #[allow(dead_code)]
    // find valid path very quickly but takes for ever to find short-est path, use bfs
    fn find_different_paths_dfs (player: &mut Player, points: &Vec<PointInfo>, winner_list: &mut Vec<Player>, map_info: &mut MapInfo, time: &Instant, min_score: &mut u64, point_map: &HashMap<Point, PointInfo>) -> i32
    {
        let mut dead_end = -1;

        loop
        {
            let duration = time.elapsed();
            // if duration > std::time::Duration::from_millis(100) {
            //     println !("timeout");
            //     return true;
            // }

            // nothing for 40000
            // nothing for 60000

            if player.score >= *min_score
            {
                draw_map_player_path (&map_info, &player, &points, point_map);
                return -1;
            }

            if let Some (p_info) = get_point_info_copy (point_map, &player.location)
            {
                if p_info.tile_type == 'E'
                {
                    let mut player_to_cache = player.clone();
                    player_to_cache.travelled_points.clear();
                    winner_list.push (player_to_cache.clone());

                    println! ("winner added: {:?}", duration);

                    let score = player.score.clone();

                    if score < *min_score {
                        *min_score = score;
                        draw_map_player_path (&map_info, &player, &points, point_map);
                        println! ("NEW low score: {:?}", score);
                    }

                    return 1;
                }

                let foot_print = FootPrint { point:player.location.clone(), orientation: player.orientation.clone()};

                // if map_info.dead_end.contains_key (&foot_print) {
                //     return -1;
                // }


                let paths = get_path_options (player.orientation, p_info.linked_nodes.clone());

                if paths.len() == 0
                { // dead end
                    // println! ("deadend");
                    return 0;
                }

                else if paths.len() == 1 { // move forward

                    if contains_point (&player.travelled_points, &foot_print, true) {
                        return -1;
                    }
                    else {
                        player.travelled_points.insert (foot_print.point);
                    }

                    player.location = paths[0].clone();

                    let turn = did_turn (player.orientation, paths[0].clone(), p_info.linked_nodes.clone());
                    player.orientation = turn.1;

                    player.score += if turn.0 { 1001 } else { 1 };
                }

                else if paths.len() > 1 // split off and replicate
                {
                    // start new branches
                    // let mut rng = rand::thread_rng();
                    // let random_index = rng.gen_range(0..paths.len());

                    if all_points_used (&player.travelled_points, p_info.linked_nodes.clone()) {
                        return -1;
                    }

                    let mut failed_paths = 0;
                    let path_size = paths.len();

                    for path in paths {
                        // let path = &paths[random_index];

                        let mut another_player = player.clone();

                        another_player.location = path.clone();

                        let foot_print_local = foot_print.clone();

                        if contains_point (&another_player.travelled_points, &foot_print_local, true) {
                            return -1;
                        }
                        else
                        {
                            another_player.travelled_points.insert (foot_print_local.point.clone());

                            let turn = did_turn (player.orientation, path.clone(), p_info.linked_nodes.clone());

                            another_player.orientation = turn.1;
                            another_player.score += if turn.0 { 1001 } else { 1 };

                            let result = find_different_paths_dfs (&mut another_player, points, winner_list, map_info, time, min_score, point_map);

                            if result == 0 {
                                map_info.dead_end.insert (foot_print_local, true);
                                failed_paths += 1;
                            }
                        }
                    }

                    if failed_paths == path_size {
                        dead_end = 0;
                    }

                    return dead_end
                }
            }
        }
    }

    fn _draw_map (map_size: &MapInfo, player: Player, _cells: &Vec<PointInfo>, point_map: &HashMap<Point, PointInfo>) {
        let mut map: String = String::from("");

        // place map
        for h in 0..map_size.max_rows {
            for w in 0..map_size.max_cols {

                let point = Point { col: w, row: h};

                if player.location == point {
                    let char = match player.orientation {
                        'N' => '^',
                        'E' => '>',
                        'S' => 'v',
                        'W' => '<',
                        _ => 'A'
                    };

                    map.push (char);
                }
                else
                {
                    if let Some(c) = get_point_info_copy (point_map, &point) {
                        if c.linked_nodes.active_num > 0 {
                            map.push('.');
                        }
                        else
                        {
                            map.push('#');
                        }
                    } else {
                        panic!("nope");
                    }
                }
            }

            map.push('\n');
        }

        println!("{}", map);
    }

    #[allow(dead_code)]
    fn draw_map_player_path (map_size: &MapInfo, player: &Player, _cells: &Vec<PointInfo>, point_map: &HashMap<Point, PointInfo>) {
        let mut map: String = String::from("");

        // place map
        for h in 0..map_size.max_rows {
            for w in 0..map_size.max_cols {

                let point = Point { col: w, row: h};

                let new_point = point.clone();
                let foot_print  = FootPrint { point: new_point, orientation: '>'};
                if contains_point (&player.travelled_points, &foot_print, true ) {
                    map.push ('*');
                }
                else
                {
                    if let Some(c) = get_point_info_copy (point_map, &point) {
                        if c.linked_nodes.active_num > 0 {
                            map.push('.');
                        }
                        else
                        {
                            map.push('#');
                        }
                    } else {
                        panic!("nope");
                    }
                }
            }

            map.push('\n');
        }

        println!("{}", map);
    }

    fn contain_point_in_queue (players: &BinaryHeap<PlayerBFS>, point: &Point) -> bool {
        for player in players.iter() {

            if player.location == *point {
                return true;
            }
        }

        false
    }

    fn contain_point_in_queue_travel_path (players: &PlayerBFS, point: &Point) -> Option<char> {
        for player in players.travelled_points.iter() {

            if player.0 == *point {
                return Some (player.1);
            }
        }

        None
    }

    fn draw_map_player_queue (map_size: &MapInfo, players: &BinaryHeap<PlayerBFS>, _cells: &Vec<PointInfo>, point_map: &HashMap<Point, PointInfo>) {
        let mut map: String = String::from("");

        // place map
        for h in 0..map_size.max_rows {
            for w in 0..map_size.max_cols {

                let point = Point { col: w, row: h};

                if contain_point_in_queue (players, &point){
                    map.push('*');
                }
                else
                {
                    if let Some(c) = get_point_info_copy (point_map, &point) {
                        if c.linked_nodes.active_num > 0 {
                            map.push('.');
                        }
                        else
                        {
                            map.push('#');
                        }
                    }
                    else {
                        panic!("nope");
                    }
                }
            }

            map.push('\n');
        }

        println!("{}", map);
    }

    fn draw_map_player_path_queue (map_size: &MapInfo, player: &PlayerBFS, point_map: &HashMap<Point, PointInfo>) {
        let mut map: String = String::from("");

        // place map
        for h in 0..map_size.max_rows {
            for w in 0..map_size.max_cols {

                let point = Point { col: w, row: h};

                let new_point = point.clone();

                if let Some (c) = contain_point_in_queue_travel_path (player, &new_point) {
                    map.push (c);
                }
                else
                {
                    if let Some(c) = get_point_info_copy (point_map, &point) {
                        if c.linked_nodes.active_num > 0 {
                            map.push('.');
                        }
                        else
                        {
                            map.push('#');
                        }
                    } else {
                        panic!("nope");
                    }
                }
            }

            map.push('\n');
        }

        println!("{}", map);
    }

    fn make_point_map (cells: &Vec<PointInfo>) -> HashMap<Point, PointInfo> {

        let mut map: HashMap<Point, PointInfo> = HashMap::new();

        for point in cells {
            map.insert (point.position.clone(), point.clone());
        }

        map
    }

    fn common_points_count (best_path: &Vec<PlayerBFS>) -> usize {
        let mut unique_location =  HashSet::new();

        for path in best_path {
            for l in path.travelled_points.iter() {
                unique_location.insert (l.0.clone());
            }
        }

        unique_location.len() + 1
    }

    fn find_different_paths_bfs (points: &Vec<PointInfo>, map_info: &mut MapInfo, point_map: &HashMap<Point, PointInfo>)
    {
        let mut queue: BinaryHeap <PlayerBFS> = BinaryHeap::new();
        let mut visited_nodes: HashMap<(Point, char), u64> = HashMap::new();

        // add start position
        let player = PlayerBFS { location: map_info.start.clone(), orientation: 'E', score: 0, travelled_points: Vec::new() };
        queue.push (player.clone());

        let mut best_paths: Vec<PlayerBFS> = Vec::new();
        let mut best_score = 0;

        while ! queue.is_empty()
        {
            if let Some (p) = queue.pop()
            {
                if let Some (p_info) = get_point_info_copy (point_map, &p.location)
                {
                    if p_info.tile_type == 'E'
                    {
                        if best_score == 0 {
                            best_score = p.score;
                        }
                        else
                        {
                            if best_score != p.score {

                                let num_location = common_points_count (&best_paths);
                                println! ("number of unique_locations: {}", num_location);
                                return;
                            }
                        }

                        best_paths.push (p.clone());
                        draw_map_player_path_queue (&map_info, &p, point_map);
                        println! ("reached end - num paths : {:?}", best_paths.len());
                    }

                    if let Some (north) = p_info.linked_nodes.north {
                        let mut new_player = p.clone();
                        new_player.location = north;
                        new_player.orientation = 'N';
                        new_player.score += if new_player.orientation == p.orientation { 1 } else { 1001 };

                        let foot_step = (new_player.location.clone(), new_player.orientation.clone());

                        if p.orientation != 'S' {

                            if ! visited_nodes.contains_key (&foot_step) || (visited_nodes.contains_key (&foot_step) && visited_nodes.get(&foot_step).unwrap() >= &new_player.score ) {
                                new_player.travelled_points.push(foot_step.clone());
                                visited_nodes.insert((new_player.location.clone(), new_player.orientation.clone()),new_player.score);
                                queue.push(new_player.clone());
                            }
                        }
                    }

                    if let Some (east) = p_info.linked_nodes.east {
                        let mut new_player = p.clone();
                        new_player.location = east;
                        new_player.orientation = 'E';
                        new_player.score += if new_player.orientation == p.orientation { 1 } else { 1001 };

                        let foot_step = (new_player.location.clone(), new_player.orientation.clone());

                        if p.orientation != 'W' {

                            if ! visited_nodes.contains_key (&foot_step) || (visited_nodes.contains_key (&foot_step) && visited_nodes.get(&foot_step).unwrap() >= &new_player.score ) {
                                new_player.travelled_points.push(foot_step.clone());
                                visited_nodes.insert((new_player.location.clone(), new_player.orientation.clone()),new_player.score);
                                queue.push(new_player.clone());
                            }

                        }
                    }

                    if let Some (south) = p_info.linked_nodes.south {
                        let mut new_player = p.clone();
                        new_player.location = south;
                        new_player.orientation = 'S';
                        new_player.score += if new_player.orientation == p.orientation { 1 } else { 1001 };

                        let foot_step = (new_player.location.clone(), new_player.orientation.clone());

                        if  p.orientation != 'N' {

                            if ! visited_nodes.contains_key (&foot_step) || (visited_nodes.contains_key (&foot_step) && visited_nodes.get(&foot_step).unwrap() >= &new_player.score ) {
                                new_player.travelled_points.push(foot_step.clone());
                                visited_nodes.insert((new_player.location.clone(), new_player.orientation.clone()),new_player.score);
                                queue.push(new_player.clone());
                            }
                        }
                    }

                    if let Some (west) = p_info.linked_nodes.west {
                        let mut new_player = p.clone();
                        new_player.location = west;
                        new_player.orientation = 'W';
                        new_player.score += if new_player.orientation == p.orientation { 1 } else { 1001 };

                        let foot_step = (new_player.location.clone(), new_player.orientation.clone());

                        if  p.orientation != 'E' {

                            if ! visited_nodes.contains_key (&foot_step) || (visited_nodes.contains_key (&foot_step) && visited_nodes.get(&foot_step).unwrap() >= &new_player.score ) {
                                new_player.travelled_points.push(foot_step.clone());
                                visited_nodes.insert((new_player.location.clone(), new_player.orientation.clone()),new_player.score);
                                queue.push(new_player.clone());
                            }
                        }
                    }
                }

            }

            draw_map_player_queue (&map_info, &queue, points, point_map);
        }
    }

    pub fn do_task()
    {
        // mini2 - 21148
        let contents = utils::read_file("/Users/reubenduckering/Documents/Personal Repo/Advent-Of-Code-2024/files/day_16.txt");

        let mut points: Vec<PointInfo> = Vec::new();
        convert_to_map_points(&contents, &mut points);
        let mut map_info = get_map_info (&points);
        map_surrounding_points(&mut points, &mut map_info);

        let mut _winner_list: Vec<Player> = Vec::new();
        let mut _min_score: u64 = u64::MAX;
        let mut _player = Player { location: map_info.start.clone(), orientation: 'E', score: 0, travelled_points: HashSet::new() };
        let time = Instant::now();

        let point_map = make_point_map (&points);

        // let _score = find_different_paths_dfs (&mut player, &points, &mut winner_list, &mut map_info, &time, &mut min_score, &point_map);
        let _score = find_different_paths_bfs (&points, &mut map_info, &point_map);

        let duration = time.elapsed();
        println!("Time Taken: {:?}", duration);

        println! ("winners: {:?}", _winner_list);
    }
}