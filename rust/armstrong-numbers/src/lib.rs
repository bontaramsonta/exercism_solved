pub fn is_armstrong_number(num: u64) -> bool {
    let mut digits: Vec<u64> = Vec::new();
    let mut n = num;
    while n > 0 {
        digits.push((n % 10) as u64);
        n /= 10;
    }
    let len = digits.len() as u32;
    num == digits.iter().map(|&d| d.pow(len)).sum()
}
