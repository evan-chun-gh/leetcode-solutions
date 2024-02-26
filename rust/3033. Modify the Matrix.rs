pub fn modified_matrix(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut maxes = vec![0; matrix[0].len()];

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            maxes[j] = maxes[j].max(matrix[i][j]);
        }
    }

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if (matrix[i][j] == -1) {
                matrix[i][j] = maxes[j];
            }
        }
    }

    matrix
}