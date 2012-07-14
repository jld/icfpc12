type coord = i16;
type length = u16;
type area = u32;
type dist = area;

type point = { x: coord, y: coord };
type rect = { x: coord, y: coord, w: length, h: length };
enum dir { left, right, up, down }

impl geom for rect {
    pure fn area() -> area {
        ret (self.w as area) * (self.h as area);
    }
    pure fn trans(p: point) -> option<point> {
        let q = { x: p.x - self.x, y: p.y - self.y };
        if (q.x as length) < self.w &&
            (q.y as length) < self.h { some(q) } else { none }
    }
    pure fn contains(p : point) -> bool {
        self.trans(p) != none
    }
    pure fn +(other: rect) -> rect {
        let xl = i16::min(self.x, other.x);
        let yl = i16::min(self.y, other.y);
        let xh = i16::max(self.x + (self.w as coord),
                          other.x + (other.w as coord));
        let yh = i16::max(self.y + (self.h as coord),
                          other.y + (other.h as coord));
        { x: xl, y: yl, w: (xh - xl) as length, h: (yh - yl) as length }
    }
    pure fn *(other: rect) -> rect {
        let xl = i16::max(self.x, other.x);
        let yl = i16::max(self.y, other.y);
        let xh = i16::min(self.x + (self.w as coord),
                          other.x + (other.w as coord));
        let yh = i16::min(self.y + (self.h as coord),
                          other.y + (other.h as coord));
        { x: xl, y: yl, w: (xh - xl) as length, h: (yh - yl) as length }
    }
    pure fn grow(t: coord, r: coord, b: coord, l: coord) -> rect {
        // Parameter order stolen from CSS; blame them.
        {x: self.x - l,
         y: self.y - b,
         w: self.w + ((l + r) as length),
         h: self.h + ((t + b) as length)}
    }
    fn iter_x(f: fn(coord)) {
        let mut x = self.x;
        let xh = self.x + (self.w as coord);
        while x != xh { f(x); x += 1; }
    }
    fn iter_y(f: fn(coord)) {
        let mut y = self.y;
        let yh = self.y + (self.h as coord);
        while y != yh { f(y); y += 1; }
    }
    fn iter(f: fn(point)) {
        do self.iter_y |y| {
            do self.iter_x |x| {
                f({ x: x, y: y })
            }
        }
    }
}

impl geom for point {
    pure fn box() -> rect {
        { x: self.x, y: self.y, w: 1, h: 1 }
    }
    pure fn step(d : dir) -> point {
        alt d {
          left  {{ x: self.x - 1 with self }}
          right {{ x: self.x + 1 with self }}
          up    {{ y: self.y + 1 with self }}
          down  {{ y: self.y - 1 with self }}
        }
    }
}
