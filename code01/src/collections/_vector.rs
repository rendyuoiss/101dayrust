// ;Vec<T>
pub fn ex_vector () {
    // new vector Vec<T>
    let mut new_vec = vec![1, 2, 3];
    // generic vector Vec<&T>
    let mut other_vec = Vec::new();
    other_vec.push(&new_vec);

    for item in &mut new_vec {
        *item += 1;
        // println!("{}", item);
    }
    let x = vec![1, 3];
    some_vector(x);
}

fn some_vector (v: Vec<i32>) {
    let data = vec![v];
    for (_, k) in data.iter().enumerate() {
        let result = k[0];
        println!("{:?}", result);
    }
}
