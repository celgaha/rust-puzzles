use std::fmt::Debug;
use std::io::stdin;
use std::str::FromStr;

pub fn read_line(s: &mut String) {
    stdin().read_line(s).unwrap();
    if s.ends_with("\r\n") {
        s.pop();
        s.pop();
    } else if s.ends_with("\n") {
        s.pop();
    }
}

pub fn readone<T: FromStr>(s: &mut String) -> T
where
    <T as FromStr>::Err: Debug,
{
    read_line(s);
    let t = s.parse::<T>().unwrap();
    s.clear();
    t
}

#[allow(dead_code)]
pub fn readmore<T: FromStr>(s: &mut String) -> impl '_ + Iterator<Item = T>
where
  <T as FromStr>::Err: Debug,
{
    read_line(s);
    s.split(' ').map(|r| r.parse::<T>().unwrap())
}

#[allow(dead_code)]
pub fn readvec() -> Vec<i32>{
    let mut s = String::new();
    let v = readmore::<i32>(&mut s).collect::<Vec<_>>();
    v
}