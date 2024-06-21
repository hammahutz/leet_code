use std::result;

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut i = 0;
    let mut j = 0;
    let mut resutl_vec: Vec<i32> = vec![];

    loop {
        let i_value = nums1.get(i);
        let j_value = nums2.get(j);

        match i_value {
            Some(ival) => match j_value {
                Some(_) => todo!(),
                None => todo!(),
            },
            None => match j_value {
                Some(_) => todo!(),
                None => todo!(),
            },
        }

        break;
    }
    0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = find_median_sorted_arrays(vec![1, 3], vec![2]);
        assert_eq!(result, 2.0);
    }
}
