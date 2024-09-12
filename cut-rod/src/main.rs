
fn main() {

    let numbers: Vec<f64> = vec![1., 2., 3., 4., 5.];
    println!("Hello, world!");
    println!("{}", cut_rot(numbers, 23))
}


fn cut_rot(p: Vec<f64>, n: i32) -> f64 {
    if n == 0 {
        return 0.0
    }
    let mut q = f64::INFINITY;  
    for i in p {
        q = q.max(i)
    }
    q
}