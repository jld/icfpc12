use std;
import io::{reader,reader_util,writer_util};
import state::{state, outcome, cont, died, won, toolong, cmd, move, wait, shave};
import geom::{left, down, up, right};
import std::{map, sort};
import std::map::hashmap;

type posn = @{ state: state, res: outcome, last: hist };
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
        let mut y = (lines.len() - 1) as geom::coord;
        for lines.each |line| {
            if y == self.state.water { out.write_str("\x1b[34;46m"); }
            out.write_line(line);
            y -= 1;
        }
        out.write_str("\x1b[0m\n"+alt self.res {
            died { "\x1b[41;37;1mYOU ARE DESTROYED.\n" }
            toolong { "\x1b[47;31mTurn limit exceeded.\n" }
            won { "\x1b[40;33;1mYou win!\n" }
            cont { "" }
        }+#fmt("\x1b[1mScore:%? \x1b[0mT:%? L:%?/%? %sWd:%?/%? Rz:%?%s\n",
               // TODO: show trampoline map
               self.score(), self.state.time,
               self.state.lgot, self.state.c.lamb,
               if self.state.c.flood > 0 {
                   #fmt("Wt:%?/%? ",
                        self.state.c.flood
                        - (self.state.time % self.state.c.flood),
                        self.state.c.flood)
               } else { "" },
               self.state.wdmg, self.state.c.wproof,
               self.state.razors,
               if self.state.c.growth > 0 {
                   #fmt(" Bt:%?/%?",
                        self.state.c.growth
                        - (self.state.time % self.state.c.growth),
                        self.state.c.growth)
               } else { "" }));
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
        let movep = here.res == cont;
        let razorp = movep && here.state.razors > 0;
        let undop = here.last != initial;
        let travp = marks.size() > 0;
        here.show(out, get_cmd(#fmt("Commands: %s%s%s%smq", 
                                    if movep { "hjkl." } else { "" },
                                    if razorp { "s" } else { "" },
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
          's' if razorp { cmd = shave }
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
