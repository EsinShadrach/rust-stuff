use stuffs::{binary_search_vs_linear::run_binary_search_vs_linear, is_armstrong::is_armstrong2};

use crate::stuffs::is_armstrong::total_sum;

pub mod stuffs;

fn main() {
    run_binary_search_vs_linear();
    let num_is_armstrong = is_armstrong2(153);
    total_sum(132);
    println!("{num_is_armstrong}");
}
