use std::ops::Range;

pub fn overflow_with_range(range: &Range<i32>, value: i32) -> i32 {
    let len = range.end - range.start;
    let mut v = (value - range.start) % len;
    if v < 0 {
        v += len;
    }
    range.start + v
}