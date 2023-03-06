pub mod quick_sort;
fn main() {
    let mut vec = vec![1,3,2,5,4,45,4561,13423,235167,985,3457,1,3,167,909843,243565645,2452];
    quick_sort::quick_sort(&mut vec);
    println!("{:?}", vec);
}
