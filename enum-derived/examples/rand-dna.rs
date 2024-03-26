use std::collections::HashSet;

use enum_derived::Rand;

#[derive(Rand)]
pub struct Hello(
    #[custom_rand(simple_rand)] u8,
    #[custom_rand_any(is_rand)] bool,
);

fn simple_rand(_rng: &mut impl rand::Rng) -> u8 {
    0
}

fn is_rand(_rng: &mut impl rand::Rng, usr: &dyn std::any::Any) -> bool {
    let _usr = usr.downcast_ref::<u64>().unwrap();
    false
}

fn main() {
    let mut seen_values = HashSet::new();
    for _ in 0..10000 {
        let r = Hello::rand_deprecated();
        assert_eq!(r.1, false);
        seen_values.insert(r.0);
    }
    assert_eq!(seen_values.len(), u8::MAX as usize + 1);
}
