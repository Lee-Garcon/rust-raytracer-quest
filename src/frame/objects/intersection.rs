//Not to be confused with assets/intersect.rs, which houses the trait!

use crate::frame::objects::element::Element;

pub struct Intersection<'a> {
    pub distance: f64,
    pub element: &'a Element
}

impl<'a> Intersection<'a> {
    pub fn new<'b>(distance: f64, element: &'b Element) -> Intersection<'b> {
        Intersection{ distance: distance, element: element }
    }
}
