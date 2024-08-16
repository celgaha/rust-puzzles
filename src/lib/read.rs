use std::fmt::Debug;
use std::io::{stdin, BufRead};
use std::str::FromStr;

pub fn buf_read_line<B: BufRead>(buf: &mut B, s: &mut String) {
    buf.read_line(s).unwrap();
    if s.ends_with("\r\n") {
        s.pop();
        s.pop();
    } else if s.ends_with("\n") {
        s.pop();
    }
}

pub fn read_line(s: &mut String) {
    buf_read_line(&mut stdin().lock(), s)
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

pub fn readpair<T: FromStr>(s: &mut String) -> (T, T)
where
    <T as FromStr>::Err: Debug,
{
    let mut it = readmore(s);
    (it.next().unwrap(), it.next().unwrap())
}

pub fn readtriple<T: FromStr>(s: &mut String) -> (T, T, T)
where
    <T as FromStr>::Err: Debug,
{
    let mut it = readmore(s);
    (it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
}
