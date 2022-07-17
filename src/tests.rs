use anyhow::Result;
use rand::{thread_rng, Rng};

use crate::H3Cell;

#[test]
fn check_resolution() -> Result<()> {
    let mut h3: u64 = 0x8a2a1072b59ffff;
    assert_ne!(unsafe { h3ron_h3_sys::h3IsValid(h3) }, 0);
    let lib_res: i32 = unsafe { h3ron_h3_sys::h3GetResolution(h3) };
    let rust_h3 = H3Cell::try_from(h3).unwrap();
    let rust_res = rust_h3.get_resolution() as i32;
    assert_eq!(lib_res, rust_res);
    Ok(())
}

#[test]
fn randomly_generated_h3cell_is_valid() -> Result<()> {
    (0..=15u32).for_each(|res| {
        let generated = H3Cell::generate_random(res).expect("randomly h3 generation not possible");
        println!("res: {}, generated h3: {:x}", res, generated.as_u64());
        println!("{}", generated.pretty_print());
        assert_ne!(unsafe { h3ron_h3_sys::h3IsValid(generated.as_u64()) }, 0);
    });

    Ok(())
}

#[test]
fn randomly_generated_from_parent_is_valid() {
    let mut rng = rand::thread_rng();
    (0..=15u32).for_each(|res| {
        let parent = H3Cell::generate_random(res).unwrap();
        let mut child = H3Cell::generate_from_parent(parent, rng.gen_range(res..=15)).unwrap();
        println!("generated h3: {:x}", child.as_u64());
        println!("{}", child.pretty_print());
        assert_ne!(unsafe { h3ron_h3_sys::h3IsValid(child.as_u64()) }, 0);
        assert_eq!(child.get_parent(res).unwrap(), parent);
    })
}
