use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

/// Helper struct to be able to compare type `T` using an arbitrary comparison function (see:
/// [Comparator]).
///
/// Construct an instance using [IntoComparableBy::into_comparable_by].
///
/// *NB:* `ComparableBy` implements the [`Deref<T>`] and [`DerefMut<T>`] traits, so you can use it
/// as if it were a `T`.
///
/// # Examples
///
/// Here's a silly example, where you want to sort even numbers before the odd ones.
///
/// ```
/// use comparable::IntoComparableBy;
/// use std::cmp::Ordering;
///
/// let even_odd_comparator = |a: &u32, b: &u32| match (a % 2, b % 2) {
///     (0, 1) => Ordering::Less,
///     (1, 0) => Ordering::Greater,
///     _ => a.cmp(b),
/// };
/// let mut numbers: Vec<_> = (0u32..6)
///     .map(|n| n.into_comparable_by(&even_odd_comparator))
///     .rev()
///     .collect();
/// numbers.sort();
///
/// assert!(
///     [0u32, 2, 4, 1, 3, 5]
///     .into_iter()
///     .zip(numbers)
///     .all(|(a, b)| a == *b)
/// );
/// ```
///
/// The following will not compile, as both `ComparableBy`s refer to a different comparator
/// function.
///
/// ```compile_fail,E0369
/// use comparable::IntoComparableBy;
///
/// let comparator_a = |a: &usize, b: &usize| a.cmp(b);
/// let comparator_b = |a: &usize, b: &usize| a.cmp(b);
///
/// 1u32.into_comparable_by(&comparator_a) > 2u32.into_comparable_by(&comparator_a);
/// ```
pub struct ComparableBy<'a, T: Eq, C: Comparator<T>> {
    t: T,
    cmp: &'a C,
}

impl<T: Eq, C: Comparator<T>> Deref for ComparableBy<'_, T, C> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.t
    }
}

impl<T: Eq, C: Comparator<T>> DerefMut for ComparableBy<'_, T, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.t
    }
}

impl<'a, T: Eq, C: Comparator<T>> PartialEq for ComparableBy<'a, T, C> {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
    }
}

impl<'a, T: Eq, C: Comparator<T>> Eq for ComparableBy<'a, T, C> {}

impl<'a, T: Eq, C: Comparator<T>> Ord for ComparableBy<'a, T, C> {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.cmp)(&self.t, &other.t)
    }
}

impl<'a, T: Eq, C: Comparator<T>> PartialOrd for ComparableBy<'a, T, C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a, T: Eq + Debug, C: Comparator<T>> Debug for ComparableBy<'a, T, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.t.fmt(f)
    }
}

/// Trait of any function that compares two `T`s.
pub trait Comparator<T>: Fn(&T, &T) -> Ordering {
    fn cmp(&self, a: &T, b: &T) -> Ordering {
        self(a, b)
    }
}

impl<T, F: Fn(&T, &T) -> Ordering> Comparator<T> for F {}

/// Helper trait to provide the [`comparable`][Self::into_comparable_by] function.
pub trait IntoComparableBy<T: Eq> {
    /// Construct an instance of [ComparableBy], which should be compared using `cmp` (a [Comparator]).
    fn into_comparable_by<C: Comparator<T>>(self, cmp: &C) -> ComparableBy<'_, T, C>;
}

impl<T: Eq> IntoComparableBy<T> for T {
    fn into_comparable_by<C: Comparator<T>>(self, cmp: &C) -> ComparableBy<'_, T, C> {
        ComparableBy { t: self, cmp }
    }
}

// pub trait IntoComparableByKey<T: Eq> {
//     fn into_comparable_by_key<Key: Eq + Ord, KeyExtractor: Fn(&T) -> Key, C: Comparator<T>>(
//         self,
//         key_extractor: &KeyExtractor,
//     ) -> ComparableBy<'_, T, C>;
// }
//
// impl<T: Eq> IntoComparableByKey<T> for T {
//     fn into_comparable_by_key<Key: Eq + Ord, KeyExtractor: Fn(&T) -> Key, C: Comparator<T>>(
//         self,
//         key_extractor: &KeyExtractor,
//     ) -> ComparableBy<'_, T, C> {
//         self.into_comparable_by(&|a: &T, b: &T| key_extractor(a).cmp(&key_extractor(b)))
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comparable_deref() {
        let abs_cmp = |a: &i32, b: &i32| a.abs().cmp(&b.abs());
        let minus_two = (-2).into_comparable_by(&abs_cmp);
        let positive_one = 1.into_comparable_by(&abs_cmp);

        assert!(minus_two > positive_one);
        assert_eq!(minus_two.abs(), 2);
    }

    #[test]
    fn test_comparable() {
        let min = |a: &usize, b: &usize| a.cmp(b);
        let max = |a: &usize, b: &usize| a.cmp(b).reverse();

        let a = 1.into_comparable_by(&min);
        let b = 2.into_comparable_by(&min);

        assert!(a < b);

        let a = 1.into_comparable_by(&max);
        let b = 2.into_comparable_by(&max);

        assert!(a > b);
    }
}
