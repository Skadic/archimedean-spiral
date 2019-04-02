use std::rc::Rc;

pub struct CartesianPos {
    pub x: f64,
    pub y: f64,
}

impl CartesianPos {
    pub fn to_polar(&self) -> PolarPos{
        PolarPos {
            r: (self.x.powi(2) + self.y.powi(2)).sqrt(),
            phi: f64::atan2(self.y, self.x)
        }
    }
}

pub struct PolarPos {
    pub r: f64,
    pub phi: f64,
}

impl PolarPos {
    pub fn to_cartesian(&self) -> CartesianPos {
        CartesianPos {
            x: self.r * (self.phi.cos()),
            y: self.r * (self.phi.sin())
        }
    }
}

pub struct ArchimedeanSpiral {
    a: f64,
    b: f64,
    initial: Rc<[f64; 4]>,
    step: f64,
}

impl ArchimedeanSpiral {
    pub fn new(a: f64, b: f64) -> ArchimedeanSpiral {
        const STEP: f64 = 0.1;

        let CartesianPos {x, y} = PolarPos {r: a + b , phi: STEP}.to_cartesian();
        ArchimedeanSpiral {
            a,
            b,
            initial: Rc::new([0.0, 0.0, x, y]),
            step: STEP
        }
    }

    pub fn a(&self) -> f64 {
        self.a
    }

    pub fn b(&self) -> f64 {
        self.b
    }
}

impl Iterator for ArchimedeanSpiral {
    type Item = Rc<[f64; 4]>;

    fn next(&mut self) -> Option<Self::Item> {
        let old_pos = CartesianPos { x: self.initial[2], y: self.initial[3]};
        let new_phi = (old_pos.to_polar().r - self.a) / self.b + self.step;
        let new_pos = PolarPos {r: self.a + self.b * new_phi , phi: new_phi}.to_cartesian();
        self.initial = Rc::new([old_pos.x, old_pos.y, new_pos.x, new_pos.y]);
        Some(Rc::clone(&self.initial))
    }
}