use core::f64;

use crate::{Coordinate, CoordinateElevation, CoordinateMeasure};

#[derive(Debug,Default,PartialEq, PartialOrd, Clone, Copy)]
pub struct CoordinateXY
{
    pub x: f64,
    pub y: f64,
}

impl Coordinate for CoordinateXY
{
    fn is_valid(&self) -> bool {
        if self.x.is_nan() {return false;}
        if self.y.is_nan() {return false;}
        return true;
    }

    fn set_null(& mut self) {
        self.x = f64::NAN;
        self.y = f64::NAN;
    }

    fn get_horizontal(&self) -> f64 {
        self.x
    }

    fn get_vertical(&self) -> f64 {
        self.y
    }

    fn translate(&mut self, horizontal:f64, vertical:f64) {
        self.x += horizontal;
        self.y += vertical;
    }

    fn is_null(&self) -> bool {
        self.x.is_nan() || self.y.is_nan()
    }
}

#[derive(Debug,Default,PartialEq, PartialOrd, Clone, Copy)]
pub struct CoordinateXYZ
{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Into<CoordinateXY> for CoordinateXYZ
{
    fn into(self) -> CoordinateXY
    {
        CoordinateXY{x: self.x,y: self.y}
    }
}

impl Coordinate for CoordinateXYZ{
    fn get_horizontal(&self) -> f64 {
        self.x
    }

    fn get_vertical(&self) -> f64 {
        self.y
    }

    fn is_valid(&self) -> bool {
        if self.x.is_nan() {return false;}
        if self.y.is_nan() {return false;}
        if self.z.is_nan() {return false;}
        return true;
    }

    fn set_null(& mut self) {
        self.x = f64::NAN;
        self.y = f64::NAN;
        self.z = f64::NAN;
    }

    fn translate(& mut self, horizontal:f64, vertical:f64) {
        self.x += horizontal;
        self.y += vertical;
    }

    fn is_null(&self) -> bool {
       self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }
}

impl CoordinateElevation for CoordinateXYZ
{
    fn get_elevation(&self) -> f64 {
        self.z
    }

    fn translate_elevation(&mut self, elevation:f64) {
        self.z += elevation;
    }
}

impl CoordinateXYZ{
    pub fn translate_all(&mut self, horizontal:f64, vertical:f64, elevation:f64)
    {
        self.x += horizontal;
        self.y += vertical;
        self.z += elevation;
    }
}

struct CoordinateXYM
{
    pub x: f64,
    pub y: f64,
    pub m: f64,
}

impl Coordinate for CoordinateXYM
{
    fn get_horizontal(&self) -> f64 {
        self.x
    }

    fn get_vertical(&self) -> f64 {
       self.y
    }

    fn is_valid(&self) -> bool {
        if self.x.is_nan() {return false;}
        if self.y.is_nan() {return false;}
        if self.m.is_nan() {return false;}
        return true;
    }

    fn set_null(& mut self) {
        self.x = f64::NAN;
        self.y = f64::NAN;
        self.m = f64::NAN;
    }

    fn translate(& mut self, horizontal:f64, vertical:f64) {
        self.x += horizontal;
        self.y += vertical;
    }

    fn is_null(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.m.is_nan()
    }
}

impl CoordinateMeasure for CoordinateXYM
{
    fn get_measure(&self) -> f64 {
        self.m
    }

    fn translate_measure(&mut self, measure:f64) {
        self.m += measure;
    }
}

impl CoordinateXYM{
    pub fn translate_all(&mut self, horizontal:f64, vertical:f64, measure:f64)
    {
        self.x += horizontal;
        self.y += vertical;
        self.m += measure;
    }
}

struct CoordinateXYZM
{
    pub x:f64,
    pub y:f64,
    pub z:f64,
    pub m:f64,
}

impl Coordinate for CoordinateXYZM
{
    fn get_horizontal(&self) -> f64 {
        todo!()
    }

    fn get_vertical(&self) -> f64 {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }

    fn set_null(& mut self) {
        self.x = f64::NAN;
        self.y = f64::NAN;
        self.z = f64::NAN;
        self.m = f64::NAN;
    }

    fn translate(& mut self, horizontal:f64, vertical:f64) {
        self.x += horizontal;
        self.y += vertical;
    }

    fn is_null(&self) -> bool {
        self.x == f64::NAN || self.y == f64::NAN || self.z == f64::NAN || self.m == f64::NAN
    }
}

impl CoordinateElevation for CoordinateXYZM
{
    fn get_elevation(&self) -> f64 {
        self.z
    }

    fn translate_elevation(&mut self, elevation:f64) {
        self.z += elevation;
    }
}

impl CoordinateMeasure for CoordinateXYZM
{
    fn get_measure(&self) -> f64 {
        self.m
    }

    fn translate_measure(&mut self, measure:f64) {
        self.m += measure;
    }
}

impl CoordinateXYZM{
    pub fn translate_all(&mut self, horizontal:f64, vertical:f64, elevation:f64, measure:f64)
    {
        self.x  += horizontal;
        self.y += vertical;
        self.z += elevation;
        self.m += measure;
    }
}