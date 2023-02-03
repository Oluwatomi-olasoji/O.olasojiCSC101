fn main() {
    let vector: Vec<i64> = Vec::new();

    println!("the length of Vec::new is:{}",vector.len());

    let vector = vec!["tomi","Olasoji"];
    let vector2 = vec!["tola","Olasoji"];

    println!("\n the length of the vector macro is: {}",vector.len() );

    for num in 0..2{
        println!("{:?} {:?}" ,vector[num],vector2[num] );

    }
}
