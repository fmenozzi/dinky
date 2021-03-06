pub struct Rect {
    left:   f32,
    top:    f32,
    right:  f32,
    bottom: f32,
}

impl Rect {
    pub fn make_ltrb(l: f32, t: f32, r: f32, b: f32) -> Rect {
        Rect{left: l, top: t, right: r, bottom: b}
    }
    pub fn make_xywh(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect{left: x, top: y, right: x + w, bottom: y + h}
    }
    pub fn make_wh(w: f32, h: f32) -> Rect {
        Rect{left: 0.0, top: 0.0, right: w, bottom: h}
    }

    pub fn left(&self)   -> f32 { self.left   }
    pub fn top(&self)    -> f32 { self.top    }
    pub fn right(&self)  -> f32 { self.right  }
    pub fn bottom(&self) -> f32 { self.bottom }

    pub fn x(&self)      -> f32 { self.left() }
    pub fn y(&self)      -> f32 { self.top()  }
    pub fn width(&self)  -> f32 { self.right()  - self.left() }
    pub fn height(&self) -> f32 { self.bottom() - self.top()  }

    pub fn empty(&self) -> bool {
        self.left() >= self.right() || self.top() >= self.bottom()
    }

    pub fn round(&self) -> Rect {
        let round_left   = self.left.floor();
        let round_top    = self.top.floor();
        let round_right  = self.right.floor();
        let round_bottom = self.bottom.floor();

        Rect::make_ltrb(round_left, round_top, round_right, round_bottom)
    }

    pub fn offset(&mut self, dx: f32, dy: f32) {
        self.left   += dx;
        self.top    += dy;
        self.right  += dx;
        self.bottom += dy;
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        let l = self.left.max(other.left);
        let t = self.top.max(other.top);
        let r = self.right.min(other.right);
        let b = self.bottom.min(other.bottom);

        l < r && t < b
    }
    pub fn intersect(&mut self, other: &Rect) -> bool {
        let l = self.left.max(other.left);
        let t = self.top.max(other.top);
        let r = self.right.min(other.right);
        let b = self.bottom.min(other.bottom);
    
        if l < r && t < b {
            self.left   = l;
            self.top    = t;
            self.right  = r;
            self.bottom = b;
            true
        } else {
            false
        }
    }
}
