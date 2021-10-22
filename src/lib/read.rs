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
    s.clear();
    read_line(s);
    s.parse::<T>().unwrap()
}

pub fn readmore<T: FromStr>(s: &mut String) -> impl '_ + Iterator<Item = T>
where
    <T as FromStr>::Err: Debug,
{
    s.clear();
    read_line(s);
    s.split(' ').map(|r| r.parse::<T>().unwrap())
}

pub fn readvec<T: FromStr>(s: &mut String) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    readmore::<T>(s).collect::<Vec<_>>()
}
