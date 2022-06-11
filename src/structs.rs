pub fn run() {
 let person: Person = Person::new("Mburu", 45);

 person.print_info();
 
}

struct Person {
 name: String,
 age: u32
}

impl Person {
 fn new(name: &str, age: u32) -> Self {
  Self { name: name.to_string(), age }
 }

 fn print_info(&self) {
  println!("{} is {} years old", self.name, self.age);
 }
}

