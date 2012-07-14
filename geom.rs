type coord = i16;
type length = u16;
type area = u32;
type dist = area;
type score = i64;

type point = { x: coord, y: coord };
type rect = { x: coord, y: coord, w: length, h: length };

impl geom for rect {
    pure fn area() -> area {
        ret (self.w as area) * (self.h as area);
    }
    pure fn trans(p: point) -> option<point> {
        let q = { x: p.x - self.x, y: p.y - self.y };
        if (q.x as length) < self.w &&
            (q.y as length) < self.h { some(q) } else { none }
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
}

impl geom for point {
    pure fn box() -> rect {
        { x: self.x, y: self.y, w: 1, h: 1 }
    }
}

