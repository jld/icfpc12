use std;
import io::{reader,reader_util,writer_util};
import state::{state, result, cont, died, won, cmd, move, wait};
import geom::{left, down, up, right};
import std::{map, sort};
import std::map::hashmap;

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
    fn tail() -> posn {
        alt self.last {
          initial { fail }
          from(_cmd, there) { there }
        }
    }
}

pure fn char_le(&&a : char, &&b: char) -> bool { a <= b } // ???

fn main(argv: ~[str]) {
    termstuff::game_mode(true);
    let in = io::stdin();
    let out = io::stdout();
    let state0 = get_map(io::file_reader(argv[1]).get());
    let mut here = @{ state: state0, res: cont, last: initial };
    let marks = map::int_hash();
    let markstr = || {
        let mut buf = ~[];
        for marks.each_key |k| { buf += ~[k as char] }
        str::from_chars(sort::merge_sort(char_le, buf))
    };
    loop {
        let undop = here.last != initial;
        let movep = here.res == cont;
        let travp = marks.size() > 0;
        here.show(out, #fmt("Commands: %s%s%smq", 
                            if movep { "hjkl." } else { "" },
                            if undop { "-" } else { "" },
                            if travp { "'" } else { "" }));
        let mut cmd;
        alt in.read_char() {
          'h' if movep { cmd = move(left) }
          'j' if movep { cmd = move(down) }
          'k' if movep { cmd = move(up) }
          'l' if movep { cmd = move(right) }
          '.' if movep { cmd = wait }
          '-' if undop { here = here.tail(); again }
          'm' { 
            out.write_str("\x1b8\x1b[1K\x0dEnter mark.");
            out.flush();
            let m = in.read_char() as int;
            marks.insert(m, here);
            again
          }
          '\'' if travp {
            out.write_str("\x1b8\x1b[1K\x0dMarks: " + markstr());
            out.flush();
            let m = in.read_char() as int;
            alt marks.find(m) {
              none { out.write_char('\x07'); again }
              some(there) { here = there; again }
            }
          }
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
