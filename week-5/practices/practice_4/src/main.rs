use std::io;

fn main() {
let mut input = String::new();
let mut input2 = String::new();

println!("Please Enter your name:");
io::stdin().read_line(&mut input).expect("Not a valid string");

println!("Please enter your age:");
io::stdin().read_line(&mut input2).expect("Not a valid string");
let age:i32= input2.trim().parse().expect("Not a valid number :/");

if age>= 18 {
    println!("Welcome to the party {}",input );
}
else {
    println!("Opps, you're not old enough to enter the party {}",input);
}
}
