fn main () {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let a : i32 = s.trim().parse().ok().unwrap();
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).ok();
    let b : i32 = t.trim().parse().ok().unwrap();
    println!("{}", (2 * b - a));
}
