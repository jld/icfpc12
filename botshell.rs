use std;
import state::{state, outcome, cont, died, won, cmd, move, wait, score};
import geom::{left, down, up, right};
import std::list;
import std::list::{list, nil, cons};
import result::{result, ok, err};

type posn = { state: state, outcome: outcome, trail: @list<cmd>, score: score };
type shell = { mut best: posn };
enum stop { timeout }
type res<T> = result<T, stop>;

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
    let posn = { state: state, outcome: cont, trail: @nil, score: 0 };
    ret ({ mut best: posn }, posn)
}

impl stuff for shell {
    fn step(posn: posn, cmd: cmd) -> res<posn> {
        if sigwrap::get() {
            io::println((copy self.best).to_str());
            err(timeout)
        } else {
            let (res, state) = posn.state.step(cmd);
            let score = state.score(alt res { cont { none } r { some(r) } });
            let retv = {state: state,
                        outcome: res,
                        trail: @cons(cmd, posn.trail),
                        score: score};
            if score > self.best.score {
                #info("Score: %?; Trail: %s", score, retv.to_str());
                self.best = copy retv;
            }
            ok(retv)
        }
    }
    fn finish() {
        io::println((copy self.best).to_str());
        // XXX might want to reenable the signals....
    }
}

