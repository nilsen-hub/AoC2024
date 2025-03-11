pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    } else {
        return gcd(b, a % b);
    }
}
pub fn lcm(a: usize, b: usize) -> usize {
    return a * (b / gcd(a, b));
}
