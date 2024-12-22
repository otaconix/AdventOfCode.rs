/// Trait to get all variants from an enum
pub trait EnumVariants<const N: usize>
where
    Self: Sized,
{
    /// Get all variants of the enum this trait is implemented for.
    fn variants() -> [Self; N];
}

pub trait PartitionEnumerated<T> {
    fn partition_enumerated<O: Default + Extend<T>, F: Fn(usize, &T) -> bool>(self, f: F)
        -> (O, O);
}

impl<T, I: Iterator<Item = T> + Sized> PartitionEnumerated<T> for I {
    fn partition_enumerated<O: Default + Extend<T>, F: Fn(usize, &T) -> bool>(
        self,
        f: F,
    ) -> (O, O) {
        let mut left = O::default();
        let mut right = O::default();

        for (i, t) in self.enumerate() {
            if f(i, &t) {
                left.extend(Some(t));
            } else {
                right.extend(Some(t));
            }
        }

        (left, right)
    }
}

#[allow(dead_code)]
trait AoCInspector {
    /// Allows inspecting a value, while also returning it
    ///
    /// Note that you cannot modify the value using this function.
    ///
    /// Contrived example:
    /// ```
    /// use aoc_utils::AoCInspector;
    ///
    /// assert_eq!([1, 2, 3].aoc_inspect(|array| println!("{array:?}")).iter().sum::<usize>(), 6)
    /// ```
    fn aoc_inspect<F>(self, f: F) -> Self
    where
        Self: Sized,
        F: Fn(&Self),
    {
        f(&self);

        self
    }
}

impl<T> AoCInspector for T {}
