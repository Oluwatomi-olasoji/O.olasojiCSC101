fn main() {
    let a: i32= 2; //Bit representation 0010
    let b: i32= 3; //Bit representation 0011

    let mut result:i32;

    result= a & b;
    println!("(a & b) => {}", result);

    result= a | b;
    println!("(a | b) => {}", result);

    result= a ^ b;
    println!("(a ^ b) => {}", result);

    result = !b;
    println!("(!b) => {}", result);

    result= a << b;
    println!("(a << b) => {}", result);

    result= a >> b;
    println!("(a >> b) => {}", result);
}
