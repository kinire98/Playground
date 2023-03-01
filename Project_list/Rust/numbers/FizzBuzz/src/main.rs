use std::io;
pub mod fizzbuzz;
fn main() {
    let mut _limit = String::new();
    println!("Input the number up to FizzBuzz will be played (max. 255)");
    io::stdin()
        .read_line(&mut _limit)
        .expect("Failed to read line");
    let _limit:u8 = match _limit.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Input a number between 0 and 255");
            0
        }
    };
    fizzbuzz::fizzbuzz(_limit);
}
