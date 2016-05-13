pub struct Rect {
    pub left:   f32,
    pub top:    f32,
    pub right:  f32,
    pub bottom: f32,
}

impl Rect {
    pub fn make_ltrb(l: f32, t: f32, r: f32, b: f32) -> Rect {
        let mut rect = Rect{left: 0f32, top: 0f32, right: 0f32, bottom: 0f32};
        rect.set_ltrb(l,t,r,b);
        rect
    }
    pub fn make_xywh(x: f32, y: f32, w: f32, h: f32) -> Rect {
        let mut rect = Rect{left: 0f32, top: 0f32, right: 0f32, bottom: 0f32};
        rect.set_xywh(x, y, w, h);
        rect
    }
    pub fn make_wh(w: f32, h: f32) -> Rect {
        let mut rect = Rect{left: 0f32, top: 0f32, right: 0f32, bottom: 0f32};
        rect.set_wh(w,h);
        rect
    }

    pub fn left(&self)   -> f32 { self.left   }
    pub fn top(&self)    -> f32 { self.top    }
    pub fn right(&self)  -> f32 { self.right  }
    pub fn bottom(&self) -> f32 { self.bottom }

    pub fn x(&self)      -> f32 { self.left() }
    pub fn y(&self)      -> f32 { self.top()  }
    pub fn width(&self)  -> f32 { self.right()  - self.left() }
    pub fn height(&self) -> f32 { self.bottom() - self.top()  }

    pub fn is_empty(&self) -> bool {
        self.left() >= self.right() || self.top() >= self.bottom()
    }

    pub fn set_ltrb(&mut self, l: f32, t: f32, r: f32, b: f32) {
        self.left   = l;
        self.top    = t;
        self.right  = r;
        self.bottom = b;
    }
    pub fn set_xywh(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.left   = x;
        self.top    = y;
        self.right  = x + w;
        self.bottom = y + h;
    }
    pub fn set_wh(&mut self, w: f32, h: f32) {
        self.left   = 0f32;
        self.top    = 0f32;
        self.right  = w;
        self.bottom = h;
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
        let r = self.right.max(other.right);
        let b = self.bottom.max(other.bottom);

        l < r && t < b
    }
    pub fn intersect(&mut self, other: &Rect) -> bool {
        let l = self.left.max(other.left);
        let t = self.top.max(other.top);
        let r = self.right.max(other.right);
        let b = self.bottom.max(other.bottom);
    
        if l < r && t < b {
            self.set_ltrb(l,t,r,b);
            true
        } else {
            false
        }
    }
}
