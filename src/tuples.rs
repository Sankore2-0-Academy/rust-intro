// Tuples group together values of different types

pub fn run() {
 let person: (&str, u32) = ("Abel", 59);
 println!("{} {}", person.0, person.1);

 println!("{:?}", &person);
}