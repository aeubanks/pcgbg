use crate::pcgbg_buf::ValuePlane;
#[cfg(test)]
use approx::assert_relative_eq;
use num_complex::Complex64;
use rand::distributions::Distribution;

pub struct ComplexEntryDistribution {
    pub width: usize,
    pub height: usize,
}

impl Distribution<ComplexEntry> for ComplexEntryDistribution {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> ComplexEntry {
        let mut coefficients = Vec::new();
        for _ in 0..3 {
            coefficients.push(rng.gen_range(0.1..3.0));
        }
        let mut points = Vec::new();
        for _ in 0..5 {
            points.push(Complex64::from_polar(
                rng.gen_range(0.1..1.5),
                rng.gen_range(0.0..(2.0 * std::f64::consts::PI)),
            ));
        }
        ComplexEntry {
            width: self.width as f64,
            height: self.height as f64,
            coefficients,
            iterations: 4,
            points,
        }
    }
}

pub struct ComplexEntry {
    width: f64,
    height: f64,
    coefficients: Vec<f64>,
    iterations: usize,
    points: Vec<Complex64>,
}

impl ComplexEntry {
    fn complex_value_for_input(&self, x: f64, y: f64) -> Complex64 {
        let min = self.width.min(self.height);
        Complex64::new(
            (x - self.width / 2.0) / min * 2.0,
            (y - self.height / 2.0) / min * 2.0,
        )
    }

    fn multiply(&self, mut val: Complex64) -> Complex64 {
        for _ in 0..self.iterations {
            let mut next_val = Complex64::new(0.0, 0.0);
            for (i, c) in self.coefficients.iter().enumerate() {
                next_val += val.powi(i as i32) * c;
            }
            val = next_val;
        }
        val
    }

    fn dist(a: Complex64, b: Complex64) -> f64 {
        (a - b).norm()
    }

    fn closest_point(&self, val: Complex64) -> usize {
        let mut closest_index = 0;
        let mut closest_dist = Self::dist(val, self.points[0]);
        for (i, p) in self.points.iter().enumerate() {
            let dist = Self::dist(val, *p);
            if dist < closest_dist {
                closest_dist = dist;
                closest_index = i;
            }
        }
        closest_index
    }
}

impl ValuePlane for ComplexEntry {
    fn val(&self, x: f64, y: f64) -> f64 {
        let mut val = self.complex_value_for_input(x, y);
        val = self.multiply(val);
        self.closest_point(val) as f64 / (self.points.len() - 1) as f64
    }
}

#[test]
fn test_complex() {
    let c = ComplexEntry {
        width: 10.0,
        height: 20.0,
        coefficients: vec![1.0, 2.0, 3.0],
        iterations: 2,
        points: vec![
            Complex64::new(1.0, 1.0),
            Complex64::new(-1.0, 1.0),
            Complex64::new(1.0, -1.0),
            Complex64::new(-1.0, -1.0),
        ],
    };
    assert_relative_eq!(
        c.complex_value_for_input(0.0, 0.0),
        Complex64::new(-1.0, -2.0)
    );
    assert_relative_eq!(
        c.complex_value_for_input(5.0, 10.0),
        Complex64::new(0.0, 0.0)
    );
    assert_relative_eq!(
        c.complex_value_for_input(10.0, 20.0),
        Complex64::new(1.0, 2.0)
    );
    assert_relative_eq!(
        ComplexEntry::dist(Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0)),
        1.0
    );
    assert_relative_eq!(
        ComplexEntry::dist(Complex64::new(0.0, 0.0), Complex64::new(0.0, 1.0)),
        1.0
    );
    assert_relative_eq!(
        ComplexEntry::dist(Complex64::new(0.0, 0.0), Complex64::new(1.0, 1.0)),
        2.0_f64.sqrt()
    );
    assert_relative_eq!(
        ComplexEntry::dist(Complex64::new(-2.0, -1.0), Complex64::new(-1.0, 0.0)),
        2.0_f64.sqrt()
    );
    assert_relative_eq!(
        c.multiply(Complex64::new(0.0, 0.0)),
        Complex64::new(6.0, 0.0)
    );
    assert_relative_eq!(
        c.multiply(Complex64::new(1.0, 1.0)),
        Complex64::new(-158.0, 160.0)
    );
    assert_eq!(c.closest_point(Complex64::new(1.0, 1.0)), 0);
    assert_eq!(c.closest_point(Complex64::new(2.0, 1.0)), 0);
    assert_eq!(c.closest_point(Complex64::new(-1.0, 1.0)), 1);
    assert_eq!(c.closest_point(Complex64::new(-1.5, 1.5)), 1);
    assert_eq!(c.closest_point(Complex64::new(1.5, -1.5)), 2);
    assert_eq!(c.closest_point(Complex64::new(-3.0, -0.5)), 3);
}
