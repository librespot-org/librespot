#![allow(clippy::suspicious_operation_groupings)]

use std::{convert::TryInto, fmt};

mod range;
pub(crate) use range::Range;

// An ordered set of ranges. These *must* be  upheld for the implementation to be correct.
//
// ### Invariants:
// 1. `ranges` is ordered by `range.start()`.
// 2. The ranges do not touch. Upon insertion of a range that touches another, instead, those
// ranges are merged.
// 3. None of the ranges are empty.
#[derive(Clone, Default, Debug, PartialEq)]
pub(crate) struct RangeSet {
    ranges: Vec<Range>,
}

impl fmt::Display for RangeSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        for range in &self.ranges {
            write!(f, "{}", range)?;
        }
        write!(f, ")")
    }
}

/// `RangeSet` can implement `Index`, but not `IndexMut`, since that would allow the user to modify
/// the ranges in the set, which may fail to uphold the invariant around ordering.
impl std::ops::Index<usize> for RangeSet {
    type Output = Range;

    fn index(&self, index: usize) -> &Self::Output {
        &self.ranges[index]
    }
}

impl IntoIterator for RangeSet {
    type Item = Range;

    type IntoIter = <Vec<Range> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.ranges.into_iter()
    }
}

impl From<Vec<Range>> for RangeSet {
    fn from(mut ranges: Vec<Range>) -> Self {
        ranges.sort_unstable_by_key(|r| r.start());
        let mut result: Vec<Range> = Vec::with_capacity(ranges.len()); // asssume worst case
        for range in ranges {
            match result.last_mut() {
                Some(r) if r.touches(range) => *r = r.union(range),
                _ => result.push(range),
            }
        }

        Self { ranges: result }
    }
}

impl RangeSet {
    /// Construct a new `RangeSet`. Equivalent to `RangeSet::default()`.
    pub fn new() -> RangeSet {
        Self::default()
    }

    /// Checks whether this `RangeSet` contains no values.
    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }

    /// Returns the total length of all ranges contained in this `RangeSet`.
    pub fn len(&self) -> usize {
        self.ranges.iter().map(Range::len).sum()
    }

    /// Returns a borrowed iterator over the ranges in this `RangeSet`.
    pub fn iter(&self) -> impl Iterator<Item = &Range> {
        self.ranges.iter()
    }

    /// Returns `true` if the value is contained, returns `false` otherwise.
    pub fn contains(&self, value: usize) -> bool {
        // implemented using a binary search, since that is faster than naively iterating on our
        // sorted data.
        self.ranges
            .binary_search_by(|r| match (r.start() <= value, r.end() > value) {
                (true, true) => std::cmp::Ordering::Equal,
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => unreachable!(),
            })
            .is_ok()
    }

    /// Finds the first `Range` in `self.ranges` which contains `value`, and returns the number of
    /// elements that are left in that `Range`.
    pub fn contained_length_from_value(&self, value: usize) -> usize {
        self.iter()
            .take_while(|r| value >= r.start()) // stop once the ranges no longer contain this value
            .find(|r| r.contains(value)) // find the first range that contains this value
            .map(|r| r.end() - value) // calculate the remaining elements in that range
            .unwrap_or(0) // return zero otherwise
    }

    #[allow(dead_code)]
    pub fn contains_range_set(&self, other: &RangeSet) -> bool {
        other
            .iter()
            .all(|r| self.contained_length_from_value(r.start()) >= r.len())
    }

    pub fn add_range(&mut self, range: Range) {
        if range.is_empty() {
            // the interval is empty -> nothing to do.
            return;
        }

        for index in 0..self.ranges.len() {
            let mut cur = self[index];
            // Note that the new range is clear of any ranges we already iterated over since
            // `self.ranges` is ordered.
            if range < cur {
                // The new range starts after anything we already passed and ends before the next
                // range starts (they don't touch) -> insert it.
                self.ranges.insert(index, range);
                return;
            } else if range.touches(cur) {
                // The new range overlaps (or touches) the first range. They are to be merged.
                // In addition we might have to merge further ranges in as well.

                let mut new_range = range;
                while cur.start() <= new_range.end() {
                    new_range = new_range.union(cur);
                    self.ranges.remove(index);
                    if index >= self.ranges.len() {
                        break;
                    }
                    cur = self[index];
                }

                self.ranges.insert(index, new_range);
                return;
            }
        }

        // the new range is after everything else -> just add it
        self.ranges.push(range);
    }

    #[allow(dead_code)]
    pub fn add_range_set(&mut self, other: &RangeSet) {
        for &range in other.ranges.iter() {
            self.add_range(range);
        }
    }

    /// Returns a new `RangeSet` that contains all values with are contained in either of the
    /// original `RangeSet`s.
    pub fn union(&self, other: &RangeSet) -> RangeSet {
        let mut result = self.clone();
        result.add_range_set(other);
        result
    }

    pub fn subtract_range(&mut self, to_sub: Range) {
        if to_sub.is_empty() {
            return;
        }

        for index in 0..self.ranges.len() {
            let cur = self[index];
            // The ranges we already passed don't overlap with the range to remove
            if to_sub <= cur {
                // the remaining ranges are past the one to subtract. -> we're done.
                return;
            }
            if to_sub.start() <= cur.start() && cur.start() < to_sub.end() {
                // The range to subtract started before the current range and reaches into the
                // current range -> We have to remove the beginning of the range or the entire range
                // and do the same for following ranges.

                while index < self.ranges.len() && self[index].end() <= to_sub.end() {
                    self.ranges.remove(index); // todo: O(n) operation in a loop may be bad?
                }

                if index < self.ranges.len() && self[index].start() < to_sub.end() {
                    // since our while loop ran as long as `self[index].end() <= to_sub.end()`, we
                    // know that `to_sub.end() < self[index].end()`, and therefore the conversion
                    // is safe.
                    self.ranges[index] = (to_sub.end()..self[index].end()).try_into().unwrap();
                }
                return;
            } else if to_sub.end() < cur.end() {
                // The range to subtract punches a hole into the current range. This means we need
                // to create two smaller ranges. It also means we can safely assume that the current
                // range starts before `to_sub`, and that it ends after `to_sub`. Therefore the
                // following two unwraps are safe.
                let first_range = (cur.start()..to_sub.start()).try_into().unwrap();
                self.ranges[index] = (to_sub.end()..cur.end()).try_into().unwrap();
                self.ranges.insert(index, first_range);
                return;
            } else if to_sub.start() < cur.end() {
                // The range truncates the existing range -> Truncate the range. Let the next
                // iteration take care of overlaps with other ranges. This overlap also means that
                // the current range must begin before `to_sub` begins, and therefore that
                // `self[index].start() < to_sub.start()`. Then we can safely construct this range.
                self.ranges[index] = (self[index].start()..to_sub.start()).try_into().unwrap();
            }
        }
    }

    pub fn subtract_range_set(&mut self, other: &RangeSet) {
        for &range in other.ranges.iter() {
            self.subtract_range(range);
        }
    }

    pub fn minus(&self, other: &RangeSet) -> RangeSet {
        let mut result = self.clone();
        result.subtract_range_set(other);
        result
    }

    /// Returns a new `RangeSet` that contains all values with are contained in both original
    /// `RangeSet`s.
    pub fn intersection(&self, other: &Self) -> Self {
        let mut result = RangeSet::new();
        let mut self_index: usize = 0;
        let mut other_index: usize = 0;

        while self_index < self.ranges.len() && other_index < other.ranges.len() {
            let self_range = self[self_index];
            let other_range = other[other_index];
            if self_range <= other_range {
                self_index += 1; // skip the interval
            } else if other_range <= self_range {
                other_index += 1; // skip the interval
            } else {
                // the two intervals overlap. Add the union and advance the index of the one that
                // ends first.
                result.add_range(self_range.union(other_range));
                if self_range.end() <= other_range.end() {
                    self_index += 1;
                } else {
                    other_index += 1;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    fn test_set1() -> RangeSet {
        vec![(0..10).try_into().unwrap(), (20..30).try_into().unwrap()].into()
    }

    fn test_set2() -> RangeSet {
        vec![(0..20).try_into().unwrap(), (20..30).try_into().unwrap()].into()
    }

    fn test_set3() -> RangeSet {
        vec![
            (0..10).try_into().unwrap(),
            (10..20).try_into().unwrap(),
            (20..30).try_into().unwrap(),
        ]
        .into()
    }

    fn test_set4() -> RangeSet {
        vec![(20..30).try_into().unwrap()].into()
    }

    fn is_sorted(rs: &RangeSet) -> bool {
        let first = rs[0];
        rs.ranges[1..]
            .iter()
            .fold((true, first), |(is_sorted, prev), &r| {
                (is_sorted && prev.start() <= r.start(), r)
            })
            .0
    }

    #[test]
    fn test_is_empty() {
        let empty_set: RangeSet = Vec::new().into();
        assert!(empty_set.is_empty());
    }

    #[test]
    fn test_len() {
        assert_eq!(test_set1().len(), 20);
        assert_eq!(test_set2().len(), 30);
        assert_eq!(test_set3().len(), 30);
    }

    #[test]
    fn test_contains() {
        assert!(test_set1().contains(0));
        assert!(test_set1().contains(5));
        assert!(!test_set1().contains(10));

        assert!(test_set2().contains(0));
        assert!(test_set2().contains(20));
        assert!(!test_set2().contains(30));

        assert!(test_set3().contains(0));
        assert!(test_set3().contains(5));
        assert!(test_set3().contains(10));
    }

    #[test]
    fn contained_length_from_value() {
        assert_eq!(test_set1().contained_length_from_value(0), 10);
        assert_eq!(test_set1().contained_length_from_value(10), 0);
        assert_eq!(test_set1().contained_length_from_value(20), 10);

        // todo: should these tests pass?
        assert_eq!(test_set2().contained_length_from_value(0), 30);
        assert_eq!(test_set2().contained_length_from_value(30), 0);

        assert_eq!(test_set3().contained_length_from_value(0), 30);
        assert_eq!(test_set3().contained_length_from_value(10), 20);
        assert_eq!(test_set3().contained_length_from_value(20), 10);
        assert_eq!(test_set3().contained_length_from_value(30), 0);
    }

    #[test]
    fn test_contains_range_set() {
        assert!(test_set1().contains_range_set(&test_set1()));
        assert!(!test_set1().contains_range_set(&test_set2()));
        assert!(!test_set1().contains_range_set(&test_set3()));

        assert!(test_set2().contains_range_set(&test_set1()));
        assert!(test_set2().contains_range_set(&test_set2()));
        assert!(test_set2().contains_range_set(&test_set3()));

        assert!(test_set3().contains_range_set(&test_set1()));
        assert!(test_set3().contains_range_set(&test_set2()));
        assert!(test_set3().contains_range_set(&test_set3()));
    }

    #[test]
    fn test_add() {
        let mut test1 = test_set1();
        let to_add: Range = (0..10).try_into().unwrap();
        test1.add_range(to_add);
        assert_eq!(test_set1(), test1);
        let to_add: Range = (10..20).try_into().unwrap();
        test1.add_range(to_add);
        assert_eq!(test_set2(), test1);
        let to_add: Range = (5..15).try_into().unwrap();
        test1.add_range(to_add);
        assert_eq!(test_set2(), test1);

        assert!(is_sorted(&test1));
    }

    #[test]
    fn test_sub() {
        let mut test1 = test_set1();
        let to_sub: Range = (0..10).try_into().unwrap();
        test1.subtract_range(to_sub);
        assert_eq!(test_set4(), test1);

        let mut test3 = test_set3();
        let to_sub: Range = (10..20).try_into().unwrap();
        test3.subtract_range(to_sub);
        assert_eq!(test_set1(), test3);

        let mut test4: RangeSet = vec![(0..221184).try_into().unwrap()].into();
        let to_sub: Range = (0..69632).try_into().unwrap();
        test4.subtract_range(to_sub);
        let res: RangeSet = vec![(69632..221184).try_into().unwrap()].into();
        assert_eq!(res, test4);
    }
}
