#![feature(min_specialization)]
use std::io;
use std::ops::Add;

#[derive(Debug)]
struct Wrapper<T> {
    x: T,
}

impl Add<Wrapper<String>> for Wrapper<String> {
    type Output = Wrapper<String>;
    fn add(mut self, other: Self) -> Self::Output {
        self.x.push_str(&other.x);
        self
    }
}

impl<T> Add for Wrapper<T> 
where T: Add<Output = T>, 
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self{
            x: self.x + other.x
        }
    }
}


// fn do_math<T>(a: u8, b: u8, op: &str) -> u8 {
//     match op {
//         "add" => a + b,
//         "sub" => a - b,
//         "mul" => a * b,
//         "div" => a / b,
//         _ => std::process::exit(1),
//     }
// }

fn main()  {
    println!("Enter first number: ");
    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read line");
    let _a: u8 = input1.trim().parse().expect("Please enter a valid number");
    
    println!("Enter second number: ");
    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line");
    let _b: u8 = input2.trim().parse().expect("Please enter a valid number");
    
    let new_int = Wrapper {x: 1};
    let new_int2 = Wrapper {x: 2};
    println!("Sum {:?} ", new_int + new_int2);

    let new_int: Wrapper<String> = Wrapper {x: String::from("Hello")};
    let new_int2: Wrapper<String> = Wrapper {x: String::from("World")};
    println!("Sum {:?} ", new_int + new_int2);

    // println!("Enter operation (add/sub/mul/div): ");
    // let mut op = String::new();
    // io::stdin()
    //     .read_line(&mut op)
    //     .expect("Failed to read line");
    // let op = op.trim();
    
    // let result = do_math(a, b, op);
    // println!("Result is {}", result);

}