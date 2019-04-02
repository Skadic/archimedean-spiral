use piston::input::*;
use graphics::math::Matrix2d;
use std::rc::Rc;
use opengl_graphics::GlGraphics;

mod rainbow;
mod archimedean_spiral;

use crate::archimedean_spiral::ArchimedeanSpiral;
use crate::rainbow::RainbowIterator;

pub struct App {
    gl: GlGraphics,
    frame: f64,
    lines: Vec<([f32; 4], Rc<[f64; 4]>)>,
    spiral: ArchimedeanSpiral,
    color: RainbowIterator,
}

impl App {

    pub fn new(gl: GlGraphics) -> App {
        App {
            gl,
            frame: 0.0,
            lines: Vec::new(),
            spiral: ArchimedeanSpiral::new(1.0, 8.0),
            color: RainbowIterator::new(),
        }
    }

    pub fn render(&mut self, args: &RenderArgs){
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let scale = self.spiral.b() * 0.8;

        let (x, y) = (args.width / 2.0,
                      args.height / 2.0);

        let frame = self.frame;
        let lines = &mut self.lines;
        let spiral = &mut self.spiral;
        let color = &mut self.color;
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK,gl);

            let transform: Matrix2d = c.transform.trans(x, y).rot_deg(-90.0).scale(scale / frame, scale / frame);

            let l = spiral.next().unwrap();
            lines.push((color.next().unwrap(), l));

            for (col, l) in lines {
                let ln = *l.as_ref();
                line(*col, 2.0, ln, transform, gl);
            }
        });

    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.frame += args.dt;
    }
}



#[cfg(test)]
mod tests {
    use std::f64::consts;
    use crate::archimedean_spiral::{PolarPos, ArchimedeanSpiral};
    use crate::rainbow::RainbowIterator;

    #[test]
    fn correct_polar_to_cartesian() {
        let polar = PolarPos{ r: 2f64.sqrt(), phi: consts::PI / 4.0};
        println!("{} {}", polar.r, polar.phi);
        let cartesian = polar.to_cartesian();
        println!("{} {}", cartesian.x, cartesian.y);
        let polar = cartesian.to_polar();
        println!("{} {}", polar.r, polar.phi);
    }

    #[test]
    fn test_spiral() {
        let mut spiral = ArchimedeanSpiral::new(0.0, 1.0);

        for line in 0..10 {
            println!("{:?}", spiral.next().unwrap());
        }
    }

    #[test]
    fn test_color_func() {
        use std::f32::consts;
        let red = |x: f32| {
            let n = x % (2.0 * consts::PI);
            if n > consts::PI {
                0.0
            } else {
                n.sin().powi(2)
            }
        };

        for i in 0..63 {
            println!("{}", red((i as f32) / 10.0));
        }
    }

}
