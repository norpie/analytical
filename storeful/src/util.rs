use std::collections::HashSet;

pub fn intersect(a: &mut HashSet<Box<[u8]>>, b: HashSet<Box<[u8]>>) {
    if a.is_empty() {
        a.extend(b);
    } else {
        a.retain(|item| b.contains(item));
    }
}
