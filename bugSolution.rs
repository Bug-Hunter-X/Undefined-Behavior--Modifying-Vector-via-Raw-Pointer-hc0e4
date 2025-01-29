fn main() {
    let mut v = vec![1, 2, 3];
    // Correct way to modify elements
    v[0] = 10; // Or v.iter_mut().next().unwrap() = 10; 
    println!("{:?}", v);
} 