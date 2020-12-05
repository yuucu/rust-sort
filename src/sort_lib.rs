pub mod sort {
    pub fn bubble_sort(array: &mut [i32]) {
        for i in 0..array.len() {
            for j in i + 1..array.len() {
                if array[i] > array[j] {
                    array.swap(i, j);
                }
            }
        }
    }
}
