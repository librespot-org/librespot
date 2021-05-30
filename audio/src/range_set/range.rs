use std::{
    cmp::{self, max, min},
    convert::{TryFrom, TryInto},
    fmt,
};

/// A struct that represents a range between two values. The lower boundary is closed, and the upper
/// boundary is open. This means that a `Range` from 2 to 5 includes 2, 3 and 4, but not 5.
/// `std::ops::Range` is not used since it is not `Copy`, which uglifies the resulting code
/// significantly. For more info, see [this](https://github.com/rust-lang/rust/issues/18045).
///
/// ### Invariants
/// For a range type to be valid, the end must be greater than or equal to the start. For this
/// reason the following code panics:
/// ```ignore
/// let r: Range = (2..0).into();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Range {
    start: usize,
    len: usize,
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.start, self.end() - 1)
    }
}

impl TryFrom<std::ops::Range<usize>> for Range {
    type Error = &'static str;

    fn try_from(r: std::ops::Range<usize>) -> Result<Self, Self::Error> {
        if r.start <= r.end {
            Ok(Self {
                start: r.start,
                len: r.end - r.start,
            })
        } else {
            Err("`start` must be less than or equal to `end`")
        }
    }
}

impl Range {
    /// Construct a new `Range` from a given `start` and `len`.
    pub const fn new(start: usize, len: usize) -> Range {
        Self { start, len }
    }

    /// Returns the start of the range.
    pub const fn start(&self) -> usize {
        self.start
    }

    /// Returns the end of the range.
    pub const fn end(&self) -> usize {
        self.start + self.len
    }

    /// Returns the length of the range.
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the range contains no values, that is, the start is equal to the end, and
    /// returns `false` otherwise.
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Clears the range, making it contain no values by setting the end of the range equal to the
    /// start of the range.
    pub fn clear(&mut self) {
        self.len = 0;
    }

    /// Sets the length of the range to the provided value.
    pub fn set_len(&mut self, len: usize) {
        self.len = len;
    }

    /// Returns `true` if `val` is inside the range, returns `false` otherwise.
    pub const fn contains(&self, val: usize) -> bool {
        val >= self.start() && val < self.end()
    }

    /// Returns `true` if the range overlaps anywhere with the other provided range, and returns
    /// `false` otherwise.
    pub const fn overlaps(&self, other: Self) -> bool {
        self.start() < other.end() && self.end() > other.start()
    }

    /// Returns `true` if the range touches or overlaps anywhere with the other provided range, and
    /// returns `false` otherwise.
    pub const fn touches(&self, other: Self) -> bool {
        self.start() <= other.end() && self.end() >= other.start()
    }

    /// Returns the union of the two ranges. Since the union of two ranges can only be represented
    /// as a single range when the two ranges touch, it is assumed that this is the case.
    pub fn union(&self, other: Self) -> Self {
        let start = min(self.start(), other.start());
        let end = max(self.end(), other.end());
        // this is safe because `self` and `other` both uphold their respective invariants
        (start..end).try_into().unwrap()
    }

    pub fn intersection(&self, other: Self) -> Option<Self> {
        let start = max(self.start(), other.start());
        let end = min(self.end(), other.end());
        if start <= end {
            (start..end).try_into().ok() // should be Some because we just checked this
        } else {
            None
        }
    }
}

impl PartialOrd for Range {
    /// The `Range` type forms a partial order, not a total order, since for overlapping `Range`s
    /// there is no meaningful comparison to be made.
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        if self.end() <= other.start() {
            Some(cmp::Ordering::Less)
        } else if self.start() >= other.end() {
            Some(cmp::Ordering::Greater)
        } else {
            None
        }
    }

    /// Override the `<=` function, because it defaults to using equality, which is not relevant for
    /// ranges.
    fn le(&self, other: &Self) -> bool {
        self.end() <= other.start()
    }

    /// Override the `>=` function, because it defaults to using equality, which is not relevant for
    /// ranges.
    fn ge(&self, other: &Self) -> bool {
        self.start() >= other.end()
    }
}

#[cfg(test)]
mod tests {
    use std::{cmp, convert::TryInto};

    use super::Range;

    fn range1() -> Range {
        (0..2).try_into().unwrap()
    }

    fn range2() -> Range {
        (1..3).try_into().unwrap()
    }

    fn range3() -> Range {
        (2..10).try_into().unwrap()
    }

    fn range4() -> Range {
        (3..5).try_into().unwrap()
    }

    #[test]
    fn test_compare() {
        assert!(!(range1() > range2())); // overlapping ranges should be neither greater nor smaller
        assert!(!(range2() > range1())); // overlapping ranges should be neither greater nor smaller
        assert!(!(range1() > range4()));
        assert!(range1() < range4());
    }

    #[test]
    fn test_new() {
        let new = Range::new(1, 2);
        assert_eq!(range2(), new)
    }

    #[test]
    fn test_start() {
        assert_eq!(range1().start(), 0);
        assert_eq!(range2().start(), 1);
        assert_eq!(range3().start(), 2);
        assert_eq!(range4().start(), 3);
    }

    #[test]
    fn test_end() {
        assert_eq!(range1().end(), 2);
        assert_eq!(range2().end(), 3);
        assert_eq!(range3().end(), 10);
        assert_eq!(range4().end(), 5);
    }

    #[test]
    fn test_len() {
        assert_eq!(range1().len(), 2);
        assert_eq!(range2().len(), 2);
        assert_eq!(range3().len(), 8);
        assert_eq!(range4().len(), 2);
    }

    #[test]
    fn test_is_empty() {
        let range: Range = (10..10).try_into().unwrap();
        assert!(range.is_empty());
    }

    #[test]
    fn test_clear() {
        let mut r1 = range1();
        r1.clear();
        assert_eq!(r1, (0..0).try_into().unwrap());

        let mut r2 = range2();
        r2.clear();
        assert_eq!(r2, (1..1).try_into().unwrap());
    }

    #[test]
    fn test_set_len() {
        let mut r1 = range1();
        r1.set_len(20);
        assert_eq!(r1, (0..20).try_into().unwrap());
    }

    #[test]
    fn test_contains() {
        assert!(!range2().contains(0));
        assert!(range2().contains(1));
        assert!(range2().contains(2));
        assert!(!range2().contains(3));
        assert!(!range2().contains(4));
    }

    #[test]
    fn test_overlaps() {
        assert!(range1().overlaps(range2()));
        assert!(!range1().overlaps(range3()));
        assert!(!range1().overlaps(range4()));

        assert!(range2().overlaps(range1()));
        assert!(range2().overlaps(range3()));
        assert!(!range2().overlaps(range4()));

        assert!(!range3().overlaps(range1()));
        assert!(range3().overlaps(range2()));
        assert!(range3().overlaps(range4()));

        assert!(!range4().overlaps(range1()));
        assert!(!range4().overlaps(range2()));
        assert!(range4().overlaps(range3()));
    }

    #[test]
    fn test_touches() {
        assert!(range1().touches(range2()));
        assert!(range1().touches(range3()));
        assert!(!range1().touches(range4()));

        assert!(range2().touches(range1()));
        assert!(range2().touches(range3()));
        assert!(range2().touches(range4()));

        assert!(range3().touches(range1()));
        assert!(range3().touches(range2()));
        assert!(range3().touches(range4()));

        assert!(!range4().touches(range1()));
        assert!(range4().touches(range2()));
        assert!(range4().touches(range3()));
    }

    #[test]
    fn test_union() {
        assert_eq!(range1().union(range2()), (0..3).try_into().unwrap());
        assert_eq!(range1().union(range4()), (0..5).try_into().unwrap()); // breaking the invariant should not panic
    }

    #[test]
    fn test_intersection() {
        assert_eq!(
            range1().intersection(range2()),
            Some((1..2).try_into().unwrap())
        );
        assert!(range1().intersection(range4()).is_none());
    }

    #[test]
    fn test_ord() {
        assert_eq!(range1().partial_cmp(&range2()), None);
        assert_eq!(range1().partial_cmp(&range3()), Some(cmp::Ordering::Less));
        assert_eq!(range1().partial_cmp(&range4()), Some(cmp::Ordering::Less));

        assert_eq!(range2().partial_cmp(&range1()), None);
        assert_eq!(range2().partial_cmp(&range3()), None);
        assert_eq!(range2().partial_cmp(&range4()), Some(cmp::Ordering::Less));

        assert_eq!(
            range3().partial_cmp(&range1()),
            Some(cmp::Ordering::Greater)
        );
        assert_eq!(range3().partial_cmp(&range2()), None);
        assert_eq!(range3().partial_cmp(&range4()), None);

        assert_eq!(
            range4().partial_cmp(&range1()),
            Some(cmp::Ordering::Greater)
        );
        assert_eq!(
            range4().partial_cmp(&range2()),
            Some(cmp::Ordering::Greater)
        );
        assert_eq!(range4().partial_cmp(&range3()), None);
    }
}
