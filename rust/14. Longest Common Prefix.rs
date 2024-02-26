pub fn longest_common_prefix(mut strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }

    if strs.len() == 1 {
        return strs.remove(0);
    }

    let strs: Vec<Vec<u8>> = strs.into_iter().map(|x| x.into_bytes()).collect();
    let mut res = 0;

    loop {
        if !strs.iter().all(|x| x.len() > res) {
            break;
        }

        let mut c = strs[0][res];

        if !strs.iter().all(|x| x[res] == c) {
            break
        }

        res += 1;
    }

    String::from_utf8(strs[0][..res].to_vec()).unwrap()
}