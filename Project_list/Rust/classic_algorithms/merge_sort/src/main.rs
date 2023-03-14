pub mod merge_sort;
fn main() {
    let mut vec = vec![400000000,9,1,10,3,2,5,7,4,12,12384,1,234,5,1234234,1234,156];
    let length = vec.len();
    merge_sort::merge_sort(&mut vec, 0, length);
    println!("{:?}", vec);
}
