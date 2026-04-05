use core::f64;

use crate::Coordinate;


#[derive(Debug,Default,PartialEq,PartialOrd, Clone, Copy)]
pub struct CoordinateLatLon
{
    pub lat: f64,
    pub lon: f64,
}

impl Coordinate for CoordinateLatLon
{
    fn is_valid(&self) -> bool {
        if self.lat.is_nan() { return false;}
        if self.lon.is_nan() { return false;}
        return true;
    }
    fn set_null(& mut self) {
        self.lat = f64::NAN;
        self.lon = f64::NAN;
    }
    fn get_horizontal(&self) -> f64 {
        self.lon
    }
    fn get_vertical(&self) -> f64 {
        self.lat
    }
     fn translate(&mut self, horizontal:f64, vertical:f64) {
        self.lon += horizontal;
        self.lat += vertical;
    }
}
