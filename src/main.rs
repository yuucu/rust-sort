
mod sort_lib;

pub use crate::sort_lib::sort;

fn main() {
    let mut test1: Vec<i32> = vec![5, 4, 9, 2, 1];
    sort::bubble_sort(&mut test1);
    println!("{}", test1.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(","));
}
