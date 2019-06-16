/// Sign of coefficients
///
/// This struct represents the coefficient of number limited to +1, -1 and zero.
/// It supports the multiplication.
///
/// # Examples
/// The multiplication is done as follows:
/// ```
/// use teia::sign::Sign;
///
/// assert_eq!(Sign::positive()*Sign::positive(), Sign::positive());
/// assert_eq!(Sign::positive()*Sign::negative(), Sign::negative());
/// assert_eq!(Sign::negative()*Sign::negative(), Sign::positive());
/// assert_eq!(Sign::positive()*Sign::zero(), Sign::zero());
/// assert_eq!(Sign::negative()*Sign::zero(), Sign::zero());
/// assert_eq!(Sign::zero()*Sign::zero(), Sign::zero());
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sign(i8);

impl Eq for Sign {}

impl Sign {
    /// Creates a +1
    #[inline]
    pub fn positive() -> Sign {
        Sign(1i8)
    }

    /// Creates a -1
    #[inline]
    pub fn negative() -> Sign {
        Sign(-1i8)
    }

    /// Createz a 0
    #[inline]
    pub fn zero() -> Sign {
        Sign(0)
    }

    #[inline]
    pub fn is_positive(&self) -> bool {
        self.0 > 0
    }

    #[inline]
    pub fn is_negative(&self) -> bool {
        self.0 < 0
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    /// Make `self` into `Option<Sign>`
    ///
    /// If `self` is +1 or -1, it is converted into `Some(self)`.
    /// Otherwise it is converted into `None`.
    ///
    /// # Examples
    /// ```
    /// use teia::sign::Sign;
    ///
    /// assert_eq!(Sign::positive().into_option(), Some(Sign::positive()));
    /// assert_eq!(Sign::negative().into_option(), Some(Sign::negative()));
    /// assert_eq!(Sign::zero().into_option(), None);
    /// ```
    #[inline]
    pub fn into_option(self) -> Option<Sign> {
        if self.0 == 0 {
            None
        } else {
            Some(self)
        }
    }
}

impl std::ops::Mul<Sign> for Sign {
    type Output = Sign;

    #[inline]
    fn mul(self, rhs: Sign) -> Self::Output {
        Sign(self.0 * rhs.0)
    }
}

impl std::ops::MulAssign<Sign> for Sign {
    #[inline]
    fn mul_assign(&mut self, rhs: Sign) {
        self.0 *= rhs.0;
    }
}

impl std::fmt::Display for Sign {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
