use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut hash: HashMap<char, char> = HashMap::new();
    let mut longest_substring = 0;
    let mut counter = 0;

    for c in s.chars() {
        if hash.contains_key(&c) {
            if longest_substring < counter {
                longest_substring = counter
            }

            counter = 0;
            hash = HashMap::new();
        }

        counter += 1;
        hash.insert(c, c);
    }

    if longest_substring < counter {
        longest_substring = counter;
    }

    longest_substring
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = length_of_longest_substring(String::from("abcabcbb"));
        assert_eq!(result, 3);
    }

    #[test]
    fn test2() {
        let result = length_of_longest_substring(String::from("bbbbb"));
        assert_eq!(result, 1);
    }

    #[test]
    fn test3() {
        let result = length_of_longest_substring(String::from("pwwkew"));
        assert_eq!(result, 3);
    }

    #[test]
    fn test4() {
        let result = length_of_longest_substring(String::from(""));
        assert_eq!(result, 0);
    }

    #[test]
    fn test5() {
        let result = length_of_longest_substring(String::from(" "));
        assert_eq!(result, 1);
    }
}
