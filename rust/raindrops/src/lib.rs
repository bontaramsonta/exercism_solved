pub fn raindrops(n: u32) -> String {
    let map = [(3, "Pling"), (5, "Plang"), (7, "Plong")];
    let mut result = String::new();
    for &(factor, sound) in map.iter() {
        if n % factor == 0 {
            result.push_str(sound);
        }
    }
    if result.is_empty() {
        result.push_str(&n.to_string());
    }
    result
}
