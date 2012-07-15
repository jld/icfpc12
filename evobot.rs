use std;
import io::{reader,reader_util,writer_util};
import option::{option,some,none};
import state::{state, result, cont, died, won, cmd, move, wait};
import geom::{left, down, up, right};
import botshell::{stuff, shell, res, posn};
import result::{result, ok, err};


type larva = botshell::posn;
type pool = { sacs: ~[mut sac], mut vol: float };
type sac = { vol: float, larva: larva };

fn mkpool(size: uint, progenitor: larva) -> pool {
    assert(size > 0);
    let sac = { vol: 1.0, larva: progenitor };
    { sacs: vec::to_mut(vec::from_elem(size, sac)), mut vol: size as float }
}

fn lch(x: uint) -> uint { 2*x+1 }
fn rch(x: uint) -> uint { 2*x+2 }

impl extensions for rand::rng {
    fn gen_float_fast() -> float {
        // Also not broken like the real gen_f64 is.
        const scale : float = 4294967296.0;
        (self.gen_u32() as float) / scale
    }
}

const scale_max : float = 1048576.;
impl stuff for pool {
    pure fn len() -> uint { self.sacs.len() }
    pure fn scale() -> float { self.vol / (self.len() as float) }

    fn rescale() {
        let scale = self.scale();
        //#info("Rescaling from %?", scale);
        let mut tl = 0.;
        for uint::range(0, self.len()) |i| {
            let vol = self.sacs[i].vol / scale;
            self.sacs[i] = { vol: vol with self.sacs[i] };
            tl += vol;
        }
        //#info("Should be around %?: %?", self.len(), tl);
        self.vol = tl;
    }

    fn add(larva: posn, vol: float) {
        if self.scale() > scale_max { self.rescale(); }
        let vol = vol * self.scale();
        self.vol += vol - self.sacs[0].vol;
        self.sacs[0] = { vol: vol, larva: larva };
        self.bubble(0);
    }
    fn bubble(i: uint) {
        if lch(i) >= self.len() { ret; }
        let next;
        if rch(i) >= self.len()
            || self.sacs[lch(i)].vol < self.sacs[rch(i)].vol {
            next = lch(i);
        } else {
            next = rch(i);
        }
        if self.sacs[next].vol < self.sacs[i].vol {
            vec::swap(self.sacs, i, next);
            self.bubble(next);
        }
    }
    fn select(rng: rand::rng) -> larva {
        let mut chosen = rng.gen_float_fast() * self.vol;
        for uint::range(0, self.len()) |i| {
            if chosen < self.sacs[i].vol {
                ret self.sacs[i].larva;
            }
            chosen -= self.sacs[i].vol;
        }
        #warn("%? > 0", chosen); // I do not trust floating point.
        ret self.sacs[0].larva;
    }
}

impl mut for larva {
    pure fn useful(c: cmd) -> bool {
        self.state.useful(c)
    }
    fn mut_add1_uniform(sh: shell, r: rand::rng) -> res<option<larva>> { 
        let cmds = ~[move(left), move(right), move(up), move(down), wait];
        let cmds = vec::filter(cmds, |c| self.useful(c));
        alt r.choose_option(cmds) {
          none { ok(none) }
          some(cmd) { sh.step(self, cmd).map(|l| some(l)) }
        }
    }
}

fn get_map(-fh: reader) -> state {
    let mut lines = ~[];
    for fh.each_line |line| { lines += ~[copy line]; }
    state::parse(lines)
}

const dfl_poolsize : uint = 100;

fn main(argv: ~[str]) {
    let poolsize = if argv.len() > 1 { 
        option::get(uint::from_str(argv[1]))
    } else { 
        dfl_poolsize
    };
    let r = if argv.len() > 2 {
        rand::seeded_rng(str::bytes(argv[2]))
    } else {
        rand::rng()
    };
    let (sh, pos0) = botshell::start(get_map(io::stdin()));
    let pool = mkpool(poolsize, pos0);
    loop {
        let chosen = pool.select(r);
        alt chosen.mut_add1_uniform(sh, r) {
          err(_) { break }
          ok(none) { again }
          ok(some(desc)) {
            pool.add(desc, (0.9 + 0.2 * r.gen_float_fast()) * 
                     (if desc.state.collected { 10. } else { 1. }))
          }
        }
    }
    /*
    log(info, pool.vol);
    for pool.sacs.each |sac| {
        log(info, (sac.vol, sac.larva.to_str()));
    }
    */
}
