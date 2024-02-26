pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let dirs = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1)
    ];

    let word = word.chars().collect();
    
    use std::collections::HashSet;
    let mut set = HashSet::new();

    fn dfs(x: i32, y: i32, i: usize, board: &Vec<Vec<char>>, dirs: &[(i32, i32); 4], word: &Vec<char>, set: &mut HashSet<(i32, i32)>) -> bool {
        if i == word.len() {
            return true;
        }

        if x < 0 || y < 0 || x == board.len() as i32 || y == board[0].len() as i32 {
            return false;
        }

        if board[x as usize][y as usize] != word[i] {
            return false;
        }

        if set.contains(&(x, y)) {
            return false;
        }

        set.insert((x, y));

        for (a, b) in dirs {
            if dfs(x + *a, y + *b, i + 1, &board, dirs, &word, set) {
                return true;
            }
        }

        set.remove(&(x, y));

        false
    }

    for x in 0..board.len() {
        for y in 0..board[x].len() {
            if dfs(x as i32, y as i32, 0, &board, &dirs, &word, &mut set) {
                return true;
            }
        }
    }

    false
}