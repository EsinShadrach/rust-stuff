pub fn add(mut a: i32, mut b: i32) -> i32 {
    if a == 0 {
        return b;
    }

    if b == 0 {
        return a;
    }

    let carry = a & b;
    a = a ^ b;
    b = carry << 1;

    if b != 0 {
        return add(a, b);
    }

    return a;
}
