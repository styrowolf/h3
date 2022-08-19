use crate::ffi;
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
        let gc: h3ron_h3_sys::GeoCoord = self.clone().into();
        let h3index = ffi::geo_to_h3(gc, res);
        H3Cell(h3index)
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

impl From<[f64; 2]> for LatLon {
    fn from(coordinates: [f64; 2]) -> Self {
        LatLon {
            lat: coordinates[0],
            lon: coordinates[1],
        }
    }
}

impl Into<[f64; 2]> for LatLon {
    fn into(self) -> [f64; 2] {
        [self.lat, self.lon]
    }
}

impl Into<h3ron_h3_sys::GeoCoord> for LatLon {
    fn into(self) -> h3ron_h3_sys::GeoCoord {
        h3ron_h3_sys::GeoCoord {
            lat: self.lat / 180f64 * std::f64::consts::PI,
            lon: self.lon / 180f64 * std::f64::consts::PI,
        }
    }
}

impl From<H3Cell> for LatLon {
    fn from(h3index: H3Cell) -> Self {
        LatLon::from(ffi::h3_to_geo(h3index.as_u64()))
    }
}

impl Into<H3Cell> for LatLon {
    fn into(self) -> H3Cell {
        self.to_h3_cell(15)
    }
}

impl Into<google_maps::LatLng> for LatLon {
    fn into(self) -> google_maps::LatLng {
        use google_maps::prelude::Decimal;
        google_maps::LatLng {
            lat: Decimal::try_from(self.lat).unwrap(),
            lng: Decimal::try_from(self.lon).unwrap(),
        }
    }
}
