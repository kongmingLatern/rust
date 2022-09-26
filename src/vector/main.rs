fn main() {
    let mut v = vec![100, 32, 58];
    for i in &mut v {
        *i += 50;
    }
    for i in &mut v {
        println!("{}", i);
    }
}