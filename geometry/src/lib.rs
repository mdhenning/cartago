pub mod geodetic;
pub mod cartesian;

pub trait Coordinate
{ 
    fn get_horizontal(&self) -> f64;
    fn get_vertical(&self) -> f64;
    fn is_valid(&self) -> bool;
    fn set_null(& mut self);
    fn translate(& mut self, horizontal:f64, vertical:f64);
    fn is_null(&self) -> bool;
}

pub trait CoordinateElevation
{
    fn get_elevation(&self) -> f64;
    fn translate_elevation(&mut self, elevation:f64);
}

pub trait CoordinateMeasure
{
    fn get_measure(&self) -> f64; 
    fn translate_measure(&mut self, measure:f64);
}


pub fn equals2d<T: Coordinate>(item: &T, other: &T, tolerance: Option<f64>) -> bool
{
    let t:f64 = tolerance.unwrap_or(1E-20);
    let dh = f64::abs(item.get_horizontal() - other.get_horizontal());
    let dv = f64::abs(item.get_vertical() - other.get_vertical());
    dh < t && dv < t
}

pub fn squared_distance<T: Coordinate>(item: &T, other: &T) -> f64
{
    let dh = item.get_horizontal() - other.get_horizontal();
    let dv = item.get_vertical() - other.get_vertical();
    let h2 = dh*dh;
    let v2 = dv*dv;
    h2 + v2
}

pub fn distance<T: Coordinate>(item: &T, other: &T) -> f64
{
    f64::sqrt( squared_distance(item, other) )
}

#[cfg(test)]
mod tests {

    use crate::cartesian::CoordinateXY;

    use super::*;

    #[test]
    fn test_equals_2d() 
    {
       let a = CoordinateXY{x:10.,y:5.};
       let mut b = a.clone();
       let c= CoordinateXY { x: 10.01, y: 5. };
        assert!(equals2d(&a, &b, None));
        assert!(equals2d(&a,&c,Some(0.5)));
        b.translate(0.01, 0.);
        assert!(equals2d(&c,&b,Some(0.00000001)));
    }

    #[test]
    fn test_distance()
    {

    }
}