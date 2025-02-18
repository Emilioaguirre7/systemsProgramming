fn clone_and_modify(s: &String) -> String {
    let mut cloned = s.clone(); 
    cloned.push_str("World!"); 
    cloned 
}

fn main() {
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); 
    println!("Modified: {}", modified); 
}
