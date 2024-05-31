use std::fs;

pub fn final_position_multiplied() -> i32{
    let positional_instructions: String = fs::read_to_string("day2_move_instructions")
        .expect("Something went wrong reading the file");
    let instructions:Vec<&str> = positional_instructions.lines().collect();
    //let mut parsed_instructions: Vec<Vec<&str>> = Vec::new();

    let mut horisontal: i32 = 0;
    let mut vertical: i32 = 0;

    for instruction in instructions {
        let parsed_instruciton: Vec<&str> = instruction.split(' ').collect();
        //parsed_instructions.push(parsed_instruciton);

        let parsed_instruction_int = parsed_instruciton[1].parse::<i32>().unwrap();

        match parsed_instruciton[0] {
            "up" => vertical -= parsed_instruction_int,
            "down" => vertical += parsed_instruction_int,
            "forward" => horisontal += parsed_instruction_int,
            _ => println!("Unknown instruction")
        }
    }
    println!("{}",horisontal*vertical);
    horisontal*vertical
}