use std::fs;

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
}
