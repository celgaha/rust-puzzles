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

pub fn read<T: FromStr>(s: &mut String) -> T
where
    <T as FromStr>::Err: Debug,
{
    read_line(s);
    let t = s.parse::<T>().unwrap();
    s.clear();
    t
}
