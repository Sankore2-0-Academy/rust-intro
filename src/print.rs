pub fn show(name: &str) {
 println!("Hello");

 println!("Hello {}", name);

 // Positional Formatting
 println!("{1} loves color {0}", name, "blue");

 // Named Argument
 println!("{name} prefers {car} over {house}", name = name, house = "Bungalow", car = "Toyota");

 println!("{:?}", ("34", "98"));
}