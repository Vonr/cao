use crate::Input::*;
use std::{
    collections::{hash_map::Entry, HashMap},
    env,
    io::{self, BufRead, BufWriter, Write},
    ops::{Shl, Shr},
    process::exit,
    rc::Rc,
    u128,
};

enum Input {
    Op(Rc<String>),
    Num(f64),
}

impl Clone for Input {
    fn clone(&self) -> Self {
        match self {
            Op(s) => Op(Rc::clone(s)),
            Num(n) => Num(*n),
        }
    }
}

fn main() {
    let istty = atty::is(atty::Stream::Stdin);

    if istty {
        let args = env::args();
        let mut out = Vec::new();
        for arg in args.skip(1) {
            arg.split_ascii_whitespace()
                .for_each(|word| match fast_float::parse(word) {
                    Ok(num) => out.push(Num(num)),
                    Err(_) => out.push(Op(Rc::new(word.to_string()))),
                });
        }
        calc(&mut out, None);
    } else {
        let mut stdin = io::stdin().lock();
        let mut out = Vec::new();
        let mut line = String::new();
        while let Ok(bytes) = stdin.read_line(&mut line) {
            if bytes == 0 {
                break;
            }

            for word in line
                .trim_end_matches(|c| c == '\n' || c == '\r')
                .split_ascii_whitespace()
            {
                match fast_float::parse(word) {
                    Ok(num) => out.push(Num(num)),
                    Err(_) => out.push(Op(Rc::new(word.to_string()))),
                }
            }
            calc(&mut out, None);
            line.clear();
            out.clear();
        }
    }
}

fn calc(args: &mut Vec<Input>, pocket: Option<HashMap<u128, f64>>) {
    let mut stack: Vec<f64> = Vec::with_capacity(8);
    let mut pocket = pocket.unwrap_or_default();
    let mut index = 0;
    let mut size = args.len();

    loop {
        if index >= size {
            break;
        }
        let arg = args.get(index).unwrap();
        match arg {
            Num(num) => stack.push(*num),
            Op(op) => {
                let op = op.to_ascii_lowercase();
                match op.as_ref() {
                    "pi" => stack.push(std::f64::consts::PI),
                    "tau" => stack.push(std::f64::consts::TAU),
                    "e" => stack.push(std::f64::consts::E),
                    "len" => {
                        stack.push(stack.len() as f64);
                    }
                    "fold" => {
                        let op = args.get(index - 1).unwrap().to_owned();
                        let stack_size = stack.len() - 1;
                        let mut after = args.split_off(index - 1);
                        after.remove(1);
                        args.resize_with(args.len() + stack_size + 1, || op.clone());
                        args.append(&mut after);
                        size += stack_size;
                    }
                    "map" => {
                        let op = args.get(index - 1).unwrap().to_owned();
                        let mut new_args: Vec<Input> =
                            Vec::with_capacity(size - index + stack.len() * 2 + 1);
                        let stack_size = stack.len();
                        for n in stack.iter().take(stack_size - 1) {
                            new_args.push(Num(*n));
                            new_args.push(op.clone());
                        }
                        new_args.push(Num(stack.pop().unwrap()));
                        for i in index + 1..size {
                            new_args.push(args.get(i).unwrap().to_owned());
                        }
                        calc(&mut new_args, Some(pocket));
                        return;
                    }
                    "rev" => stack.reverse(),

                    _ => {
                        let a = stack.pop().unwrap();
                        match op.as_ref() {
                            "dup" => {
                                stack.push(a);
                                stack.push(a)
                            }
                            "pop" => {}
                            "take" => {
                                if a.fract() == 0.0 {
                                    let pos = &(a as u128);
                                    if pocket.contains_key(pos) {
                                        stack.push(pocket.get(pos).unwrap().to_owned());
                                        pocket.remove(pos);
                                    } else {
                                        eprintln!("take: pocket is empty");
                                        exit(1);
                                    }
                                } else {
                                    eprintln!("take: index must be integer");
                                    exit(1);
                                }
                            }
                            "abs" => stack.push(a.abs()),
                            "sqrt" => stack.push(a.sqrt()),
                            "sin" => stack.push(a.sin()),
                            "cos" => stack.push(a.cos()),
                            "tan" => stack.push(a.tan()),
                            "ln" => stack.push(a.ln()),
                            "log" => stack.push(a.log10()),
                            "log2" => stack.push(a.log2()),
                            "fac" => {
                                if a.fract() == 0.0 {
                                    stack.push(fac(a as u128) as f64)
                                } else {
                                    eprintln!("fac: input must be integer");
                                    exit(1);
                                }
                            }
                            "fib" => {
                                if a.fract() == 0.0 {
                                    stack.push(fib(a as u128) as f64)
                                } else {
                                    eprintln!("fib: input must be integer");
                                    exit(1);
                                }
                            }
                            "ceil" => stack.push(a.ceil()),
                            "floor" => stack.push(a.floor()),
                            "round" => stack.push(a.round()),
                            "trunc" => stack.push(a.trunc()),
                            "fract" => stack.push(a.fract()),
                            "sign" => stack.push(a.signum()),
                            "asin" => stack.push(a.asin()),
                            "acos" => stack.push(a.acos()),
                            "atan" => stack.push(a.atan()),
                            "sinh" => stack.push(a.sinh()),
                            "cosh" => stack.push(a.cosh()),
                            "tanh" => stack.push(a.tanh()),
                            "asinh" => stack.push(a.asinh()),
                            "acosh" => stack.push(a.acosh()),
                            "atanh" => stack.push(a.atanh()),
                            "exp" => stack.push(a.exp()),
                            "expm1" => stack.push(a.exp_m1()),
                            "not" => {
                                if a.fract() == 0.0 {
                                    stack.push(!(a as i64) as f64);
                                } else {
                                    eprintln!("not: input must be integer");
                                    exit(1);
                                }
                            }
                            "unot" => {
                                if a.fract() == 0.0 {
                                    stack.push(!(a as u64) as f64);
                                } else {
                                    eprintln!("unot: input must be integer");
                                    exit(1);
                                }
                            }
                            "prime" => {
                                if a.fract() == 0.0 {
                                    stack.push(num_prime::nt_funcs::is_prime64(a as u64) as u8 as f64)
                                } else {
                                    eprintln!("prime: input must be integer");
                                    exit(1);
                                }
                            }

                            _ => {
                                let b = stack.pop().unwrap();
                                match op.as_ref() {
                                    "+" => stack.push(a + b),
                                    "-" => stack.push(b - a),
                                    "*" => stack.push(a * b),
                                    "/" => stack.push(b / a),
                                    "^" | "**" => stack.push(b.powf(a)),
                                    "mod" => stack.push(b % a),
                                    "%" => stack.push(b % a),
                                    "logn" => stack.push(b.log2() / a.log2()),
                                    "=" => stack.push((a == b) as u8 as f64),
                                    "!=" => stack.push((a != b) as u8 as f64),
                                    "<" => stack.push((b < a) as u8 as f64),
                                    ">" => stack.push((b > a) as u8 as f64),
                                    "<=" => stack.push((b <= a) as u8 as f64),
                                    ">=" => stack.push((b >= a) as u8 as f64),
                                    "~" | "~=" => stack.push(((a - b).abs() < 1e-6) as u8 as f64),
                                    "store" => {
                                        if a.fract() == 0.0 {
                                            let pos = a as u128;
                                            if let Entry::Vacant(e) = pocket.entry(pos) {
                                                e.insert(b);
                                            } else {
                                                eprintln!("hold: pocket is full at {}", pos);
                                                exit(1);
                                            }
                                        } else {
                                            eprintln!("hold: index must be integer");
                                            exit(1);
                                        }
                                    }
                                    "seq" => {
                                        if a.fract() == 0.0 && b.fract() == 0.0 {
                                            let ai = a as usize;
                                            let bi = b as usize;
                                            let mut curr = bi - 1;
                                            stack.resize_with(
                                                (stack.len() + ai - bi) as usize,
                                                || {
                                                    curr += 1;
                                                    curr as f64
                                                },
                                            );
                                        } else {
                                            eprintln!("seq: indices must be integers");
                                            exit(1);
                                        }
                                    }
                                    "over" => {
                                        stack.push(b);
                                        stack.push(a);
                                        stack.push(b);
                                    }
                                    "swap" => {
                                        stack.push(a);
                                        stack.push(b);
                                    }
                                    "min" => stack.push(a.min(b)),
                                    "max" => stack.push(a.max(b)),
                                    "dp" => {
                                        if a.fract() == 0.0 {
                                            let truncated =
                                                format!("{:.1$}", b, a.floor() as usize);
                                            stack.push(truncated.parse().unwrap())
                                        } else {
                                            eprintln!("dp: scale must be integer");
                                            exit(1);
                                        }
                                    }
                                    "gcd" => {
                                        if a.fract() == 0.0 && b.fract() == 0.0 {
                                            stack.push(gcd(b as u128, a as u128) as f64);
                                        } else {
                                            eprintln!("gcd: inputs must be integers");
                                            exit(1);
                                        }
                                    }
                                    "<<" => {
                                        if a.fract() == 0.0 && b.fract() == 0.0 {
                                            stack.push((b as u64).shl(a as u64) as f64);
                                        } else {
                                            eprintln!("<<: inputs must be integers");
                                            exit(1);
                                        }
                                    }
                                    ">>" => {
                                        if a.fract() == 0.0 && b.fract() == 0.0 {
                                            stack.push((a as u64).shr(b as u64) as f64);
                                        } else {
                                            eprintln!(">>: inputs must be integers");
                                            exit(1);
                                        }
                                    }
                                    "or" => {
                                        if a.fract() == 0.0 && b.fract() == 0.0 {
                                            stack.push((b as i64 | a as i64) as f64);
                                        } else {
                                            eprintln!("or: inputs must be integers");
                                            exit(1);
                                        }
                                    }
                                    "and" => {
                                        if a.fract() == 0.0 && b.fract() == 0.0 {
                                            stack.push((b as i64 & a as i64) as f64);
                                        } else {
                                            eprintln!("and: inputs must be integers");
                                            exit(1);
                                        }
                                    }
                                    "xor" => {
                                        if a.fract() == 0.0 && b.fract() == 0.0 {
                                            stack.push((b as i64 ^ a as i64) as f64);
                                        } else {
                                            eprintln!("xor: inputs must be integers");
                                            exit(1);
                                        }
                                    }

                                    _ => {
                                        eprintln!("{}: invalid operator", op);
                                        exit(1);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        index += 1;
    }
    let stdout = io::stdout().lock();
    let mut writer = BufWriter::new(stdout);
    for num in stack {
        write!(writer, "{} ", num).unwrap();
    }
    writer.write_all(b"\r\n").unwrap();
    writer.flush().unwrap();
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn fac(mut n: u128) -> u128 {
    let mut result = 1;
    while n > 1 {
        result *= n;
        n -= 1;
    }
    result
}

fn fib(n: u128) -> u128 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        (a, b) = (b, a + b);
    }
    a
}
