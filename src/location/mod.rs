pub mod point_2d;
pub mod point_3d;

pub trait Location<RHS = Self> {
    type ValueOutput;

    fn manhattan_distance_to(&self, other: &RHS) -> Self::ValueOutput;

    fn distance_to(&self, other: &RHS) -> f64;

    fn add(&self, other: &RHS) -> Self;
}
