import libc::funcs::posix88::unistd::sleep;
import libc::{c_int,c_uint};
import sigwrap::*;

extern mod sigwrap {
    fn sigwrap_enable(sig: c_int);
    fn sigwrap_disable(sig: c_int);
    fn sigwrap_get() -> c_uint;
    fn sigwrap_reset();
}

fn main() {
    sigwrap_enable(2);
    while sigwrap_get() == 0 {
        io::print(".");
        sleep(3);
    }
    io::println("Done.");
}
