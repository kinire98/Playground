pub mod change;
use std::io;
fn main() {
    let mut _price = String::new();
    let mut _paid = String::new();
    println!("Input the price of the articles:");
    io::stdin()
        .read_line(&mut _price)
        .expect("Failed to read line");
    let _price: f32 = match _price.trim().parse() {
        Ok (num) => num,
        Err (_) => {
            println!("Input a valid number!");
            0.0
        }
    };
    println!("Input the paid amount:");
    io::stdin()
        .read_line(&mut _paid)
        .expect("Failed to read line");
    let _paid: f32 = match _paid.trim().parse() {
        Ok (num) => num,
        Err (_) => {
            println!("Input a valid number!");
            0.0
        }
    };
    if _paid < _price {return;}
    println!("{:#?}", change::change(_paid - _price));
}
