use std::cmp::{max, min, Ordering};

#[cfg(test)]
mod test;

pub struct SortedRanges {
    ranges: Vec<(i32, i32)>
}

fn compare_ranges(lhs: &(i32, i32), rhs: &(i32, i32)) -> Ordering {
    let min = lhs.0 - 1;
    let max = lhs.1 + 1;

    if (rhs.0 >= min && rhs.0 <= max) || (rhs.1 <= max && rhs.1 >= min) || (rhs.0 < min && rhs.1 > max) {
        return Ordering::Equal;
    }
    if rhs.0 < min {
        return Ordering::Less;
    }
    return Ordering::Greater;
}

impl SortedRanges {
    pub fn new() -> Self {
        SortedRanges { ranges: Vec::new() }
    }

    pub fn insert(&mut self, new_range: &(i32, i32)) -> bool {
        match self.binary_search(new_range) {
            Ok (index) => {
                let mut range_min: i32 = min(new_range.0, self.ranges[index].0);
                let mut range_max: i32 = max(new_range.1, self.ranges[index].1);
                let mut lowest_removed_index = index;
                self.ranges.remove(index);

                if index != 0 {
                    for low_index in (index - 1)..0 {
                        match compare_ranges(&(self.ranges[low_index]), new_range) {
                            Ordering::Equal => {
                                self.ranges.remove(low_index);
                                lowest_removed_index = low_index;
                                range_min = min(range_min, self.ranges[low_index].0);
                            }
                            Ordering::Less => break,
                            _ => continue
                        }
                    }
                }

                if !self.ranges.is_empty() && index != self.ranges.len() - 1 {
                    for high_index in (index + 1)..(self.ranges.len() - 1) {
                        match compare_ranges(&(self.ranges[high_index]), new_range) {
                            Ordering::Equal => {
                                self.ranges.remove(high_index);
                                range_max = min(range_max, self.ranges[high_index].0);
                            }
                            Ordering::Greater => break,
                            _ => continue,
                        }
                    }
                }

                self.ranges.insert(lowest_removed_index, (range_min, range_max));
                return true;
            }
            Err (index) => {
                self.ranges.insert(index, *new_range);
                return false;
            }
        }
    }

    pub fn binary_search(&mut self, comparison_range: &(i32, i32)) -> Result<usize, usize> {
        return self.ranges.binary_search_by(|existing_range| compare_ranges(comparison_range, existing_range));
    }

    pub fn drain <R> (&mut self, range: R) -> std::vec::Drain <(i32, i32)> where R : std::ops::RangeBounds <usize> {
        return self.ranges.drain(range);
    }
}
