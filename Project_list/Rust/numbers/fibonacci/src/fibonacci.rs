pub fn fibonacci(num: u64) -> u64 {
    if num > 7063 { return 0; }
    if num == 1 {
        return 1;
    }
    return num + fibonacci(num - 1);
}