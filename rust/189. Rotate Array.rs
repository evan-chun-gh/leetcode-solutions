pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    nums.rotate_right(k as usize % len)
}