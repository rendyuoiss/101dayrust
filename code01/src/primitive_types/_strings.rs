
// * string slice str
pub fn string_slice () {
    let s = "hi mom!";
    some_literal_string(s);
    some_lifetime_string(s);
}

// string literal
fn some_literal_string (s: &str) {
    s.to_string();
    println!("Literal string : {s}");
}

// lifetime string
fn some_lifetime_string (s: &'static str) {
    s.to_string();
    println!("Lifetime string : {s}");
}

// * wadah dan koleksi
// -- Vec<T> ;vector yang dialokasikan tumpukan yang dapat diubah ukurannya saat runtime
// -- [T; N] ;array sebaris dengan ukuran yang tetap pada waktu kompilasi
// -- [T] ;irisan berukuran dinamis menjadi jenis lain yang bersebelasan,apakah dialokasikan tumpukan atau tidak
// -- &[T] ;irisan bersama
// -- &mut [T] :irisan yang dapat di ubah
// -- <[T]> ;irisan kotak milik

// irisan yang dapat di ubah &mut [T]
pub fn slices_a (s: &mut [i32]) {
    for ss in s {
        *ss += 1;
        println!("{ss}");
    }
}
