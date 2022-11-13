#![allow(unused)]

pub fn code02() {
    println!("ini folder code02");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(true);
    }
}
