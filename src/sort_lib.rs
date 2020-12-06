pub mod sort {
    pub fn bubble_sort(array: &mut [i32]) {
        for i in 0..array.len() {
            for j in (i + 1..array.len()).rev() {
                if array[j - 1] > array[j] {
                    array.swap(j, j - 1);
                }
            }
        }
    }
}
