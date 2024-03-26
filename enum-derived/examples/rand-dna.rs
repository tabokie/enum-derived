use std::collections::HashSet;

use enum_derived::Rand;

struct TestStruct;
impl TestStruct {
    pub fn is_rand(&self, _rng: &mut impl rand::Rng) -> bool {
        false
    }
}

#[derive(Rand)]
#[factory(TestStruct)]
pub struct Hello(
    #[custom_rand(simple_rand)] u8,
    #[custom_rand_member(is_rand)] bool,
);

fn simple_rand(_rng: &mut impl rand::Rng) -> u8 {
    0
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
