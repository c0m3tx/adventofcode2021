use std::collections::HashMap;

pub fn intersection(a: &str, b: &str) -> String {
    let mut chars = a
        .chars()
        .chain(b.chars())
        .fold(HashMap::new(), |mut map, c| {
            let val = map.entry(c).or_insert(0u64);
            *val += 1;

            map
        })
        .iter()
        .filter(|(_, &val)| val >= 2)
        .map(|(k, _)| *k)
        .collect::<Vec<char>>();
    chars.sort();
    chars.iter().collect()
}

pub fn union(a: &str, b: &str) -> String {
    let mut all_chars = a.chars().chain(b.chars()).collect::<Vec<char>>();
    all_chars.sort();
    all_chars.dedup();
    all_chars.iter().collect()
}

pub fn contains(container: &str, content: &str) -> bool {
    content.chars().all(|c| container.chars().any(|x| x == c))
}

pub fn matches(a: &str, b: &str) -> bool {
    a.len() == b.len() && a.chars().all(|a| b.chars().any(|b| a == b))
}

pub fn find<'a>(needle: &str, stack: &Vec<String>) -> String {
    stack
        .iter()
        .find(|&x| matches(x.as_str(), needle))
        .unwrap()
        .clone()
}

pub fn for_length(patterns: &Vec<String>, length: usize) -> String {
    patterns
        .iter()
        .find(|&x| x.len() == length)
        .unwrap()
        .clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union() {
        assert_eq!(union("abc", "cde"), "abcde")
    }

    #[test]
    fn test_intersection() {
        assert_eq!(intersection("dbca", "cde"), "cd")
    }

    #[test]
    fn test_matches() {
        assert!(matches("abcde", "edbca"));
        assert!(!matches("abcde", "abcdf"));
        assert!(!matches("ab", "abc"));
    }

    #[test]
    fn test_contains() {
        assert!(contains("abc", "cb"));
    }
}
