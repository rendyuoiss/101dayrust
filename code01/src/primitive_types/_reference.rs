/*
Reference &T &mut T
*/

pub fn ex_reference () {
    let five = 5;
    let other_five = 5;
    let ref_five = &five;
    let result = ref_five;
    println!("main scope : {}", result);

    {// in scope 1
        let scope_ref = result;
        println!("my scope 1 : {}", scope_ref);

        {// scope 2
            let scope2 = 5;
            let result = scope2 == five;
            println!("my scope 2 : {}", result);
        }
    }
    // value di dalam scope 1 & scope 2 tidak valid setelah di baris ini
    // println!("{}", scope2); => error!
}