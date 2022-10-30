fn main() {
    let fees= 25_000;
    println!("fees is {}", fees );
    //now lets try reassigning the variable without "let"

    fees= 35_000;
    println!("fees changed to {}",fees );
    //this wont run but with a let it works
}
