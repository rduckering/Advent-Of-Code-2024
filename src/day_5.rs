pub mod day_5
{
    use crate::utils::utils;

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_check_obey_rules()
        {
            let page = vec![3,4,7,2,1];
            let rules: Vec<(i32,i32)> = vec![(2,1), (4,7)];
            let rules2: Vec<(i32,i32)> = vec![(1,2), (4,7)];
            assert_eq!(check_obey_rules (&page, &rules), true);
            assert_eq!(check_obey_rules (&page, &rules2), false);
        }

        #[test]
        fn test_correct_pages()
        {
            let mut page = vec![3,4,7,2,1];
            let mut page2 = vec![3,4,7,2,1];
            let rules: Vec<(i32,i32)> = vec![(2,1), (4,7)];
            let rules2: Vec<(i32,i32)> = vec![(1,2), (4,7)];

            correct_pages (&mut page, &rules);
            correct_pages (&mut page2, &rules2);

            assert_eq!(check_obey_rules (&page, &rules), true);
            assert_eq!(check_obey_rules (&page, &rules2), false);
        }
    }

    fn parse_pairs (contents: &str) -> Vec<(i32, i32)> {

        let mut result: Vec<(i32, i32)> = Vec::new();
        for line in contents.split("\n") {
            let split_line = line.split ("|").collect::<Vec<&str>>();
            result.push ((split_line[0].parse().unwrap(), split_line[1].parse().unwrap()));
        }
        result
    }

    fn parse_pages (contents: &str) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();

        for line in contents.split("\n") {
            let split_number = line.split(",").collect::<Vec<&str>>();

            let convert_vec: Vec<i32> = split_number.iter().filter_map(|s| s.parse::<i32>().ok()).collect();
            result.push(convert_vec);
        }

        result
    }

    fn get_rules_that_apply (pages: &Vec<i32>, rules: &Vec<(i32,i32)>) -> Vec<(i32,i32)> {
        let mut rules_that_apply: Vec<(i32,i32)> = Vec::new();

        for rule in rules {
            if pages.contains (&rule.0) && pages.contains (&rule.1) {
                rules_that_apply.push (*rule);
            }
        }

        rules_that_apply
    }

    fn check_obey_rules (pages: &Vec<i32>, rules: &Vec<(i32,i32)>) -> bool {

        for rule in rules {
            let mut index_0: Option<usize> = None;
            let mut index_1: Option<usize> = None;

            if let Some(index) = pages.iter().position(|&x| x == rule.0) {
                index_0 = Some (index);
            }

            if let Some(index) = pages.iter().position(|&x| x == rule.1) {
                index_1 = Some (index);
            }

            if index_0.is_none() || index_1.is_none() {
                panic!("this shouldn't happen");
            }

            if index_0 > index_1 {
                return false
            }
        }

        true
    }

    fn correct_pages (pages: &mut Vec<i32>, rules: &Vec<(i32,i32)>) {
        for rule in rules {
            let mut index_0: Option<usize> = None;
            let mut index_1: Option<usize> = None;

            if let Some(index) = pages.iter().position(|&x| x == rule.0) {
                index_0 = Some (index);
            }

            if let Some(index) = pages.iter().position(|&x| x == rule.1) {
                index_1 = Some (index);
            }

            if index_0.is_none() || index_1.is_none() {
                panic!("this shouldn't happen");
            }

            if index_0 > index_1 {
                let hold = pages[index_0.unwrap()];
                pages.remove (index_0.unwrap());
                pages.insert (index_1.unwrap(), hold);
            }
        }
    }

    fn sort_pages (contents: &Vec<Vec<i32>>, rules: &Vec<(i32,i32)>) -> (i32, i32) {
        let mut correct_order_sum: i32 = 0;
        let mut incorrect_order_sum: i32 = 0;

        for pages in contents {
            let rules_that_apply = get_rules_that_apply (&pages, &rules);
            let mut pages_changed = pages.clone();
            let mut order_changed = false;

            if check_obey_rules (&pages, &rules_that_apply)
            {
                let index_to_capture = (pages.len() - 1) / 2;
                correct_order_sum += pages_changed[index_to_capture];
            }

            while ! check_obey_rules (&pages_changed, &rules_that_apply) {
                correct_pages (&mut pages_changed, &rules_that_apply);
                order_changed = true;
            }

            if order_changed {
                let index_to_capture = (pages.len() - 1) / 2;
                incorrect_order_sum += pages_changed[index_to_capture];
            }
        }

        (correct_order_sum, incorrect_order_sum)
    }

    pub fn do_task()
    {
        let contents_rules = utils::read_file ("/Users/reubenduckering/Documents/Personal Repo/Advent-Of-Code-2024/files/day_5_pairs.txt");
        let contents_pages = utils::read_file ("/Users/reubenduckering/Documents/Personal Repo/Advent-Of-Code-2024/files/day_5_pages.txt");
        let page_rules_array = parse_pairs (&contents_rules);
        let pages_array = parse_pages (&contents_pages);

        let result = sort_pages (&pages_array, &page_rules_array);
        println!("Sorted results: {:?}", result.0);
        println!("Sorted results: {:?}", result.1);
    }
}