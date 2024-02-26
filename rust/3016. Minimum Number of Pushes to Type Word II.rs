pub fn minimum_pushes(s: String) -> i32 {
    let s = s.into_bytes();

    use std::collections::HashMap;
    let mut c_cnt: HashMap<u8,usize> = HashMap::new();

    for c in s.into_iter() {
        c_cnt.entry(c).and_modify(|x| *x += 1).or_insert(1);
    }
    
    let mut cnt = Vec::new();
    
    for x in c_cnt.into_iter() {
        cnt.push(x.1);
    }
    
    cnt.sort();

    let mut cost = 0;
        
    for (i, times) in cnt.into_iter().rev().enumerate() {
        cost += (i / 8 + 1) * times;
    }
        
    cost as _
}