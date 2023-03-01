use std::io;
pub mod pi_nth_digit;
fn main() {
    // ! This program is a piece of shit put together, my attempt is put toghether, but it is a piece of shti anyways
    // ! I'm still learning rust and I wanted to try
    let mut _input = String::new();
    println!("The digit you want of PI:");
    io::stdin()
        .read_line(&mut _input)
        .expect("Failed to read line");
    println!("Pi to the {}th digit is:", _input); 
    let _input: f32 = match _input.trim().parse() {
            Ok (num) => num,
            Err (_) => {
                println!("Input a valid number!");
                0.0
            }
    };
    println!("{}", pi_nth_digit::pi_nth_digit(_input));
}