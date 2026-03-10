mod bufio {
    use std::io;
    use std::fmt::Debug;
    use std::str::FromStr;
    use std::io::BufReader;
    use std::io::prelude::*;

    pub type StdinScanner = Scanner<io::StdinLock<'static>>;

    pub struct Scanner<R: Sized> {
        reader: BufReader<R>,
        buf: String,
        p: usize,
    }

    impl<R: Read> Scanner<R> {
        pub fn new(source: R) -> Scanner<R> {
            let reader = BufReader::new(source);
            let buf = String::new();
            let p = 0_usize;
            Scanner { reader, buf, p }
        }
        fn ensure_buf(&mut self) {
            if self.p == self.buf.len() {
                self.buf.clear();
                self.reader.read_line(&mut self.buf).unwrap();
                self.p = 0;
            }
        }
        pub fn next_line(&mut self) -> String {
            self.ensure_buf();
            let s = &self.buf[self.p..];
            self.p = self.buf.len();
            s.trim_end_matches(|s| s == '\n' || s == '\r').to_owned()
        }
        pub fn next<T: FromStr>(&mut self) -> T where <T as FromStr>::Err: Debug {
            self.ensure_buf();
            let s = self.buf[self.p..].trim_start();
            let wsp = s.find(char::is_whitespace).unwrap();
            self.p += wsp + 1;
            s[..wsp].parse::<T>().unwrap()
        }
        pub fn next_str(&mut self) -> String {
            self.ensure_buf();
            let s = self.buf[self.p..].trim_start();
            let wsp = s.find(char::is_whitespace).unwrap();
            self.p += wsp + 1;
            s[..wsp].to_owned()
        }
        pub fn next_vec<T: FromStr>(&mut self) -> Vec<T> where <T as FromStr>::Err: Debug {
            self.ensure_buf();
            let s = self.buf[self.p..].trim_start();
            self.p = self.buf.len();
            s.split_whitespace().map(|s| s.parse::<T>().unwrap()).collect()
        }
        pub fn next_vec_with_len<T: FromStr>(&mut self, len: usize) -> Vec<T> where <T as FromStr>::Err: Debug {
            (0..len).map(|_| self.next::<T>()).collect()
        }
    }

    pub fn print_vec<T: std::fmt::Display>(val: &Vec<T>) {
        for i in val {
            print!("{} ", i);
        }
    }
}