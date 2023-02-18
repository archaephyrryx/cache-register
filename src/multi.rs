use std::collections::{LinkedList, VecDeque};

pub mod limit;

/// [VecDeque]-based FIFO cache structure for storing values that may be dropped
/// if enough newer values are added
///
/// This implementation model is optimal for cases when:
///   - The cache has a constant maximum occupancy (ideally, provided during creation)
///   - Values are rarely, if ever, dropped from the middle, or inserted anywhere but the ends
///   - The relative order of elements does not need to change
///   - Constant-time random access is desirable
///   - Contiguous views of the data (e.g. slices) may be required, though not frequently
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VecCache<T> {
    storage: VecDeque<T>,
    limit: limit::OccupancyLimit,
}

impl<T> Default for VecCache<T> where VecDeque<T>: Default {
    #[inline]
    fn default() -> Self {
        Self { storage: VecDeque::new(), limit: limit::OccupancyLimit::Unlimited }
    }
}

impl<T> VecCache<T> {
    /// Creates a new [`VecCache<T>`].
    ///
    /// The returned [`VecCache<T>`] will begin without a concrete occupancy limit,
    /// and adding elements incrementally will incur the exact same amortized reallocation
    /// cost as with [`VecDeque`].
    ///
    /// If there is a good notion of the desired maximum occupancy, and it is believed that
    /// the cache will quickly reach that occupancy after creation,
    /// [`with_limit`] may be a better alternative.
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Self { storage: VecDeque::new(), limit: limit::OccupancyLimit::Unlimited }
    }

    /// Creates a new [`VecCache<T>`] with a fixed upper bound on maximum occupancy
    ///
    /// The occupancy-limit can be increased later, but will incur an amortized reallocation
    /// penalty if the initial limit is subsequently exceeded
    #[must_use]
    #[inline]
    pub fn with_limit(max_occupancy: usize) -> Self {
        Self { storage: VecDeque::with_capacity(max_occupancy), limit: limit::OccupancyLimit::Limited(max_occupancy) }
    }

    /// Returns `true` if the cache is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    /// Returns the current occupancy (total number of elements) of the cache.
    pub fn occupancy(&self) -> usize {
        self.storage.len()
    }

    /// Un-sets the maximum occupancy limit of this [`VecCache<T>`], returning the
    /// old limit.
    #[inline]
    pub fn unset_limit(&mut self) -> Option<usize> {
        self.limit.unset_limit()
    }

    pub fn limit(&self) -> Option<usize> {
        self.limit.get()
    }

    pub fn try_push(&mut self, value: T) -> Result<(), limit::OccupancyError> {
        match self.limit.get() {
            Some(0) => return Err(limit::OccupancyError::ZeroMaxOccupancy),
            Some(lim) => {
                if self.occupancy() + 1 > lim {
                    return Err(limit::OccupancyError::ReachedMaxOccupancy(lim))
                }
            }
            None => (),
        }
        self.storage.push_back(value);
        Ok(())
    }
}


/// [LinkedList]-based FIFO cache structure for storing values that may be dropped
/// if enough newer values are added
///
/// This implementation model may win over [VecCache<T>] if:
///   - The maximum occupancy of the cache is unknown or unbounded
///   - Values are removed or inserted anywhere besides at the ends
///   - Non-constant-time random access is permissible
///   - Contiguous views of the data are frequently required
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct LLCache<T> {
    storage: LinkedList<T>,
    limit: Option<usize>,
}

impl<T> Default for LLCache<T> where LinkedList<T>: Default {
    fn default() -> Self {
        Self { storage: Default::default(), limit: Default::default() }
    }
}

impl<T> LLCache<T> {
    #[must_use]
    #[inline]
    /// Creates an empty [LLCache<T>] with an unrestricted maximum occupancy
    pub fn new() -> Self {
        Self { storage: LinkedList::new(), limit: None }
    }

    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    /// Returns the current occupancy (total number of elements) of the cache.
    pub fn occupancy(&self) -> usize {
        self.storage.len()
    }



}


