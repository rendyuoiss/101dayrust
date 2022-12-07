// function
pub fn ex_functions ()
{
    fizzbuss_to(100);
    ex_closures();
}

fn is_dividibly (lhs: i32, rhs: i32) -> bool {
    if lhs == 0 {
        false;
    }
    lhs % rhs == 0
}

fn fizzbuss (n: i32)
{
    if is_dividibly(n, 15) {
        println!("Fizzbuss");
    } else if is_dividibly(n, 3) {
        println!("Fizz");
    } else if is_dividibly(n, 5) {
        println!("buss");
    } else {
        println!("{n}");
    }
}

fn fizzbuss_to (n: i32) 
{
    for n in 1..=n {
        fizzbuss(n);
    }
}

// closures
pub fn ex_closures ()
{
    let print = || println!("my closure");
    print();
}
