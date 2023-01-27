use std::{
    cmp::{max, min},
    fmt,
    slice::Iter,
};

#[derive(Copy, Clone, Debug)]
pub struct Range {
    pub start: usize,
    pub length: usize,
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.start, self.start + self.length - 1)
    }
}

impl Range {
    pub fn new(start: usize, length: usize) -> Range {
        Range { start, length }
    }

    pub fn end(&self) -> usize {
        self.start + self.length
    }
}

#[derive(Debug, Clone)]
pub struct RangeSet {
    ranges: Vec<Range>,
}

impl fmt::Display for RangeSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        for range in self.ranges.iter() {
            write!(f, "{range}")?;
        }
        write!(f, ")")
    }
}

impl RangeSet {
    pub fn new() -> RangeSet {
        RangeSet {
            ranges: Vec::<Range>::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }

    pub fn len(&self) -> usize {
        self.ranges.iter().map(|r| r.length).sum()
    }

    pub fn get_range(&self, index: usize) -> Range {
        self.ranges[index]
    }

    pub fn iter(&self) -> Iter<'_, Range> {
        self.ranges.iter()
    }

    pub fn contains(&self, value: usize) -> bool {
        for range in self.ranges.iter() {
            if value < range.start {
                return false;
            } else if range.start <= value && value < range.end() {
                return true;
            }
        }
        false
    }

    pub fn contained_length_from_value(&self, value: usize) -> usize {
        for range in self.ranges.iter() {
            if value < range.start {
                return 0;
            } else if range.start <= value && value < range.end() {
                return range.end() - value;
            }
        }
        0
    }

    #[allow(dead_code)]
    pub fn contains_range_set(&self, other: &RangeSet) -> bool {
        for range in other.ranges.iter() {
            if self.contained_length_from_value(range.start) < range.length {
                return false;
            }
        }
        true
    }

    pub fn add_range(&mut self, range: &Range) {
        if range.length == 0 {
            // the interval is empty -> nothing to do.
            return;
        }

        for index in 0..self.ranges.len() {
            // the new range is clear of any ranges we already iterated over.
            if range.end() < self.ranges[index].start {
                // the new range starts after anything we already passed and ends before the next range starts (they don't touch) -> insert it.
                self.ranges.insert(index, *range);
                return;
            } else if range.start <= self.ranges[index].end()
                && self.ranges[index].start <= range.end()
            {
                // the new range overlaps (or touches) the first range. They are to be merged.
                // In addition we might have to merge further ranges in as well.

                let mut new_range = *range;

                while index < self.ranges.len() && self.ranges[index].start <= new_range.end() {
                    let new_end = max(new_range.end(), self.ranges[index].end());
                    new_range.start = min(new_range.start, self.ranges[index].start);
                    new_range.length = new_end - new_range.start;
                    self.ranges.remove(index);
                }

                self.ranges.insert(index, new_range);
                return;
            }
        }

        // the new range is after everything else -> just add it
        self.ranges.push(*range);
    }

    #[allow(dead_code)]
    pub fn add_range_set(&mut self, other: &RangeSet) {
        for range in other.ranges.iter() {
            self.add_range(range);
        }
    }

    #[allow(dead_code)]
    pub fn union(&self, other: &RangeSet) -> RangeSet {
        let mut result = self.clone();
        result.add_range_set(other);
        result
    }

    pub fn subtract_range(&mut self, range: &Range) {
        if range.length == 0 {
            return;
        }

        for index in 0..self.ranges.len() {
            // the ranges we already passed don't overlap with the range to remove

            if range.end() <= self.ranges[index].start {
                // the remaining ranges are past the one to subtract. -> we're done.
                return;
            } else if range.start <= self.ranges[index].start
                && self.ranges[index].start < range.end()
            {
                // the range to subtract started before the current range and reaches into the current range
                // -> we have to remove the beginning of the range or the entire range and do the same for following ranges.

                while index < self.ranges.len() && self.ranges[index].end() <= range.end() {
                    self.ranges.remove(index);
                }

                if index < self.ranges.len() && self.ranges[index].start < range.end() {
                    self.ranges[index].length -= range.end() - self.ranges[index].start;
                    self.ranges[index].start = range.end();
                }

                return;
            } else if range.end() < self.ranges[index].end() {
                // the range to subtract punches a hole into the current range -> we need to create two smaller ranges.

                let first_range = Range {
                    start: self.ranges[index].start,
                    length: range.start - self.ranges[index].start,
                };

                self.ranges[index].length -= range.end() - self.ranges[index].start;
                self.ranges[index].start = range.end();

                self.ranges.insert(index, first_range);

                return;
            } else if range.start < self.ranges[index].end() {
                // the range truncates the existing range -> truncate the range. Let the for loop take care of overlaps with other ranges.
                self.ranges[index].length = range.start - self.ranges[index].start;
            }
        }
    }

    pub fn subtract_range_set(&mut self, other: &RangeSet) {
        for range in other.ranges.iter() {
            self.subtract_range(range);
        }
    }

    pub fn minus(&self, other: &RangeSet) -> RangeSet {
        let mut result = self.clone();
        result.subtract_range_set(other);
        result
    }

    pub fn intersection(&self, other: &RangeSet) -> RangeSet {
        let mut result = RangeSet::new();

        let mut self_index: usize = 0;
        let mut other_index: usize = 0;

        while self_index < self.ranges.len() && other_index < other.ranges.len() {
            if self.ranges[self_index].end() <= other.ranges[other_index].start {
                // skip the interval
                self_index += 1;
            } else if other.ranges[other_index].end() <= self.ranges[self_index].start {
                // skip the interval
                other_index += 1;
            } else {
                // the two intervals overlap. Add the union and advance the index of the one that ends first.
                let new_start = max(
                    self.ranges[self_index].start,
                    other.ranges[other_index].start,
                );
                let new_end = min(
                    self.ranges[self_index].end(),
                    other.ranges[other_index].end(),
                );
                result.add_range(&Range::new(new_start, new_end - new_start));
                if self.ranges[self_index].end() <= other.ranges[other_index].end() {
                    self_index += 1;
                } else {
                    other_index += 1;
                }
            }
        }

        result
    }
}
