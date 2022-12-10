use crate::utils::{self, read_input_of_the_day_lines};
use std::fmt;

//Struct of the rope: it has a current head position, a current tail position, a list of position visited
struct Rope{
    head: (i64, i64),
    tail: (i64, i64),
    //We will keep track of the positions visited as a matrix 1000*1000
    //We assume that that will be enough, the starting point will be (500, 500) 
    positions: Vec<Vec<u64>>,
    visited: u64
}
//Method of the rope
impl Rope{

    //Main method: using an instruction this method will:
    //update the head position
    //move the tail accordingly
    //update the position visited
    fn next_instruction(&mut self, instruction: (char, u64)){
        //Extract the move from the instruction
        let to_move_head = match instruction.0 {
            'R' => (0, 1),
            'L' => (0, -1),
            'U' => (-1, 0),
            'D' => (1, 0),
             _ => panic!("Not an instruction")
        };

        //We move the head accordingly, for each step we update the tail position:
        for i in 1..=instruction.1{
            //Move head one step
            self.head = (self.head.0 + to_move_head.0, self.head.1 + to_move_head.1);

            //update the tail position
            self.update_tail_position();

            //update the matrix of positions visited
            self.update_position_visited();

            //println!("{}", instruction.0);
            //for line in self.matrix() {
            //    println!("{:?}", line);
            //}
            //println!("     ");
        }
        
    }

    //Update the position of the tail after a move of one of the head
    fn  update_tail_position(&mut self) {

        //If the head and tail are in the same "radius" of one we do nothing:
        if (self.head.0 - self.tail.0).abs() > 1 || (self.head.1 - self.tail.1).abs() > 1{

            //in which direction lateraly we need to move the tail, zero if above/below
            let lat_dir = self.tail.1 - self.head.1;

            //in which direction verticaly we need to move the tail, zero if left right
            let vert_dir = self.tail.0 - self.head.0;

            //Case were the head is at the same level of the head 
            if vert_dir == 0 {

                self.tail.1 = self.tail.1 - lat_dir/lat_dir.abs()

            } else {

                //The head is above or below the tail, not directly diagonal
                //in any case we move the head up/down 
                self.tail.0 = self.tail.0 - vert_dir/vert_dir.abs();

                //and the tail move lateraly in the direction of the head if the head isn't just above
                if lat_dir != 0 {
                    //move diag
                    self.tail.1 = self.tail.1 - lat_dir/lat_dir.abs();
                }
                
            }
        }
    }
    
    //Update the matrix of visited position
    fn update_position_visited(&mut self){
        //Check if the tail is in and unvisited case, if it is we change the matrix
        if self.positions[self.tail.0.abs() as usize][self.tail.1.abs() as usize] == 0 {
            self.positions[self.tail.0.abs() as usize][self.tail.1.abs() as usize] = 1;
            self.visited += 1
        }
    }
    //position to matrix of 0, T, H
    fn matrix(&self) -> Vec<Vec<i64>>{
        let len = self.positions[0].len();
        let mut matrix = std::vec::from_elem(std::vec::from_elem(0, len), len);
        matrix[self.tail.0.abs() as usize][self.tail.1.abs() as usize] = 2;
        matrix[self.head.0.abs() as usize][self.head.1.abs() as usize] = 1;
        matrix
    }
}

//Display trait to print the current position
impl fmt::Display for Rope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let matrix = self.matrix();

        write!(f, "   ")
    }
}




pub fn solve_1() -> u64 {
    //load the inputs 
    let instruction_string = read_input_of_the_day_lines(9);

    //Parse the input
    let instruction_chars:Vec<Vec<String>> = instruction_string.iter().map(|str| str.split_whitespace().map(String::from).collect()).collect();
    let instructions:Vec<(char, u64)> = instruction_chars.iter().map(|string| (string[0].parse::<char>().unwrap(),string[1].parse::<u64>().unwrap())).collect();
    println!("{:?}", instructions.len());

    //initialize the rope, it start in the middle of a 2000*2000 grid
    let mut rope_simulation = Rope {
        head: (500, 500),
        tail: (500, 500),
        positions: std::vec::from_elem(std::vec::from_elem(0 as u64, 1000), 1000),
        visited: 0
    };

    //Add the starting position to the matrix of visited position
    rope_simulation.update_position_visited();

    //Simulate
    for ins in instructions{
        rope_simulation.next_instruction(ins)
    }

    return  rope_simulation.visited;

}

pub fn solve_2() -> u64 {

    0
}

mod tests {
    
    use itertools::Positions;

    use super::*;
    //Test first part of the puzzle
    #[test]
    fn test_solve_1(){
        //initialize rope struc
        let len = 6;
        let position = std::vec::from_elem(std::vec::from_elem(0 as u64, 6), 5);

        let mut ropes = Rope {
            head: (4, 0),
            tail: (4, 0),
            positions: position,
            visited: 0
        };
            let instructions = [('U', 1),
                                ('R', 1),
                                ('U', 1),
                                ('D', 1),
                                ('R', 4),
                                ('D', 1),
                                ('L', 5),
                                ('R', 2)];
        //let instructions = [('U', 1),('L', 2)];
        for ins in instructions{
            ropes.next_instruction(ins);
        }
        let case_visited: u64 = ropes.positions.iter().map(|vec| vec.iter().sum::<u64>()).sum();
        assert_eq!(case_visited, 13)
    }
}