pub mod bubble_sort;
fn main() {
    let vec = vec![1,3,2,1,45,45,42,45,6,7,89876,25435,614352];
    let vec = bubble_sort::bubble_sort(vec);
    println!("{:?}", vec);
}
