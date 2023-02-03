use std::io;

fn main(){

let mut name= String::new();
io::stdin().read_line(&mut name).expect("Cannot read input");
let name = name.trim();

let mut age= 19;

println!("hello, my name is {}. I am {} years old",name,age );
}