fn build(pat: &Vec<i32>) -> Vec<usize> {
    let mut lps = vec![0; pat.len()];

    let mut j = 0;

    for i in 1..lps.len() {
        while j > 0 && pat[j] != pat[i] {
            j = lps[j - 1];
        }

        if pat[i] == pat[j] {
            j += 1;
            lps[i] = j
        }
    }

    lps
}

pub fn count_matching_subarrays(mut nums: Vec<i32>, pat: Vec<i32>) -> i32 {
    let mut arr = vec![0; nums.len()];
    
    for i in 1..nums.len() {
        if nums[i] == nums[i - 1] {
            arr[i] = 0;
        } else if nums[i] > nums[i - 1] {
            arr[i] = 1;
        } else {
            arr[i] = -1;
        }
    }

    let lps = Self::build(&pat);
    let mut cnt = 0;

    let mut j = 0;

    for i in 1..arr.len() {
        while j > 0 && pat[j] != arr[i] {
            j = lps[j - 1];
        }

        if pat[j] == arr[i] {
            j += 1;
            if j == lps.len() {
                cnt += 1;
                j = lps[j - 1];
            }
        }
    }

    cnt
}