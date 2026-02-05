fn permutations(s: &str) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    let mut result = Vec::new();
    permute(&chars, 0, &mut result);
    result
}

fn permute(chars: &[char], start: usize, result: &mut Vec<String>) {
    if start == chars.len() {
        result.push(chars.iter().collect());
        return;
    }
    let mut chars = chars.to_vec();
    for i in start..chars.len() {
        chars.swap(start, i);
        permute(&chars, start + 1, result);
        chars.swap(start, i);
    }
}
