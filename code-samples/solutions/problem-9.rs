fn contains<T: PartialEq>(arr: &[T], value: &T) -> bool {
    arr.iter().any(|x| x == value)
}
