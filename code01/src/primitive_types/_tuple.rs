// tuple
pub fn ex_tuple () 
{
    let t = (29, 20);
    some_tuple(t);
}

fn some_tuple (t: (i32, i32))
{
    match t {
        (mut a, mut b) => {
            if a >= b {
                a -= 1;
            } else if a <= b {
                b += 1;
            } else if a == b {
                a + b * 2;
            } else {
                todo!();
            }
            println!("{a}");
        },
        _ => todo!()
    }
}
