pub fn main() {
    let using_web_sys = cfg!(feature = "web_sys");
    let using_std_web = cfg!(feature = "std_web");
    if using_web_sys && using_std_web {
        panic!("Yew does not allow the `web_sys` and `std_web` cargo features to be used simultaneously");
    } else if !using_web_sys && !using_std_web {
        panic!("Yew requires selecting either the `web_sys` or `std_web` cargo feature");
    }
}
