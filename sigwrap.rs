import libc::{c_int,c_uint};
import sigwrap_c::{enable,disable,reset};
export enable, disable, reset, get;
export SIGHUP, SIGINT, SIGQUIT, SIGPIPE, SIGALRM, SIGTERM;

extern mod sigwrap_c {
    #[link_name="sigwrap_enable"]
    fn enable(sig: c_int);
    #[link_name="sigwrap_disable"]
    fn disable(sig: c_int);
    #[link_name="sigwrap_get"]
    fn get() -> c_uint;
    #[link_name="sigwrap_reset"]
    fn reset();
}

fn get() -> bool {
    sigwrap_c::get() != 0
}

// These are fairly widespread, but may not be formally portable.
const SIGHUP : c_int = 1;
const SIGINT : c_int = 2;
const SIGQUIT : c_int = 3;
const SIGPIPE : c_int = 13;
const SIGALRM : c_int = 14;
const SIGTERM : c_int = 15;

