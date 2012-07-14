import option::{option,none,some};
import geom::*;

enum space {
    robot, wall, rock, lambda, 
    closed_lift, open_lift, earth, empty
}

fn print(mine: mine) -> ~[str] {
    do mine.read |img| {
        let n = img.len();
        do vec::from_fn(n) |i| {
            str::from_chars(img[n - 1 - i].map(|s| char_of_space(s)))
        }
    }
}

fn parse(lines: &[str]) -> (mine_image, &[str]) {
    let mut maxlen = 0;
    let mut rows = ~[];
    let mut metaline = 0;
    
    for lines.eachi |i, line| {
        metaline = i + 1;
        if line == "" { break; }
        maxlen = uint::max(maxlen, line.len());
        let row = vec::to_mut(str::chars(line).map(|c| space_of_char(c)));
        vec::unshift(rows, row);
    }
    let img = do vec::map_consume(rows) |+line| {
        if line.len() == maxlen { line }
        else { line + vec::from_elem(maxlen - line.len(), empty) }
    };
    ret (img, vec::view(lines, metaline, lines.len()))
}
    

type mine = @{ mut repr: mine_repr };
type mine_image = ~[~[mut space]];
type mine_change = ~[{where: point, what: space}];
enum mine_repr {
    root(mine_image),
    diff(mine_change, mine),
    under_construction
}

impl geom for mine_image {
    pure fn get(p: point) -> space { let {x, y} = p; self[y][x] }
    pure fn box() -> rect {
        {x: 0, y: 0, 
         w: self[0].len() as length,
         h: self.len() as length}
    }
}

fn new_mine(+image : mine_image) -> mine { @{ mut repr: root(image) } }

impl mine for mine {
    fn read<R>(f: fn (&mine_image) -> R) -> R {
        let rval: R;
        self.focus();
        let mut self_repr = under_construction;
        self_repr <-> self.repr;
        alt check self_repr {
          root(img) { rval = f(img) }
        }
        self.repr <-> self_repr;
        ret rval;
    }

    fn edit(+ch: mine_change) -> mine { 
        @{ mut repr: diff(ch, self) }
    }

    fn focus() {
        alt check self.repr { 
          root(*) { ret }
          diff(*) { }
        }
        let mut diff_repr = under_construction;
        let mut root_repr = under_construction;
        let rdiff_repr: mine_repr;
        let other: mine;
        diff_repr <-> self.repr;
        alt check diff_repr {
          diff(diffs, d_other) { 
            other = d_other;
            other.focus();
            root_repr <-> other.repr;
            alt check root_repr {
              root(img) {
                let n = diffs.len();
                // Yes, we have no rev_map.
                // Also, in principle this could reuse the old vector.
                let rdiffs = do vec::from_fn(n) |i| {
                    let d = diffs[n - 1 - i];
                    {where: d.where, what: img.get(d.where)}
                };
                // Cannot iterate an impure action over the borrowed diffs.
                for uint::range(0, n) |i| {
                    let {where, what} = diffs[i];
                    img[where.y][where.x] = what;
                }
                rdiff_repr = diff(rdiffs, self);
              }
            }
          }
        }
        self.repr <- root_repr;
        other.repr <- rdiff_repr;
    }
}

impl geom for mine {
    pure fn get(p: point) -> space {
        alt check self.repr {
          root(img) { img.get(p) }
          diff(diffs, other) {
            alt vec::rfind(diffs, |d|d.where == p) {
              some(d) { d.what }
              none { other.get(p) }
            }
          }
        }
    }
}

pure fn space_of_char(c: char) -> space { 
    alt c {
      'R' { robot }
      '#' { wall }
      '*' { rock }
      '\\' { lambda }
      'L' { closed_lift }
      'O' { open_lift }
      '.' { earth }
      ' ' { empty }
      _ { fail }
    }
}

pure fn char_of_space(s: space) -> char {
    alt s {
      robot { 'R' }
      wall { '#' }
      rock { '*' }
      lambda { '\\' }
      closed_lift { 'L' }
      open_lift { 'O' }
      earth { '.' }
      empty { ' ' }
    }
}

