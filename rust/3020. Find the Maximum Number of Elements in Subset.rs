pub fn maximum_length(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut num_pair: HashMap<i32, usize> = HashMap::new();

    for x in nums {
        num_pair.entry(x).and_modify(|x| *x += 1).or_insert(1);
    }

    let mut max = 1;

    for x in num_pair.iter() {
        let (x, f) = (*x.0, *x.1);

        if x == 1 {
            max = max.max(f - (f % 2 == 0) as usize);
            continue;
        }

        let mut size = 0;
        let mut curr = x;

        while let Some(y) = num_pair.get(&curr) {
            if *y < 2 {
                max = max.max(size + 1);
                break;
            }

            size += 2;
            curr *= curr;

            if num_pair.get(&curr).is_some() {
                max = max.max(size + 1);
            }
        }
    }

    max as _
}