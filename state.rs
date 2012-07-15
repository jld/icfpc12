import option::{option,some,none};
import geom::*;
import mine::*;

enum cmd { move(dir), wait }
type score = i64;

type state = {
    mine: mine,
    rloc: point,
    time: area,
    lgot: area,
    lamb: area,
    touched: rect
};

enum result {
    cont,
    died,
    won
}

pure fn rock_change(img: mine_image, here: point) -> option<point> {
    if img.get(here) != rock { ret none }
    let hd = here.step(down);
    let hl = here.step(left);
    let hr = here.step(right);
    let hdl = hd.step(left);
    let hdr = hd.step(right);

    if img.get(hd) == empty {
        ret some(hd);
    } else if img.get(hd) == rock
        && img.get(hr) == empty && img.get(hdr) == empty {
        ret some(hdr);
    } else if img.get(hd) == rock 
        // "is not empty or" entailed by "else if"
        && img.get(hl) == empty && img.get(hdl) == empty {
        ret some(hdl);
    } else if img.get(hd) == lambda
        && img.get(hr) == empty && img.get(hdr) == empty {
        ret some(hdr);
    } else {
        ret none;
    }
}

impl state for state {
    fn step(cmd: cmd) -> (result, state) {
        step(self, cmd)
    }
    pure fn score(result: option<result>) -> score {
        let lgot = self.lgot as score;
        let base = 25 * lgot - (self.time as score);
        alt result {
          none { base + 25 * lgot }
          some(won) { base + 50 * lgot }
          _ { base }
        }
    }
    pure fn bonkp(bonk: &[point]) -> bool {
        let airspace = self.rloc.step(up);
        ret self.mine.get(airspace) == rock && vec::contains(bonk, airspace);
    }
    fn print() -> ~[str] { print(self) }
}

fn step(state: state, cmd: cmd) -> (result, state) {
    let mut state = { time: state.time + 1 with state };
    let mut completing = false;
    let mut collected = false;

    // 2.2 Robot Movement
    assert(state.mine.get(state.rloc) == robot);
    alt cmd {
      wait { }
      move(d) {
        let mut edits = ~[];
        let nrloc = state.rloc.step(d);
        let moved = alt state.mine.get(nrloc) {
          empty { true }
          earth { true }
          open_lift { completing = true; true }
          lambda { state = {lgot: state.lgot + 1 with state};
                  collected = true; true }
          
          rock { 
            alt d {
              left | right {
                let rollto = nrloc.step(d);
                if state.mine.get(rollto) == empty {
                    edits += ~[{ where: rollto, what: rock }];
                    true
                } else {
                    false
                }
              }
              _ { false }
            }
          }
          _ { false }
        };
        if moved {
            edits += ~[{ where: nrloc, what: robot },
                       { where: state.rloc, what: empty }];
            let touched = do vec::foldl(state.touched, edits) |r,e| {
                r + e.where.box()
            };
            state = {rloc: nrloc, mine: state.mine.edit(edits),
                     touched: touched with state};
        }
      }
    }
    
    // 2.3 Map Update
    let mut edits = ~[];
    let mut bonk = ~[];
    let mut ntouch = state.rloc.box();
    do state.mine.read |img| {
        let rolling = state.touched.grow(1, 1, 0, 1) * img.box();
        do rolling.iter |here| {
            alt rock_change(img, here) {
              none { }
              some(there) {
                edits += ~[{ where: here, what: empty },
                           { where: there, what: rock }];
                bonk += ~[there];
                ntouch += here.box();
                ntouch += there.box();
              }
            }
        }
    }
    if (collected || state.lamb == 0) && state.lgot == state.lamb {
        // Note: if state.lamb == 0, best move is immediate abort.
        // (So worry not about efficiency for that case.)
        do state.mine.read |img| {
            do img.box().iter |here| {
                if img.get(here) == closed_lift {
                    edits += ~[{ where: here, what: open_lift }];
                }
            }
        }
    }
    state = {mine: state.mine.edit(edits),
             touched: ntouch with state};
    
    // 2.4 Ending Conditions
    if completing {
        ret (won, state);
    } else if state.bonkp(bonk) {
        ret (died, state);
    } else {
        ret (cont, state);
    }
}

fn print(state: state) -> ~[str] {
    assert(state.mine.get(state.rloc) == robot);
    mine::print(state.mine)
}

fn parse(lines: &[str]) -> state {
    let (img, metalines) = mine::parse(lines);
    if metalines.len() > 0 { fail }
    let mut rloc = none;
    let mut lamb = 0;
    do img.iteri |y,line| {
        do line.iteri |x,cell| {
            alt space_show_(cell) {
              robot { rloc = some({x: x as coord, y: y as coord}); }
              lambda { lamb += 1 }
              _ { }
            }
        }
    }

    {mine: new_mine(copy img),
     rloc: option::get(rloc),
     time: 0,
     lgot: 0,
     lamb: lamb,
     touched: img.box()}
}

pure fn cmd_of_char(c: char) -> cmd {
    option::get(cmd_opt_of_char(c))
}

pure fn cmd_opt_of_char(c: char) -> option<cmd> {
    alt c {
      'L' { some(move(left)) }
      'R' { some(move(right)) }
      'U' { some(move(up)) }
      'D' { some(move(down)) }
      'W' { some(wait) }
      _ { none }
    }
}

pure fn char_of_cmd(c: cmd) -> char {
    alt c {
      move(left) { 'L' }
      move(right) { 'R' }
      move(up) { 'U' }
      move(down) { 'D' }
      wait { 'W' }
    }
}

