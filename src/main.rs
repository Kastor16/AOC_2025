use std::fs::read_to_string;
fn main() {
    println!("AOC 2025");
    let input = read_lines("input_1.txt");
    
    println!("Contents of Input:");
    
    let mut dial = 50;
    let mut prev_dial:i32;
    let mut zero_count = 0;
    
    println!("Dial: {}",dial);  
    for x in input.iter() {      
        let line=&x[1..];
        let mut amount: i32 = line.parse().unwrap();
        prev_dial = dial;

        if amount > 99 {
           zero_count += amount / 100;
           println!("Zero Count: {}",zero_count);
           let tmp = &x[x.chars().count()-2..];
           amount = tmp.parse().unwrap()
        }
        if x.starts_with("L"){         
            dial -= amount;
            if dial < 0{
                dial += 100;
                if prev_dial != 0 && dial != 0{
                    zero_count += 1;  
                }              
            }  
        }else{
            dial += amount; 
            if dial > 99 {
                dial -= 100;
                if prev_dial != 0 && dial != 0 {
                    zero_count += 1;
                }
            } 
        }
        if dial == 0 {
            zero_count += 1;
        }         
        
        println!("Prev Dial: {} Turn: {} Dial: {}",prev_dial,x,dial);        
        println!("Zero Count: {}",zero_count);

    }
    println!("{}",zero_count);
  
    
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}
