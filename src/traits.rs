pub fn run() {
 let car: Toyota = Toyota::new();
 car.start_engine();
}

trait Car {
 fn start_engine(&self);
}

struct Toyota {
 name: String,
}

impl Toyota {

 pub fn new() -> Self {
  Self { name: "RAV4".to_string() }
 }

}

impl Car for Toyota {
 fn start_engine(&self) {
     println!("Engine started");
 }

}