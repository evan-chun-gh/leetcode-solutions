pub fn is_possible_to_split(nums: Vec<i32>) -> bool {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    for x in nums {
        *map.entry(x).or_insert(0) += 1;
    }

    map.iter().all(|x| *x.1 <= 2)
}