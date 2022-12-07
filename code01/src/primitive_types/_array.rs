// ;[T; N] array
pub fn ex_array () {
    let mut data = [2, 3, 10];
    let mut deref = &mut data[2];
    let result = *deref * 10;
    println!("{result:?}");
}

fn some_array_own (some: &[u8], key: u8) {
}
