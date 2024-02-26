pub fn return_to_boundary_count(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut cnt = 0;

    for x in nums.into_iter() {
        sum += x;

        if sum == 0 {
            cnt += 1;
        }
    }

    cnt
}