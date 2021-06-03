mod math;

fn main() {
    println!("sum(10): {}", math::sum(10));
    println!("triangle(10): {}", math::triangle(10));
    let p = math::formula::irreducible_pythagoras_numbers(3, 1);
    match p {
        Ok(p) => println!("irreducible_pythagoras_numbers(3, 1) = ({})", p),
        Err(e) => println!("{}", e),
    }
}
