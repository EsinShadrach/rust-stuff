use std::time::Instant;

pub fn run_binary_search_vs_linear() {
    // Dataset size
    const DATASET_SIZE: usize = 100_000_000;

    let mut search_data = Vec::with_capacity(DATASET_SIZE);
    for i in 0..DATASET_SIZE {
        search_data.push(i);
    }

    // let target_value = DATASET_SIZE / 2;
    let target_value = 1;

    let start_linear = Instant::now();
    let linear_result = linear_search(&search_data, target_value);
    let linear_elapsed = start_linear.elapsed().as_secs_f32();

    let start_binary = Instant::now();
    let binary_result = binary_search(&search_data, target_value);
    let binary_elapsed = start_binary.elapsed().as_secs_f32();

    match linear_result {
        Some(index) => {
            println!("Linear search found value at index {}", index);
        }
        None => {
            println!("Linear search: Value not found");
        }
    }

    match binary_result {
        Some(index) => {
            println!("Binary search found value at index {}", index);
        }
        None => {
            println!("Binary search: Value not found");
        }
    }

    println!("Linear search time: {} seconds", linear_elapsed);
    println!("Binary search time: {} seconds", binary_elapsed);
}

fn linear_search(haystack: &[usize], needle: usize) -> Option<usize> {
    for i in 0..haystack.len() {
        let current = haystack[i];
        if current == needle {
            return Some(i);
        }
    }
    return None;
}

fn binary_search(haystack: &[usize], needle: usize) -> Option<usize> {
    let mut low = 0;
    let mut high = haystack.len();

    while low < high {
        // in the commneted line below we are adding low and high and left shifting it
        // you can think of it as low + high / 2
        let middle = (low + high) >> 1;
        // The commented line let middle = low + (high - low) / 2;
        // is an alternative way to calculate the middle index.
        // However, it uses integer division (/),
        // which can cause rounding errors in certain scenarios.
        // let middle = low + (high - low) / 2;
        let value = haystack[middle];

        if value == needle {
            return Some(middle);
        } else if needle < value {
            high = middle;
        } else {
            low = middle + 1
        }
    }

    return None;
}
