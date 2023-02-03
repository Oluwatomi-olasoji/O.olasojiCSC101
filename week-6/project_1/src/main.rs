use std::io;

fn main() {
    println!("\n What calculation would you like to perform?\n 
    press: 1 for Area of a trapezium\n
           2 for Area of a rhombus\n 
           3 for Area of a parallologram\n
           4 for Area of a cube\n
           5 for Volume of cylinder\n ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let calcno:i32 = input.trim().parse().expect("Invalid input, please enter a number");

    if calcno==1 {
    println!("Input the height of the trapezium: ");
    let mut i1= String::new(); 
    io::stdin().read_line(&mut i1).expect("Failed to read input");
    let traph:f32 = i1.trim().parse().expect("Not a valid number");

    println!("Input the 1st base of the trapezium: ");
    let mut i2= String::new(); 
    io::stdin().read_line(&mut i2).expect("Failed to read input");
    let trapb1:f32 = i2.trim().parse().expect("Not a valid number");
    
    println!("Input the 2nd base of the trapezium: ");
    let mut i3= String::new(); 
    io::stdin().read_line(&mut i3).expect("Failed to read input");
    let trapb2:f32 = i3.trim().parse().expect("Not a valid number");
    
    let aot:f32 = traph * (trapb1 + trapb2)/2.0;
    println!("The area of the trapezium is {}",aot );

    }
    
    else if calcno==2 {
    println!("Input 1st diagonal of the rhombus: ");
    let mut i1= String::new(); 
    io::stdin().read_line(&mut i1).expect("Failed to read input");
    let rhomd1:f32 = i1.trim().parse().expect("Not a valid number");

    println!("Input the 2nd diagonal of the rhombus: ");
    let mut i2= String::new(); 
    io::stdin().read_line(&mut i2).expect("Failed to read input");
    let rhomd2:f32 = i2.trim().parse().expect("Not a valid number");
    
    let aor:f32 = (rhomd1 * rhomd2) /2.0;
    println!("The area of the rhombus is {}",aor );

    }

    else if calcno==3 {
    println!("Input base of the parallologram:" );
    let mut i1= String::new(); 
    io::stdin().read_line(&mut i1).expect("Failed to read input");
    let basep:f32 = i1.trim().parse().expect("Not a valid number");

    println!("Input height of the parallologram:");
    let mut i2= String::new(); 
    io::stdin().read_line(&mut i2).expect("Failed to read input");
    let heightp:f32 = i2.trim().parse().expect("Not a valid number");
    
    let aop:f32 = basep * heightp;
    println!("The area of the rhombus is {}",aop );
    }

    else if calcno==4 {
    println!("Input length of the cube: ");
    let mut i1= String::new(); 
    io::stdin().read_line(&mut i1).expect("Failed to read input");
    let lenc:f32 = i1.trim().parse().expect("Not a valid number");
    
    let aoc:f32 = 6.0 * (f32::powf(lenc,2.0));
    println!("The area of the cube {}",aoc );
    }

    else if calcno==5 {
    let pi= 3.142; 
    println!("Input radius of the cylinder: ");
    let mut i1= String::new(); 
    io::stdin().read_line(&mut i1).expect("Failed to read input");
    let radc:f32 = i1.trim().parse().expect("Not a valid number");

    println!("Input height of the cylinder: ");
    let mut i2= String::new(); 
    io::stdin().read_line(&mut i2).expect("Failed to read input");
    let heightc:f32 = i2.trim().parse().expect("Not a valid number");
    
    let voc:f32 = (pi * f32::powf(radc,2.0)) * heightc ;
    println!("The volume of the cylinder is {}",voc ); 
    }

    else {
        println!("Sorry, invalid input");
    }
}
