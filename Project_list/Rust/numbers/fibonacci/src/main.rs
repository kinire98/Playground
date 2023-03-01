use std::io;
pub mod fibonacci;
fn main() {
    let mut _input = String::new();
    println!("The number of the fibonacci sequence:");
    io::stdin()
        .read_line(&mut _input)
        .expect("Failed to read line");
    let _input: u64 = match _input.trim().parse() {
        Ok (num) => num,
        Err (_) => {
            println!("Input a valid number!");
            0
        }
    };
    println!("The {}nth number of the fibonacci sequence is:", _input); 
    println!("{}", fibonacci::fibonacci(_input));
}
