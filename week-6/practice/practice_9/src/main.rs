//the inter() function fetches values of all ements in an array.

fn main() {
    let arr:[i32;4]= [10,20,30,40];
    println!("array is {:?}",arr);
    println!("array size is {}",arr.len());

    for val in arr.iter(){
            println!("array value is: {}",val ); 
        }
       
    }

