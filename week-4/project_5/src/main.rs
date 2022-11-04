fn main() {

    let uni= " Pan-Atlantic University "; //notice the leading and trailing space
    println!();
    println!("Name: {}", uni);
    println!();
    println!("Before the trim, the length is {}",uni.len() );
    println!();
    println!("After the trim, the uni name is {}, and the lenth is {}",uni.trim(),uni.trim().len() );
}
