use crate::{utils::{self, read_input_of_the_day_lines}};
use std::iter::Peekable;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;


//Struct representing a directory, it has a name, a size, a Vec of reference to subdirectory, a vec of file
//And a reference to the directory in which this directory is, this can be None if this is the main directory

#[derive(Debug)]
struct Directory{
    name: String,
    size: u32,
    subdirectory: Vec<Rc<RefCell<Directory>>>,
    files: Vec<(String, u32)>,
    parent_directory: Option<Rc<RefCell<Directory>>>,
} 

//Methods for the Directory struct

impl Directory{
    //We can add a subdirectory to the directory
    fn add_directory(&mut self, directory_to_add: Rc<RefCell<Directory>>){

        self.subdirectory.push(directory_to_add);
        
    }
    //We can add a file
    fn add_file(&mut self, file_name: String, file_size: u32){

        self.files.push((file_name, file_size));
        
    }
    //Compute the size of the current directory, this will recursively update the sizes of the subdir
    //need to pass the directory architecture in param. This is meant to be called elsewhere only on the main directory
    fn recompute_and_update_size(&mut self){
        self.size = self.files.iter().map(|(name, size)| size).sum();

        for subdirectory in self.subdirectory.iter_mut() {
            subdirectory.borrow_mut().recompute_and_update_size();
            self.size += subdirectory.borrow().size;
        }
    }

    //return the sum of the sizes of all subdirectory that have size below 10000
    fn sum_subdir_below(&self) -> u32{
        let mut size_total = 0;
        if self.size <= 100000 {
            size_total += self.size
        }
        for subdirectory in self.subdirectory.iter() {
            size_total += subdirectory.borrow_mut().sum_subdir_below();
            
        }
        size_total
    }

    //Return the size of the subdirectory that is minimal among those that have size bigger than above_n, 
    //need a current minimum to recursively call the function
    fn min_directory_size(&self, minimum_yet: f64, above_n : f64) -> f64{
        let mut size = minimum_yet;
        if self.size as f64 >= above_n && self.size as f64 <= minimum_yet {size = self.size as f64}

        for subdirectory in self.subdirectory.iter() {

            size = subdirectory.borrow().min_directory_size(size, above_n);

        }
        return size
    } 
    
}


//Take a reference to an iterator over input lines, assuming the next lines are the result
//of a ls command (otherwise return None)
//Return a Vec containing the list of files
fn extract_list_file<T>(iterator: &mut Peekable<T>) -> Vec<String>
where T: Iterator<Item = String>{
    let mut vec_files = Vec::new();
    //We loop over the iterator, taking a peek at the next value to see if we are at the end of the list of files:
    loop {
        let next_value = match iterator.peek(){
            Some(line) => line,
            None => {break;}
        };
        //If the next line is a command we stop
        if next_value.starts_with("$") {
            break;
        } else {
            //We add the next line into the vector
            vec_files.push(iterator.next().unwrap());
        }
    }
    vec_files
}


//From the list of input create the main directory and its nested directory
fn create_directory_struct(input: Vec<String>) -> Rc<RefCell<Directory>>{

    //Initialize the main directory "/"
    let main_directory = Rc::new(RefCell::new(Directory {
        name: String::from("/"),
        size: 0,
        subdirectory: Vec::new(),
        files: Vec::new(),
        parent_directory: None
    }));

    //create an iterator from the input lines
    let mut input_iterator = input.into_iter().peekable();

    //As the first line is always "$ cd /", we skip it
    input_iterator.next();

    //We keep track of which directory we are with this clone of reference on a directory:
    let mut current_dir = Rc::clone(&main_directory);

    //Main loop where we will iterate over all the input lines:
    loop {
        //First we read the input, if we are done with the input we break the loop
        let line = match input_iterator.next(){
            Some(line) => line,
            None => {break;}, 
        };
        //println!("computing line {}", line);
        //We must have a command line, since non command lines come after a "ls" and should be handled at that time 
        if line.starts_with("$ cd") {
            
            //extract the name of the directory we are moving in
            let name = String::from(line.strip_prefix("$ cd ").unwrap());
            //println!("moving to {}", name);

            //If name = .. we move out of the current directory into the directory_parent of the current_dir_name and restart the loop
            //otherwise we move into the named dir
            if name == ".."{
                let current_clone = Rc::clone(&current_dir);
                current_dir = Rc::clone(&&current_clone.borrow().parent_directory.as_ref().unwrap());
                continue;
            } else {
                //println!("Listing all the files");
                //Create dir with the name and add it to the current dir hashmap of subdirectory, as well to the list of subdir of the current directory
                let dir_to_be_current = Rc::new(RefCell::new(Directory {
                    name: name.clone(),
                    size: 0,
                    subdirectory: Vec::new(),
                    files: Vec::new(),
                    parent_directory: Some(Rc::clone(&current_dir)),
                }));

                let dir_clone = Rc::clone(&dir_to_be_current);
                current_dir.borrow_mut().add_directory(dir_clone);
                current_dir = Rc::clone(&dir_to_be_current);
            }
            
        } else {
            //We are in the case where the command line is "$ ls"
            //We need to extract all the lines after the command that correspond to files/directory until the next command
            let list_of_files = extract_list_file(&mut input_iterator);
            //For all files, we add the file into our current dir
            for file_or_dir in list_of_files {
                if !file_or_dir.starts_with("dir"){
                    //add the file to the files of the current directory
                    let file: Vec<String> = file_or_dir.split_whitespace().map(String::from).collect();
                    let (file_name, file_size) = (file[1].clone(), file[0].parse::<u32>().unwrap());
                    current_dir.borrow_mut().add_file(file_name, file_size);
                    current_dir.borrow_mut().size += file_size;
                }
            }

        }
        //We update all the sizes
        main_directory.borrow_mut().recompute_and_update_size();
    };
        return main_directory  
}

//Return the sum of the sizes of all directory below 100000
fn directory_sizes(input: Vec<String>) -> u32{
    //First we need to create all directories as nested Directory structs
    //Initialize the main directory
    let main_directory = create_directory_struct(input);
    
    //We now extract all Directory that have a size less than 100000:
    //To do that we use the recursive method n_subdir_above() at the root of our directory
    let size: u32 = main_directory.borrow().sum_subdir_below();

    size
}

//Return the size of the min directory that have a big enough size 
fn min_free_space(input: Vec<String>) -> u32{
    //First we need to create all directories as nested Directory structs
    //Initialize the main directory
    let main_directory = create_directory_struct(input);

    //Find the size needed to be freed
    let size_free = 70000000 - main_directory.borrow().size;
    let size_needed = (30000000 - size_free) as f64;

    //We then use the _min_directory_size recursive method of 
    let size = main_directory.borrow().min_directory_size(f64::INFINITY, size_needed) as u32;

    size
}


pub fn solve_1()-> u32{
    //Read the input
    let input = read_input_of_the_day_lines(7);

    //Find all the directory that have size less than 100000, sum the size
    let sizes = directory_sizes(input);

    sizes
} 

pub fn solve_2()-> u32{
    //Read the input
    let input = read_input_of_the_day_lines(7);

    //Find all the directory that have size more than 3000000, find the minimal among them
    let size = min_free_space(input);

    size
} 


//TESTS
mod tests {
    
    use super::*;
    //Test first part of the puzzle
    #[test]
    fn test_solve_1(){
        //test input for first part
        let test_input = String::from("$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k").lines().map(String::from).collect::<Vec<String>>();
        println!("{:?}", test_input);
        let size = directory_sizes(test_input);

        assert_eq!(size, 95437)


    }

    fn test_solve_2(){
        //test input for second part
        let test_input = String::from("$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k").lines().map(String::from).collect::<Vec<String>>();
        println!("{:?}", test_input);
        let size = min_free_space(test_input);

        assert_eq!(size, 24933642)


    }


}
