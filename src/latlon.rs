use h3ron_h3_sys::GeoCoord;

use crate::H3Cell;

#[derive(Debug, Copy, Clone)]
pub struct LatLon {
    pub lat: f64,
    pub lon: f64,
}

impl LatLon {
    pub fn lat_lon_string(&self) -> String {
        let LatLon { lat, lon } = self;
        format!("{lat},{lon}")
    }

    pub fn lon_lat_string(&self) -> String {
        let LatLon { lat, lon } = self;
        format!("{lon},{lat}")
    }

    pub fn to_h3_cell(&self, res: u32) -> H3Cell {
        let gc: GeoCoord = self.clone().into();
        let h3index = unsafe { h3ron_h3_sys::geoToH3(&gc, res as std::os::raw::c_int) };
        unsafe { H3Cell::new_unchecked(h3index) }
    }
}

impl From<h3ron_h3_sys::GeoCoord> for LatLon {
    fn from(gc: h3ron_h3_sys::GeoCoord) -> Self {
        LatLon {
            lat: gc.lat * 180f64 / std::f64::consts::PI,
            lon: gc.lon * 180f64 / std::f64::consts::PI,
        }
    }
}

impl Into<GeoCoord> for LatLon {
    fn into(self) -> GeoCoord {
        GeoCoord {
            lat: self.lat / 180f64 * std::f64::consts::PI,
            lon: self.lon / 180f64 * std::f64::consts::PI,
        }
    }
}

impl From<H3Cell> for LatLon {
    fn from(h3index: H3Cell) -> Self {
        let mut gc = h3ron_h3_sys::GeoCoord { lat: 0.0, lon: 0.0 };
        unsafe { h3ron_h3_sys::h3ToGeo(h3index.into(), &mut gc) };
        LatLon::from(gc)
    }
}
