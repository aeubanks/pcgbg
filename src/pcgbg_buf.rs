#[cfg(test)]
use approx::assert_relative_eq;
use ndarray::{Array2, Array3, Axis, RemoveAxis};

pub struct Buf {
    pub width: usize,
    pub height: usize,
    // 2D array of RGB values
    vals: Array3<f64>,
}

impl Buf {
    pub fn new(width: usize, height: usize) -> Self {
        Buf {
            width,
            height,
            vals: Array3::zeros((width, height, 3)),
        }
    }

    pub fn add(&mut self, plane: &dyn ValuePlane, color_scales: &[f64]) {
        let num_colors = self.vals.raw_dim()[2];
        assert_eq!(color_scales.len(), num_colors);
        let mut min = std::f64::MAX;
        let mut max = std::f64::MIN;
        let plane_vals_dim = self.vals.raw_dim().remove_axis(Axis(2));
        let mut plane_vals = Array2::<f64>::zeros(plane_vals_dim);
        for (i, j) in ndarray::indices(self.vals.raw_dim().remove_axis(Axis(2))) {
            let val = plane.val(i as f64, j as f64);
            min = min.min(val);
            max = max.max(val);
            plane_vals[[i, j]] = val;
        }
        for (i, j) in ndarray::indices(self.vals.raw_dim().remove_axis(Axis(2))) {
            let val = normalized(plane_vals[[i, j]], min, max);
            for (c, color_scale) in color_scales.iter().enumerate() {
                self.vals[[i, j, c]] += val * color_scale;
            }
        }
    }

    pub fn normalize(&mut self) {
        let num_colors = self.vals.raw_dim()[2];
        let mut mins = vec![std::f64::MAX; num_colors];
        let mut maxs = vec![std::f64::MIN; num_colors];
        for (i, j) in ndarray::indices(self.vals.raw_dim().remove_axis(Axis(2))) {
            for c in 0..num_colors {
                let val = self.vals[[i, j, c]];
                mins[c] = mins[c].min(val);
                maxs[c] = maxs[c].max(val);
            }
        }
        for (i, j) in ndarray::indices(self.vals.raw_dim().remove_axis(Axis(2))) {
            for c in 0..num_colors {
                self.vals[[i, j, c]] = normalized(self.vals[[i, j, c]], mins[c], maxs[c]);
            }
        }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> f64 {
        self.vals[[x, y, z]]
    }
}

pub trait ValuePlane {
    fn val(&self, x: f64, y: f64) -> f64;
}

fn normalized<F: num_traits::Float>(val: F, min: F, max: F) -> F {
    (val - min) / (max - min)
}

#[test]
fn test_normalized() {
    assert_relative_eq!(0.5, normalized(2.0, 1.0, 3.0));
}
