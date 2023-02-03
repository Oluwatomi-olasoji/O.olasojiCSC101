//recall that fn main always runs first beacuse it is the main funtion 
//for rust
fn mutate_num_to_zero(mut param_num:i32 ){
    param_num = param_num*0;
    println!("param_num value is {}",param_num );
}

fn main() {
    let num:i32= 5;
    mutate_num_to_zero(num);
    println!("The value of num is: {}",num);
}
/* when num is in the function it does what its meant to do but as control 
returns back to the main function, num remains the same. this happens 
because when passing num by value a new storage location is made named
paranum and the value of num is copied to it. then what ever is done to
taht clone returns as if it was num, but num in its OG location remains 
the same*/