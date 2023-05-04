#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(f: &[T], s: &[T]) -> Comparison {
    fn find_subslice<T: PartialEq>(f: &[T], s: &[T]) -> bool {
        f.is_empty() || s.windows(f.len()).any(|v| f == v)
    }

    if f.len() == s.len() && find_subslice(f, s) {
        Comparison::Equal
    } else if f.len() < s.len() && find_subslice(f, s) {
        Comparison::Sublist
    } else if s.len() < f.len() && find_subslice(s, f) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}
