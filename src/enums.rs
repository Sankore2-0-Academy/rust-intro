pub fn run() {
 let state = Status::None;
 
 match state {
  Status::COMPLETED => println!("Task complete"),
  Status::ACTIVE => println!("Task in progress"),
  Status::PENDING => println!("Task pending"),
  _ => println!("Something went wrong")
 }
}

enum Status {
 PENDING,
 ACTIVE,
 COMPLETED,
 None
}
