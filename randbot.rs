use std;
import io::{reader,reader_util,writer_util};
import state::{state, outcome, cont, died, won, cmd, move, wait};
import geom::{left, down, up, right};
import botshell::stuff;
import result::{result, ok, err};

fn get_map(-fh: reader) -> state {
    let mut lines = ~[];
    for fh.each_line |line| { lines += ~[copy line]; }
    state::parse(lines)
}

const dfl_baglen : uint = 100;

pure fn opposed(c0: cmd, c1: cmd) -> bool {
    alt (c0,c1) {
      (move(left), move(right)) { true }
      (move(right), move(left)) { true }
      (move(up), move(down)) { true }
      (move(down), move(up)) { true }
      _ { false }
    }
}

fn main(argv: ~[str]) {
    let bagsize = if argv.len() > 1 { 
        option::get(uint::from_str(argv[1]))
    } else { 
        dfl_baglen
    };
    let rng = if argv.len() > 2 {
        rand::seeded_rng(str::bytes(argv[2]))
    } else {
        rand::rng()
    };
    let (shell, pos0) = botshell::start(get_map(io::stdin()));
    let mut bag = ~[mut pos0];
    let cmds = ~[move(left), move(right), move(up), move(down), wait];
    let weights = vec::to_mut(vec::from_elem(cmds.len(), 0));
    loop {
        let chosen = rng.gen_uint_range(0, bagsize);
        let chosen = if chosen >= bag.len() { 0 } else { chosen };
        let pos = bag[chosen];
        assert(pos.outcome == cont);
        let mut tl = 0;
        for uint::range(0, cmds.len()) |i| {
            let cmd = cmds[i];
            let weight =
                if !pos.state.useful(cmd) { 0 } else 
                if opposed(cmd, pos.head()) { 1 } else
                if cmd == wait { 5 } else { 10 };
            weights[i] = weight;
            tl += weight;
        }
        if tl == 0 { 
            bag[chosen] = bag[0];
            again
        }
        let mut impulse = rng.gen_uint_range(0, tl);
        let mut ocmd = none;
        for cmds.eachi |i,cmd| {
            if impulse < weights[i] {
                ocmd = some(cmd);
                break;
            } else {
                impulse -= weights[i];
            }
        }
        alt shell.step(pos, option::get(ocmd)) {
          err(_) { break }
          ok(npos) {
            if npos.outcome == cont {
                if bag.len() - 1 < bagsize {
                    bag += ~[npos];
                } else {
                    bag[rng.gen_uint_range(1, bag.len())] = npos;
                }
            }
          }
        }
    }
}
