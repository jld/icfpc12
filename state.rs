import geom::*;
import mine::*;

enum dir { left, right, up, down }
enum cmd { move(dir), wait, abort }

type state = {
    mine: mine,
    rloc: point,
    time: area,
    lgot: area,
    lrem: area,
    touched: rect
};

/*
fn step(state: state, cmd: cmd) -> state {
    let {mine, rloc, time, lgot, lrem, touched} = state;
    
}
*/

fn print(state: state) -> ~[str] {
    assert(state.mine.get(state.rloc) == robot);
    mine::print(state.mine) +
        ~["", 
          #fmt("Time %u", state.time as uint),
          #fmt("Lambdas %u/%u", state.lgot as uint, state.lrem as uint)]
}

fn parse(lines: &[str]) -> state {
    let (img, metalines) = mine::parse(lines);
    if metalines.len() > 0 { fail }
    let mut rloc = none;
    let mut lrem = 0;
    do img.iteri |y,line| {
        do line.iteri |x,cell| {
            alt cell {
              robot { rloc = some({x: x as coord, y: y as coord}) }
              lambda { lrem += 1 }
              _ { }
            }
        }
    }

    {mine: new_mine(copy img),
     rloc: option::get(rloc),
     time: 0,
     lgot: 0,
     lrem: lrem,
     touched: img.box()}
}

pure fn cmd_of_char(c: char) -> cmd {
    alt c {
      'L' { move(left) }
      'R' { move(right) }
      'U' { move(up) }
      'D' { move(down) }
      'W' { wait }
      'A' { abort }
      _ { fail }
    }
}

pure fn char_of_cmd(c: cmd) -> char {
    alt c {
      move(left) { 'L' }
      move(right) { 'R' }
      move(up) { 'U' }
      move(down) { 'D' }
      wait { 'W' }
      abort { 'A' }
    }
}


