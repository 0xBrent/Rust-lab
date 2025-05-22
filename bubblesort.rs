pub fn bubble_sort<T: std::cmp::PartialOrd>(list: &mut [T]) {
    let length = list.len();
    for i in 0..length {
        for j in 0..length - 1 - i {
            if list[j] > list[j + 1] {
                list.swap(j, j + 1);
            }
        }
    }
}
