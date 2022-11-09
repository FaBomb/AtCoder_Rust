//------snippet------

//-------------------
macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

macro_rules! stdin {
    () => {{
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).expect("読み込みに失敗しました。");
        s
    }};
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! test {
    ($($input: expr => $output: expr), * $(,) *) => {
        #[test]
        fn solve_test() {
            $(
                assert_eq!(solve($input), $output);
            )*
        }
    };
}

fn solve(src: &str) -> String {
    input! {
        source = src,
        l: usize,    // 問題に合わせて記述
        r: usize
    }
    // 問題を解くコード
    "atcoder"[l-1..r].to_string()
}

test! {
    "3 6" => "code",
    "4 4" => "o",
    "1 7" => "atcoder"
}

fn main() {
    println!("{}", solve(&stdin!()))
}
