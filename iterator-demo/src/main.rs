fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];

    // Filter even numbers
    let evens: Vec<i32> = numbers
        .iter()
        .filter(|n| *n % 2 == 0)
        .cloned()
        .collect();

    println!("Even numbers: {:?}", evens);

    // Square all numbers
    let squares: Vec<i32> = numbers
        .iter()
        .map(|n| n * n)
        .collect();

    println!("Squares: {:?}", squares);

    // Sum using iterator
    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum);

    // Chained iterator example
    let processed: Vec<i32> = numbers
        .iter()
        .filter(|n| *n > 2)
        .map(|n| n * 10)
        .collect();

    println!("Filtered >2 then *10: {:?}", processed);
}
