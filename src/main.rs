
mod sort_lib;

pub use crate::sort_lib::sort;

fn main() {

    let mut bubble_test: Vec<i32> = vec![5, 10, 9, 2, 1, 4, -5];
    sort::bubble_sort(&mut bubble_test);
    println!("{}", bubble_test.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(","));

    let mut quick_test: Vec<i32> = vec![7, 8, 9, 0, 1, -1, 10];
    let quick_test_num = quick_test.len() - 1;
    sort::quick_sort(&mut quick_test, 0, quick_test_num);
    println!("{}", quick_test.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(","));
}
