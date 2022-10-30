fn main () {
	let p = 520000000.0;
	let r = 0.1;
	let t = 5.0;
let a = p * (1.0 + r) * t;
let i = a - p;
println!("Your Amount is {} and your interst is {}",a, i);
}