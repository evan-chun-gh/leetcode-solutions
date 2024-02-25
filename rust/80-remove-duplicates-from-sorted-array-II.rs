pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut i = 0;

    while i + 2 < nums.len() {
        if nums[i] == nums[i + 1] && nums[i] == nums[i + 2] {
            nums.remove(i);
        } else {
            i += 1;
        }
    }

    nums.len() as i32
}