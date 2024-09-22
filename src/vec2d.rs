use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Iter<T> {
    container: Vec2d<T>,
    pos: usize,
}

impl<T> Iter<T> {
    pub fn new(container: Vec2d<T>) -> Iter<T> {
        Iter { container, pos: 0 }
    }
}

impl<T: Copy> Iterator for Iter<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let item = Some(Vec::from(self.container.get_row(0)?));
        self.pos += 1;
        item
    }
}

#[derive(Debug, Clone)]
pub struct Vec2d<T> {
    vec: Vec<T>,
    dim: usize,
}

impl<T: Copy> Vec2d<T> {
    pub fn new(dim: usize) -> Vec2d<T> {
        Vec2d {
            vec: Vec::new(),
            dim,
        }
    }

    pub fn idx(&self, row: usize, col: usize) -> T {
        self.vec[col * self.dim + row]
    }

    pub fn get_row(&self, idx: usize) -> Option<&[T]> {
        let row_start = idx * self.dim;
        let row_end = row_start + self.dim;
        if row_end > self.vec.len() {
            return None;
        }
        Some(&self.vec[row_start..row_end])
    }

    pub fn get_col(&self, idx: usize) -> Option<Vec<T>> {
        if idx >= self.dim {
            return None;
        }

        let mut res = Vec::new();
        let (rows, _cols) = self.shape();

        for i in 0..rows {
            let row = self.get_row(i)?;
            let e = row[idx];
            res.push(e);
        }

        Some(res)
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.vec.len() / self.dim, self.dim)
    }

    pub fn push(&mut self, item: T) {
        self.vec.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.vec.pop()
    }

    pub fn pop_head(&mut self) -> Vec<T> {
        let (_rows, cols) = self.shape();
        let mut res = Vec::new();
        for i in (0..cols).rev() {
            let e = self.vec.remove(i);
            res.push(e);
        }

        res
    }

    pub fn push_slice(&mut self, items: &[T]) {
        for e in items {
            self.vec.push(*e);
        }
    }

    pub fn into_iter(&self) -> Iter<T> {
        Iter::new(self.clone())
    }

    pub fn split_right(&self) -> (Vec2d<T>, Vec<T>) {
        let (rows, cols) = self.shape();
        let mut left_cols = Vec2d::<T>::new(cols - 1);
        let mut right_col = Vec::new();

        for i in 0..rows {
            let row = self
                .get_row(i)
                .expect("indexed by row bounded range, should not overrun");
            let (right, lefts) = row.split_last().unwrap();
            left_cols.push_slice(lefts);
            right_col.push(*right);
        }

        (left_cols, right_col)
    }

    pub fn split_left(&self) -> (Vec<T>, Vec2d<T>) {
        let (rows, cols) = self.shape();
        let mut right_cols = Vec2d::<T>::new(cols - 1);
        let mut left_col = Vec::new();

        for i in 0..rows {
            let row = self
                .get_row(i)
                .expect("indexed by row bounded range, should not overrun");
            let (left, rights) = row.split_first().unwrap();
            right_cols.push_slice(rights);
            left_col.push(*left);
        }

        (left_col, right_cols)
    }
}

impl<T: Copy + std::fmt::Debug> Vec2d<T> {
    pub fn print_head(&self) {
        let (rows, _cols) = self.shape();
        for i_row in 0..rows.min(10) {
            let row = self.get_row(i_row).unwrap();
            row.iter().for_each(|e| print!("{e:?}, "));
            println!("");
        }
    }
}

// This is much easier as a non-generic function
pub fn categorize_cols(input: Vec2d<&str>) -> Vec2d<f64> {
    let (rows, cols) = input.shape();
    let mut vec2d_f64 = Vec2d::<f64>::new(cols);
    let mut categories = vec![HashMap::new(); cols];
    let mut parse_results = vec![0.0; rows * cols];

    for i_col in 0..cols {
        let col = input.get_col(i_col).unwrap();
        let mut map = HashMap::new();
        let mut enumeration = 0;

        for (i_row, e) in col.into_iter().enumerate() {
            if let Ok(x) = e.parse::<f64>() {
                parse_results[i_col * cols + i_row] = x;
            } else if !map.contains_key(e) {
                map.insert(e, enumeration);
                enumeration += 1;
            }
        }

        categories[i_col] = map;
    }

    for i_row in 0..rows {
        let row = input.get_row(i_row).unwrap();
        let parsed_row: Vec<f64> = row
            .iter()
            .enumerate()
            .map(|(i_col, e)| {
                if let Some(enumeration) = categories[i_col].get(e) {
                    (*enumeration).into()
                } else {
                    parse_results[i_col * cols + i_row]
                }
            })
            .collect();
        vec2d_f64.push_slice(&parsed_row);
    }

    vec2d_f64
}
