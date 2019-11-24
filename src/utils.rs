use num::{bigint::BigUint, rational::Ratio, traits::FromPrimitive};

type BigURatio = Ratio<BigUint>;

pub fn from_u32(numer: u32, denom: u32) -> BigURatio {
    BigURatio::new(
        BigUint::from_u32(numer).unwrap(),
        BigUint::from_u32(denom).unwrap(),
    )
}

pub static DICE_DATA: &[(&[usize], u32)] = &[
    (&[1, 1, 1, 1], 1),
    (&[1, 2], 2),
    (&[1, 3], 2),
    (&[1, 4], 2),
    (&[1, 5], 2),
    (&[1, 6], 2),
    (&[2, 2, 2, 2], 1),
    (&[2, 3], 2),
    (&[2, 4], 2),
    (&[2, 5], 2),
    (&[2, 6], 2),
    (&[3, 3, 3, 3], 1),
    (&[3, 4], 2),
    (&[3, 5], 2),
    (&[3, 6], 2),
    (&[4, 4, 4, 4], 1),
    (&[4, 5], 2),
    (&[4, 6], 2),
    (&[5, 5, 5, 5], 1),
    (&[5, 6], 2),
    (&[6, 6, 6, 6], 1),
];
