use std::io;

fn main() {

    println!("Enter a number");
    let mut input= String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut num:i32 = input.trim().parse().expect("Failed to input");

    while num < 10 {
        println!("inside loop number value is {}",num );
        let mut newnum= String::new();
        io::stdin().read_line(&mut newnum).expect("Failed to read input");
        let num:i32 = newnum.trim().parse().expect("Failed to input");

    }
    println!("outside loop number value is {}",num );

}





















/*use std::io;

fn main() {

    println!("Enter a numeber");
    let mut input= String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut num:i32 = input.trim().parse().expect("Failed to input");

    while num < 10 {
        println!("inside loop number value is {}",num );
        num +=1;
    }
    println!("outside loop number value is {}",num );

}*/
