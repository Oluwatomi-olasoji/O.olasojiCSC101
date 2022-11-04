fn main() {
    let name= "Oluwatomi Olasoji";
    let uni:&str= "Pan-Atlantic University";
    let add: &str= "Km 52 Lekki-Epe Expressway, Ibeju-Lekki, Lagos";

    println!("Name: {}",name );
    println!("University: {}, \nAddress: {}", uni,add );

    let depart: &'static str= "Computer Science";
    let school: &'static str= "Scholl of Science and Technology";
    println!("Department: {}, \nSchool: {}",depart, school );
}
