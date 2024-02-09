fn main() {
    let mut unsorted_array = vec![1, 5, 3, 2, 9, 11, 6, -1];
    sort_array(&mut unsorted_array);
    println!("{:?}", unsorted_array);
}

fn sort_array(array: &mut Vec<i32>) {
    let length = array.len();

    for i in (0 as usize)..length {
        for j in (0 as usize)..length - 1 - i {
            if array[j] > array[j + 1] {
                let tmp = array[j];
                array[j] = array[j + 1];
                array[j + 1] = tmp;
            }
        }
    }
}