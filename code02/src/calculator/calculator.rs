use std::{error, env};

pub fn app_c2_calculator ()
{
    println!("Calculator App!\n");
    let n1 = 23;
    let n2 = 5;
    let opt = Some('x'); //;operator + - * /
    // new_calculator(n1, n2, opt);
    calculator_cli();
}

fn new_calculator (n1: i32, n2: i32, opt: Option<char>) -> Result<i32, Box<dyn error::Error>> {
    if opt == Some('+') {
        let result = n1 + n2;
        match opt {
            Some(ok) => {
                println!("{n1} {ok} {n2} = {}", result);
                Ok(result)
            },
            _ => panic!("Not Operator Yet!"),
        }
    } else if opt == Some('-') {
        let result = n1 - n2;
        match opt {
            Some(ok) => {
                println!("{n1} {ok} {n2} = {}", result);
                Ok(result)
            },
            None  => panic!("Not Operator Yet!"),
        }
    } else if opt == Some('x') {
        let result = n1 * n2;
        match opt {
            Some(ok) => {
                println!("{n1} {ok} {n2} = {}", result);
                Ok(result)
            },
            None  => panic!("Not Operator Yet!"),
        }
    } else if opt == Some('/') {
        let result = n1 / n2;
        match opt {
            Some(ok) => {
                println!("{n1} {ok} {n2} = {}", result);
                Ok(result)
            },
            None  => panic!("Not Operator Yet!"),
        }
    } else {
        match opt {
            Some(ok) => {
                eprint!("Operator Input Error {}! ", ok)
            },
            _ => eprint!("Not Operator Yet!")
        }
        panic!("Calculator Error!")
    }
}

use std::env::args;

fn calculator_cli () -> Result<(), Box<dyn error::Error>> {
    let mut args: Vec<_> = env::args().skip(0).collect();
    let num_1 = args.iter().nth(1).unwrap().parse::<f32>().expect("Input Your Number!");
    let num_2 = args.iter().nth(3).unwrap().parse::<f32>().expect("Input Your Number!");
    let opt = args.iter().nth(2).unwrap();
    let parse = opt.parse::<char>().expect("Input Your Operator!");
    match (num_1, num_2, parse) {
        (n1, n2, opt) => {
            if opt == '+' {
                let result = n1 + n2;
                println!("{n1} {opt} {n2} = {result}");
                match Some(result) {
                    Some(ok) => {
                        if ok == 0. {
                            println!("You Ok!");
                        } else if ok <= 100. {
                            println!("Good!");
                        } else if ok >= 1000. {
                            println!("Cool!");
                        } else {
                            println!("No Bad!");
                        }
                    },
                    None => panic!()
                };
            } else if opt == '-' {
                let result = n1 - n2;
                println!("{n1} {opt} {n2} = {result}");
            } else if opt == 'x' {
                let result = n1 * n2;
                println!("{n1} {opt} {n2} = {result}");
            } else if opt == '/' {
                let result = n1 / n2;
                println!("{n1} {opt} {n2} = {result}");
            } else {
                println!("Not Operator!");
            }
        },
        _ => panic!("Calculator Error!")
    };
    println!("\nGood by!");
    Ok(())
}
