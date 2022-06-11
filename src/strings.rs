pub fn run() {
 // Fixed-length string
 let name = "He is going";

 // Growing string
 let mut greeting = String::from("Hello");
 greeting.push_str(" World");

 println!("{}", greeting.contains("Hello"));
}