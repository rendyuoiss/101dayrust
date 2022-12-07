use std::{error, fs::File};

// panic!()
pub fn ex_panic () {
    panic!("Panic!");
}

// Result <T, E> { Ok(T), Err(E) }
pub fn ex_result () -> Result<(), Box<dyn error::Error>> {
    let path = File::open("test.txt");
    match path {
        Ok(fl) => fl,
        Err(_) => panic!("Not Found File!")
    };
    let some = "Hallo";
    Ok(())
}

fn my_result (t: &str, r: Result<&str, Box<dyn error::Error>>) {
    match r {
        Ok(expr) => {
            t.to_string();
        },
        Err(_) => todo!(),
    }
}
