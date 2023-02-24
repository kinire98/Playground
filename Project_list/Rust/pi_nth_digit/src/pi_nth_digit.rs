pub fn pi_nth_digit(digit: u32) -> f64 {
    if digit > 12 {return 0.0}
    // let first_pow = if is_even(digit) {1} else {-1};
    // let first_factorial: u32 = (1..=digit).product();
    // let first_sum = f64::from(545140134) * f64::from(digit) + f64::from(13591409);
    // let numerator = f64::from(first_factorial) * first_sum * f64::from(first_pow);
    // let second_factorial: u32 = (1..=3 * digit).product();
    // let third_factorial: u32 = (1..= digit).product();
    // let third_factorial_powered = third_factorial.pow(3);
    // let num_for_pow: u32 = 640320;
    // let first_pow: u32 = num_for_pow.pow(3 * digit + (3/2));
    // let denominator = third_factorial_powered * first_pow * second_factorial; 
    // let pi: f64 = f64::from(12) * (f64::from(numerator) / f64::from(denominator));
    let pi: f64 = (f64::from(4)/f64::from(1)) - (f64::from(4)/f64::from(3)) + (f64::from(4)/f64::from(5)) - (f64::from(4)/f64::from(7)) + (f64::from(4)/f64::from(9)) - (f64::from(4)/f64::from(11)) + (f64::from(4)/f64::from(13)) - (f64::from(4)/f64::from(15)) + (f64::from(4)/f64::from(17)) - (f64::from(4)/f64::from(19)) + (f64::from(4)/f64::from(21)) - (f64::from(4)/f64::from(23)) + (f64::from(4)/f64::from(25));
    pi
}
// fn factorial(num: u32)  -> u32 {
//     let count = 1;
//     let mut result = 1;
//     while count <= num {
//         result *= count;
//     }
//     result
// }
fn is_even(num: u32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    false
}