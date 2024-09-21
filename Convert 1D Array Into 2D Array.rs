impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        if original.len() != (m * n) as usize {
            return vec![];  // Return empty array if dimensions don't match
        }
        
        let mut result = vec![vec![0; n as usize]; m as usize];  // Create 2D array
        for i in 0..original.len() {
            result[i / n as usize][i % n as usize] = original[i];  // Populate 2D array
        }
        
        result
    }
}
