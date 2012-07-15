import io::{reader,reader_util,writer_util};
import state::{state, result, cont, died, won, cmd, move, wait};
import geom::{left, down, up, right};

type posn = @{ state: state, res: result, last: hist };
enum hist {
    initial,
    from(cmd, posn)
}

fn get_map(-fh: reader) -> state {
    let mut lines = ~[];
    for fh.each_line |line| { lines += ~[copy line]; }
    state::parse(lines)
}

impl posn for posn {
    fn show(out: io::writer, extra: str) {
        out.write_str("\x1b[2J\x1b[H");
        let lines = self.state.print();
        for lines.each |line| { out.write_line(line); }
        out.write_str("\n"+alt self.res {
            died { "\x1b[47;31;1mYOU ARE DESTROYED.\n" }
            won { "\x1b[40;33;1mYou win!\n" }
            cont { "" }
        }+#fmt("\x1b[1mScore: %? \x1b[0m  Time: %?   Lambdas: %?/%?\n",
               self.state.score(alt self.res { cont { none } r { some(r) } }),
               self.state.time, self.state.lgot, self.state.lamb));
        let cx = (self.state.rloc.x as uint) + 1;
        let cy = lines.len() - (self.state.rloc.y as uint);
        out.write_str(extra + #fmt("\x1b7\x1b[%u;%uH", cy, cx));
        out.flush();
    }
    fn each_cmd(f: fn(cmd) -> bool) {
        alt self.last {
          initial { }
          from(cmd, last) {
            if f(cmd) { last.each_cmd(f) }
          }
        }
    }
    fn to_str() -> str {
        let mut chars = ~[];
        for self.each_cmd |cmd| {
            chars += ~[state::char_of_cmd(cmd)];
        }
        str::from_chars(vec::reversed(chars))
    }
}

fn main(argv: ~[str]) {
    termstuff::game_mode(true);
    let in = io::stdin();
    let out = io::stdout();
    let state0 = get_map(io::file_reader(argv[1]).get());
    let mut here = @{ state: state0, res: cont, last: initial };
    loop {
        let movep = here.res == cont;
        here.show(out, #fmt("Commands: %sq", 
                            if movep { "hjkl." } else { "" }));
        let mut cmd;
        alt in.read_char() {
          'h' if movep { cmd = move(left) }
          'j' if movep { cmd = move(down) }
          'k' if movep { cmd = move(up) }
          'l' if movep { cmd = move(right) }
          '.' if movep { cmd = wait }
          'q' { break }
          _ { out.write_char('\x07'); again }
        }
        let (res, state) = here.state.step(cmd);
        here = @{ state: state, res: res, last: from(cmd, here) };
    }
    out.write_str("\x1b8\n");
    out.write_line(here.to_str());
    termstuff::game_mode(false);
}
