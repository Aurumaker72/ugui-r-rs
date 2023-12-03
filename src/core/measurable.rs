use crate::core::geo::Point;

trait Measurable {
    fn measure() -> Point;
}
