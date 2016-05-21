use point::Point;

pub struct Matrix {
    mat: [f32; 6],
}

impl Matrix {
    pub fn new(mat: [f32; 6]) -> Matrix {
        Matrix {
            mat: mat,
        }
    }

    pub fn identity() -> Matrix {
        Matrix {
            mat: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
        }
    }

    pub fn apply(&self, point: &Point) -> Point {
        let (a, b, c) = (self.mat[0], self.mat[1], self.mat[2]);
        let (d, e, f) = (self.mat[3], self.mat[4], self.mat[5]);

        let (x, y) = (point.x, point.y);

        let xp = a*x + b*y + c;
        let yp = d*x + e*y + f;

        Point::new(xp, yp)
    }

    pub fn inv(&self) -> Matrix {
        let mut res = [0.0; 6];

        let (a, b, c) = (self.mat[0], self.mat[1], self.mat[2]);
        let (d, e, f) = (self.mat[3], self.mat[4], self.mat[5]);

        let (ae, bd) = (a*e, b*d);

        let aebd_inv = 1.0 / (ae - bd);
        let bdae_inv = 1.0 / (bd - ae);

        res[0] = aebd_inv * e;
        res[1] = bdae_inv * b;
        res[2] = bdae_inv * (c*e - b*f);
        res[3] = bdae_inv * d;
        res[4] = aebd_inv * a;
        res[5] = aebd_inv * (c*d - a*f);

        Matrix::new(res)
    }

    pub fn mul(&self, other: &Matrix) -> Matrix {
        let mut res = [0.0; 6];

        let (a, b, c) = (self.mat[0], self.mat[1], self.mat[2]);
        let (d, e, f) = (self.mat[3], self.mat[4], self.mat[5]);

        let (g, h, i) = (other.at(0), other.at(1), other.at(2));
        let (j, k, l) = (other.at(3), other.at(4), other.at(5));

        res[0] = a*g + b*j;
        res[1] = a*h + b*k;
        res[2] = a*i + b*l + c;
        res[3] = d*g + e*j;
        res[4] = d*h + e*k;
        res[5] = d*i + e*l + f;

        Matrix::new(res)
    }

    pub fn at(&self, i: usize) -> f32 {
        self.mat[i]
    }

    pub fn get_floats(&self) -> [f32; 6] {
        self.mat
    }
}

/*
impl Index<usize> for Matrix {
    type Output = f32;
    fn index(&self, i: usize) -> f32 {
        self.mat[i]
    }
}
*/
