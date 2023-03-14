pub fn merge_sort(vec: &mut Vec<i32>, left: usize, right: usize) {
    if left >= right || left == right - 1 {return;}
    if right - left == 2 {
        match vec[left] > vec[right - 1] {
            true => {
                vec.swap(left, right - 1);
            }
            false => (),
        }
        return;
    }
    let middle = left + (right - left) / 2;
    merge_sort(vec, left, middle);
    merge_sort(vec, middle, right);
    let left_content = &vec.clone()[left..middle];
    let right_content = &vec.clone()[middle..right]; 
    let len = left_content.len() + right_content.len();
    let mut right_pointer = 0;
    let mut left_pointer = 0;
    for i in 0..len {
        if left + i >= vec.len() {break;}
        if left_pointer >= left_content.len() {
            vec[left + i] = right_content[right_pointer];
            right_pointer += 1;
            continue;
        }
        if right_pointer >= right_content.len() {
            vec[left + i] = left_content[left_pointer];
            left_pointer += 1;
            continue;
        } 
        if right_content[right_pointer] < left_content[left_pointer] {
            vec[left + i] = right_content[right_pointer];
            right_pointer += 1;
            continue;
        }
        vec[left + i] = left_content[left_pointer];
        left_pointer += 1;
    }
}