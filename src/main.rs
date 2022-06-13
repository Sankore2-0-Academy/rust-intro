mod print;
mod variables;
mod types;
mod conditional;
mod strings;
mod array;
mod vectors;
mod tuples;
mod loops;
mod functions;
mod structs;
mod traits;
mod enums;
mod ref_pointer;

fn main() {
//  let name = "George";
    // print::show(name);
    // variables::vars();
    // types::run();
    // conditional::run();
    // strings::run();
    // array::run();
    // vectors::run();
    // tuples::run();
    // loops::run();
    // functions::run();
    // structs::run();
    // traits::run();
    // enums::run();
    ref_pointer::run();
}

pub fn add_some_number(num: i32) -> i32 {
    if num > 50 {
        num + num
    } else {
        panic!("Number is less than 50");
        0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected="Number is less than 50")]
    fn add_numbers() {
        // assert!(8 < 3);
        // assert_eq!(5 + 3, 8);
        // println!("Expect to fail");
        // assert_ne!(8, 8);
        let sum = add_some_number(20);
    }
}