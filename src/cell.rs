/// Container type for caches with a one-element limit
///
/// `CacheCell` is a utility type that holds up to one value at
/// a time, but which can be used to store arbitrarily many distinct
/// values (of the same, statically known type) over the course of its lifetime.
///
/// A CacheCell can be optionally set to one of two modes, 'transient' or
/// 'persistent', which control how 'sticky' values are once stored.
/// In principle, a 'persistent' `CacheCell` will only forget
/// a value it holds when explicitly requested to, and will otherwise safeguard
/// its current value against being discarded or overwritten by standard operations
/// (though some operations may specifically flaunt this intention).
///
/// In contrast, 'transient' `CacheCell`s will discard values with
/// little to no resistance, possibly even if the specific operation being performed
/// may not deterministically require it to do so.
///
/// A newly-created `CacheCell` is initially in an indeterminate 'ambiguous' mode,
/// but can be set to either concrete mode even before it holds a value; further,
/// a 'transient' `CacheCell` can become 'persistent' and vice versa, but it is not
/// possible to revert to the 'ambiguous' mode again.
///
/// Note that some methods may
/// not respect the intended semantics of transient versus persistent, and it may be
/// the caller's responsibility to check the state before attempting certain mutative
/// operations on a CacheCell.
#[derive(Debug, Clone)]
pub struct CacheCell<T> {
    value: Option<T>,
    pub(crate) keep_alive: Option<bool>,
}

impl<T> std::default::Default for CacheCell<T> {
    /// Returns a [CacheCell<T>] that holds no value and has an indeterminate mode
    ///
    /// # Examples
    ///
    /// ```
    /// # use cache_register::cell::CacheCell;
    /// let cache = CacheCell::default();
    /// assert!(cache.is_empty());
    /// assert!(cache.is_ambiguous());
    /// ```
    #[inline(always)]
    fn default() -> Self {
        Self { value: None, keep_alive: None }
    }
}

impl<T> CacheCell<T> {
    /// Returns a [CacheCell<T>] that holds no value and has an indeterminate mode.
    pub const fn new() -> Self {
        Self { value: None, keep_alive: None }
    }

    /// Sets a mutably borrowed [CacheCell<T>] to persistent-mode,
    /// regardless of its previous mode.
    #[inline(always)]
    pub fn set_persistent(&mut self) {
        let _ = self.keep_alive.replace(true);
    }


    /// Sets a mutably borrowed [CacheCell<T>] to transient-mode,
    /// regardless of its previous mode.
    #[inline(always)]
    pub fn set_transient(&mut self) {
        let _ = self.keep_alive.replace(false);
    }

    /// Returns true if `self` has been set to persistent-mode, and false otherwise
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn is_persistent(&self) -> bool {
        matches!(self.keep_alive, Some(true))
    }

    /// Returns true if `self` has been set to transient-mode, and false otherwise
    #[inline(always)]
    pub const fn is_transient(&self) -> bool {
        matches!(self.keep_alive, Some(false))
    }

    /// Returns true if the mode of `self` is unchanged from initialization
    #[allow(dead_code)]
    #[inline(always)]
    pub const fn is_ambiguous(&self) -> bool {
        self.keep_alive.is_none()
    }

    /// Returns true if `self` holds no value.
    #[inline(always)]
    #[allow(dead_code)]
    pub const fn is_empty(&self) -> bool {
        self.value.is_none()
    }

    #[inline(always)]
    #[must_use]
    #[allow(dead_code)]
    /// Returns true if `self` holds a value.
    pub const fn has_value(&self) -> bool {
        self.value.is_some()
    }

    /// Returns an immutable reference to the interior value of a [CacheCell],
    /// wrapped in [Some] when the cache is non-empty, and [None] if it is empty.
    #[inline(always)]
    pub const fn get(&self) -> Option<&T> {
        self.value.as_ref()
    }

    /// Replaces the contents of a [CacheCell<T>] with the specified
    /// value, returning the previous contents as an [Option<T>].
    #[inline(always)]
    pub fn swap(&mut self, replacement: T) -> Option<T> {
        self.value.replace(replacement)
    }

    /// Discards the value held by a [CacheCell<T>] (whether or not it is empty),
    /// unless it is set to persistent-mode.
    ///
    /// Returns a sentinel value of [None] if the value was not discarded (on account
    /// of the CacheCell being marked as persistent), and otherwise returns [Some(())]
    /// for unmarked and transient [CacheCell]s.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut cache = CacheCell::new();
    /// cache.set_persistent();
    /// assert_eq!(cache.checked_clear(), None);
    /// ```
    ///
    #[inline]
    #[allow(dead_code)]
    pub fn checked_clear(&mut self) -> Option<()> {
        if self.is_persistent() {
            None
        } else {
            let _ = self.value.take();
            Some(())
        }
    }

    /// Discards any value held by a [CacheCell<T>], ignoring its mode.
    ///
    /// # Safety
    ///
    /// This method is marked as unsafe only to ensure that it is called
    /// in a disciplined fashion, as it violates the contract of persistent-mode
    /// [CacheCells]. It is not inherently unsafe in any other respect
    /// (i.e. it will not produce undefined behavior, nor panic).
    #[inline(always)]
    #[allow(dead_code)]
    pub unsafe fn unchecked_clear(&mut self) {
        let _ = self.value.take();
    }

    /// Converts a [CacheCell<T>] into a [CacheCell<U>] for `U: From<T>`,
    /// preserving the mode and mapping the cached value as appropriate.
    #[allow(dead_code)]
    #[inline]
    pub fn migrate<U>(self) -> CacheCell<U> where U: From<T> {
        let value = match self.value {
            Some(x) => Some(x.into()),
            None => None
        };
        CacheCell { value, keep_alive: self.keep_alive }
    }

    pub fn keep_alive(&self) -> Option<bool> {
        self.keep_alive
    }
}

impl<T> From<CacheCell<T>> for Option<T> {
    fn from(value: CacheCell<T>) -> Self {
        value.value
    }
}

impl<T> From<T> for CacheCell<T> {
    fn from(value: T) -> Self {
        Self { value: Some(value), keep_alive: None }
    }
}