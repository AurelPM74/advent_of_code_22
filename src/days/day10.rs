use crate::utils::{self, read_input_of_the_day_lines};

#[derive(Debug)]
struct State {
    x: i64,
    cycles: Vec<i64>,
}

struct Crt{
    sprite_position: i64,
    drawing: Vec<char>,
}

//Compute the crt lines according to the states of a processor
fn compute_crt_lines(state: State, crt: &mut Crt) {

    let mut curren_drawing_position = 0;
    //for each cycle
    for cycle in state.cycles{
        //Update the position of the sprite
        crt.sprite_position = cycle;
        if curren_drawing_position >= crt.sprite_position - 1 && curren_drawing_position <= crt.sprite_position + 1{
            crt.drawing.push('#');
        } else {
            crt.drawing.push('.')
        }
        curren_drawing_position += 1;
        if curren_drawing_position > 39 {curren_drawing_position = 0}
    }
}

//Compute the state of the processor on each cycle 
fn compute_states(input: Vec<String>, processor: &mut State){

    //iterate over the input
    for line in input {
        //if line = noop, pass one cycle with the value x
        if line == String::from("noop"){
            processor.cycles.push(processor.x)
        } else {
            //extract the number of time to add
            let to_add = line.strip_prefix("addx ").unwrap().parse::<i64>().unwrap();
            //add two cycle without chnging the state
            processor.cycles.push(processor.x);
            processor.cycles.push(processor.x);
            //change the state
            processor.x = processor.x + to_add;
        }
    }
}

pub fn solve_1() -> i64{

    //Read the input
    let inputs = read_input_of_the_day_lines(10);

    //Initialize the processor and compute the states
    let mut processor = State{
        x: 1,
        cycles: Vec::new(),
    };

    compute_states(inputs, &mut processor);

    //Compute the strenght
    let mut strenght = 0;

    let index = [20, 60, 100, 140, 180, 220];
    for i in index {
        strenght += i as i64 * processor.cycles[i-1];
    }
    strenght
}

pub fn solve_2() {
    //Read the input
    let inputs = read_input_of_the_day_lines(10);

    //Initialize the processor and compute the states
    let mut processor = State{
        x: 1,
        cycles: Vec::new(),
    };

    compute_states(inputs, &mut processor);

    //initialize crt
    let mut crt = Crt {
        sprite_position: 1,
        drawing: Vec::new(),
    };

    compute_crt_lines(processor, &mut crt);

    //Draw lines
    println!("{:?}", crt.drawing[0..39].iter().collect::<String>());
    println!("{:?}", crt.drawing[40..79].iter().collect::<String>());
    println!("{:?}", crt.drawing[80..119].iter().collect::<String>());
    println!("{:?}", crt.drawing[120..159].iter().collect::<String>());
    println!("{:?}", crt.drawing[160..199].iter().collect::<String>());
    println!("{:?}", crt.drawing[200..239].iter().collect::<String>());
}

//TESTS
mod tests {
    
    use super::*;
    //Test first part of the puzzle
    #[test]
    fn test_solve_1(){
        //test input for first part
        let test_input = utils::read_input_path("input/day_10_input_test.txt");

        let mut processor = State{
            x: 1,
            cycles: Vec::new(),
        };

        compute_states(test_input, &mut processor);

        let mut strenght = 0;

        let index = [20, 60, 100, 140, 180, 220];
        for i in index {
            strenght += i as i64 * processor.cycles[i-1];
        }

        assert_eq!(strenght,13140)
    }

    #[test]
    fn test_solve_2 (){
        //test input for first part
        let test_input = utils::read_input_path("input/day_10_input_test.txt");

        let mut processor = State{
            x: 1,
            cycles: Vec::new(),
        };

        compute_states(test_input, &mut processor);

        //initialize crt
        let mut crt = Crt {
            sprite_position: 1,
            drawing: Vec::new(),
        };

        compute_crt_lines(processor, &mut crt);

        //println!("{:?}", crt.drawing[0..39].iter().collect::<String>());
        //println!("{:?}", crt.drawing[40..79].iter().collect::<String>());
        //println!("{:?}", crt.drawing[80..119].iter().collect::<String>());
        //println!("{:?}", crt.drawing[120..159].iter().collect::<String>());
        //println!("{:?}", crt.drawing[160..199].iter().collect::<String>());
        //println!("{:?}", crt.drawing[200..239].iter().collect::<String>());
        assert_eq!(crt.drawing.iter().collect::<String>(), String::from("##..##..##..##..##..##..##..##..##..##..###...###...###...###...###...###...###.####....####....####....####....####....#####.....#####.....#####.....#####.....######......######......######......###########.......#######.......#######....."));

    }

}