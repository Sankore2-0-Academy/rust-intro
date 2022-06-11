pub fn run() {
 let num = 45;
 let age = num;

 let nums: Vec<i32> = vec![34,56,7,89];
 let ages = &nums;

 println!("{:?}", ages[0]);
}