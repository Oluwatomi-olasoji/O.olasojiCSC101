use std::in;
fn main(){
	println!("Please enter your name: ");
	let mut name= String::new();
		io::stdin().read_line(&mut name).expect("Not a valid input");
	let mut papernum= String::new();
		println!("Enter the number of papers you have published : ");
		io::stdin().read_line(&mut papernum).expect("Not a valid input");
		let papernum:u32 = papernum.trim().parse().expect("Not a valid number") 

	Facpub(name,papernum);
}

fn Facpub(name:&str,papernum:u32){
   if 3<= papernum <=5 {
   println!("congratulations {},Your incentive is N500,000",name );
}

   else if 5< papernum < 10 {
   	println!("congratulations {},Your incentive is N800,000",name );
   }
   
   else if papernum>= 10 {
    println!("congratulations {},Your incentive is N1,000,000",name );
   }
 
else if papernum< 3 {
    println!("congratulations {},Your incentive is N100,000",name );

   }
else {
    println!("Invlaid input" );
}

} 