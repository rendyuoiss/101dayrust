#![allow(unused)]

pub fn code01() {
    println!("ini folder code01!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(true);
    }
}
