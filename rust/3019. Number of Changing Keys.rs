pub fn count_key_changes(s: String) -> i32 {
    let mut cnt = 0;
    let s: Vec<char> = s.chars().collect();

    for i in 1..s.len() {
        let (prev, curr) = (s[i - 1], s[i]);

        if prev.to_lowercase().next().unwrap() != curr.to_lowercase().next().unwrap() {
            cnt += 1;
        }
    }

    cnt
}