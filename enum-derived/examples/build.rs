use enum_derived::Rand;

#[derive(Rand)]
#[usr(u8)]
pub enum Sample {
    A,
    B,
    C,
}

fn main() {}
