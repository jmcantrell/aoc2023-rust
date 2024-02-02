use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

impl Range {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn size(&self) -> usize {
        self.end - self.start + 1
    }

    pub fn contains(&self, coord: usize) -> bool {
        self.start <= coord && coord <= self.end
    }

    pub fn iter(&self) -> std::ops::RangeInclusive<usize> {
        self.start..=self.end
    }

    pub fn update(&mut self, value: usize) {
        if value < self.start {
            self.start = value;
        }
        if value > self.end {
            self.end = value;
        }
    }
}

type Tuple = (usize, usize);

impl From<Range> for Tuple {
    fn from(Range { start, end }: Range) -> Self {
        (start, end)
    }
}

impl From<Tuple> for Range {
    fn from((start, end): Tuple) -> Self {
        Self::new(start, end)
    }
}

impl From<usize> for Range {
    fn from(value: usize) -> Self {
        Self::new(value, value)
    }
}

impl Add<usize> for Range {
    type Output = Self;

    fn add(mut self, n: usize) -> Self {
        self.start += n;
        self.end += n;
        self
    }
}

impl AddAssign<usize> for Range {
    fn add_assign(&mut self, n: usize) {
        *self = *self + n;
    }
}

impl Sub<usize> for Range {
    type Output = Self;

    fn sub(mut self, n: usize) -> Self {
        self.start -= n;
        self.end -= n;
        self
    }
}

impl SubAssign<usize> for Range {
    fn sub_assign(&mut self, n: usize) {
        *self = *self - n;
    }
}
