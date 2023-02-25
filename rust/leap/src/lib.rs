// basic
pub fn is_leap_year(year: u64) -> bool {
    if year % 400 == 0 {
        true
    } else if year % 100 == 0 {
        false
    } else if year % 4 == 0 {
        true
    } else {
        false
    }
}

// match style
pub fn is_leap_year(year: u64) -> bool {
    match year {
        x if x % 400 == 0 => true,
        x if x % 100 == 0 => false,
        x if x % 4 == 0 => true,
        _ => false,
    }
}
