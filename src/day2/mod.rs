use std::cmp::min;

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

    pub fn get_multiplications_dp(&self) -> u32 {
        let total = self.size.len() + 1;
        let mut dp: Vec<Vec<u32>> = vec![vec![]];

        //Selecting same matrix twice is cost 0;
        for i in 1..self.size.len() {
            let mut row = dp.get_mut(i).unwrap();
            row.insert(i, 0);
        }

        //Subsequence length
        for len in 2..self.size.len() + 1 {
            for i in 0..self.size.len() - len + 1 {
                let j = len + i - 1; //Spl
                let mut best = None;

                for k in i..j {
                    let mul = (self.size.get((i - 1) as usize).unwrap() * self.size.get(k as usize).unwrap() * self.size.get(j as usize).unwrap()) as u32;
                    let cost = dp.get(i).unwrap().get(k).unwrap() + dp.get(k + 1).unwrap().get(j).unwrap() + mul;

                    best = match best {
                        Some(b) => Some(min(cost, b)),
                        None => Some(cost)
                    }
                }
                let mut row = dp.get_mut(i).unwrap();
                row.insert(j, best.unwrap());
            }
        }
        dp.get(self.size.len()).unwrap().get(self.size.len()).unwrap().clone()
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

    #[test]
    pub fn test_multiplications_dp() {
        let mm = MatrixMultiplication::new(vec![1, 2, 3, 4, 3]);
        let cnt = mm.get_min_multiplications();

        assert_eq!(cnt, 30);
    }
}