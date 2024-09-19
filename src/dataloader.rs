use std::fs;

pub struct DataLoader {
    path: String,

    // For larger datasets, data
    // may be streamed directly
    // from disk, instead of being
    // loaded in all at once.
    //
    // This could be done, for
    // example using the `BufReader`
    // type.
    data: String,
    read_ptr: usize,
}

impl DataLoader {
    pub fn new(path: &str) -> std::io::Result<DataLoader> {
        let data = fs::read_to_string(path)?;

        Ok(DataLoader {
            path: path.to_string(),
            data,
            read_ptr: 0,
        })
    }

    pub fn pop_row(&mut self) -> Option<Vec<&str>> {
        let i = &mut self.read_ptr;
        let mut prev_break = *i;

        let mut res = Vec::new();

        loop {
            let c = self.data.as_bytes().get(*i)?;

            match c {
                b'\n' => {
                    res.push(&self.data[prev_break..*i]);
                    *i += 1;
                    break;
                }
                b'\r' => {
                    res.push(&self.data[prev_break..*i]);

                    // Double increment for CRLF line breaks
                    *i += 2;
                    break;
                }
                b',' => {
                    res.push(&self.data[prev_break..*i]);
                    prev_break = *i + 1; // Don't include comma
                }
                _ => (),
            }

            *i += 1;
        }
        Some(res)
    }
}
