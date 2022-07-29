pub fn is_h3_valid(h3: u64) -> bool {
    unsafe { h3ron_h3_sys::h3IsValid(h3) != 0 }
}

pub fn get_parent(h3: u64, res: u32) -> u64 {
    unsafe { h3ron_h3_sys::h3ToParent(h3, res as i32) }
}

pub fn h3_to_geo(h3: u64) -> h3ron_h3_sys::GeoCoord {
    let mut gc = h3ron_h3_sys::GeoCoord { lat: 0.0, lon: 0.0 };
    unsafe { h3ron_h3_sys::h3ToGeo(h3, &mut gc) };
    gc
}

pub fn geo_to_h3(gc: h3ron_h3_sys::GeoCoord, res: u32) -> u64 {
    unsafe { h3ron_h3_sys::geoToH3(&gc, res as std::os::raw::c_int) }
} 

pub fn k_ring(h3: u64, k: u32) -> Vec<u64> {
    let capacity = unsafe { h3ron_h3_sys::maxKringSize(k as i32) };
    let vec: Vec<u64> = Vec::with_capacity(capacity as usize);
    let ptr = vec.as_ptr() as *mut u64;
    unsafe { h3ron_h3_sys::kRing(h3, k as i32, ptr) };
    vec
}

pub fn is_neighbor(h1: u64, h2: u64) -> bool {
    unsafe { h3ron_h3_sys::h3IndexesAreNeighbors(h1, h2) == 1 }
}