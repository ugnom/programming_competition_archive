use std::str::FromStr;

fn get_one<T : FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    return s.trim().parse().ok().unwrap();
}

fn get_vec<T : FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    return s.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
}

fn main() {
    let ss : Vec<String> = get_vec();
    let s : String = ss[0].to_string();
    let t : String = ss[1].to_string();
    let u : String = ss[2].to_string();

    let mut ans : String = s.to_uppercase().chars().nth(0).unwrap().to_string();
    ans.push(t.to_uppercase().chars().nth(0).unwrap());
    ans.push(u.to_uppercase().chars().nth(0).unwrap());

    println!("{}", ans);
}
