impl Solution {
    pub fn most_frequent_prime(mat: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashSet;
    
        fn sum(startx: i32, starty: i32, dirx: i32, diry: i32, mat: &Vec<Vec<i32>>) -> (bool, i32) {
            let (n, m) = (mat.len(), mat[0].len());
    
            let prev = mat[startx as usize][starty as usize];
    
            let (newx, newy) = (startx + dirx, starty + diry);
    
            if newx >= 0 && newx < n as i32 && newy >= 0 && newy < m as i32 {
                return (true, prev);
            }
    
            (false, prev)
        }
    
        fn is_prime(x: i32) -> bool {
            for i in 2..=((x as f64).sqrt() as i32) {
                if x % i == 0 {
                    return false;
                }
            }
    
            true
        }
    
        use std::collections::HashMap;
        let mut map: HashMap<i32, usize> = HashMap::new(); //prime to cnt
    
        let mut handleval = |mut i: i32, mut j: i32, dirx: i32, diry: i32, mat: &Vec<Vec<i32>>, set: &mut Vec<HashSet<i32>>| {
            let mut myset: HashSet<i32> = HashSet::new();
            
            let mut prev = 0;
            
            loop {
                let s = sum(i, j, dirx, diry, mat);
                prev *= 10;
                prev += s.1;
    
                if prev > 10 && is_prime(prev) {
                    myset.insert(prev);
                }
    
                if !s.0 {
                    break;
                }
    
                i += dirx;
                j += diry;
            }
    
            set.push(myset);
        };
    
        for i in 0..mat.len() {
            for j in 0..mat[i].len() {
                let (i, j) = (i as i32, j as i32);
    
            
                let mut vec = Vec::new();
                let mutref = &mut vec;
    
                handleval(i, j, 1, 1, &mat, mutref);
                handleval(i, j, 1, 0, &mat, mutref);
                handleval(i, j, 1, -1, &mat, mutref);
                handleval(i, j, 0, 1, &mat, mutref);
                handleval(i, j, 0, -1, &mat, mutref);
                handleval(i, j, -1, 1, &mat, mutref);
                handleval(i, j, -1, 0, &mat, mutref);
                handleval(i, j, -1, -1, &mat, mutref);
    
                for set in vec {
                    for x in set {
                        *map.entry(x).or_insert(0) += 1;
                    }
                }
            }
        }
    
        let (mut bigi, mut bigv) = (-1, 0);
    
        for (i, v) in map {
            if v > bigv || (v == bigv && i > bigi) {
                bigv = v;
                bigi = i;
            }
        }
    
        bigi
    }
    }