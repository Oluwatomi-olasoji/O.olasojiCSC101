use std::io; 

fn mutate_num_to_zero(paranum:&mut i32) {
    *paranum = *paranum * 0; //the thing in the location of what is passed so it  
    println!("paranum value is {}",paranum );//acctually changes the value of num. 
}
fn main() {
    let mut input= String::new();

    println!("Please enter the value of num to mutate");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut num:i32 = input.trim().parse().expect("Not a valid input");
    println!("The previous value of num is {}, after this line the mutate function is called",num );
    mutate_num_to_zero(&mut num);
    println!("The value of num is {}",num );
    
}
