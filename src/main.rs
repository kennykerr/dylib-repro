fn main() {
    unsafe { lib::WaitForSingleObject(0, 0); }
    println!("ok");
}
