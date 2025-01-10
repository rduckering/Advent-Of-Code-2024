pub mod day_17
{
    use crate::utils::utils;

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_case_1 () {
            let input = vec![(2, 6)];
            let mut instructions = Instructions { reg_a: 0, reg_b:0, reg_c: 9, opcodes: Vec::new() };

            process_instructions (&input, &mut instructions);
            assert_eq!(instructions.reg_a, 0);
            assert_eq!(instructions.reg_b, 1);
            assert_eq!(instructions.reg_c, 9);
        }

        #[test]
        fn test_case_2 () {
            let input = vec![(5, 0), (5, 1), (5, 4)];
            let mut instructions = Instructions { reg_a: 10, reg_b:0, reg_c: 0, opcodes: Vec::new() };

            println! ("");
            println! ("opcode 5");
            process_instructions (&input, &mut instructions);

            println! ("");
        }

        #[test]
        fn test_case_div_1 () {
            let input = vec![(0,1)];
            let mut instructions = Instructions { reg_a: 2024, reg_b:0, reg_c: 0, opcodes: Vec::new() };

            process_instructions (&input, &mut instructions);
            assert_eq!(instructions.reg_a, 1012);
        }

        #[test]
        fn test_case_div_2 () {
            let input = vec![(0,5)];
            let mut instructions = Instructions { reg_a: 2048, reg_b:4, reg_c: 0, opcodes: Vec::new() };

            process_instructions (&input, &mut instructions);
            assert_eq!(instructions.reg_a, 128);
        }

        // #[test]
        // fn test_case_loop_div () {
        //     let input = vec![(0, 1), (5, 4), (3, 0)];
        //     let mut instructions = Instructions { reg_a: 2024, reg_b: 0, reg_c: 0, opcodes: Vec::new() };
        //
        //     println! ("");
        //     println! ("opcode loop test");
        //     process_instructions (&input, &mut instructions);
        //     assert_eq!(instructions.reg_a, 0);
        //     println! ("");
        // }

        #[test]
        fn test_case_3 () {
            let input = vec![(1, 7)];
            let mut instructions = Instructions { reg_a: 0, reg_b: 29, reg_c: 0, opcodes: Vec::new() };

            process_instructions (&input, &mut instructions);
            assert_eq!(instructions.reg_b, 26);
        }

        #[test]
        fn test_case_4 () {
            let input = vec![(4, 0)];
            let mut instructions = Instructions { reg_a: 0, reg_b: 2024, reg_c: 43690, opcodes: Vec::new() };

            process_instructions (&input, &mut instructions);
            assert_eq!(instructions.reg_b, 44354);
        }
    }

    // instructions

    struct Instructions {
        reg_a: u64,
        reg_b: u64,
        reg_c: u64,

        opcodes: Vec<u32>,
    }

    fn parse_input (contents: &str) -> Instructions {

        let mut input = Instructions { reg_a: 0, reg_b: 0, reg_c: 0, opcodes: Vec::new() };

        for line in contents.lines() {
            if line.contains ("Register A:") {
                if let Some((_, right)) = line.split_once(':') {
                    input.reg_a = right.trim().parse::<u64>().unwrap();
                }
            }

            else if line.contains ("Register B:") {
                if let Some((_, right)) = line.split_once(':') {
                    input.reg_b = right.trim().parse::<u64>().unwrap();
                }
            }

            else if line.contains ("Register C:") {
                if let Some((_, right)) = line.split_once(':') {
                    input.reg_c = right.trim().parse::<u64>().unwrap();
                }
            }

            else if line.contains ("Program") {

                if let Some((_, right)) = line.split_once(':') {
                    let split_line: Vec<&str> = right.trim().split(',').collect();

                    for instruction in split_line.iter() {
                        input.opcodes.push(instruction.parse::<u32>().unwrap());
                    }
                }
            }
        }

        input
    }

    fn opcode_0 (oprand: u64, instructions : &mut Instructions, _output: &mut Vec<u32>) -> bool {
        // adv, opcode 0 divides - numerator is in register A, trucated to a int and put in register a

        let oprand_lit = oprand & 0b11;
        let nom = instructions.reg_a.clone();
        let mut demon = u64::pow ( oprand_lit, 2);

        if oprand > 3 {

            let value =  match oprand {
                4 => Some (instructions.reg_a),
                5 => Some (instructions.reg_b),
                6 => Some (instructions.reg_c),
                _ => None,
            };

            if let Some (v) = value {
                demon = u64::pow (v, 2);
            }
        }

        if nom == 0 || demon == 0 {
            return false;
        }

        let result = nom / demon;

        instructions.reg_a = result;

        true
    }

    fn opcode_1 (oprand: u64, instructions : &mut Instructions, _output: &mut Vec<u32>) -> bool {
        // bxl, opcode 1 bitwise op of reg b store result in b
        let oprand_lit = oprand;

        instructions.reg_b = oprand_lit ^ instructions.reg_b;
        true
    }

    fn opcode_2 (oprand: u64, instructions : &mut Instructions, _output: &mut Vec<u32>) -> bool {
        // bst, opcode 2 % 8 and store to reg b
        let oprand_lit = oprand & 0b11;

        if oprand > 3 {
            let value = match oprand {
                4 => Some(instructions.reg_a & 0b111),
                5 => Some(instructions.reg_b & 0b111),
                6 => Some(instructions.reg_c & 0b111),
                _ => None,
            };

            if let Some(v) = value {
                instructions.reg_b = v;
                return true;
            }
        }

        instructions.reg_b = oprand_lit;
        true
    }

    #[allow(dead_code)]

    fn opcode_3 (_oprand: u64, _instructions : &mut Instructions, _output: &mut Vec<u32>) -> bool {
        // jnz, opcode 3 do nothing  if reg a is 0, jumps instruction point to literal oparand (dont not increase point by 2 )
        //implemented else where
        true
    }

    fn opcode_4 (_oprand: u64, instructions : &mut Instructions, _output: &mut Vec<u32>) -> bool {
        // bxc, opcode 4 bitwise of reg b and reg c, store in reg b. reads operand and ignores it
        instructions.reg_b = instructions.reg_b ^ instructions.reg_c;
        true
    }

    fn opcode_5 (oprand: u64, instructions : &mut Instructions, output: &mut Vec<u32>) -> bool {
        // out, opcode 5 calculates value of combo oerand module 8, multiple value seperated by commas
        let oprand_lit = oprand & 0b11;

        if oprand > 3 {

            let value =  match oprand {
                4 => Some (instructions.reg_a & 0b111),
                5 => Some (instructions.reg_b & 0b111),
                6 => Some (instructions.reg_c & 0b111),
                _ => None,
            };

            if let Some(v) = value {
                output.push (v as u32);
                // print!("{},", v);
                return true;
            }
        }

        // print!("{},", oprand_lit);
        true
    }

    fn opcode_6 (oprand: u64, instructions : &mut Instructions, _output: &mut Vec<u32>) -> bool {
        // bdv, opcode 0 divides - numerator is in register A, trucated to a int and put in register a

        let oprand_lit = oprand & 0b11;
        let nom = instructions.reg_a.clone();
        let mut demon = u64::pow (oprand_lit, 2);

        if oprand > 3 {

            let value =  match oprand {
                4 => Some (instructions.reg_a),
                5 => Some (instructions.reg_b),
                6 => Some (instructions.reg_c),
                _ => None,
            };

            if let Some (v) = value {
                demon = u64::pow  (v, 2);
            }
        }

        if nom == 0 || demon == 0 {
            return false;
        }

        let result = nom / demon;

        instructions.reg_b = result;

        true
    }

    fn opcode_7 (oprand: u64, instructions : &mut Instructions, _output: &mut Vec<u32>) -> bool {
        // cdv, opcode 0 divides - numerator is in register A, trucated to a int and put in register a

        let oprand_lit = oprand & 0b11;
        let nom = instructions.reg_a.clone();
        let mut demon = u64::pow  (oprand_lit, 2);

        if oprand > 3 {

            let value =  match oprand {
                4 => Some (instructions.reg_a),
                5 => Some (instructions.reg_b),
                6 => Some (instructions.reg_c),
                _ => None,
            };

            if let Some (v) = value {
                demon = u64::pow (v, 2);
            }
        }

        if nom == 0 || demon == 0 {
            return false;
        }

        let result = nom / demon;

        instructions.reg_c = result;

        true
    }

    #[allow(dead_code)]
    fn woops (_oprand: u64, _instructions : &mut Instructions, _output: &mut Vec<u32>) -> bool {
        println!("Woops!");
        false
    }

    fn get_opcode_function (opcode:u32) -> fn(u64, &mut Instructions, &mut Vec<u32>) -> bool {
        match opcode {
            0 => opcode_0,
            1 => opcode_1,
            2 => opcode_2,
            3 => opcode_3,
            4 => opcode_4,
            5 => opcode_5,
            6 => opcode_6,
            7 => opcode_7,
            _ => woops
        }
    }

    fn process_instructions (pairs: &Vec<(u32, u32)>, instructions : &mut Instructions) {

        let mut output: Vec<u32> = Vec::new();
        let mut reg_value: u64 = 1;

        let mut largest_num_outputs = 0;

        loop
        {
            let mut index = 0;
            instructions.reg_a = 0 + u64::pow (reg_value,255); //u64::pow (reg_value, 2);
            instructions.reg_b = 0;
            instructions.reg_c = 0;

            output.clear();
            let mut major_fail = true;

            // if reg_value % 1000000 == 0 {
            //     println!("trying value: {:?}", reg_value);
            // }

            loop
            {
                if (index >= pairs.len()) {
                    break;
                }

                let pair = pairs.get(index).unwrap();

                if pair.0 == 3 && instructions.reg_a != 0 {
                    index = pair.1 as usize;
                    continue;
                }

                let opcode = get_opcode_function(pair.0);
                major_fail = opcode(pair.1 as u64, instructions, &mut output);
                index += 1;

                if ! major_fail {
                    break;
                }

                if output.len() > instructions.opcodes.len() {
                    break;
                }

                let mut index = 0;
                let mut mis_matching = false;

                for i in 0..output.len() {
                    if output[i] != instructions.opcodes[i] {
                        mis_matching = true;
                        break;
                    }
                }

                if mis_matching {
                    break;
                }


                if index >= pairs.len() {
                    break;
                }
            }

            if output.len() == instructions.opcodes.len() {
                let mut mis_matching = false;

                for i in 0..output.len() {
                    if output[i] != instructions.opcodes[i] {
                        mis_matching = true;
                        break;
                    }
                }

                if ! mis_matching {
                    break;
                }
            }
            reg_value += 1;

            if output.len() > largest_num_outputs {
                largest_num_outputs = output.len();
                println! ("new max amount: {:?}", largest_num_outputs);
            }
        }

        println! ("register a value: {:?}", reg_value);
    }


    // Part 2 not complete, may not run without error
    pub fn do_task()
    {
        let contents = utils::read_file("/Users/reubenduckering/Documents/Personal Repo/Advent-Of-Code-2024/files/day_17_mini.txt");
        let mut instructions = parse_input (&contents);

        let instruction_pairs: Vec<(u32, u32)> = instructions.opcodes
                                .iter()
                                .step_by(2)
                                .zip(instructions.opcodes.iter().skip(1).step_by(2))
                                .map(|(&a, &b)| (a, b))
                                .collect();

        process_instructions (&instruction_pairs, &mut instructions);

        println! ("stop");
    }
}