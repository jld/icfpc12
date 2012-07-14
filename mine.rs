import option::{option,none,some};

type coord = u16;
type area = u32;
type dist = area;
type score = i64;

type point = { x: coord, y: coord };
type rect = { x: coord, y: coord, w: coord, h: coord };

enum space {
    robot, wall, rock, lambda, 
    closed_lift, open_lift, earth, empty
}

type state = {
    mine: mine,
    robot: point,
    score: score,
    time: area,
    collected: area,
    remaining: area,
    touched: rect
};

type mine = @{ mut repr: mine_repr };
type mine_image = ~[~[mut space]];
type mine_change = ~[{where: point, what: space}];
enum mine_repr {
    root(mine_image),
    diff(mine_change, mine),
    under_construction
}

impl geom for mine_image {
    pure fn get(p: point) -> space { let {x, y} = p; ret self[y][x] }
}

impl mine for mine {
    fn read<R>(f: pure fn (mine_image) -> R) -> R {
        let rval: R;
        self.focus();
        alt check self.repr {
          root(img) { rval = f(img) }
        }
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
                // Can't iterate an impure action over the borrowed diffs.
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

/*

fn make_mine(lines : ~[str]) -> mine {
    let height = lines.len() as coord;
    if height as uint != lines.len() { fail }
    let maxlen = lines.map(|line|line.len()).max();
    let width = maxlen as coord;
    if width as uint != maxlen { fail }

    let padded = vec::to_mut(do lines.map |line| {
        let line = str::chars(line);
        line + vec::from_elem(maxlen - line.len(), ' ')
    });
    vec::reverse(padded);
    let grid = do padded.map |line| {
        line.map(|c|space_of_char(c))
    };

    let bounds = { xl: 0, xh: width, yl: 0, yh: height };
    composite(bounds, grid, walls)
}
*/

impl geom for rect {
    pure fn area() -> area {
        ret (self.w as area) * (self.h as area);
    }
    pure fn trans(p: point) -> option<point> {
        let q = { x: p.x - self.x, y: p.y - self.y };
        if q.x < self.w && q.y < self.h { some(q) } else { none }
    }
    pure fn +(other: rect) -> rect {
        let xl = u16::min(self.x, other.x);
        let yl = u16::min(self.y, other.y);
        let xh = u16::max(self.x + self.w, other.x + other.w);
        let yh = u16::max(self.y + self.h, other.y + other.h);
        { x: xl, y: yl, w: xh - xl, h: yh - yl }
    }
    pure fn grow(t: coord, r: coord, b: coord, l: coord) -> rect {
        // Parameter order stolen from CSS; blame them.
        {x: self.x - l,
         y: self.y - b,
         w: self.w + l + r,
         h: self.h + t + b}
    }
}

impl geom for point {
    pure fn box() -> rect {
        { x: self.x, y: self.y, w: 1, h: 1 }
    }
}

fn space_of_char(c: char) -> space { 
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

fn char_of_space(s: space) -> char {
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