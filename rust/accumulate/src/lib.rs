pub fn map<A, B>(input: Vec<A>, mut f: impl FnMut(A) -> B) -> Vec<B> {
    let mut result = Vec::with_capacity(input.len());
    for a in input {
        result.push(f(a));
    }
    result
}
