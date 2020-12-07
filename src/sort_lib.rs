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


    pub fn quick_sort(array: &mut [i32], start: usize, end: usize) {
        if start < end {
            let split = partition(array, start, end);
            if split > 0 {
                quick_sort(array, start, split - 1);
            }
            quick_sort(array, split + 1, end);
        }
    }

    fn partition(array: &mut [i32], p: usize, r: usize) -> usize {
        let pivot = array[r];
        let mut i = p;
        for j in p..r {
            if array[j] <= pivot {
                array.swap(i, j);
                i = i + 1;
            }
        }
        array.swap(i, r);
        return i
    }

}
