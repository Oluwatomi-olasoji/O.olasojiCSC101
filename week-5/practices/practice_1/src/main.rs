use std::io;

fn main() {
    println!("\n Student Information Manangement System");

    //input name
    println!("\nPlease Enter your name:" );
    let mut name= String::new();
      io::stdin()
      .read_line(&mut name)
      .expect("Failed to read input");
      println!("Your name is: {}", name);

      //input age
      println!("\nEnter your age: ");
      let mut age= String::new();
         io::stdin().read_line(&mut age).expect("Failed to read input");
        let age:i32= age.trim().parse().expect("Input not an integer"); 
        //in the above line we typed cast age into an integer from a string 
         println!("Your age is: {}", age );

}
