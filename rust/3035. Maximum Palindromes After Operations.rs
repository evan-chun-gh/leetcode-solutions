pub fn max_palindromes_after_operations(words: Vec<String>) -> i32 {
    let mut cnt = [0usize; 26];
    let mut lens = Vec::new();
    let mut out = 0;

    for word in words {
        let word = word.into_bytes();
        lens.push(word.len());

        for c in word {
            let c = c as u8 - 'a' as u8;

            cnt[c as usize] += 1;
        }
    }

    let mut pairs = 0;

    for c in cnt {
        pairs += c / 2;
    }

    lens.sort();

    let mut pairs = pairs as i32;
    let alllen = lens.len();

    for (i, v) in lens.into_iter().enumerate() {
        pairs -= v as i32 / 2;

        if pairs < 0 {
            return i as i32;
        }
    }

    alllen as _
}