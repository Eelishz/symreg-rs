use std::{collections::HashMap, fs};

use crate::vec2d::Vec2d;

#[derive(Debug, Clone)]
pub struct DataLoader {
    // For larger datasets, data
    // may be streamed directly
    // from disk, instead of being
    // loaded in all at once.
    //
    // This could be done, for
    // example using the `BufReader`
    // type.
    data: String,
}

impl DataLoader {
    pub fn new(path: &str) -> std::io::Result<DataLoader> {
        let data = fs::read_to_string(path)?;

        Ok(DataLoader { data })
    }

    fn read_row(&self, read_position: usize) -> Option<(Vec<&str>, usize)> {
        let mut i = read_position;
        let mut prev_break = i;

        let mut res = Vec::new();

        loop {
            let c = self.data.as_bytes().get(i)?;

            match c {
                b'\n' => {
                    res.push(&self.data[prev_break..i]);
                    i += 1;
                    break;
                }
                b'\r' => {
                    res.push(&self.data[prev_break..i]);

                    // Double increment for CRLF line breaks
                    i += 2;
                    break;
                }
                b',' => {
                    res.push(&self.data[prev_break..i]);
                    prev_break = i + 1; // Don't include comma
                }
                _ => (),
            }

            i += 1;
        }
        Some((res, i))
    }

    pub fn columns(&self) -> usize {
        let mut i = 0;
        let mut n_cols = 1;

        loop {
            if let Some(c) = self.data.as_bytes().get(i) {
                match c {
                    b'\n' => {
                        break;
                    }
                    b',' => {
                        n_cols += 1;
                    }
                    _ => (),
                }

                i += 1;
            } else {
                break;
            }
        }
        n_cols
    }

    pub fn vec2d(&self) -> Vec2d<&str> {
        let cols = self.columns();
        let mut res = Vec2d::new(cols);
        let mut read_position = 0;

        while let Some((row, i)) = self.read_row(read_position) {
            res.push_slice(row.as_slice());
            read_position = i;
        }

        res
    }

    pub fn categorize_cols(&self) -> Vec2d<f64> {
        let vec2d_str = self.vec2d();
        let (rows, cols) = vec2d_str.shape();
        let mut vec2d_f64 = Vec2d::<f64>::new(cols);
        let mut categories = vec![HashMap::new(); cols];
        let mut parse_results = vec![0.0; rows * cols];

        for i_col in 0..cols {
            let col = vec2d_str.get_col(i_col).unwrap();
            let mut map = HashMap::new();
            let mut enumeration = 0;

            for (i_row, e) in col.into_iter().enumerate() {
                if let Ok(x) = e.parse::<f64>() {
                    parse_results[i_col * cols + i_row] = x;
                } else if map.insert(e, enumeration).is_none() {
                    enumeration += 1;
                }
            }

            categories[i_col] = map;
        }

        for i_row in 0..rows {
            let row = vec2d_str.get_row(i_row).unwrap();
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
}
