use std::fs;
use std::io::{BufReader, BufRead};

//read input of day number "day" and return a String (with breaklines)
pub fn read_input_string(day: u8) -> String{

    let path = format!("input/day_{}_input.txt", day);
    
    let input_of_the_day  = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("Unable to retreive inputs from {}, error: {}", path, err));

    input_of_the_day

}

//read input of day number "day" and return vector of String, one string per line
pub fn read_input_of_the_day_lines(day: u8) -> Vec<String>{
    
    let path = format!("input/day_{}_input.txt", day);

    //Open file
    let file = fs::File::open(&path)
        .unwrap_or_else(|err| panic!("Unable to retreive inputs from {}, error: {}", path, err));

    let reader = BufReader::new(file);

    reader.lines()
          .map(|l| l.unwrap())
          .collect()
}  
//REad lines from path 
pub fn read_input_path(path : &str) -> Vec<String>{

    //Open file
    let file = fs::File::open(path)
        .unwrap_or_else(|err| panic!("Unable to retreive inputs from {}, error: {}", path, err));

    let reader = BufReader::new(file);

    reader.lines()
          .map(|l| l.unwrap())
          .collect()
}  


//take a reference to an array containing strings and return the array of u32, panic if not possible
pub fn array_string_to_int(array_string: &Vec<String>) -> Vec<u32>{
    //iterate over the words, map a string to an int (panic if not possible), and collect
    array_string.iter()
                .map(|s| s.parse::<u32>().unwrap())
                .collect()

} 

//take vectors of variables having the len() method, encapsulate in a sub-array, all blocks delimited by empty strings,
//Destroy original array
// ["a", "a", "", "b"] --> [["a","a"],["b"]]
pub fn sub_array_block_from_delim<T>(vector: Vec<T>, delimiter: T) -> Vec<Vec<T>>
where T : PartialEq + Clone{
    
    let mut final_array = Vec::new();
    let mut temp_sub_array = Vec::new();
    
    for word in vector.into_iter(){
        if word != delimiter{

            temp_sub_array.push(word);

        } else {

            final_array.push(temp_sub_array.clone());
            temp_sub_array.clear();

        }
    }
    if !temp_sub_array.is_empty() {
        final_array.push(temp_sub_array.clone())
    }
    final_array

}


