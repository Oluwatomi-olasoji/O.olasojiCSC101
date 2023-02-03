use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("This is a program to calculate your incentive");
    println!("Please enter your name: ");
    io::stdin().read_line(&mut input3).expect("Not a valid input");


    println!("Enter your age: ");
    io::stdin().read_line(&mut input1).expect("Not a valid input");
    let age:u32 = input1.trim().parse().expect("Not a valid age");

    println!("Please enter your number of years of experience: ");
    io::stdin().read_line(&mut input2).expect("Not a valid input");
    let exp:f32 = input2.trim().parse().expect("Not a valid number");

    if exp >= 10.0 && age >= 40 {
    }
    else if exp >= 10.0 && age < 40 && age >= 30 {
        println!("Congratulations {}your incentive is N1,480,000", input3);
    }
    else if exp >= 10.0 && age < 28 {
        println!("Congratulations {}your incentive is N1,300,000", input3); 
    }
    //exp 10 age 27
    else if exp < 10.0 {

        println!("Congratulations {}your incentive is N100,000", input3);
    }
    else {
        println!("Sorry {}I am unable to calculate your incentive",input3 );
    }

    }

