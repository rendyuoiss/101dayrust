// ;Box<T> owned slice
pub fn ex_box_pointer () {
    let x = String::from("Smart Pointer");
    let new_box = Box::new(x);
    // let ptr = Box::into_raw(new_box);
    match new_box.get(..) {
        Some(ok) => my_box(ok),
        None => todo!()
    };
}

fn my_box (p: impl Into<String>) -> String {
    p.into()
}
