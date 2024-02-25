pub fn count_of_pairs(n: i32, x: i32, y: i32) -> Vec<i32> {
    let (x, y) = (x - 1, y - 1);

    let dist = |i: i32, j: i32| {
        let mut min: i32;

        min = (i - j).abs();
        min = min.min((i - x).abs() + (j - y).abs() + 1);
        min = min.min((i - y).abs() + (j - x).abs() + 1);

        min
    };

    let mut res = vec![0; n as usize];

    for k in 1..=n {
        for i in 0..n {
            for j in (i + 1)..n {
                if dist(i, j) == k {
                    res[(k - 1) as usize] += 2;
                }
            }
        }
    }

    res
}