use crate::utils;

#[derive(Debug, Clone)]
enum Packet {
    Number(usize),
    List(Vec<Packet>),
}

//recursively compare two packet:
fn less(packet_1: &Packet, packet_2: &Packet) -> bool {

    //packet 1 is a number
    if let Packet::Number(number_1) = &packet_1 {
        //packet 1 and 2 are numbers
        if let Packet::Number(number_2) = &packet_2{
            return number_1 <= number_2
        } else if let Packet::List(list_2) = &packet_2{
            //packet 1 is a number but packet 2 is a list
            let new_packet_1 = Packet::List(Vec::from([packet_1.clone()]));
            return less(&new_packet_1, packet_2)
        }
    }
    //packet 1 is not a number
    if let Packet::List(list_1) = &packet_1{
        //packet 1 and 2 are lists
        if let Packet::List(list_2) = &packet_2{
            for (sub_1, sub_2) in list_1.iter().zip(list_2.iter()){
                if !less(sub_1, sub_2){
                    return false
                }
            } 
            return true
        } else if let Packet::Number(number_2) = &packet_2{
            //packet 1 is a list but packet 2 is a number
            let new_packet_2 = Packet::List(Vec::from([packet_2.clone()]));
            return less(&new_packet_2, packet_2)
        }
    }
    return false
}

//parse a packet recursively
fn parse_packet(string_to_parse: &Vec<char>) -> Packet{
    //packet is a line of type "[1,[2,[3,[4,[5,6,7]]]],8,9]"

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
                    let num_packet = Packet::Number(string_to_parse[i].to_digit(10).unwrap() as usize);
                    list.push(num_packet);
                }
            }
        }
        i += 1  
    }
    return packet
}


//First part of the day
pub fn solve_1() -> u32{

    0
}

//First part of the day
pub fn solve_2() -> u32{

    0
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
            //if less(&packet_1, &packet_2){
            //    number_ordered += (index + 1);
            //}
        }
        
        assert_eq!(number_ordered, 13)

    }


}