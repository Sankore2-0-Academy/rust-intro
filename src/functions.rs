pub fn run() {
 let str: String = show("Abel");
 println!("{}", str);
}

fn greeting(name: &str) {
 println!("Hello there {}", name);
}

fn show(name: &str) -> String {
 let mut gret = String::from("Hello ");
 gret.push_str(name);
 gret
}