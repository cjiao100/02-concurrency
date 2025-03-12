use anyhow::Result;
use concurrency::Matrix;

fn main() -> Result<()> {
    // println!("f64 default value: {}", f64::default());

    let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
    let b = Matrix::new(vec![1, 2, 3, 4, 5, 6], 3, 2);

    let c = a * b;
    println!("c = {}", c);
    Ok(())
}
