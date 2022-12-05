use crate::utils::{self, read_input_of_the_day_lines, sub_array_block_from_delim};

//From vec of string containing the position, we extract the list of crate in heach stack,
//Return a vec of vec of strings
fn stack_from_position(positions:&Vec<String>) -> Vec<Vec<char>>{
    //initialize the stacks
    let mut stacks = Vec::new();

    //infer the number of stacks
    let n_stacks = positions.last().unwrap().len()/4 + 1;

    //The last line of the positions contains the numbering of the stacks, use it to initialize the stack
    for i in 0..=n_stacks - 1{
        stacks.push(Vec::new())
    }
    //We drop the last line of numbering
    let mut stacks_temp = positions.clone();
    stacks_temp.pop();

    //
    for line in stacks_temp.iter_mut().rev(){
        //add a whitespace at the end in order to make each stack containing 4 char
        line.push(' ');

        //empty stacks for matching:
        let empty_stack = String::from("    ");

        //for every chunck of 4 characters, we read the name, or abscence of, the crate and push it 
        for (stack_i, chunk) in line.chars().collect::<Vec<_>>().chunks(4).enumerate(){

            let chunk_string = String::from_iter(chunk);

            if chunk_string != empty_stack{
                //remove the [ and ] and push it into the corresponding stack,
                stacks[stack_i].push(chunk[1])
            }
        } 
    }
    stacks

}

//From an instruction move n from s_1 to s_2, we extract n s_1 s_2
fn values_from_instruction(instruction : &String) -> Vec<usize>{
    //Split along the whitespaces
    let split_string: Vec<String> = instruction.split_whitespace().map(String::from).collect();
    //We extract the 2nd, 4th, and 6th word and convertthem to int
    let values = Vec::from([split_string[1].parse::<usize>().unwrap(),
        split_string[3].parse::<usize>().unwrap(),
        split_string[5].parse::<usize>().unwrap()]);
    values
}


pub fn solve_1() -> String{
    //We read the input, vec of strings
    let input_lines = read_input_of_the_day_lines(5);

    //We extract the crate positions from the move, separating the to at the empty line
    let position_instruction = sub_array_block_from_delim(input_lines, String::from(""));
    let (positions, instructions) = (position_instruction[0].clone(), position_instruction[1].clone()); 

    //We compute the list of name of each crate in each stack
    let mut stack = stack_from_position(&positions);

    //From an instruction "move n from s_1 to s_2", we extract n s_1 s_2
    let instruction_values: Vec<Vec<usize>> = instructions.iter().map(|ins| values_from_instruction(ins)).collect();

    //We execute all the instructions by first poping n crate from stack s_1 and
    //pushing those crate into s_2:
    for instruction in instruction_values{
        //for each crate to move 
        for i in 0..=instruction[0]-1{
            let crate_to_move = stack[instruction[1]-1].pop().unwrap();
            stack[instruction[2]-1].push(crate_to_move);
        }
    }

    //Extract the top crate of each stack:
    let top_crates:Vec<char> = stack.into_iter().map(|stack| *stack.last().unwrap()).collect();
    String::from_iter(top_crates)
}

pub fn solve_2() -> String{
    //We read the input, vec of strings
    let input_lines = read_input_of_the_day_lines(5);

    //We extract the crate positions from the move, separating the to at the empty line
    let position_instruction = sub_array_block_from_delim(input_lines, String::from(""));
    let (positions, instructions) = (position_instruction[0].clone(), position_instruction[1].clone()); 

    //We compute the list of name of each crate in each stack
    let mut stack = stack_from_position(&positions);

    //From an instruction "move n from s_1 to s_2", we extract n s_1 s_2
    let instruction_values: Vec<Vec<usize>> = instructions.iter().map(|ins| values_from_instruction(ins)).collect();

    //We execute all the instructions by first poping n crate from stack s_1 and
    //pushing those crate into s_2 in the reverse order as the crane move all the crate at once:
    for instruction in instruction_values{
        //Temp stack of crate, used to be reversed
        let mut temp_stack = Vec::new();
        //for each crate to move 
        for i in 0..=instruction[0]-1{
            let crate_to_move = stack[instruction[1]-1].pop().unwrap();
            temp_stack.push(crate_to_move);
        }
        //Reverse the stack of crate to move
        temp_stack = temp_stack.into_iter().rev().collect();
        //append the temp stack to the corresponding stack, this clear the temp_stack
        stack[instruction[2]-1].append(&mut temp_stack);

    }

    //Extract the top crate of each stack:
    let top_crates:Vec<char> = stack.into_iter().map(|stack| *stack.last().unwrap()).collect();
    String::from_iter(top_crates)
}