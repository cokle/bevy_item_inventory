use super::*;
use std::ops::*;

pub(super) fn plugin(_app: &mut App) {}

// 物品数量
#[derive(
    Component, Debug, Default, Deref, DerefMut, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Reflect,
)]
#[reflect(Component)]
pub struct ItemAmount(pub u32);

impl From<u32> for ItemAmount {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

// 实现+、+=、-、-=、*、*=运算符
impl Add for ItemAmount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for ItemAmount {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for ItemAmount {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for ItemAmount {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Mul for ItemAmount {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl MulAssign for ItemAmount {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}
