pub mod index_shifter;
pub mod segment_permutation_shifter;
pub mod encapsulated_shifter;
use std::rc::Rc;

/// Purpose:
///      To allow for shifting forward-and-backward across elements, incrementing their states individually
///      This would allow for optimizing on situations where states can be skipped immediately without needing to calculate deeper permutations

pub struct ShiftedElement<T> {
    element: Rc<T>,
    index: usize
}

impl<T> ShiftedElement<T> {
    pub fn new(element: Rc<T>, index: usize) -> Self {
        ShiftedElement {
            element: element,
            index: index
        }
    }
}
trait Shifter {
    type T;

    fn try_forward(&mut self) -> bool;
    fn try_backward(&mut self) -> bool;
    fn try_increment(&mut self) -> bool;
    fn get(&self) -> ShiftedElement<Self::T>;
    fn length(&self) -> usize;
}