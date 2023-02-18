use std::hint::unreachable_unchecked;

use std;

#[derive(Clone, Copy, Debug, Hash, Default)]
pub enum OccupancyLimit {
    #[default]
    Unlimited,
    Limited(usize),
}

impl PartialEq<usize> for OccupancyLimit {
    fn eq(&self, other: &usize) -> bool {
        match self {
            Self::Limited(lim) => lim.eq(other),
            Self::Unlimited => false,
        }
    }
}

impl Eq for OccupancyLimit {}

impl PartialEq for OccupancyLimit {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Limited(l0), Self::Limited(r0)) => l0 == r0,
            (Self::Unlimited, Self::Unlimited) => true,
            _ => false,
        }
    }
}

impl PartialOrd<usize> for OccupancyLimit {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Limited(l0), r0) => Some(l0.cmp(r0)),
            (Self::Unlimited, _) => Some(std::cmp::Ordering::Greater)
        }
    }
}

impl PartialOrd for OccupancyLimit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Limited(l0), Self::Limited(r0)) => Some(l0.cmp(r0)),
            (Self::Unlimited, Self::Unlimited) => Some(std::cmp::Ordering::Equal),
            (Self::Limited(_), Self::Unlimited) => Some(std::cmp::Ordering::Less),
            (Self::Unlimited, Self::Limited(_)) => Some(std::cmp::Ordering::Greater),
        }
    }
}

impl Ord for OccupancyLimit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Limited(l0), Self::Limited(r0)) => l0.cmp(r0),
            (Self::Unlimited, Self::Unlimited) => std::cmp::Ordering::Equal,
            (Self::Limited(_), Self::Unlimited) => std::cmp::Ordering::Less,
            (Self::Unlimited, Self::Limited(_)) => std::cmp::Ordering::Greater,
        }
    }
}

impl OccupancyLimit {
    /// Sets a mutably borrowed [`OccupancyLimit`] to [`Unlimited`], returning
    /// the limit that was replaced.
    #[inline]
    pub fn unset_limit(&mut self) -> Option<usize> {
        match self {
            Self::Unlimited => None,
            &mut Self::Limited(val) => {
                *self = Self::Unlimited;
                Some(val)
            },
        }
    }

    /// Returns `true` if the [`OccupancyLimit`] is equal to `0`.
    pub const fn is_zero(&self) -> bool {
        matches!(self, Self::Limited(0))
    }

    #[inline]
    pub const fn get(&self) -> Option<usize> {
        match self {
            Self::Unlimited => None,
            &Self::Limited(val) => Some(val),
        }
    }

    /// Returns the raw internal limit of this [`OccupancyLimit`].
    ///
    /// Unless performance is critical, [`get`] is a safe alternative.
    ///
    /// # Safety
    ///
    /// This function implicitly assumes that it will never be called on any [`OccupancyLimit::Unlimited`]
    /// values, and doing so will result in undefined behavior. At best, this function should be used
    /// after an assertion, within a match case, or in the appropriate branch of an 'if else' block, or
    /// otherwise not at all.
    #[inline]
    #[must_use]
    pub const unsafe fn get_unchecked(&self) -> usize {
        if let &Self::Limited(value) = self {
            value
        } else {
            unreachable_unchecked()
        }
    }

    #[inline]
    pub fn get_mut(&mut self) -> Option<&mut usize> {
        match self {
            OccupancyLimit::Unlimited => None,
            OccupancyLimit::Limited(ref mut value) => Some(value),
        }
    }

    #[inline(always)]
    pub fn set_limit(&mut self, limit: usize) {
        *self = Self::Limited(limit)
    }

    /// Returns a mutable reference to the internal limit of this [`OccupancyLimit`].
    ///
    /// Unless performance is critical, [`get_mut`] or [`get_mut_or`] are preferable,
    /// safe variants of this function.
    ///
    /// # Safety
    ///
    /// This function implicitly assumes that it will never be called on any [`OccupancyLimit::Unlimited`]
    /// values, and doing so will result in undefined behavior. At best, this function should be used
    /// after an assertion, within a match case, or in the appropriate branch of an 'if else' block, or
    /// otherwise not at all.
    pub unsafe fn get_mut_unchecked(&mut self) -> &mut usize {
        if let Self::Limited(ref mut value) = self {
            value
        } else {
            unreachable_unchecked()
        }
    }

    /// Returns a direct mutable reference to the internal limit of this [`OccupancyLimit`],
    /// initializing it to a specified value if it was [`Unlimited`] and otherwise leaving
    /// it alone.
    ///
    /// To instead obtain an [`Option`]-wrapped mutable reference to the limit, without
    /// changing its value if [`Unlimited`], use [`get_mut`] instead.
    ///
    /// # Note
    #[inline]
    pub fn get_mut_or(&mut self, init: usize) -> &mut usize {
        match self {
            OccupancyLimit::Unlimited => self.set_limit(init),
            _ => (),
        }
        unsafe { self.get_mut_unchecked() }
    }


    /// Replaces the internal limit of this [`OccupancyLimit`] with the specified
    /// new value `limit`, returning the previous limit, wrapped in an [`Option`]
    /// in order to encapsulate [`Limited(value)`] and [`Unlimited`].
    ///
    /// If the return value is not needed, it may be sensible to call [`set_limit`] instead
    pub fn replace_limit(&mut self, limit: usize) -> Option<usize> {
        match self {
            Self::Unlimited => {
                *self = Self::Limited(limit);
                None
            },
            &mut Self::Limited(old_value) => {
                *self = Self::Limited(limit);
                Some(old_value)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OccupancyError {
    ZeroMaxOccupancy,
    ReachedMaxOccupancy(usize),
}

impl std::fmt::Display for OccupancyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OccupancyError::ZeroMaxOccupancy => write!(f, "occupancy error: maximum occupancy is currently set to 0"),
            OccupancyError::ReachedMaxOccupancy(max) => write!(f, "occupancy error: cache has reached its maximum occupancy of {}", max),
        }
    }
}

impl std::error::Error for OccupancyError {}
