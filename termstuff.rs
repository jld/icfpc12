import libc::c_int;
export game_mode, game_mode_on, STDIN_FILENO;

extern mod cstuff {
    fn tty_game_mode(fd: c_int, on: c_int) -> c_int;
}

const STDIN_FILENO : c_int = 0;

fn game_mode_on(fd: c_int, on: bool) {
    if 0 != cstuff::tty_game_mode(fd, if on { 1 } else { 0 }) {
        str::as_c_str("tc[gs]etattr", libc::perror);
        fail
    }
}
fn game_mode(on: bool) {
    game_mode_on(STDIN_FILENO, on);
}
