fn main() {
    let s1 = "32".to_string();
    let s2 = Rc::new();
    println!("{:?}", s1.cmp(&s2));
}
