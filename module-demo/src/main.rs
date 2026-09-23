mod math;
mod utils;

fn main() {
    let sum = math::add(10, 20);
    let sq = math::square(7);

    println!("Sum: {}", sum);
    println!("Square: {}", sq);

    utils::greet::hello("Dave");
}
