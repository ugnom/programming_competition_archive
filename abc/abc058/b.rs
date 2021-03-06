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
    let s : String = get_one();
    let t : String = get_one();

    let ss : Vec<char> = s.chars().collect();
    let tt : Vec<char> = t.chars().collect();

    let mut ans = String::new();
    for i in 0..(ss.len()) {
        ans.push(ss[i]);
        if i < tt.len() {
            ans.push(tt[i]);
        }
    }
    println!("{}", ans);
}
