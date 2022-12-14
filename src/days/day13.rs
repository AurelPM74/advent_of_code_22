use itertools::Itertools;
use std::cmp::Ordering;
use crate::utils;


#[derive(Debug, Clone)]
enum Packet {
    Number(usize),
    List(Vec<Packet>),
}

//Ord trait implementation for packet
impl Ord for Packet{
    fn cmp(&self, other: &Self) -> Ordering {
       let comparaison = stricly_less(&self, &other);
       match comparaison {
        -1 => {return Ordering::Greater},
        0 => {return Ordering::Equal},
        1 => {return Ordering::Less},
        _ => {panic!("impossible")}
       }
    }
}
//Partial Ord trait implementation for packet
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Packet) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
//PArtial Eq trait implementation for packet
impl PartialEq for Packet {
    fn eq(&self, other: &Packet) -> bool {
        let comparaison = stricly_less(&self, &other);
        return comparaison == 0
    }
}
impl Eq for Packet {
    
}

//recursively compare two packet, return 0: equal, -1 greater, 1, smaller
fn stricly_less(packet_1: &Packet, packet_2: &Packet) -> i32 {
    //println!("compare {:?} with {:?}", packet_1, packet_2);
    //packet 1 is a number
    if let Packet::Number(number_1) = &packet_1 {
        //packet 1 and 2 are numbers
        if let Packet::Number(number_2) = &packet_2{
            if number_1 < number_2 {
                return 1
            } else if number_1 == number_2{
                return 0
            } else {
                return -1
            }
        } else if let Packet::List(list_2) = &packet_2{
            //packet 1 is a number but packet 2 is a list
            let new_packet_1 = Packet::List(Vec::from([packet_1.clone()]));
            return stricly_less(&new_packet_1, packet_2)
        }
    }
    //packet 1 is not a number
    if let Packet::List(list_1) = &packet_1{
        //packet 1 and 2 are lists
        if let Packet::List(list_2) = &packet_2{
            for (sub_1, sub_2) in list_1.iter().zip(list_2.iter()){
                match stricly_less(&sub_1, &sub_2){
                    -1 => return -1,
                    0 => (),
                    1 => return 1,
                    _ => ()
                }
            }
            if list_2.len() == list_1.len(){
                return 0
            } else if list_1.len() < list_2.len(){
                return 1
            } else {
                return -1
            }
        } else if let Packet::Number(number_2) = &packet_2{
            //println!("packet 1 is a list but packet 2 is a number");
            let new_packet_2 = Packet::List(Vec::from([packet_2.clone()]));
            return stricly_less(packet_1, &new_packet_2)
            
            
        }
    }
    0
}

//parse a packet recursively
fn parse_packet(string_to_parse: &Vec<char>) -> Packet{
    //packet is a line of type "[1,[2,[3,[4,[5,6,7]]]],8,9]"

    //string = string.replace("10", "&");
    //let string_to_parse:Vec<char> = string.chars().collect();

    //if the string is one number:
    if string_to_parse[0] != '[' {

        let mut string_pop = string_to_parse.clone();
        string_pop.pop();
        let packet = Packet::Number(string_pop
                                                           .iter()
                                                           .collect::<String>()
                                                           .parse::<usize>()
                                                           .unwrap());
    }
    //otherwise we have a list
    let mut packet = Packet::List(Vec::new());

    //current position in the list, don't consider the first [ 
    let mut i = 1;

    while i <= string_to_parse.len()-1{

        match string_to_parse[i] {
            //if comma we do nothing
            ',' => {},

            //If we have a [, we search the index of the closing ] corresponding
            '[' => {
                let mut into_substring = 1;
                let mut substring = String::new();
                substring.push('[');
                //until we see the right closing ] 
                while into_substring != 0{
                    //next char
                    i += 1;

                    match string_to_parse[i] {
                        '[' => into_substring += 1,
                        ']' => into_substring -= 1,
                         _  => ()
                    }
                    substring.push(string_to_parse[i]);
                }
                let subpacket = parse_packet(&substring.chars().collect());
                //Add the subpacket to the packet
                if let Packet::List(list) = &mut packet {
                    list.push(subpacket)
                }


            }
            //We shouldn't have a closing bracket except if its the last one
            ']' => {return packet},
            
            //Last case, we have a number, then we add a number packet to the packet
            _ => {
                if let Packet::List(list) = &mut packet{
                    if string_to_parse[i] == '1' && string_to_parse[i+1] == '0'{
                        let num_packet = Packet::Number(10 as usize);
                        list.push(num_packet);
                    } else {
                        let num_packet = Packet::Number(string_to_parse[i].to_digit(10).unwrap() as usize);
                        list.push(num_packet);
                    }
                    
                }
            }
        }
        i += 1  
    }
    return packet
}


//First part of the day
pub fn solve_1() -> u32{
    //input
    let lists = utils::read_input_of_the_day_lines(13);
        
    //We separate the pairs:
    let pairs = utils::sub_array_block_from_delim(&lists, String::from(""));
    let mut number_ordered= 0;
    

    for (index, pair) in pairs.iter().enumerate(){
        //println!("trying pairs n {}", index); 
        let packet_1 = parse_packet(&pair[0].chars().collect::<Vec<char>>());
        let packet_2 = parse_packet(&pair[1].chars().collect::<Vec<char>>());
        match stricly_less(&packet_1, &packet_2) {
            1 => {number_ordered += index + 1},
            _ => ()
        } 
    }
    number_ordered as u32   
}

//First part of the day
pub fn solve_2() -> u32{
    //input

    let mut lists = utils::read_input_of_the_day_lines(13);
    
    //We remove the lines
    lists.retain(|line| !line.is_empty());

    //We transform the string into array of char
    let chars = lists.iter()
                                     .map(|s| s.chars().collect())
                                     .collect::<Vec<Vec<char>>>();

    //We parse the packet 
    let mut packets: Vec<Packet> = chars.iter().map(|s| parse_packet(s)).collect();
    
    //We add the two delimiters:
    let deli_1 = Packet::List(Vec::from([Packet::Number(2)]));
    let deli_2 = Packet::List(Vec::from([Packet::Number(6)]));
    packets.push(deli_1.clone());
    packets.push(deli_2.clone());
    println!("&");
    //We sort
    packets.sort();

    println!("&");
    //Search the index 
    let indices =  packets.into_iter()
                                        .enumerate()
                                        .filter(|(_, p)| *p == deli_1 ||*p == deli_2)
                                        .map(|(index, _)| index)
                                        .collect::<Vec<_>>();
    return ((indices[0]+1) * (indices[1]+1)) as u32;

}

mod tests {
    
    use super::*;
    //Test first part of the puzzle
    #[test]
    fn test_solve_1(){
        //input
        let lists = utils::read_input_path("input/day_13_input_test.txt");
        
        //We separate the pairs:
        let pairs = utils::sub_array_block_from_delim(&lists, String::from(""));
        let mut number_ordered= 0;
        
    
        for (index, pair) in pairs.iter().enumerate(){
            println!("trying pairs n {}", index); 
            let packet_1 = parse_packet(&pair[0].chars().collect::<Vec<char>>());
            let packet_2 = parse_packet(&pair[1].chars().collect::<Vec<char>>());
            //println!("{:?}", packet_2);
            match stricly_less(&packet_1, &packet_2) {
                1 => {number_ordered += index + 1},
                _ => ()
            }
            
        }
        println!("{}", number_ordered);
        
        assert_eq!(number_ordered, 13)

    }
    #[test]
    fn test_solve_2(){
        //input
        println!("&");
        let mut lists = utils::read_input_path("input/day_13_input_test.txt");
            
        //We remove the lines
        lists.retain(|line| !line.is_empty());

        //We transform the string into array of char
        let chars = lists.iter()
                                        .map(|s| s.chars().collect())
                                        .collect::<Vec<Vec<char>>>();

        //We parse the packet 
        let mut packets: Vec<Packet> = chars.iter().map(|s| parse_packet(s)).collect();
        
        //We add the two delimiters:
        let deli_1 = Packet::List(Vec::from([Packet::Number(2)]));
        let deli_2 = Packet::List(Vec::from([Packet::Number(6)]));
        packets.push(deli_1.clone());
        packets.push(deli_2.clone());
        
        //We sort
        packets.sort();


        //Search the index 
        let indices =  packets.into_iter()
                                            .enumerate()
                                            .filter(|(_, p)| *p == deli_1 ||*p == deli_2)
                                            .map(|(index, _)| index)
                                            .collect::<Vec<_>>();
        assert_eq!((indices[0]+1) * (indices[1]+1), 140)
        
    }


}