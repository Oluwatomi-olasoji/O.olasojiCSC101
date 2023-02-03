fn main() {
    
    let n1= "Electrical ".to_string();
    let n2= "Electronic ".to_string();
    let n3= "Engineering".to_string();
    let n4= n1 + &n2 + &n3; /*to concatinate strings you need the & sign
                              AND the to_string() function*/
    println!("\n The {} is informed by the aspiration to train 
            electical/electronic professionals in the areas of design,
            building and maintenance of electrical control systems", n4);

    let w1= "Computer ".to_string();
    let w2= "Science".to_string();
    let w3= w1 + &w2;
    println!();
    println!("{} is a really cool course", w3);
}
