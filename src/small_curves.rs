use ark_ff::fields::{Field};

use crate::small_fields::{F13, F17};

use ark_ec::{
    models::CurveConfig,
    short_weierstrass::{self, *},
};
use ark_ff::MontFp;


// C17: y^2 = x^3 + 2x + 4 over F13, with 17 elements

pub type C17Affine = Affine<Config>;
pub type C17Projective = Projective<Config>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Config;

impl CurveConfig for Config {
    type BaseField = F13;
    type ScalarField = F17;

    /// COFACTOR = 1
    const COFACTOR: &'static [u64] = &[0x0000000000000001];

    /// COFACTOR_INV = COFACTOR^{-1} mod r = 1
    #[rustfmt::skip]
    const COFACTOR_INV: F17 = MontFp!("1");
}

impl short_weierstrass::SWCurveConfig for Config {
    /// COEFF_A = 2
    const COEFF_A: F13 = MontFp!("2");

    /// COEFF_B = 4
    #[rustfmt::skip]
    const COEFF_B: F13 = MontFp!("4");

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const GENERATOR: C17Affine = C17Affine::new_unchecked(G1_GENERATOR_X, G1_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(elem: Self::BaseField) -> Self::BaseField {
        elem.double()
    }
}

/// G1_GENERATOR_X = 7
#[rustfmt::skip]
pub const G1_GENERATOR_X: F13 = MontFp!("7");

/// G1_GENERATOR_Y = 7
#[rustfmt::skip]
pub const G1_GENERATOR_Y: F13 = MontFp!("7");
