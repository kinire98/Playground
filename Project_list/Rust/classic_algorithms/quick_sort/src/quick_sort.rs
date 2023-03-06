pub fn quick_sort(vec: &mut Vec<i32>) {
    if !vec.is_empty() {
        if vec.len() == 1 || vec.len() == 0 {return;}
        let pivot = vec[vec.len() - 1];
        let mut left: Vec<i32> = vec![];
        let mut right: Vec<i32> = vec![];
        for i in 0..vec.len() - 1 {
            if vec[i] >= pivot {
                right.push(vec[i]);
            }
            if vec[i] < pivot {
                left.push(vec[i]);
            }
        }
        vec.clear();
        quick_sort(&mut left);
        quick_sort(&mut right);
        vec.append(&mut left);
        vec.push(pivot);
        vec.append(&mut right);
    }
}