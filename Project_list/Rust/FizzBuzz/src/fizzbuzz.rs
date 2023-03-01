pub fn fizzbuzz(num: u8) {
    for i in 1..num + 1 {
        let mut output = String::new();
        if i % 3 == 0 { output = output + "Fizz"}
        if i % 5 == 0 { output = output + "Buzz"}
        if output == "" {output = output + &i.to_string()}
        println!("{}", output);
    }
}