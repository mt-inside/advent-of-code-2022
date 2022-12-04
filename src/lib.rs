pub mod two;

pub fn lift_m2<T, U, V, E, F: Fn(T, U) -> V>(
    f: F,
    a: Result<T, E>,
    b: Result<U, E>,
) -> Result<V, E> {
    a.and_then(|a| b.map(|b| f(a, b)))
}
