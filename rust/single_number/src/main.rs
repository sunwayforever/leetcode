// 2019-01-03 17:32
// @lc id=136 lang=rust
// problem: single_number
struct Solution {}
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |ret, i| ret ^ i)
    }
}

fn main() {
    println!("{:?}", Solution::single_number(vec![4, 1, 2, 1, 2]));
}
