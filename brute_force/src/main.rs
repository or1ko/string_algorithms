fn main() {
    println!("{:?}", index_of("hello", "ello"));
}

fn index_of(target: &'static str, s: &'static str) -> Option<usize> {
    for j in 0..(target.len() - s.len()+1) {
        if target[j..(j+s.len())] == *s {
            return Some(j);
        }
    }
    None
}

#[test]
fn test() {
    assert_eq!(index_of("Hello, world!", "world!"), Some(7));
}