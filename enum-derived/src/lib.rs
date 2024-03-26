#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

/// Derive [Rand] for any enum or struct
pub use enum_derived_macro::Rand;

/// Generate a random version of the implementor
pub trait Rand<F>: Sized {
    fn rand_deprecated() -> Self {
        unimplemented!()
    }

    fn rand<R: Rng>(factory: &F, rng: &mut R) -> Self;
}

impl<S, F> Rand<F> for S
where
    Standard: Distribution<S>,
{
    fn rand<R: Rng>(_f: &F, rng: &mut R) -> Self {
        rng.gen()
    }
}
