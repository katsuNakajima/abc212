#[allow(unused_macros)]
macro_rules! parse_line {
    ( $t:ty ) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            iter.next().unwrap().parse::<$t>().unwrap()
        }
    );

    ( $( $t:ty), +) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            ( $(iter.next().unwrap().parse::<$t>().unwrap()),* )
        }
    );
}

#[allow(unused_macros)]
macro_rules! read_line {
    () => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        line.pop();
        line
    }};
}

#[allow(unused_macros)]
macro_rules! parse_vec {
    ( $t:ty ) => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        let iter = line.split_whitespace();
        iter.map(|v| v.parse::<$t>().unwrap()).collect::<Vec<_>>()
    }};
}

fn main() {
    let mut x_org = parse_line!(u32);
    let x4 = x_org % 10;
    x_org = x_org / 10;
    let x3 = x_org % 10;
    x_org = x_org / 10;
    let x2 = x_org % 10;
    x_org = x_org / 10;
    let x1 = x_org;
    let x = vec![x1, x2, x3, x4];
    let mut ans = "Strong";
    let mut cnt = 0;
    if x1 == x2 && x1 == x3 && x1 == x4 && x2 == x3 && x2 == x4 && x3 == x4 {
        ans = "Weak";
    }
    for i in 0..3 {
        if x[i] + 1 == x[i + 1] || x[i] == 9 && x[i + 1] == 0 {
            cnt += 1;
        }
    }
    if cnt == 3 {
        ans = "Weak";
    }
    println!("{}", ans);
}
