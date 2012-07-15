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
    loop {
        let pos = bag[rng.gen_uint_range(0, bag.len())];
        assert(pos.result == cont);
        let cmd = rng.choose(cmds);
        alt shell.step(pos, cmd) {
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
