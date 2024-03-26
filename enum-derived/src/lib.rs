#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

/// Derive [Rand] for any enum or struct
pub use enum_derived_macro::Rand;

/// Generate a random version of the implementor
pub trait Rand<Usr>: Sized {
    fn rand_deprecated() -> Self {
        unimplemented!()
    }

    fn rand<R: Rng>(usr: &Usr, rng: &mut R) -> Self;
}

impl<S, U> Rand<U> for S
where
    Standard: Distribution<S>,
{
    fn rand<R: Rng>(_usr: &U, rng: &mut R) -> Self {
        rng.gen()
    }
}
