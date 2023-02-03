use std::io; 

fn main() {

    let mut input_a = String::new();
    let mut input_b = String::new();
    let mut input_c = String::new();

    println!("The quadratic expression is Ax^2 + Bx + C\n");
    println!("Enter value for A:" );
    io::stdin().read_line( &mut input_a ).expect("Not a valid input");
    let a:f32 = input_a.trim().parse().expect("Not a valid input");

    println!("Enter value for B:" );
    io::stdin().read_line( &mut input_b ).expect("Not a valid input");
    let b:f32 = input_b.trim().parse().expect("Not a valid input");

    println!("Enter value for C:" );
    io::stdin().read_line( &mut input_c ).expect("Not a valid input");
    let c:f32 = input_c.trim().parse().expect("Not a valid input");

  let d:f32= f32::powf(2.0, b ) - 4.0 * a * c;
  if d > 0.0 {
    let q1:f32= (- b + f32::powf(0.5, d))/ (2.0 * a);
    let q2:f32= (- b - f32::powf(0.5, d))/ (2.0 * a);
    println!("The distinct roots of the quadratic equation are {} and {}",q1,q2 );
  }
  else if d == 0.0 {
    let q3= (- b + 0.0 )/ (2.0 * a );
    println!("The only root of the quadratic equation is {}",q3 );
  }
  else {
    println!("Sorry, there are no real roots for this equation");
      }
}
