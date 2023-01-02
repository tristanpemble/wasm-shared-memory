#![feature(vec_into_raw_parts)]

extern "C" {
    fn print(msg: String) -> ();
}

#[no_mangle]
pub unsafe fn alloc_11(instance: i32) -> i32 {
    let val = 11;
    print(format!("alloc_11[#{}]: {}", instance, val));
    &val as *const i32 as i32
}

#[no_mangle]
pub unsafe fn add_23(instance: i32, ptr: i32) {
    let mut val = *(ptr as *mut i32);
    val += 23;
    print(format!("add_23[#{}]: {}", instance, val));
    std::mem::forget(val);
}
