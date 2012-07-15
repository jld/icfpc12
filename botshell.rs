use std;
import state::{state, result, cont, died, won, cmd, move, wait, score};
import geom::{left, down, up, right};
import std::list;
import std::list::{list, nil, cons};

type posn = { state: state, result: result, trail: @list<cmd> };
type shell = { mut hiscore: score, mut best: posn };

impl stuff for posn {
    fn to_str() -> str {
        let mut chars = ~[];
        for list::each(self.trail) |cmd| {
            chars += ~[state::char_of_cmd(cmd)];
        }
        str::from_chars(vec::reversed(chars))
    }
    fn head() -> cmd {
        alt *self.trail {
          cons(a,_d) { a }
          nil { wait }
        }
    }
}

fn start(state: state) -> (shell,posn) {
    assert(state.time == 0);
    sigwrap::enable(sigwrap::SIGINT);
    sigwrap::enable(sigwrap::SIGALRM);
    let posn = { state: state, result: cont, trail: @nil };
    ret ({ mut hiscore: 0, mut best: posn }, posn)
}

impl stuff for shell {
    fn step(posn: posn, cmd: cmd) -> option<posn> {
        if sigwrap::get() {
            io::println((copy self.best).to_str());
            none
        } else {
            let (res, state) = posn.state.step(cmd);
            let score = state.score(alt res { cont { none } r { some(r) } });
            let retv = {state: state,
                        result: res,
                        trail: @cons(cmd, posn.trail)};
            if score > self.hiscore {
                #info("Score: %?; Trail: %s", score, retv.to_str());
                self.hiscore = score;
                self.best = copy retv;
            }
            some(retv)
        }
    }
    fn finish() {
        io::println((copy self.best).to_str());
        // XXX might want to reenable the signals....
    }
}

