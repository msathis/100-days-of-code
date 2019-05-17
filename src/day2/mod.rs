pub struct MatrixMultiplication {
    size: Vec<u8>
}

impl MatrixMultiplication {
    pub fn new(size: Vec<u8>) -> Self {
        MatrixMultiplication {
            size
        }
    }

    pub fn get_min_multiplications(&self) -> u32 {
        self.get_min_multi(1, (self.size.len() - 1) as u8)
    }

    pub fn get_min_multi(&self, i: u8, j: u8) -> u32 {
        if i == j {
            return 0;
        }

        let mut min = None;
        for k in i..j {
            let count = self.get_min_multi(i, k)
                + self.get_min_multi(k + 1, j)
                + (self.size.get((i - 1) as usize).unwrap() * self.size.get(k as usize).unwrap() * self.size.get(j as usize).unwrap()) as u32;
            let n = match min {
                Some(m) => std::cmp::min(m, count),
                None => count
            };
            min = Some(n);
        }
        min.unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_multiplications() {
        let mm = MatrixMultiplication::new(vec![1, 2, 3, 4, 3]);
        let cnt = mm.get_min_multiplications();

        assert_eq!(cnt, 30);
    }
}