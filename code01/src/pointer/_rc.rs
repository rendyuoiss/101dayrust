use std::rc::Rc;

// smart pointer Rc::<T>
pub fn ex_rc () {
    let mut x = Box::new(20);
    let mut new = Rc::new(x);
    new.as_ref();
    println!("{}", new);
}
