pub fn bubble_sort(mut arr: Vec<i32>) -> Vec<i32> {
    for i in (0..arr.len()).rev() {
        let mut swapped = false;
        for j in 0..i + 1 {
            if j == arr.len() - 1 {continue;}
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
            if swapped == false {break;}
        }
    }
    arr
}