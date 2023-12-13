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
