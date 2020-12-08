#[allow(dead_code)]

use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

pub struct SquareMatrix<T> {
    dim: usize,
    values: Vec<Vec<T>>,
    labels: Vec<String>
}

/// Implement other impl for other types...
impl SquareMatrix<u32> {
    pub fn new(n: usize) -> SquareMatrix<u32> {
        SquareMatrix {
            dim: n,
            values: vec![vec![0; n]; n],
            labels: vec!["".to_string(); n]
        }
    }

    pub fn resize(&mut self, n: usize) {
        self.dim = n;
        
        self.values.resize(n, vec![0; n]);
        for row in 0..n {
            self.values[row].resize(n, 0);
        }
        self.labels.resize(n, "".to_string());
    }

    pub fn add_labelled_row(&mut self, row_name: String) {
        self.resize(self.dim+1);
        self.labels[self.dim-1] = row_name;
    }

    pub fn get_labelled_row(&mut self, row_name: String) -> Vec<u32> {
        let index = self.labels.iter().position(|r| r.eq(&row_name));

        match index {
            Some(x) => {
                return self.values[x].clone();
            }
            None => {
                return Vec::new();
            }
        }
    }

    #[allow(dead_code)]
    pub fn set_in_labelled_row_usize(&mut self, row_name: String, col: usize, value: u32) {
        let index = self.labels.iter().position(|r| r == &row_name);
        match index {
            Some(x) => self.values[x][col] = value,
            None => {
                self.add_labelled_row(row_name);
                self.values[self.dim-1][col] = value;
            },
        }
    }

    pub fn set_in_labelled_row(&mut self, row_name: String, col_name: String, value: u32) {
        let index_row = self.labels.iter().position(|r| r == &row_name);
        
        match index_row {
            Some(x) => {
                let index_col = self.labels.iter().position(|r| r == &col_name);
                match index_col {
                    Some(y) => self.values[x][y] = value,
                    None => {
                        self.add_labelled_row(col_name);
                        self.values[x][self.dim-1] = value;
                    }
                }
            }
            None => {
                self.add_labelled_row(row_name);
                let index_col = self.labels.iter().position(|r| r == &col_name);
                match index_col {
                    Some(y) => self.values[self.dim-1][y] = value,
                    None => {
                        self.add_labelled_row(col_name);
                        self.values[self.dim-2][self.dim-1] = value;
                    }
                }
            },
        }
    }

    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.dim
    }

    pub fn index_of(&self, n: String) -> Option<usize> {
        self.labels.iter().position(|r| r == &n)
    }

    pub fn get_label(&self, n: usize) -> String {
        self.labels[n].clone()
    }

    pub fn get_values(&self) -> Vec<Vec<u32>> {
        self.values.clone()
    }

    pub fn get_labels(&self) -> Vec<String> {
        self.labels.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_creation() {
        let m = SquareMatrix::new(2);
        let _m2 = SquareMatrix::new(0);

        assert_eq!(m.dim, 2);
        assert_eq!(m.values, vec![vec![0; 2]; 2]);
    }

    #[test]
    fn test_matrix_resize() {
        let mut m = SquareMatrix::new(2);
        m.resize(3);

        assert_eq!(m.dim, 3);
        assert_eq!(m.values, vec![vec![0; 3]; 3]);
    }

    #[test]
    fn test_matrix_resize_with_values() {
        let mut m = SquareMatrix::new(2);

        m.values[0][1] = 2;

        m.resize(3);

        assert_eq!(m.dim, 3);
        assert_eq!(m.values[0][1], 2);
        assert_eq!(m.values[2][2], 0);
    }

    #[test]
    fn test_matrix_add_labelled_row() {
        let mut m = SquareMatrix::new(0);

        m.add_labelled_row("Peanut".to_string());
        m.add_labelled_row("Butter".to_string());

        m.values[0][1] = 2;
        m.get_labelled_row("Butter".to_string())[0] = 3;  // This won't set the original m
        m.set_in_labelled_row_usize("Butter".to_string(), 1, 4);
        m.set_in_labelled_row("Peanut".to_string(), "Peanut".to_string(), 5);

        assert_eq!(m.dim, 2);
        assert_eq!(m.labels, vec!["Peanut".to_string(), "Butter".to_string()]);

        assert_eq!(m.get_labelled_row("Peanut".to_string()), vec![5, 2]);
        assert_eq!(m.get_labelled_row("Butter".to_string()), vec![0, 4]);
        assert_eq!(m.values, vec![vec![5, 2], vec![0, 4]]);
    }

    #[test]
    fn test_matrix_add_labelled_row_part2() {
        let mut m = SquareMatrix::new(0);

        let peanut = "Peanut".to_string();
        let butter = "Butter".to_string();

        m.set_in_labelled_row(peanut.clone(), peanut.clone(), 5);
        m.set_in_labelled_row(peanut.clone(), butter.clone(), 2);
        m.set_in_labelled_row(butter.clone(), peanut.clone(), 0);
        m.set_in_labelled_row(butter.clone(), butter.clone(), 4);

        assert_eq!(m.values, vec![vec![5, 2], vec![0, 4]]);
    }
}