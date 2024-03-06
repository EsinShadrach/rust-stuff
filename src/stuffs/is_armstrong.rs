pub fn is_armstrong(needle: usize) -> bool {
    let mut n = needle;
    let mut sum = 0;
    let mut digits = Vec::new();

    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }

    let len = digits.len() as u32;

    for digit in digits {
        sum += digit.pow(len);
    }

    return sum == needle;
}

pub fn is_armstrong2(needle: usize) -> bool {
    let mut temp = needle;
    let mut sum = 0;

    while temp > 0 {
        let remainder = temp % 10;
        sum += remainder.pow(3);

        temp = temp / 10
    }

    return sum == needle;
}

pub fn total_sum(needle: usize) {
    let mut temp = needle;
    let mut sum = 0;

    while temp > 0 {
        let remainder = temp % 10;
        sum += remainder;

        temp = temp / 10
    }

    println!("Total sum of digits: {}", sum);
}

pub fn min_max(needle: Vec<i32>) -> (i32, i32) {
    let mut min = needle[0];
    let mut max = needle[0];

    for &num in needle.iter() {
        if num < min {
            min = num;
        }

        if num > max {
            max = num;
        }
    }

    return (min, max);
}
