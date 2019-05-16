pub fn hanoi_move(n: i32, left: String, right: String, ax: String) {
    if n != 0 {
        hanoi_move(&n - 1, left.clone(), ax.clone(), right.clone());
        println!("Moving from {} to {}", &left, &right);
        hanoi_move(n - 1, ax, right, left);
    }
}

pub fn hanoi(n: i32) {
    hanoi_move(n, "left".to_owned(), "right".to_owned(), "middle".to_owned());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_hanoi() {
        hanoi(3);
    }
}