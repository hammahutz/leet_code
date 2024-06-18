use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..(nums.len() - 1) {
        for j in (i + 1)..nums.len() {
            let sum = nums[i] + nums[j];
            if sum == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    Vec::new()
}

pub fn two_sum_hash(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash: HashMap<i32, i32> = HashMap::new();

    for i in 0..nums.len() {
        hash.insert(nums[i], i as i32);
    }

    for i in 0..nums.len() {
        let sum = target - nums[i];

        if hash.contains_key(&sum) && hash[&sum] != i as i32 {
            return vec![i as i32, hash.get(&sum).copied().unwrap()];
        }
    }

    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let result = two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(result, vec![0, 1]);
    }
    #[test]
    fn test_two() {
        let result = two_sum(vec![3, 2, 4], 6);
        assert_eq!(result, vec![1, 2]);
    }
    #[test]
    fn test_three() {
        let result = two_sum(vec![3, 3], 6);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_one_hash() {
        let result = two_sum_hash(vec![2, 7, 11, 15], 9);
        assert_eq!(result, vec![0, 1]);
    }
    #[test]
    fn test_two_hash() {
        let result = two_sum_hash(vec![3, 2, 4], 6);
        assert_eq!(result, vec![1, 2]);
    }
    #[test]
    fn test_three_hash() {
        let result = two_sum_hash(vec![3, 3], 6);
        assert_eq!(result, vec![0, 1]);
    }
}
