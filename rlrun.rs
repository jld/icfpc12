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

enum msg {
    get_cmd(str),
    final
}

impl posn for posn {
    pure fn score() -> state::score {
        self.state.score(alt self.res { cont { none } r { some(r) } })
    }
    fn show(out: io::writer, msg: msg) {
        out.write_str("\x1b[?25l\x1b[2J\x1b[H");
        let lines = self.state.print();
        for lines.each |line| { out.write_line(line); }
        out.write_str("\n"+alt self.res {
            died { "\x1b[47;31;1mYOU ARE DESTROYED.\n" }
            won { "\x1b[40;33;1mYou win!\n" }
            cont { "" }
        }+#fmt("\x1b[1mScore: %? \x1b[0m  Time: %?   Lambdas: %?/%?\n",
               self.score(), self.state.time,
               self.state.lgot, self.state.lamb));
        alt msg {
          get_cmd(text) {
            let cx = (self.state.rloc.x as uint) + 1;
            let cy = lines.len() - (self.state.rloc.y as uint);
            out.write_str(text + #fmt("\x1b7\x1b[%u;%uH\x1b[?25h", cy, cx));
          }
          final {
            out.write_str("\x1b[?25h");
          }
        }
        out.flush();
    }
    fn getc(in: io::reader, out: io::writer, text: str) -> char {
        out.write_str("\x1b[?25l\x1b8\x1b[1K\x0d" + text);
        out.flush();
        in.read_char()
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

pure fn char_le(&&a: char, &&b: char) -> bool { a <= b } // ???

fn main(argv: ~[str]) {
    termstuff::game_mode(true);
    let in = io::stdin();
    let out = io::stdout();
    let state0 = get_map(io::file_reader(argv[1]).get());
    let mut here = @{ state: state0, res: cont, last: initial };
    let mut best = (0, here);
    let marks = map::int_hash();
    let markstr = || {
        let mut buf = ~[];
        for marks.each_key |k| { buf += ~[k as char] }
        str::from_chars(sort::merge_sort(char_le, buf))
    };
    loop {
        let contender = (here.score(), here);
        if contender > best { best = contender }
        let undop = here.last != initial;
        let movep = here.res == cont;
        let travp = marks.size() > 0;
        here.show(out, get_cmd(#fmt("Commands: %s%s%smq", 
                                    if movep { "hjkl." } else { "" },
                                    if undop { "-" } else { "" },
                                    if travp { "'" } else { "" })));
        let mut cmd;
        alt in.read_char() {
          'h' if movep { cmd = move(left) }
          'j' if movep { cmd = move(down) }
          'k' if movep { cmd = move(up) }
          'l' if movep { cmd = move(right) }
          '.' if movep { cmd = wait }
          '-' if undop { here = here.tail(); again }
          'm' {
            marks.insert(here.getc(in, out, "Enter mark.") as int, here);
            again
          }
          '\'' if travp {
            alt marks.find(here.getc(in, out, "Marks: " + markstr()) as int) {
              none { out.write_char('\x07') }
              some(there) { here = there }
            }
            again
          }
          'q' { break }
          _ { out.write_char('\x07'); again }
        }
        let (res, state) = here.state.step(cmd);
        here = @{ state: state, res: res, last: from(cmd, here) };
    }
    let (hiscore, there) = best;
    there.show(out, final);
    out.write_line(#fmt("High score: %?", hiscore));
    out.write_line(there.to_str());
    termstuff::game_mode(false);
}
