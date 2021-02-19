use std::ops::Div;

use static_assertions::{assert_impl_all, assert_not_impl_any};
use typenum::{consts::*, AbsVal, Diff, GrEq, Negate, NonZero, Prod, Quot, Same, Sum};

use typenum_ratio::{consts::*, Ratio, Rational, Recip, Simplified};

macro_rules! check_ratio {
    ($actual:ty, $expected:ty) => {
        check_ratio!($actual);
        check_ratio!($expected);
        assert_impl_all!(Simplified<$actual>: Same<$expected>);
    };
    ($type:ty) => {
        assert_impl_all!($type: Rational);
        assert_impl_all!(GrEq<<AbsVal<$type> as Rational>::Numerator, Z0>: Same<True>);
        assert_impl_all!(Negate<Negate<$type>>: Same<$type>);
        assert_impl_all!(Sum<$type, R0>: Same<$type>);
        assert_impl_all!(Diff<$type, R0>: Same<$type>);
        assert_not_impl_any!(Prod<$type, R0>: NonZero, Recip);
        assert_impl_all!(Simplified<Prod<$type, R0>>: Same<R0>);
        assert_impl_all!(Prod<$type, R1>: Same<$type>);
        assert_impl_all!(Quot<$type, R1>: Same<$type>);
        assert_not_impl_any!($type: Div<R0>);
    };
}

assert_not_impl_any!(R0: NonZero, Recip);
check_ratio!(R0);
check_ratio!(R1);
check_ratio!(Negate<R1>);
check_ratio!(Ratio<N1, U100>);
check_ratio!(Ratio<P100, U100>);

check_ratio!(Sum<R1_2, R1_2>, R1);
check_ratio!(Prod<R1_2, R1_2>, R1_4);
check_ratio!(Quot<R1_2, R2>, R1_4);
check_ratio!(Sum<R1_2, R1_3>, Ratio<P5, U6>);
check_ratio!(Diff<R1_2, R1_3>, R1_6);
check_ratio!(Sum<R1_4, R1_6>, Ratio<P5, U12>);
