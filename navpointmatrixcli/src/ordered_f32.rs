use std::{cmp::Ordering, ops::Add};

#[derive(Copy, Clone)]
pub struct OrderedF32 {
    value: f32,
}

impl From<f32> for OrderedF32 {
    fn from(value: f32) -> Self {
        Self { value }
    }
}

impl pathfinding::num_traits::Zero for OrderedF32 {
    fn zero() -> Self {
        Self { value: 0.0 }
    }

    fn is_zero(&self) -> bool {
        self.value == 0.0
    }
}

impl Add for OrderedF32 {
    type Output = OrderedF32;

    fn add(self, rhs: Self) -> Self::Output {
        Self { value: self.value + rhs.value }
    }
}

impl Ord for OrderedF32 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.value < other.value {
            Ordering::Less
        } else if self.value > other.value {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for OrderedF32 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for OrderedF32 {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for OrderedF32 {}
