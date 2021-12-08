use std::collections::HashSet;

pub fn intersection(a: &str, b: &str) -> String {
    let a_chars = a.chars().collect::<HashSet<_>>();
    let b_chars = b.chars().collect::<HashSet<_>>();
    a_chars.intersection(&b_chars).collect()
}

pub fn union(a: &str, b: &str) -> String {
    let a_chars = a.chars().collect::<HashSet<_>>();
    let b_chars = b.chars().collect::<HashSet<_>>();
    a_chars.union(&b_chars).collect()
}

pub fn contains(container: &str, content: &str) -> bool {
    content.chars().all(|c| container.chars().any(|x| x == c))
}

pub fn matches(a: &str, b: &str) -> bool {
    a.chars().collect::<HashSet<_>>() == b.chars().collect::<HashSet<_>>()
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
        let result = union("abc", "cde");

        assert!(matches(&result, "abcde"))
    }

    #[test]
    fn test_intersection() {
        let result = intersection("dbca", "cde");

        assert!(matches(&result, "cd"))
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
