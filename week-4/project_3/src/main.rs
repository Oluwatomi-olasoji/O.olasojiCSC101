fn main() {
    let name1= "Oluwatomi Olasoji";
    println!("My name is {}",name1 );

    //find and replace
    let name2= name1.replace("Oluwatomi", "Ayomide");
    println!("You can also call me {}", name2 );

    let faculty= "Faculty of Science and Technology";
    let school= faculty.replace("Faculty","Science");
    println!("I am a student of the {}",school );
}
