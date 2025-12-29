use std::fs::read_to_string;
fn main() {
    println!("AOC 2025");
    let input = read_lines("input_1.txt");
    
    println!("Contents of Input:");
    
    let mut dial = 50;
    let mut zero_count = 0;
    for x in input.iter() {      
        let line=&x[1..];
        let mut amount: i32 = line.parse().unwrap();
        
       

        if amount > 99 {
           let tmp = &x[x.chars().count()-2..];
           amount = tmp.parse().unwrap()
        }


        if x.starts_with("L"){  
            
            dial -= amount;
            if dial < 0{
                dial += 100
            }
    
        }else{
            dial += amount; 
            if dial > 99 {
                dial -= 100
            }         

        }        
        if dial == 0{
            zero_count +=1;
        }
        println!("{}",x);
        println!("{}",dial);  

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
