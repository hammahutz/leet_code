use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut hash: HashMap<char, char> = HashMap::new();
    let mut longest_substring = 0;
    // let mut char_in_s = s.ch

    for (i, char) in s.chars().enumerate() {
        hash.insert(char, char);
        for j in (i + 1)..s.len() {
            let test_char = s.chars().nth(j);
            if let Some(test_char) = test_char {
                if hash.contains_key(&test_char) {
                    if longest_substring < hash.len() {
                        longest_substring = hash.len();
                    }
                    hash.clear();
                    break;
                }
                hash.insert(test_char, test_char);
            }
        }
    }

    if longest_substring < hash.len() {
        longest_substring = hash.len();
    }

    longest_substring as i32
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
    #[test]
    fn test6() {
        let result = length_of_longest_substring(String::from("dvdf"));
        assert_eq!(result, 3);
    }
    #[test]
    fn test7() {
        let result = length_of_longest_substring(String::from("kdgjkjhglfp"));
        assert_eq!(result, 7);
    }
}
