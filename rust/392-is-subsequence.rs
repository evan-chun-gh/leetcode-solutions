pub fn is_subsequence(s: String, t: String) -> bool {
    if s.is_empty() && !t.is_empty() {
        return true;
    }

    let (s, t) = (s.into_bytes(), t.into_bytes());

    let mut j = 0;

    for i in 0..t.len() {
        if s[j] == t[i] {
            j += 1;

            if j == s.len() {
                break;
            }
        }
    }

    j == s.len()
}