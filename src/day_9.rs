pub mod day_9
{
    use std::time::Instant;
    use crate::utils::utils;

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_generate_memory_layout()
        {
            let mem = "12345";

            let result = generate_memory_layout (&mem);
            assert_eq! (result, vec! [0, -1, -1, 1, 1, 1, -1, -1, -1, -1, 2, 2, 2, 2, 2]);
        }

        #[test]
        fn test_sort_memory()
        {
            let mut mem = vec! [0, -1, -1, 1, 1, 1, -1, -1, -1, -1, 2, 2, 2, 2, 2];

            sort_memory (&mut mem);
            assert_eq! (mem, vec! [0, 2, 2, 1, 1, 1, 2, 2, 2, -1, -1, -1, -1, -1, -1]);
        }

        #[test]
        fn test_sum_memory()
        {
            let mut mem = vec! [0, 2, 2, 1, 1, 1, 2, 2, 2, -1, -1, -1, -1, -1, -1];

            let sum = sum_values (&mut mem);
            assert_eq! (sum, 60);
        }

        #[test]
        fn test_struct_memory()
        {
            let mem = vec! [0, -1, -1, 1, 1, 1, -1, -1, -1, -1, 2, 2, 2, 2, 2];

            let result = create_struct_layout (&mem);

            let mut expected: Vec<MemoryBlock> = Vec::new();
            expected.push(MemoryBlock {id: 0, width: 1});
            expected.push(MemoryBlock {id: -1, width: 2});
            expected.push(MemoryBlock {id: 1, width: 3});
            expected.push(MemoryBlock {id: -1, width: 4});
            expected.push(MemoryBlock {id: 2, width: 5});

            assert_eq! (result.len(), expected.len());

            for i in 0..expected.len() {
                assert_eq! (result[i].id, expected[i].id);
                assert_eq! (result[i].width, expected[i].width);
            }
        }

        #[test]
        fn test_struct_memory_sort()
        {
            let mut result: Vec<MemoryBlock> = Vec::new();
            result.push(MemoryBlock {id: 0, width: 5});
            result.push(MemoryBlock {id: -1, width: 4});
            result.push(MemoryBlock {id: 1, width: 3});
            result.push(MemoryBlock {id: -1, width: 2});
            result.push(MemoryBlock {id: 2, width: 1});

            let mut expected: Vec<MemoryBlock> = Vec::new();
            expected.push(MemoryBlock {id: 0, width: 5});
            expected.push(MemoryBlock {id: 2, width: 1});
            expected.push(MemoryBlock {id: 1, width: 3});
            expected.push(MemoryBlock {id: -1, width: 3});
            expected.push(MemoryBlock {id: -1, width: 2});
            expected.push(MemoryBlock {id: -1, width: 1});

            sort_memory_blocks (&mut result);

            assert_eq! (result.len(), expected.len());

            for i in 0..expected.len() {
                assert_eq! (result[i].id, expected[i].id);
                assert_eq! (result[i].width, expected[i].width);
            }
        }

        #[test]
        fn test_memory_to_array()
        {
            let mut mem: Vec<MemoryBlock> = Vec::new();
            mem.push(MemoryBlock {id: 0, width: 5});
            mem.push(MemoryBlock {id: 2, width: 1});
            mem.push(MemoryBlock {id: 1, width: 3});
            mem.push(MemoryBlock {id: -1, width: 3});
            mem.push(MemoryBlock {id: -1, width: 2});
            mem.push(MemoryBlock {id: -1, width: 1});

            let result = mem_blocks_to_array (&mem);
            let expected: Vec<i32> = vec! [0, 0, 0, 0, 0, 2, 1, 1, 1, -1, -1, -1, -1, -1, -1];

            assert_eq! (result, expected);
        }

        #[test]
        fn test_concat_memory()
        {
            let mut mem: Vec<MemoryBlock> = Vec::new();
            mem.push(MemoryBlock {id: 0, width: 5});
            mem.push(MemoryBlock {id: 2, width: 1});
            mem.push(MemoryBlock {id: -1, width: 3});
            mem.push(MemoryBlock {id: -1, width: 2});
            mem.push(MemoryBlock {id: 1, width: 3});
            mem.push(MemoryBlock {id: -1, width: 1});

            let mut expected: Vec<MemoryBlock> = Vec::new();
            expected.push(MemoryBlock {id: 0, width: 5});
            expected.push(MemoryBlock {id: 2, width: 1});
            expected.push(MemoryBlock {id: -1, width: 5});
            expected.push(MemoryBlock {id: 1, width: 3});
            expected.push(MemoryBlock {id: -1, width: 1});

            concat_empty_memory (&mut mem);

            for i in 0..expected.len() {
                assert_eq!(mem[i].id, expected[i].id);
                assert_eq!(mem[i].width, expected[i].width);
            }
        }

        #[test]
        fn test_example0()
        {
            // 00...111...2...333.44.5555.6666.777.888899
            let mut memory_layout: Vec<i32> = vec! [0,0,-1,-1,-1,1,1,1,-1,-1,-1,2,-1,-1,-1,3,3,3,-1,4,4,-1,5,5,5,5,-1,6,6,6,6,-1,7,7,7,-1,8,8,8,8,9,9];

            sort_memory (&mut memory_layout);
            let result = sum_values(&memory_layout);

            assert_eq! (result, 1928);
        }

        #[test]
        fn test_example1()
        {
            // 00...111...2...333.44.5555.6666.777.888899
            let memory_layout: Vec<i32> = vec! [0,0,-1,-1,-1,1,1,1,-1,-1,-1,2,-1,-1,-1,3,3,3,-1,4,4,-1,5,5,5,5,-1,6,6,6,6,-1,7,7,7,-1,8,8,8,8,9,9];

            let mut memory_blocks = create_struct_layout(&memory_layout);
            sort_memory_blocks (&mut memory_blocks);
            let result_array = mem_blocks_to_array (&memory_blocks);
            let result = sum_values (&result_array);

            assert_eq! (result, 2858);
        }

        #[test]
        fn test_example2()
        {
            let mem = "12345";

            let memory_layout = generate_memory_layout (&mem);
            let mut memory_blocks =  create_struct_layout(&memory_layout);
            sort_memory_blocks (&mut memory_blocks);
            let result_array = mem_blocks_to_array(&memory_blocks);
            let sum = sum_values (&result_array);

            assert_eq! (sum, 132);
        }

        #[test]
        fn test_example3()
        {
            let mem = "11111";

            let memory_layout = generate_memory_layout (&mem);
            let mut memory_blocks =  create_struct_layout(&memory_layout);
            sort_memory_blocks (&mut memory_blocks);
            let result_array = mem_blocks_to_array(&memory_blocks);
            let sum = sum_values (&result_array);

            assert_eq! (sum, 4);
        }

        #[test]
        fn test_example4()
        {
            let mem = "252";

            let memory_layout = generate_memory_layout (&mem);
            let mut memory_blocks =  create_struct_layout(&memory_layout);
            sort_memory_blocks (&mut memory_blocks);
            let result_array = mem_blocks_to_array(&memory_blocks);
            let sum = sum_values (&result_array);

            assert_eq! (sum, 5);
        }

        #[test]
        fn test_example5()
        {
            let mem = "11252";

            let memory_layout = generate_memory_layout (&mem);
            let mut memory_blocks =  create_struct_layout(&memory_layout);
            sort_memory_blocks (&mut memory_blocks);
            let result_array = mem_blocks_to_array(&memory_blocks);
            let sum = sum_values (&result_array);

            assert_eq! (sum, 23);
        }

        #[test]
        fn test_example6()
        {
            let mem = "1125232";

            let memory_layout = generate_memory_layout (&mem);
            let mut memory_blocks =  create_struct_layout(&memory_layout);
            sort_memory_blocks (&mut memory_blocks);
            let result_array = mem_blocks_to_array(&memory_blocks);
            let sum = sum_values (&result_array);

            assert_eq! (sum, 58);
        }

        #[test]
        fn test_example7()
        {
            let mem = "331154321";

            let memory_layout = generate_memory_layout (&mem);
            let mut memory_blocks =  create_struct_layout(&memory_layout);
            sort_memory_blocks (&mut memory_blocks);
            let result_array = mem_blocks_to_array(&memory_blocks);
            let sum = sum_values (&result_array);
            assert_eq! (sum, 242);
        }

        #[test]
        fn test_example8()
        {
            let mem = "101010";

            let memory_layout = generate_memory_layout (&mem);
            let mut memory_blocks =  create_struct_layout(&memory_layout);
            sort_memory_blocks (&mut memory_blocks);
            let result_array = mem_blocks_to_array(&memory_blocks);
            let sum = sum_values (&result_array);
            assert_eq! (sum, 5);
        }

        #[test]
        fn test_example9()
        {
            let mem = "171010402";

            let memory_layout = generate_memory_layout (&mem);
            let mut memory_blocks =  create_struct_layout(&memory_layout);
            sort_memory_blocks (&mut memory_blocks);
            let result_array = mem_blocks_to_array(&memory_blocks);
            let sum = sum_values (&result_array);
            assert_eq! (sum, 88);
        }


        #[test]
        fn test_example10()
        {
            let mem = "0101013";

            let memory_layout = generate_memory_layout (&mem);
            let mut memory_blocks =  create_struct_layout(&memory_layout);
            sort_memory_blocks (&mut memory_blocks);
            let result_array = mem_blocks_to_array(&memory_blocks);
            let sum = sum_values (&result_array);
            assert_eq! (sum, 9);
        }

        #[test]
        fn test_example11 ()
        {
            let mem = "01001013";

            let memory_layout = generate_memory_layout (&mem);
            let mut memory_blocks =  create_struct_layout (&memory_layout);
            sort_memory_blocks (&mut memory_blocks);
            let result_array = mem_blocks_to_array(&memory_blocks);
            let sum = sum_values (&result_array);
            assert_eq! (sum, 2);
        }

        #[test]
        fn test_example12()
        {
            let mem = "1010101010101010101010";

            let memory_layout = generate_memory_layout (&mem);
            let mut memory_blocks =  create_struct_layout (&memory_layout);
            sort_memory_blocks (&mut memory_blocks);
            let result_array = mem_blocks_to_array(&memory_blocks);
            let sum = sum_values (&result_array);
            assert_eq! (sum, 385);
        }
    }

    fn generate_memory_layout (contents: &str) -> Vec<i32> {

        let mut cnt = 0;
        let mut id_cnt = 0;
        let mut output: Vec<i32> = Vec::new();

        for c in contents.chars()
        {
            let number_of_slots = c as i32 - '0' as i32;

            if number_of_slots != 0 {
                let number_to_insert: i32;

                if cnt % 2 == 0 // used memory
                {
                    number_to_insert = id_cnt;
                    id_cnt += 1;
                } else // free
                {
                    number_to_insert = -1;
                }

                for _ in 0..number_of_slots as usize {
                    output.push(number_to_insert);
                }
            }
            else
            {
                if cnt % 2 == 0 // used memory
                {
                    id_cnt += 1;
                }
            }

            cnt += 1;
        }

        output
    }

    fn sort_memory (memory: &mut Vec<i32>) {

        let mut start = 0;
        let mut end = memory.len() - 1;

        loop {
            let start_value = memory[start];
            let end_value = memory[end];

            if start >= end { break; }

            if start_value == -1 {
                if end_value != -1 {
                    memory.swap (start, end);

                    start +=1;
                    end -=1;
                    continue;
                }

                end -= 1;
            }
            else
            {
                start += 1;
            }
        }
    }

    fn sum_values (values: &Vec<i32>) -> u128 {
        let mut sum: u128 = 0;

        let mut cnt: u128 = 0;

        for v in values.iter() {
            if *v == -1 {
                cnt += 1;
                continue;
            }

            sum += cnt * (*v as u128);
            cnt += 1;
        }

        sum
    }

    struct MemoryBlock {
        id: i32,
        width: usize
    }

    fn create_struct_layout (contents: &Vec<i32>) -> Vec<MemoryBlock> {
        let mut memory: Vec<MemoryBlock> = Vec::new();
        let mut cnt = 0;
        let mut prev_value = -2;


        for v in contents.iter() {

            if prev_value != *v {

                let b = MemoryBlock {
                    id: *v,
                    width: 1,
                };

                memory.push (b);
                cnt += 1;
            }
            else
            {
                if let Some (m) = memory.get_mut ((cnt - 1) as usize) {
                    m.width += 1;
                }
            }

            prev_value = *v;
        }

        memory
    }

    fn sort_memory_blocks (memory_blocks: &mut Vec<MemoryBlock>) {
        let back = memory_blocks.len();

        let mut end = back - 1;
        // for mut end in (0..back).rev()
        loop
        {
            let end_value = memory_blocks.get (end).unwrap();

            if end_value.id != -1 {
                for start in 0..end {
                    let start_value = memory_blocks.get(start).unwrap();

                    if start_value.id != -1 { continue; }

                    if end_value.width <= start_value.width {

                        // split block
                        if end_value.width < start_value.width {
                            let size_diff = start_value.width - end_value.width;

                            // push used size to the back
                            memory_blocks.get_mut(start).unwrap().width = end_value.width;
                            memory_blocks.swap(start, end);

                            if size_diff != 0 {
                                memory_blocks.insert(start + 1, MemoryBlock { id: -1, width: size_diff });
                                end += 1;
                            }
                        } else {
                            memory_blocks.swap(start, end);
                        }

                        break;
                    }
                }
            }

            if end == 0 {
                break;
            }

            end -= 1;
        }
    }

    fn mem_blocks_to_array (memory_block: &Vec<MemoryBlock>) -> Vec<i32> {

        let mut vec: Vec<i32> = Vec::new();

        for mem in memory_block {

            for _ in 0..mem.width {
                vec.push (mem.id);
            }
        }

        vec
    }

    fn concat_empty_memory (memory_block: &mut Vec<MemoryBlock>) {

        let mem_size = memory_block.len() - 1;
        for mem in 1..mem_size{
            if memory_block[mem].id == -1 && memory_block[mem - 1].id == -1 {

                memory_block[mem].width += memory_block[mem - 1].width;
                memory_block[mem - 1].width = 0;
            }
        }

        for mem in (0..memory_block.len() - 1).rev() {
            if memory_block[mem].width == 0 {
                memory_block.remove (mem);
            }
        }
    }

    fn _print_vec (vec: &Vec<i32>) {
        for char in vec {
            if *char == -1 {
                print!("{}", '.');
            } else {
                print!("{}", char);
            }
        }

        println!();
        println!();
    }

    fn _process (contents:&str) -> u128 {
        let mut memory_layout = generate_memory_layout (&contents);
        sort_memory (&mut memory_layout);
        sum_values(&memory_layout)
    }

    fn process_v2 (contents:&str) -> u128 {
        let memory_layout = generate_memory_layout (&contents);
        let mut memory_blocks =  create_struct_layout(&memory_layout);
        sort_memory_blocks (&mut memory_blocks);
        let result_array = mem_blocks_to_array(&memory_blocks);
        sum_values (&result_array)
    }

    pub fn _do_task()
    {
        let start = Instant::now();

        let mut contents = utils::read_file("/Users/reubenduckering/Documents/Personal Repo/Advent-Of-Code-2024/files/day_9.txt");
        let result = process_v2 (&mut contents);

        let duration = start.elapsed();
        println!("Time elapsed: {:?}", duration);
        println!("{}", result);
        // 6321541228194
        // 6385949478862 wrong
    }
}