#[derive(Debug, Clone)]
struct Iter<T> {
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
        Some(Vec::from(self.container.get_row(0)?))
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

    pub fn idx(&self, x: usize, y: usize) -> T {
        self.vec[y * self.dim + x]
    }

    pub fn get_row(&self, idx: usize) -> Option<&[T]> {
        let row_start = idx * self.dim;
        let row_end = row_start + self.dim;
        if row_end > self.vec.len() {
            return None;
        }
        Some(&self.vec[row_start..row_end])
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

    pub fn push_slice(&mut self, items: &[T]) {
        for e in items {
            self.vec.push(*e);
        }
    }

    pub fn into_iter(&self) -> Iter<T> {
        Iter::new(self.clone())
    }
}
