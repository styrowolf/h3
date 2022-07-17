pub fn is_h3_valid(h3: u64) -> bool {
    unsafe { h3ron_h3_sys::h3IsValid(h3) != 0 }
}

pub fn get_parent(h3: u64, res: u32) -> u64 {
    unsafe { h3ron_h3_sys::h3ToParent(h3, res as i32) }
}

pub fn is_neighbor() {

}