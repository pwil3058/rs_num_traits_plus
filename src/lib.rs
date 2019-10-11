use num_traits::float::*;

pub trait FloatPlus: Float {
    /// Approximate number of significant digits in base 10.
    const DIGITS: u32;
    /// Machine epsilon value for `FloatPlus`.
    const EPSILON: Self;
    /// Infinity (∞).
    const INFINITY: Self;
    /// Number of significant digits in base 2.
    const MANTISSA_DIGITS: u32;
    /// Largest finite `FloatPlus` value.
    const MAX: Self;
    /// Maximum possible power of 10 exponent.
    const MAX_10_EXP: i32;
    /// Maximum possible power of 2 exponent.
    const MAX_EXP: i32;
    /// Smallest finite `FloatPlus` value.
    const MIN: Self;
    /// Minimum possible normal power of 10 exponent.
    const MIN_10_EXP: i32;
    /// One greater than the minimum possible normal power of 2 exponent.
    const MIN_EXP: i32;
    /// Smallest positive normal `FloatPlus` value.
    const MIN_POSITIVE: Self;
    /// Not a Number (NaN).
    const NAN: Self;
    /// Negative infinity (-∞).
    const NEG_INFINITY: Self;
    /// The radix or base of the internal representation of `FloatPlus`.
    const RADIX: u32;

    /// Mathematical constants
    ///
    /// Euler's number (e)
    const E: Self;
    /// 1/π
    const FRAC_1_PI: Self;
    /// 2/π
    const FRAC_2_PI: Self;
    /// 2/sqrt(π)
    const FRAC_2_SQRT_PI: Self;
    /// 1/sqrt(2)
    const FRAC_1_SQRT_2: Self;
    /// π/2
    const FRAC_PI_2: Self;
    /// π/3
    const FRAC_PI_3: Self;
    /// π/4
    const FRAC_PI_4: Self;
    /// π/6
    const FRAC_PI_6: Self;
    /// π/8
    const FRAC_PI_8: Self;
    /// ln(2)
    const LN_2: Self;
    /// ln(10)
    const LN_10: Self;
    /// log2(e)
    const LOG2_E: Self;
    /// log10(e)
    const LOG10_E: Self;
    /// Archimedes' constant (π)
    const PI: Self;
    /// sqrt(2)
    const SQRT_2: Self;
}

impl FloatPlus for f64 {
    /// Approximate number of significant digits in base 10.
    const DIGITS: u32 = std::f64::DIGITS;
    /// Machine epsilon value for `FloatPlus`.
    const EPSILON: Self = std::f64::EPSILON;
    /// Infinity (∞).
    const INFINITY: Self = std::f64::INFINITY;
    /// Number of significant digits in base 2.
    const MANTISSA_DIGITS: u32 = std::f64::MANTISSA_DIGITS;
    /// Largest finite `FloatPlus` value.
    const MAX: Self = std::f64::MAX;
    /// Maximum possible power of 10 exponent.
    const MAX_10_EXP: i32 = std::f64::MAX_10_EXP;
    /// Maximum possible power of 2 exponent.
    const MAX_EXP: i32 = std::f64::MAX_EXP;
    /// Smallest finite `FloatPlus` value.
    const MIN: Self = std::f64::MIN;
    /// Minimum possible normal power of 10 exponent.
    const MIN_10_EXP: i32 = std::f64::MIN_10_EXP;
    /// One greater than the minimum possible normal power of 2 exponent.
    const MIN_EXP: i32 = std::f64::MIN_EXP;
    /// Smallest positive normal `FloatPlus` value.
    const MIN_POSITIVE: Self = std::f64::MIN_POSITIVE;
    /// Not a Number (NaN).
    const NAN: Self = std::f64::NAN;
    /// Negative infinity (-∞).
    const NEG_INFINITY: Self = std::f64::NEG_INFINITY;
    /// The radix or base of the internal representation of `FloatPlus`.
    const RADIX: u32 = std::f64::RADIX;

    /// Mathematical constants
    ///
    /// Euler's number (e)
    const E: Self = std::f64::consts::E;
    /// 1/π
    const FRAC_1_PI: Self = std::f64::consts::FRAC_1_PI;
    /// 2/π
    const FRAC_2_PI: Self = std::f64::consts::FRAC_2_PI;
    /// 2/sqrt(π)
    const FRAC_2_SQRT_PI: Self = std::f64::consts::FRAC_2_SQRT_PI;
    /// 1/sqrt(2)
    const FRAC_1_SQRT_2: Self = std::f64::consts::FRAC_1_SQRT_2;
    /// π/2
    const FRAC_PI_2: Self = std::f64::consts::FRAC_PI_2;
    /// π/3
    const FRAC_PI_3: Self = std::f64::consts::FRAC_PI_3;
    /// π/4
    const FRAC_PI_4: Self = std::f64::consts::FRAC_PI_4;
    /// π/6
    const FRAC_PI_6: Self = std::f64::consts::FRAC_PI_6;
    /// π/8
    const FRAC_PI_8: Self = std::f64::consts::FRAC_PI_8;
    /// ln(2)
    const LN_2: Self = std::f64::consts::LN_2;
    /// ln(10)
    const LN_10: Self = std::f64::consts::LN_10;
    /// log2(e)
    const LOG2_E: Self = std::f64::consts::LOG2_E;
    /// log10(e)
    const LOG10_E: Self = std::f64::consts::LOG10_E;
    /// Archimedes' constant (π)
    const PI: Self = std::f64::consts::PI;
    /// sqrt(2)
    const SQRT_2: Self = std::f64::consts::SQRT_2;
}

impl FloatPlus for f32 {
    /// Approximate number of significant digits in base 10.
    const DIGITS: u32 = std::f32::DIGITS;
    /// Machine epsilon value for `FloatPlus`.
    const EPSILON: Self = std::f32::EPSILON;
    /// Infinity (∞).
    const INFINITY: Self = std::f32::INFINITY;
    /// Number of significant digits in base 2.
    const MANTISSA_DIGITS: u32 = std::f32::MANTISSA_DIGITS;
    /// Largest finite `FloatPlus` value.
    const MAX: Self = std::f32::MAX;
    /// Maximum possible power of 10 exponent.
    const MAX_10_EXP: i32 = std::f32::MAX_10_EXP;
    /// Maximum possible power of 2 exponent.
    const MAX_EXP: i32 = std::f32::MAX_EXP;
    /// Smallest finite `FloatPlus` value.
    const MIN: Self = std::f32::MIN;
    /// Minimum possible normal power of 10 exponent.
    const MIN_10_EXP: i32 = std::f32::MIN_10_EXP;
    /// One greater than the minimum possible normal power of 2 exponent.
    const MIN_EXP: i32 = std::f32::MIN_EXP;
    /// Smallest positive normal `FloatPlus` value.
    const MIN_POSITIVE: Self = std::f32::MIN_POSITIVE;
    /// Not a Number (NaN).
    const NAN: Self = std::f32::NAN;
    /// Negative infinity (-∞).
    const NEG_INFINITY: Self = std::f32::NEG_INFINITY;
    /// The radix or base of the internal representation of `FloatPlus`.
    const RADIX: u32 = std::f32::RADIX;

    /// Mathematical constants
    ///
    /// Euler's number (e)
    const E: Self = std::f32::consts::E;
    /// 1/π
    const FRAC_1_PI: Self = std::f32::consts::FRAC_1_PI;
    /// 2/π
    const FRAC_2_PI: Self = std::f32::consts::FRAC_2_PI;
    /// 2/sqrt(π)
    const FRAC_2_SQRT_PI: Self = std::f32::consts::FRAC_2_SQRT_PI;
    /// 1/sqrt(2)
    const FRAC_1_SQRT_2: Self = std::f32::consts::FRAC_1_SQRT_2;
    /// π/2
    const FRAC_PI_2: Self = std::f32::consts::FRAC_PI_2;
    /// π/3
    const FRAC_PI_3: Self = std::f32::consts::FRAC_PI_3;
    /// π/4
    const FRAC_PI_4: Self = std::f32::consts::FRAC_PI_4;
    /// π/6
    const FRAC_PI_6: Self = std::f32::consts::FRAC_PI_6;
    /// π/8
    const FRAC_PI_8: Self = std::f32::consts::FRAC_PI_8;
    /// ln(2)
    const LN_2: Self = std::f32::consts::LN_2;
    /// ln(10)
    const LN_10: Self = std::f32::consts::LN_10;
    /// log2(e)
    const LOG2_E: Self = std::f32::consts::LOG2_E;
    /// log10(e)
    const LOG10_E: Self = std::f32::consts::LOG10_E;
    /// Archimedes' constant (π)
    const PI: Self = std::f32::consts::PI;
    /// sqrt(2)
    const SQRT_2: Self = std::f32::consts::SQRT_2;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct WrappedFloat<F: FloatPlus>(F);

    #[test]
    fn f64_works() {
        let _f = WrappedFloat::<f64>(0.0);
    }

    #[test]
    fn f32_works() {
        let _f = WrappedFloat::<f32>(0.0);
    }
}