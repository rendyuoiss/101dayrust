pub fn ex_slice () 
{
    // [T] ;slice
    let slice = Vec::from("one shot");
    let in_slice = &slice[..];

    let mut mut_slice = vec![1, 2, 3];
    let in_mut_slice = &mut mut_slice[..];
    in_mut_slice[1] = 30;
    in_mut_slice.sort();

    // println!("{:?}", in_mut_slice);

    // &[T] ;shared slice
    let mut numbers = &mut [1];
    for x in numbers.iter() 
    {
        println!("{}", x);
    }

    // &mut [T] ;mutable slice
    for n in numbers.iter_mut() 
    {
        *n -= 1;
        println!("{n}");
    }

}
