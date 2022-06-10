// Variables hold primitive data or reference to data
// Variables are immutable by default
// Rust is ablock-scoped language

pub fn vars() {
 // let name: &str  = "George";
 let mut age = 87;
 age = 67;

 const ID: &str = "1233";

 println!("{}", ID);
}