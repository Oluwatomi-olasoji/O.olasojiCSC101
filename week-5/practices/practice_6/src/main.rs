use std::io;

fn main() {
    println!("Enter lower bound");
    let mut input= String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let lower_bound:i32 = input.trim().parse().expect("Failed to input");

    println!("Enter upper bound");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let upper_bound:i32 = input2.trim().parse().expect("Failed to input");


    for x in lower_bound..upper_bound { //upperbound not inclusive
        println!("Count level is {}",x); /* you dont even actually need to use x 
        in the code in the for loop, point is the the code will excute as many 
        times as it takes for your lowere bound to read your upper bound,
        including the former and excludidng the latter. */
        


    }
}
