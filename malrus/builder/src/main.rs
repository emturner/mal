fn main() {
    let result = include!(concat!(env!("OUT_DIR"), "/.malrus.rs"));

    println!("{}", result);
}
