pub fn is_pair(left: char, right: char) -> bool {
    match (left, right) {
        ('(', ')') | ('[', ']') | ('{', '}') => true,
        _ => false,
    }
}
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for c in string.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' | ']' | '}' => {
                if let Some(last) = stack.pop() {
                    if !is_pair(last, c) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => (),
        }
    }
    stack.is_empty()
}
