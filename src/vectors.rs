// Vectors - Resizable arrays

pub fn run() {
 // Growing arrays
 let mut num: Vec<i32> = vec![1,2,3,4];
 num.push(45);
 num.push(59);

 println!("{:?}", num.len());

 println!("{:?}", num);
 num.pop();
 println!("{:?}", num);

 println!("{}", num[2]);
}