pub mod day1;
pub mod day2;

fn main() {
    println!("Hello, world!");

    let mm = day2::MatrixMultiplication::new(vec![1, 2, 3, 4, 3]);
    println!("{}", mm.get_multiplications_dp());
}
