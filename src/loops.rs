// Loops are used to iterate until a condition is met

use crate::print;

pub fn run() {
 // Infinite loops

 let mut counter = 0;
 loop {
  println!("{}", counter);

  if counter >= 10 {
   break;
  }

  counter += 1;
 }

 counter = 0;

 // While 
 while counter <= 10 {
  println!("{}", counter);

  if counter >= 10 {
   break;
  }

  counter += 1;
 }

 // For loops
 for x in 0..11 {
  println!("{}",x);
 }
}