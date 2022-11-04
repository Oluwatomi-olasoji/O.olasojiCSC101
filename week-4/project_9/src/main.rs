fn main() {
    let A: i32= 10;
    let B: i32= 20;

    println!("the value of A is: {}", A);
    println!("the value of B is: {}", B);

    let mut res = A>B;
    println!("is A greater than B: {}", res);

    res= A<B;
    println!("is A less than B: {}", res);

    res= A>=B;
    println!("is A greater than or equal to B: {}", res);

    res= A<=B;
    println!("is A less than or equal to B: {}", res);

    res= A==B;
    println!("is A equal to B: {}", res);

    res= A!=B;
    println!("is A not equal to B: {}", res);

}
