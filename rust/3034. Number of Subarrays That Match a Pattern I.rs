pub fn count_matching_subarrays(nums: Vec<i32>, pat: Vec<i32>) -> i32 {
    let mut cnt = 0;

    'out: for win in nums.windows(pat.len() + 1) {
        for i in 0..(win.len() - 1) {
            let state = match pat[i] {
                0 => win[i] == win[i + 1],
                1 => win[i + 1] > win[i],
                -1 => win[i + 1] < win[i],
                _ => unreachable!(),
            };

            if !state {
                break;
            }

            if i == win.len() - 2 {
                cnt += 1;
            }
        }
    }

    cnt
}