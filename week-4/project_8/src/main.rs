fn main() {
    let num1= 10;
    let num2= 3;
    let mut result:i32;

    println!("mum1 is {}, and num2 is {}",num1, num2 );
    result= num1+ num2;
    println!("Sum: {}",result );

    result= num1- num2;
    println!("Difference:{} ",result);

    result= num1* num2;
    println!("Product: {}",result );

    result= num1/num2;
    println!("Quotient: {}",result );

    result= num1%num2;
    println!("the remainder after dividing num1 by num 2 is: {}",result );
}
