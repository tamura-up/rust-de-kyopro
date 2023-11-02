/// 多次元の vector を作成します
///
/// Examples
/// ```
/// N*N の vector
/// let mut result = mat![0; N; N];
/// ```
#[macro_export]
macro_rules! mat {
($($e:expr),*) => { Vec::from(vec![$($e),*]) };
($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

#[macro_export]
macro_rules! echo { ($($num:expr),*)=> {
    let mut tmp=vec![];
    $ (tmp.push(format!("{}",$num));) *
    println!("{}",tmp.join(" ")); };
}

#[macro_export]
macro_rules! mint {
    ($num:expr) => {
        Mint::new($num)
    };
}

#[macro_export]
macro_rules! YesNo {
    ($num:expr) => {
        if ($num) as i64 == 0 {
            println!("No");
        } else {
            println!("Yes");
        }
    };
}
#[macro_export]
macro_rules! Yes {
    () => {
        println!("Yes");
    };
}
#[macro_export]
macro_rules! No {
    () => {
        println!("No");
    };
}

pub trait SetMinMax {
    fn setmin(&mut self, v: Self) -> bool;
    fn setmax(&mut self, v: Self) -> bool;
}
impl<T> SetMinMax for T
where
    T: PartialOrd,
{
    fn setmin(&mut self, v: T) -> bool {
        *self > v && {
            *self = v;
            true
        }
    }
    fn setmax(&mut self, v: T) -> bool {
        *self < v && {
            *self = v;
            true
        }
    }
}

pub fn print_vec<T>(v: &[T])
where
    T: std::fmt::Display,
{
    for i in 0..v.len() {
        print!("{}{}", v[i], if i + 1 == v.len() { "" } else { " " });
    }
    println!();
}
