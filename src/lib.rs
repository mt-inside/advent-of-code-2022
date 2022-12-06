pub mod four;
pub mod three;
pub mod two;

use std::collections::HashSet;
use std::hash::Hash;

pub fn lift_m2<T, U, V, E, F: Fn(T, U) -> V>(
    f: F,
    a: Result<T, E>,
    b: Result<U, E>,
) -> Result<V, E> {
    a.and_then(|a| b.map(|b| f(a, b)))
}

pub fn intersections<T>(sets: &[HashSet<T>]) -> HashSet<T>
where
    T: Clone + Eq + Hash,
{
    match sets.len() {
        0 => HashSet::new(),
        _ => sets[1..].iter().fold(sets[0].clone(), |mut acc, set| {
            acc.retain(|item| set.contains(item));
            acc
        }),
    }
}
