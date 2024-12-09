fn main() {
    println!("{:?}", longer_wish(&"   s1", &"s2ab"));
    println!("{:?}", longer_wish(&"   s1abc    ", &"  s2a       "));
    println!("{:?}", longer_wish(&"s1", &"s2"));
}

fn longer_wish<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    // had to use hint to pass unicode test
    // before: s1.trim().len()
    if s1.trim().chars().count() > s2.trim().chars().count() {
        return Some(s1);
    }
    if s1.trim().chars().count() == s2.trim().chars().count() {
        return None;
    }
    return Some(s2);
}
