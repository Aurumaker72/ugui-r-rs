use crate::geo::Point;

trait Measurable {
    fn measure() -> Point;
}
