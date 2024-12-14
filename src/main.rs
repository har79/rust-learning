use std::io;

const GREETING: &str = "hello";

fn r_fib(n: u32) -> u32 {
    print!("#{n}: ");
    let ret = if n <= 1 {
        1
    } else {
        print!("(");
        let mut ret = r_fib(n-1);
        print!(" + ");
        ret += r_fib(n-2);
        print!(") => ");
        ret
    };
    print!("{ret}");
    ret
}

fn t_fib(n: u32) -> u32 {
    fn t_fib_(n: u32) -> (u32, u32) {
        let ret = if n == 0 {
            print!("#0,-: ");
            (1, 0)
        } else {
            print!("#{},{}: (", n, n-1);
            let prev = t_fib_(n-1);
            print!(") => ");
            (prev.0 + prev.1, prev.0)
        };
        print!("{},{}", ret.0, ret.1);
        ret
    }

    t_fib_(n).0
}

fn i_fib(n: u32) -> u32 {
    let (mut c, mut p) = (1, 0);
    print!("#0,-: {},{}", c, p);
    for i in 1..=n {
        (c, p) = (c + p, c);
        print!(" => #{},{}: {},{}", i, i-1, c, p);
    }
    c
}

fn main() {
    println!("{GREETING}");

    println!("enter n:");
    
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("failed to read line");
    let n: u32 = n.trim().parse().expect("failed to convert to int");

    print!("recursive fib: ");
    let result = r_fib(n);
    println!(" => {result}");

    print!("rec.tuple fib: ");
    let result = t_fib(n);
    println!(" => {result}");

    print!("iterative fib: ");
    let result = i_fib(n);
    println!(" => {result}");
}
