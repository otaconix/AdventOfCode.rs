/// Trait to get all variants from an enum
pub trait EnumVariants<const N: usize>
where
    Self: Sized,
{
    /// Get all variants of the enum this trait is implemented for.
    fn variants() -> [Self; N];
}
