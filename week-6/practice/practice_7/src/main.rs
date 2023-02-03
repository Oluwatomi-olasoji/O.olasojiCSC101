fn main() {
    //Different ways to make an array

    //Array with data type and size (explicit integer datatype)
    let arr1:[i32;4] = [10,20,30,40];
    println!("\n Array with explicit data type decleration");
    println!("Array is {:?}", arr1);
    println!("Array size is {}",arr1.len());

    //Array without data (implicit float datatype decleration)
    let arr2 = [10.4,20.7,30.4,40.9,32.4,65.7];
    println!("\n Array with out data type");
    println!("array is {:?}", arr2);
    println!("array size is {}",arr2.len());

    //Array with defaut values taht creates and initializes
    //all its elements with default value of -1
    let arr3: [i32;8]= [-1;8];
    println!("\nArray with default values ");
    println!("array is {:?}",arr3 );
    println!("array size is: {}",arr3.len() );
}
