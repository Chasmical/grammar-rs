use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct DeclensionFlags: u8 {
        const STAR = 1 << 0;
        const CIRCLE = 1 << 1;
        const CIRCLED_ONE = 1 << 2;
        const CIRCLED_TWO = 1 << 3;
        const CIRCLED_THREE = 1 << 4;
        const ALTERNATING_YO = 1 << 5;
    }
}

impl DeclensionFlags {
    pub const fn has_star(self) -> bool {
        self.intersects(Self::STAR)
    }
    pub const fn has_circle(self) -> bool {
        self.intersects(Self::CIRCLE)
    }
    pub const fn has_circled_one(self) -> bool {
        self.intersects(Self::CIRCLED_ONE)
    }
    pub const fn has_circled_two(self) -> bool {
        self.intersects(Self::CIRCLED_TWO)
    }
    pub const fn has_circled_three(self) -> bool {
        self.intersects(Self::CIRCLED_THREE)
    }
    pub const fn has_alternating_yo(self) -> bool {
        self.intersects(Self::ALTERNATING_YO)
    }

    const ALL_LEADING_FLAGS: Self = Self::STAR.union(Self::CIRCLE);
    const ALL_TRAILING_FLAGS: Self = Self::ALL_CIRCLED_DIGITS.union(Self::ALTERNATING_YO);
    const ALL_CIRCLED_DIGITS: Self =
        Self::CIRCLED_ONE.union(Self::CIRCLED_TWO).union(Self::CIRCLED_THREE);

    pub const fn has_any_leading_flags(self) -> bool {
        self.intersects(Self::ALL_LEADING_FLAGS)
    }
    pub const fn has_any_trailing_flags(self) -> bool {
        self.intersects(Self::ALL_TRAILING_FLAGS)
    }
    pub const fn has_any_circled_digits(self) -> bool {
        self.intersects(Self::ALL_CIRCLED_DIGITS)
    }
}
