
pub fn run() {
 let age: u32 = 18;
 let course = "CSA";

 // if age > 18 && course == "CS" {
 //  println!("You are of age to take up the course");
 // } else if age > 18 || course == "CSA" {
 //  println!("You are under-age blahh");
 // } else {
 // println!("You are under-age");
 // }

 // Match Statement
 match age {
  18 => {
   println!("You are of age to take up the course");
  },
  3 => {
   println!("You are under-age");
  }
  _ => {
   println!("None");
  }
 }

}