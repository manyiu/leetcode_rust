struct Solution;

impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&j) = map.get(&complement) {
                return vec![j as i32, i as i32];
            }
            map.insert(num, i);
        }

        vec![]
    }
}

#[test]
fn test() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let res = Solution::two_sum(nums, target);
    assert_eq!(res, vec![0, 1]);
}
