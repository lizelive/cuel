use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub enum SpecialInteger {
    Biggest,
    Smallest,
}

pub enum Round {
    NearestTiesToEven,
    TowardPositive,
    TowardNegative,
    TowardZero,
    TowardsInfinity,
    NearestTiesToAway,
}

pub trait Number: Sized + Add + Sub + Mul + Div + PartialEq + PartialOrd {
    fn real_sign(self) -> i64;

    fn possible_zero_q(self) -> bool;
    fn copy_sign(self, from: Self) -> Self;
    fn abs(self) -> Self;

    fn real_q(self) -> bool {
        true
    }
    
    fn number_q(self) -> bool;
    fn exact_number_q(self) -> bool;
    fn integer_q(self) -> bool;
    fn round(self, kind: Round) -> Self;

    fn machine_real_number(self) -> f64;
    fn inexact_number_q(self) -> bool;
    fn machine_number_q(self) -> bool;
    fn precision(self) -> f64;
    fn real_exponent(self) -> f64;

    fn positive(self) -> bool;
    fn negative(self) -> bool;

    fn re(self) -> Self;
    fn im(self) -> Self;
    //fn integer_exponent(&self) -> AnyNumber;
    //fn numerator_denominator(&self, rhs: AnyNumber) -> AnyNumber;
}

impl Number for f64 {
    fn real_sign(self) -> i64 {
        todo!()
    }

    fn possible_zero_q(self) -> bool {
        todo!()
    }

    fn copy_sign(self, from: Self) -> Self {
        todo!()
    }

    fn abs(self) -> Self {
        todo!()
    }

    fn real_q(self) -> bool {
        todo!()
    }

    fn number_q(self) -> bool {
        todo!()
    }

    fn exact_number_q(self) -> bool {
        todo!()
    }

    fn integer_q(self) -> bool {
        todo!()
    }

    fn round(self, kind: Round) -> Self {
        match kind {
            Round::NearestTiesToEven => todo!(),
            Round::TowardPositive => self.ceil(),
            Round::TowardNegative => self.floor(),
            Round::TowardZero => self.trunc(),
            Round::TowardsInfinity => todo!(),
            Round::NearestTiesToAway => todo!(),
        }
    }

    fn machine_real_number(self) -> f64 {
        todo!()
    }

    fn inexact_number_q(self) -> bool {
        todo!()
    }

    fn machine_number_q(self) -> bool {
        todo!()
    }

    fn precision(self) -> f64 {
        todo!()
    }

    fn real_exponent(self) -> f64 {
        todo!()
    }

    fn re(self) -> Self {
        todo!()
    }

    fn im(self) -> Self {
        todo!()
    }
}

impl Number for i64 {
    fn real_sign(self) -> i64 {
        self.signum()
    }

    fn copy_sign(self, from: Self) -> Self {
        self.signum()
    }

    fn abs(self) -> Self {
        self.abs()
    }

    fn real_q(self) -> bool {
        true
    }

    fn exact_number_q(self) -> bool {
        true
    }

    fn integer_q(self) -> bool {
        true
    }

    fn round(self, kind: Round) -> Self {
        match kind {}
    }

    fn machine_real_number(self) -> f64 {
        self as f64
    }

    fn inexact_number_q(self) -> bool {
        false
    }

    fn machine_number_q(self) -> bool {
        true
    }

    fn precision(self) -> f64 {
        f64::INFINITY
    }

    fn real_exponent(self) -> f64 {
        self.abs().machine_real_number().log10()
    }

    fn number_q(self) -> bool {
        true
    }

    fn re(self) -> Self {
        self
    }

    fn im(self) -> Self {
        0
    }

    fn possible_zero_q(self) -> bool {
        self == 0
    }
}

const BIGGEST_INTEGER: SpecialInteger = SpecialInteger::Biggest;

macro_rules! magic_integer_impl {
    ($($t:ty)*) => ($(
        impl PartialOrd<SpecialInteger> for $t {
            fn partial_cmp(&self, other: &SpecialInteger) -> Option<std::cmp::Ordering> {
                Some(std::cmp::Ordering::Less)
            }
        }

        impl PartialEq<SpecialInteger> for $t {
            fn eq(&self, other: &SpecialInteger) -> bool {
                false
            }
        }

        impl PartialOrd<$t> for SpecialInteger {
            fn partial_cmp(&self, other: &$t) -> Option<std::cmp::Ordering> {
                Some(std::cmp::Ordering::Greater)
            }
        }

        impl PartialEq<$t> for SpecialInteger {
            fn eq(&self, other: &$t) -> bool {
                false
            }
        }

    )*)
}

impl Add<isize> for SpecialInteger {
    type Output = SpecialInteger;

    fn add(self, rhs: isize) -> Self::Output {
        self
    }
}

magic_integer_impl! { bool char usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

fn fun() {
    1.partial_cmp(&BIGGEST_INTEGER);

    1.cmp(1);
}
