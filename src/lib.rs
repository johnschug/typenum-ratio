#![no_std]
use core::{
    convert::Infallible,
    marker::PhantomData,
    ops::{Add, Div, Mul, Neg, Sub},
};

use typenum::{
    Abs, AbsVal, Exp, Gcd, Gcf, Integer, NInt, Negate, NonZero, PInt, Pow, Prod, Quot, Sum,
    Unsigned, N1, P1, U1,
};

pub use self::consts::*;

pub trait Rational {
    type Numerator: Integer;
    type Denominator: Unsigned + NonZero;
}

pub struct Ratio<N: Integer, D: Unsigned + NonZero = U1>(Infallible, PhantomData<(N, D)>);

impl<N, D> Rational for Ratio<N, D>
where
    N: Integer,
    D: Unsigned + NonZero,
{
    type Numerator = N;
    type Denominator = D;
}

impl<N, D> NonZero for Ratio<N, D>
where
    N: Integer + NonZero,
    D: Unsigned + NonZero,
{
}

impl<N, D> Abs for Ratio<N, D>
where
    N: Integer + Abs,
    D: Unsigned + NonZero,
    AbsVal<N>: Integer,
{
    type Output = Ratio<AbsVal<N>, D>;
}

impl<N, D> Neg for Ratio<N, D>
where
    N: Integer + Neg,
    D: Unsigned + NonZero,
    Negate<N>: Integer,
{
    type Output = Ratio<Negate<N>, D>;

    fn neg(self) -> Self::Output {
        match self.0 {}
    }
}

pub trait Recip {
    type Output: Rational;
}

pub type Reciprocal<T> = <T as Recip>::Output;

impl<N> Recip for PInt<N>
where N: Unsigned + NonZero
{
    type Output = Ratio<P1, N>;
}

impl<N> Recip for NInt<N>
where N: Unsigned + NonZero
{
    type Output = Ratio<N1, N>;
}

impl<N, D> Recip for Ratio<PInt<N>, D>
where
    N: Unsigned + NonZero,
    D: Unsigned + NonZero,
{
    type Output = Ratio<PInt<D>, N>;
}

impl<N, D> Recip for Ratio<NInt<N>, D>
where
    N: Unsigned + NonZero,
    D: Unsigned + NonZero,
{
    type Output = Ratio<NInt<D>, N>;
}

pub type Lcm<A, B> = Quot<Prod<A, B>, Gcf<A, B>>;

impl<Nl, Dl, Nr, Dr> Add<Ratio<Nr, Dr>> for Ratio<Nl, Dl>
where
    Nl: Integer + Mul<PInt<Quot<Dr, Gcf<Dl, Dr>>>>,
    Dl: Unsigned + NonZero + Gcd<Dr> + Div<Gcf<Dl, Dr>> + Mul<Dr>,
    Nr: Integer + Mul<PInt<Quot<Dl, Gcf<Dl, Dr>>>>,
    Dr: Unsigned + NonZero + Div<Gcf<Dl, Dr>>,
    Lcm<Dl, Dr>: Unsigned + NonZero,
    Prod<Dl, Dr>: Div<Gcf<Dl, Dr>>,
    Quot<Dr, Gcf<Dl, Dr>>: Unsigned + NonZero,
    Quot<Dl, Gcf<Dl, Dr>>: Unsigned + NonZero,
    Prod<Nl, PInt<Quot<Dr, Gcf<Dl, Dr>>>>: Add<Prod<Nr, PInt<Quot<Dl, Gcf<Dl, Dr>>>>>,
    Sum<Prod<Nl, PInt<Quot<Dr, Gcf<Dl, Dr>>>>, Prod<Nr, PInt<Quot<Dl, Gcf<Dl, Dr>>>>>: Integer,
{
    type Output = Ratio<
        Sum<Prod<Nl, PInt<Quot<Dr, Gcf<Dl, Dr>>>>, Prod<Nr, PInt<Quot<Dl, Gcf<Dl, Dr>>>>>,
        Lcm<Dl, Dr>,
    >;

    fn add(self, _: Ratio<Nr, Dr>) -> Self::Output {
        match self.0 {}
    }
}

impl<Nl, Dl, Nr, Dr> Sub<Ratio<Nr, Dr>> for Ratio<Nl, Dl>
where
    Nl: Integer,
    Dl: Unsigned + NonZero,
    Nr: Integer + Neg,
    Dr: Unsigned + NonZero,
    Negate<Nr>: Integer,
    Self: Add<Ratio<Negate<Nr>, Dr>>,
{
    type Output = Sum<Self, Ratio<Negate<Nr>, Dr>>;

    fn sub(self, _: Ratio<Nr, Dr>) -> Self::Output {
        match self.0 {}
    }
}

impl<Nl, Dl, Nr, Dr> Mul<Ratio<Nr, Dr>> for Ratio<Nl, Dl>
where
    Nl: Integer + Mul<Nr>,
    Dl: Unsigned + NonZero + Mul<Dr>,
    Nr: Integer,
    Dr: Unsigned + NonZero,
    Prod<Nl, Nr>: Integer,
    Prod<Dl, Dr>: Unsigned + NonZero,
{
    type Output = Ratio<Prod<Nl, Nr>, Prod<Dl, Dr>>;

    fn mul(self, _: Ratio<Nr, Dr>) -> Self::Output {
        match self.0 {}
    }
}

impl<Nl, Dl, Nr, Dr> Div<Ratio<Nr, Dr>> for Ratio<Nl, Dl>
where
    Nl: Integer,
    Dl: Unsigned + NonZero,
    Nr: Integer + NonZero,
    Dr: Unsigned + NonZero,
    Ratio<Nr, Dr>: Recip,
    Self: Mul<Reciprocal<Ratio<Nr, Dr>>>,
{
    type Output = Prod<Self, Reciprocal<Ratio<Nr, Dr>>>;

    fn div(self, _: Ratio<Nr, Dr>) -> Self::Output {
        match self.0 {}
    }
}

impl<N, D, E> Pow<E> for Ratio<N, D>
where
    N: Integer + Pow<E>,
    D: Unsigned + NonZero + Pow<E>,
    Exp<N, E>: Integer,
    Exp<D, E>: Unsigned + NonZero,
{
    type Output = Ratio<Exp<N, E>, Exp<D, E>>;

    fn powi(self, _: E) -> Self::Output {
        match self.0 {}
    }
}

pub trait Simplify {
    type Output: Rational;
}

impl<F, N, D> Simplify for Ratio<N, D>
where
    N: Integer + Div<PInt<F>> + Gcd<PInt<D>, Output = PInt<F>>,
    D: Unsigned + NonZero + Div<F>,
    F: Unsigned + NonZero,
    Quot<N, PInt<F>>: Integer,
    Quot<D, F>: Unsigned + NonZero,
{
    type Output = Ratio<Quot<N, PInt<F>>, Quot<D, F>>;
}

pub type Simplified<T> = <T as Simplify>::Output;

// impl<Nl, Dl, Nr, Dr> Cmp<Ratio<Nr, Dr>> for Ratio<Nl, Dl>
// where
//     Nl: Integer + Mul<PInt<Quot<Dr, Gcf<Dl, Dr>>>>,
//     Dl: Unsigned + NonZero + Gcd<Dr> + Div<Gcf<Dl, Dr>>,
//     Nr: Integer + Mul<PInt<Quot<Dl, Gcf<Dl, Dr>>>>,
//     Dr: Unsigned + NonZero + Div<Gcf<Dl, Dr>>,
//     Prod<Nl, Quot<Dr, Gcf<Dl, Dr>>>: Cmp<Prod<Nr, Quot<Dl, Gcf<Dl, Dr>>>>,
// {
//     type Output = Compare<Prod<Nl, Quot<Dr, Gcf<Dl, Dr>>>, Prod<Nr, Quot<Dl, Gcf<Dl, Dr>>>>;
// }
//

pub mod consts {
    use crate::Ratio;
    use typenum::consts::*;

    pub type R0 = Ratio<Z0, U1>;
    pub type R1 = Ratio<P1, U1>;
    pub type R2 = Ratio<P2, U1>;
    pub type R3 = Ratio<P3, U1>;
    pub type R4 = Ratio<P4, U1>;
    pub type R5 = Ratio<P5, U1>;
    pub type R6 = Ratio<P6, U1>;
    pub type R7 = Ratio<P7, U1>;
    pub type R8 = Ratio<P8, U1>;
    pub type R9 = Ratio<P9, U1>;
    pub type R10 = Ratio<P10, U1>;

    pub type R1_2 = Ratio<P1, U2>;
    pub type R1_3 = Ratio<P1, U3>;
    pub type R1_4 = Ratio<P1, U4>;
    pub type R1_5 = Ratio<P1, U5>;
    pub type R1_6 = Ratio<P1, U6>;
    pub type R1_7 = Ratio<P1, U7>;
    pub type R1_8 = Ratio<P1, U8>;
    pub type R1_9 = Ratio<P1, U9>;
    pub type R1_10 = Ratio<P1, U10>;
}
