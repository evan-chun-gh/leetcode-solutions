pub fn minimum_time_to_initial_state(word: String, k: i32) -> i32 {
    let word = word.into_bytes();

    let mut lps = vec![0; word.len()];
    let mut j = 0;

    for i in 1..lps.len() {
        while j > 0 && word[j] != word[i] {
            j = lps[j - 1];
        }

        if word[j] == word[i] {
            j += 1;
            lps[i] = j;
        }
    }

    let mut i = lps[lps.len() - 1];
    let k = k as usize;
    let n = word.len();

    while i > 0 && (n - i) % k > 0 {
        i = lps[i - 1];
    }

    ((n - i + k - 1) / k) as i32
}