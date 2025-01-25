#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal;
    }
    if contains(_first_list, _second_list) {
        return Comparison::Superlist;
    }
    if contains(_second_list, _first_list) {
        return Comparison::Sublist;
    }
    Comparison::Unequal
}

/// determine if the _first_list contains the _second_list
fn contains<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    if second_list.is_empty() {
        return true;
    }
    first_list
        .windows(second_list.len())
        .any(|window| window == second_list)
}
