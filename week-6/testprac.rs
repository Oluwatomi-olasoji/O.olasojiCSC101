use std::io; 
fn StudentCouncil_VoteX(){
	let arr1:[&str;2] = ["Name,age,class", "Name,class,age"];
	for i in 0..arr1.len(){
		let mut i1= String::new();
		println!("Are you a class rep (Enter 1 for yes, and 0 for no)");
		io::stdin().read_line(&mut i1).expect("Not a valid input");
		let i2:i32 = i1.trim().parse().expect("Not a valid iunput");
         
    if i2== 1{
        let mut level= String::new();
		println!("Are you not in 100 level (Enter 1 for yes, and 0 for no)");
		io::stdin().read_line(&mut level).expect("Not a valid input");
		let level2:i32 = level.trim().parse().expect("Not a valid input");

       if level2== 1 {
       	let mut gpa= String::new();
		println!("Is your GPA over 4.0 (Enter 1 for yes, and 0 for no)");
		io::stdin().read_line(&mut gpa).expect("Not a valid input");
		let gpa:i32 = gpa.trim().parse().expect("Not a valid input");
       if gpa == 1{
       	   println!("Your info is as follows{}", arr1[i] );
        }
        else{
    	println!("You cannot vote");
    }
        
       }
       else{
    	println!("You cannot vote");
    }

    
    }
    else{
    	println!("You cannot vote");
    }
	
}
}
fn main(){
     StudentCouncil_VoteX();
}