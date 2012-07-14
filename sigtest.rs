import libc::funcs::posix88::unistd::sleep;
import sigwrap::{enable,get};

fn main() {
    sigwrap::enable(sigwrap::SIGINT);
    while !sigwrap::get() {
        io::print(".");
        sleep(3);
    }
    io::println("Done.");
}
