#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

/// Derive [Rand] for any enum or struct
pub use enum_derived_macro::Rand;

/// Generate a random version of the implementor
pub trait Rand: Sized {
    fn rand_deprecated() -> Self {
        unimplemented!()
    }

    fn rand<R: Rng>(rng: &mut R, usr: &dyn std::any::Any) -> Self;
}

impl<S> Rand for S
where
    Standard: Distribution<S>,
{
    fn rand<R: Rng>(rng: &mut R, _usr: &dyn std::any::Any) -> Self {
        rng.gen()
    }
}
