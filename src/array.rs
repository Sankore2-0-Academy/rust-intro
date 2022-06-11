// Array - Fixed list where elements are the same data types

pub fn run() {
 // primitive arrays
 let mut num: [i32; 4] = [1,2,3,4];
 num[3] = 45;

 println!("{:?}", num.len());

 println!("{:?}", num);

 println!("{}", num[2]);

println!("============= Array Elements =================");

 for item in num.iter() {
  println!("{}", item);
 }

}