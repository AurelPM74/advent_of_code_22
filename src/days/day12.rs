use crate::utils;
use fast_paths::InputGraph;
use crate::day8;
use itertools::Itertools;


//Create a graph, where each node is a ppoint in the map (a char in the input)
//and a edge between a and b iff a is equal or less than 1 elevation higher/lower
fn create_graph(input: Vec<Vec<i32>>) -> InputGraph{
    //Empty graph
    let mut input_graph = InputGraph::new();

    //we add a name to all the nodes
    let mut counter: i64 = -1;
    let input_named: Vec<Vec<(i64, i32)>> = input.iter()
                                                   .map(|line|line.iter().map(|&node|{counter = counter + 1; (counter, node)}).collect::<Vec<_>>())
                                                   .collect();

    //Add the edges between each point if e_i - e_j <= 1
    //We will add all edges horizontaly, flip the input using a method defined in day8 and do the same thing
    
    for line in input_named.iter(){
        //Add lateraly the edges
        for (node_1, node_2) in line.iter().tuple_windows(){
            //If we go down
            if node_2.1 <= node_1.1{
                input_graph.add_edge(node_1.0 as usize, node_2.0  as usize, 1);
            } else {
                //In that case we have an edge in the other direction
                input_graph.add_edge(node_2.0 as usize, node_1.0  as usize, 1);
            }
            //In any case if we are less then one we have an edge in each direction 
            if (node_2.1 - node_1.1).abs() <= 1{
                //If we go up, only if its by less one, but we can go in both direction
                input_graph.add_edge_bidir(node_1.0 as usize, node_2.0  as usize, 1);
            } 
            
        }
    }
    let flipped_input = day8::row_to_col(&input_named);
    for line in flipped_input.iter(){
        for (node_1, node_2) in line.iter().tuple_windows(){
            //If we go down
            if node_2.1 <= node_1.1{
                input_graph.add_edge(node_1.0 as usize, node_2.0  as usize, 1);
            } else {
                //In that case we have an edge in the other direction
                input_graph.add_edge(node_2.0 as usize, node_1.0  as usize, 1);
            }
            //In any case if we are less then one in each direction we have an edge
            if (node_2.1 - node_1.1).abs() <= 1{
                //If we go up, only if its by less one, but we can go in both direction
                input_graph.add_edge_bidir(node_1.0 as usize, node_2.0  as usize, 1);
            } 
        }
    }
    input_graph

}

//First part of the day
pub fn solve_1() -> u32{
    //read the input
    let input = utils::read_input_of_the_day_lines(12);

    //Transform each line into a vec of chars
    let mut input_char: Vec<Vec<char>> = input.iter().map(|string| string.chars().collect::<Vec<char>>()).collect();

    //For this part we need to find the shortest path between two point
    //To do that we will do a graph, and find the shortest path between S and E, first we 
    //record their position:
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (index, line) in input_char.iter().enumerate(){
        match line.iter().position(|&ch| ch == 'S'){
            Some(T) => {start = (index, T)},
            _ => ()
        }
        match line.iter().position(|&ch| ch == 'E'){
            Some(T) => {end = (index, T)},
            _ => ()
        }
    }
    //We remove E and S and put a and z instead:
    input_char[start.0][start.1] = 'a';
    input_char[end.0][end.1] = 'z';

    //parse the input a -> 0, ..., z -> 25,
    let input_digit = input_char.iter().map(|v| v.iter().map(|&c| c as i32 - 97).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();

    //Create the graph
    let mut graph = create_graph(input_digit);
    //println!("{:?}", graph);

    //freeze it
    graph.freeze();

    // prepare the graph for fast shortest path calculations.
    let fast_graph = fast_paths::prepare(&graph);

    //compute the node if of start and end
    let start_id = start.0 * input[0].len() + start.1;
    let end_id = end.0 * input[0].len() + end.1;

    let shortest_path = fast_paths::calc_path(&fast_graph, start_id, end_id);
    

    match shortest_path {
        Some(p) => {
            // the weight of the shortest path
            let weight = p.get_weight();
            return weight as u32
        },
        None => {
            panic!("No path found")
        }
    }
}
//First part of the day
pub fn solve_2() -> u32{
    //read the input
    let input = utils::read_input_of_the_day_lines(12);

    //Transform each line into a vec of chars
    let mut input_char: Vec<Vec<char>> = input.iter().map(|string| string.chars().collect::<Vec<char>>()).collect();

    //For this part we need to find the shortest path between two point
    //To do that we will do a graph, and find the shortest path between S and E, first we 
    //record their position:
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (index, line) in input_char.iter().enumerate(){
        match line.iter().position(|&ch| ch == 'S'){
            Some(T) => {start = (index, T)},
            _ => ()
        }
        match line.iter().position(|&ch| ch == 'E'){
            Some(T) => {end = (index, T)},
            _ => ()
        }
    }
    //We remove E and S and put a and z instead:
    input_char[start.0][start.1] = 'a';
    input_char[end.0][end.1] = 'z';

    //parse the input a -> 0, ..., z -> 25,
    let input_digit = input_char.iter().map(|v| v.iter().map(|&c| c as i32 - 97).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();

    //Create the graph
    let mut graph = create_graph(input_digit);
    //println!("{:?}", graph);

    //freeze it
    graph.freeze();

    // prepare the graph for fast shortest path calculations.
    let fast_graph = fast_paths::prepare(&graph);   

    let end_id = end.0 * input[0].len() + end.1;

    //Compute the id of all the possible starting points (we can see that we need to start from the 3 first columns since all the b are on the right edge)
    //Could use better opti like adding an edge to every a from S (is it more optimal ?)
    let starting_id: Vec<usize> = (0..=40).map(|x| x * input_char[0].len()).collect();
    let mut weight_min = 420;
    for i in starting_id{
        let shortest_path = fast_paths::calc_path(&fast_graph, i, end_id);
        match shortest_path {
            Some(p) => {
                // the weight of the shortest path
                let weight = p.get_weight();
                if weight <= weight_min {weight_min = weight}
            },
            None => {
                continue;
            }
        }
    }
    weight_min as u32
}

mod tests {
    
    use fast_paths::Weight;

    use super::*;
    //Test first part of the puzzle
    #[test]
    fn test_solve_1(){

        //read the input
    let input = Vec::from([String::from("Sabqponm"),
        String::from("abcryxxl"),
        String::from("accszExk"),
        String::from("acctuvwj"),
        String::from("abdefghi")]);

    //Transform each line into a vec of chars
    let mut input_char: Vec<Vec<char>> = input.iter().map(|string| string.chars().collect::<Vec<char>>()).collect();

    //For this part we need to find the shortest path between two point
    //To do that we will do a graph, and find the shortest path between S and E, first we 
    //record their position:
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (index, line) in input_char.iter().enumerate(){
        match line.iter().position(|&ch| ch == 'S'){
            Some(T) => {start = (index, T)},
            _ => ()
        }
        match line.iter().position(|&ch| ch == 'E'){
            Some(T) => {end = (index, T)},
            _ => ()
        }
    }
    //We remove E and S and put a and z instead:
    input_char[start.0][start.1] = 'a';
    input_char[end.0][end.1] = 'z';

    //parse the input a -> 0, ..., z -> 25,
    let input_digit = input_char.iter().map(|v| v.iter().map(|&c| c as i32).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();


    //Create the graph
    let mut graph = create_graph(input_digit);

    println!("{:?}", graph);    
    //freeze it
    graph.freeze();

    // prepare the graph for fast shortest path calculations.
    let fast_graph = fast_paths::prepare(&graph);

    //compute the node if of start and end
    let start_id = start.0 * input[0].len() + start.1;
    let end_id = end.0 * input[0].len() + end.1;


    let shortest_path = fast_paths::calc_path(&fast_graph, start_id, end_id);

    match shortest_path {
        Some(p) => {
            // the weight of the shortest path
            let weight = p.get_weight();
            let nodes = p.get_nodes();
            println!("{:?}", nodes);
            assert_eq!(weight, 31);
        },
        None => {
            panic!("No path found")
        }

    }
}
    



    #[test]
    //Test second part of the puzzle
fn test_solve_2 (){
    //read the input
    let input = Vec::from([String::from("Sabqponm"),
    String::from("abcryxxl"),
    String::from("accszExk"),
    String::from("acctuvwj"),
    String::from("abdefghi")]);

    //Transform each line into a vec of chars
    let mut input_char: Vec<Vec<char>> = input.iter().map(|string| string.chars().collect::<Vec<char>>()).collect();

    //For this part we need to find the shortest path between two point
    //To do that we will do a graph, and find the shortest path between S and E, first we 
    //record their position:
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (index, line) in input_char.iter().enumerate(){
        match line.iter().position(|&ch| ch == 'S'){
            Some(T) => {start = (index, T)},
            _ => ()
        }
        match line.iter().position(|&ch| ch == 'E'){
            Some(T) => {end = (index, T)},
            _ => ()
        }
    }
    //We remove E and S and put a and z instead:
    input_char[start.0][start.1] = 'a';
    input_char[end.0][end.1] = 'z';

    //parse the input a -> 0, ..., z -> 25,
    let input_digit = input_char.iter().map(|v| v.iter().map(|&c| c as i32).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();


    //Create the graph
    let mut graph = create_graph(input_digit);

    println!("{:?}", graph);    
    //freeze it
    graph.freeze();

    // prepare the graph for fast shortest path calculations.
    let fast_graph = fast_paths::prepare(&graph);

    //compute the node id of end point
    let end_id = end.0 * input[0].len() + end.1;

    //Compute the id of all the possible starting points (we can see that we need to start from the 3 first columns since all the b are on the right edge)
    //Could use better opti like adding an edge to every a from S (is it more optimal ?)
    let starting_id = Vec::from([0, 8, 16, 24, 32]);
    let mut weight_min = 32;
    for i in starting_id{
        let shortest_path = fast_paths::calc_path(&fast_graph, i, end_id);
        match shortest_path {
            Some(p) => {
                // the weight of the shortest path
                let weight = p.get_weight();
                if weight <= weight_min {weight_min = weight}
            },
            None => {
                continue;
            }
        }
    }
    assert_eq!(weight_min, 29)


    

}
}
        
