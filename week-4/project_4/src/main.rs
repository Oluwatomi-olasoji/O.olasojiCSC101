fn main() {
    let fullname= "Oluwatomi Funmilayo Ayomide Olasoji";
    let depart= "Computer Science";
    let uni= "Pan-Atlantic University";

    let mut school= "School of Science".to_string();
    school.push_str(" and Technology"); 

    println!("My fullname is: {}", fullname);
    println!("My name has {} characters, including space",fullname.len());
    println!("I am a student of the {} department",depart );
    println!("{}", school);
    println!("{}",uni );

    
}
