use std::{
    env,
    io::{self, BufRead},
    ops::{Shl, Shr},
    u128,
};

fn parse_args() {
    let istty = atty::is(atty::Stream::Stdin);

    if istty {
        let args = env::args();
        let mut out = Vec::new();
        for arg in args {
            for op in arg.split_ascii_whitespace() {
                out.push(op.to_owned());
            }
        }
        calc(&out);
    } else {
        let stdin = io::stdin().lock();
        let mut out = Vec::new();
        for line in stdin.lines() {
            if let Ok(line) = line {
                out.push("".to_owned());
                for word in line.split_ascii_whitespace() {
                    out.push(word.to_owned());
                }
                calc(&out);
                out.clear();
            }
        }
    }
}

fn main() {
    parse_args();
}

fn calc(args: &Vec<String>) {
    let mut stack: Vec<f64> = Vec::with_capacity(8);

    for arg in args.into_iter().skip(1) {
        match arg.parse() {
            Ok(num) => stack.push(num),
            Err(_) => {
                let op = arg.to_lowercase();
                match op.as_ref() {
                    "pi" => stack.push(std::f64::consts::PI),
                    "tau" => stack.push(std::f64::consts::TAU),
                    "e" => stack.push(std::f64::consts::E),
                    "sum" => {
                        let mut sum = 0.0;
                        for num in stack.drain(..) {
                            sum += num;
                        }
                        stack.push(sum);
                    }
                    "mean" => {
                        let mut sum = 0.0;
                        let length = stack.len() as f64;
                        for num in stack.drain(..) {
                            sum += num;
                        }
                        stack.push(sum / length);
                    }
                    "prod" => {
                        let mut prod = 1.0;
                        for num in stack.drain(..) {
                            prod *= num;
                        }
                        stack.push(prod);
                    }
                    "gmin" => {
                        let mut min = std::f64::MAX;
                        for num in stack.drain(..) {
                            if num < min {
                                min = num;
                            }
                        }
                        stack.push(min);
                    }
                    "gmax" => {
                        let mut max = std::f64::MIN;
                        for num in stack.drain(..) {
                            if num > max {
                                max = num;
                            }
                        }
                        stack.push(max);
                    }
                    "sd" => {
                        let mut sum = 0.0;
                        let mut sum_sq = 0.0;
                        let size = stack.len() as f64;
                        for num in stack.drain(..) {
                            sum += num;
                            sum_sq += num * num;
                        }
                        let mean = sum / size;
                        stack.push((sum_sq / size - mean * mean).sqrt())
                    }
                    "eq" => {
                        let mut eq = true;
                        let mut last = stack.pop().unwrap();
                        for num in stack.drain(..) {
                            eq = eq && num == last;
                            last = num;
                        }
                        stack.push(eq as u8 as f64);
                    }

                    _ => {
                        let a = stack.pop().unwrap();
                        match op.as_ref() {
                            "dup" => {
                                stack.push(a);
                                stack.push(a)
                            }
                            "pop" => {}
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
                                    eprintln!("fac: requires integer operand");
                                }
                            }
                            "fib" => {
                                if a.fract() == 0.0 {
                                    stack.push(fib(a as u128) as f64)
                                } else {
                                    eprintln!("fib: requires integer operand");
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
                                    eprintln!("not: requires integer operand");
                                }
                            }
                            "unot" => {
                                if a.fract() == 0.0 {
                                    stack.push(!(a as u64) as f64);
                                } else {
                                    eprintln!("not: requires integer operand");
                                }
                            }
                            "prime" => {
                                if a.fract() == 0.0 {
                                    stack.push(num_prime::nt_funcs::is_prime64(a as u64) as u8 as f64)
                                } else {
                                    eprintln!("not: requires integer operand");
                                }
                            }

                            _ => {
                                let b = stack.pop().unwrap();
                                match op.as_ref() {
                                    "+" => stack.push(a + b),
                                    "-" => stack.push(b - a),
                                    "*" => stack.push(a * b),
                                    "/" => stack.push(b / a),
                                    "^" => stack.push(b.powf(a)),
                                    "**" => stack.push(b.powf(a)),
                                    "mod" => stack.push(b % a),
                                    "%" => stack.push(b % a),
                                    "=" => stack.push((a == b) as u8 as f64),
                                    "!=" => stack.push((a != b) as u8 as f64),
                                    "<" => stack.push((b < a) as u8 as f64),
                                    ">" => stack.push((b > a) as u8 as f64),
                                    "<=" => stack.push((b <= a) as u8 as f64),
                                    ">=" => stack.push((b >= a) as u8 as f64),
                                    "seq" => {
                                        if a.fract() == 0.0 && b.fract() == 0.0 {
                                            for i in b as u64..a as u64 {
                                                stack.push(i as f64);
                                            }
                                        } else {
                                            eprintln!("seq: requires integer operands");
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
                                        let truncated = format!("{:.1$}", b, a.floor() as usize);
                                        stack.push(truncated.parse().unwrap())
                                    }
                                    "gcd" => {
                                        if a.fract() == 0.0 && b.fract() == 0.0 {
                                            stack.push(gcd(b as u128, a as u128) as f64);
                                        } else {
                                            eprintln!("gcd: requires integer operands");
                                        }
                                    }
                                    "<<" => {
                                        if a.fract() == 0.0 && b.fract() == 0.0 {
                                            stack.push((b as u64).shl(a as u64) as f64);
                                        } else {
                                            eprintln!("<<: requires integer operands");
                                        }
                                    }
                                    ">>" => {
                                        if a.fract() == 0.0 && b.fract() == 0.0 {
                                            stack.push((a as u64).shr(b as u64) as f64);
                                        } else {
                                            eprintln!(">>: requires integer operands");
                                        }
                                    }
                                    "or" => {
                                        if a.fract() == 0.0 && b.fract() == 0.0 {
                                            stack.push((b as i64 | a as i64) as f64);
                                        } else {
                                            eprintln!("or: requires integer operands");
                                        }
                                    }
                                    "and" => {
                                        if a.fract() == 0.0 && b.fract() == 0.0 {
                                            stack.push((b as i64 & a as i64) as f64);
                                        } else {
                                            eprintln!("and: requires integer operands");
                                        }
                                    }
                                    "xor" => {
                                        if a.fract() == 0.0 && b.fract() == 0.0 {
                                            stack.push((b as i64 ^ a as i64) as f64);
                                        } else {
                                            eprintln!("xor: requires integer operands");
                                        }
                                    }

                                    _ => {
                                        eprintln!("{}: invalid operator", arg);
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    for num in stack {
        print!("{} ", num);
    }
    println!();
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
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
        let t = a;
        a = b;
        b += t;
    }
    a
}
