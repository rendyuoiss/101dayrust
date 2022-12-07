// control flow

pub fn ex_control_flow () {
    // create variable count
    let mut count = 1;
    // if/else
/*  if count < 0 {
        count += 1;
        //println!("{}", count);
    } else {
        count -= 1;
        //println!("{}", count);
    }

    // loop 
    // nesting and labels
    'outer: loop {
        count += 1;
        //println!("entered outer loop : {}", count);
        'inner: loop {
            count -= 1;
            // println!("entered inner loop : {}", count);
            break 'outer;
        } // this point will never be reached
    } // exited outer loop

    // returning from loops
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };
    println!("{result}");

    // while
    while count < 101 {
        if count % 15 == 0 {
            println!("Fizzbuzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        } else {
            println!("{count}");
        }
        // increment count
        count += 1;
    }
    // for and range
    for n in 1..101 {
        if n % 15 == 0 {
             println!("Fizzbuzz");
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{n}");
        }
    }
*/
    // for and iterator
    let mut name = vec!["bob", "john", "doe"];
    /*for names in name.iter_mut() {
        *names = match names {
            &mut "bob" => "There is a rustacean among us!",
            _ => "Hallo", // println!("hallo {}", names)
        };
    }
    println!("{:?}", name);*/
    
    // match
    let x = Some(String::from("Bob"));
    let y: (Option<i32>, Option<String>, Option<bool>) = (Some(12), None, None);
    let z = &mut [12];
    // single variable
    match x 
    {
        Some(ok) => println!("Yeah. I'm Name is {ok}!"),
        _ => todo!()
    }

    // match tuple
    match y 
    {
        (a, b, _) => todo!()
    }

    // if let
    let num = Some(10);
    let second = (Some(12), Some(10));
    if let Some(i) = num 
    {
        println!("Matched {:?}", i);
    }

    // while let
    let optional = Some(-10);
    while let Some(i) = optional 
    {
        if i < 0 {
            println!("Negative");
            break;
        } else if i > 0 {
            println!("Positive");
            break;
        } else if i == 0 {
            println!("Null");
            break;
        } else {
            todo!()
        }
    }
}
