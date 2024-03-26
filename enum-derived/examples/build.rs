use enum_derived::Rand;

#[derive(Rand)]
#[factory(u8)]
pub enum Sample {
    A,
    B,
    C,
}

fn main() {}
