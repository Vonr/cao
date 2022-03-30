use std::{
    env::{self, Args},
    ops::{Shl, Shr},
};

fn parse_args(args: Args) -> Vec<String> {
    let mut out = Vec::new();
    for arg in args {
        for op in arg.split_ascii_whitespace() {
            out.push(op.to_owned());
        }
    }
    out
}

fn main() {
    calc(parse_args(env::args()));
}

fn calc(args: Vec<String>) {
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
                    _ => {
                        let a = stack.pop().unwrap();
                        match op.as_ref() {
                            "dup" => {
                                stack.push(a);
                                stack.push(a)
                            }
                            "abs" => stack.push(a.abs()),
                            "sqrt" => stack.push(a.sqrt()),
                            "sin" => stack.push(a.sin()),
                            "cos" => stack.push(a.cos()),
                            "tan" => stack.push(a.tan()),
                            "ln" => stack.push(a.ln()),
                            "log" => stack.push(a.log10()),
                            "log2" => stack.push(a.log2()),
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
                                    "over" => {
                                        stack.push(a);
                                        stack.push(b);
                                        stack.push(a);
                                    }
                                    "min" => stack.push(a.min(b)),
                                    "max" => stack.push(a.max(b)),
                                    "dp" => {
                                        let truncated = format!("{:.1$}", b, a.floor() as usize);
                                        stack.push(truncated.parse().unwrap())
                                    }
                                    "gcd" => {
                                        if a.fract() == 0.0 && b.fract() == 0.0 {
                                            stack.push(gcd(b as u64, a as u64) as f64);
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
        println!("{}", num);
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
