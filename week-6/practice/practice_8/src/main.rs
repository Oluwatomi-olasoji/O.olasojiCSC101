fn main() {
    //using a for loop in an array
    let city_arr:[&str;5] = ["Maiduguri","Lagos","Osun","Abuja","Zaria"];
    println!("array is {:?}", city_arr);
    println!("array size is {}",city_arr.len());

    for index in 0..5 {
        println!("City index {} is located in {}",index,city_arr[index] );
    }
}
