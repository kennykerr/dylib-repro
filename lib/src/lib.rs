#![feature(raw_dylib)]
#[link(name = "kernel32", kind = "raw-dylib")]
extern "system" {
    pub fn WaitForSingleObject(handle: isize, dwmilliseconds: u32) -> u32;
}