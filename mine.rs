type coord = u16;
type area = u32;
type dist = area;
type score = i64;

type point = { x: coord, y: coord };
type rect = { xl: coord, xh: coord, yl: coord, yh:coord };

enum space {
    robot, wall, rock, lambda, 
    closed_lift, open_lift, earth, empty
}

type state = {
    mine: mine,
    robot: coord,
    score: score,
    lambdas: area,
    touched: rect
};

type grid = ~[~[space]];
enum mine { walls, overlay(rect, grid, @mine) }

fn composite(r: rect, +g: grid, +back: mine) -> mine {
    // TODO: compact
    overlay(r, g, @back)
}

impl mine for mine {
    fn get(p: point) -> space {
        alt self {
          walls { wall }
          overlay(r, g, back) {
            if r.contains(p) { 
                let q = r.tr_in(p);
                g[q.y][q.x]
            } else {
                (*back).get(p)
            }
          }
        }
    }
}

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

impl geom for rect {
    fn area() -> u32 {
        assert(self.xh >= self.xl);
        assert(self.yh >= self.yl);
        let w = (self.xh - self.xl) as u32;
        let h = (self.yh - self.yl) as u32;
        ret w * h;
    }
    fn tr_in(p: point) -> point {
        assert(p.x >= self.xl);
        assert(p.y >= self.yl);
        assert(p.x < self.xh);
        assert(p.y < self.yh);
        { x: p.x - self.xl, y: p.y - self.yl }
    }
    fn tr_out(p: point) -> point {
        let q = { x: p.x + self.xl, y: p.y + self.yl };
        assert(q.x >= self.xl);
        assert(q.y >= self.yl);
        assert(q.x < self.xh);
        assert(q.y < self.yh);        
        ret q
    }
    fn contains(p: point) -> bool {
        ret (p.x >= self.xl && p.x < self.xh &&
             p.y >= self.yl && p.y < self.yh);
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