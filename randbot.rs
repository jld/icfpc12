use std;
import io::{reader,reader_util,writer_util};
import state::{state, result, cont, died, won, cmd, move, wait};
import geom::{left, down, up, right};
import botshell::stuff;

fn get_map(-fh: reader) -> state {
    let mut lines = ~[];
    for fh.each_line |line| { lines += ~[copy line]; }
    state::parse(lines)
}

const dfl_baglen : uint = 100;

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
        let chosen = rng.gen_uint_range(0, bag.len());
        let pos = bag[chosen];
        assert(pos.result == cont);
        let mut tl = 0;
        for uint::range(0, cmds.len()) |i| {
            let weight = if pos.state.useful(cmds[i]) { 1 } else { 0 };
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
          none { break }
          some(npos) {
            if npos.result == cont {
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
