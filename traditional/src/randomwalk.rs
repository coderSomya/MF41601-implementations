use rand::Rng;

pub fn y(x: &Vec<f64>) -> f64 {
    assert_eq!(x.len(), 2);
    let x1 = x[0];
    let x2 = x[1];
    4.0 * x1 * x1 + 3.0 * x2 * x2 - 6.0 * x1 * x2 - 4.0 * x1
}

pub fn random_vector(m: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();

    let mut r_vec = vec![0.0; m];
    let mut norm: f64 = 0.0;

    for i in 0..m {
        r_vec[i] = rng.gen_range(-1.0..1.0);
        norm += r_vec[i] * r_vec[i];
    }

    norm = norm.sqrt();
    for i in 0..m {
        r_vec[i] /= norm;
    }

    r_vec
}

pub fn random_walk<F>(
    mut x: Vec<f64>,
    lambda: f64,
    epsilon: f64,
    max_iter: u8,
    f: F,
) -> (Vec<f64>, f64)
where
    F: Fn(&Vec<f64>) -> f64,
{
    let mut lambda = lambda;
    let mut f1 = f(&x);
    let m = x.len();

    for _ in 0..max_iter {
        let u = random_vector(m);
        let x2: Vec<f64> = x.iter().zip(&u).map(|(xi, ui)| xi + lambda * ui).collect();

        let f2 = f(&x);

        if f2 < f1 {
            x = x2;
            f1 = f2;
        } else {
            lambda = 0.5 * lambda;
        }

        if lambda < epsilon {
            break;
        }
    }

    (x, f1)
}

mod tests {
    use super::*;
    #[test]
    fn test_y() {
        let v = vec![0.0, 1.0];
        let a = y(&v);
        assert_eq!(a, 3.0)
    }
    #[test]
    fn test_random() {
        let m = 2; // Number of variables
        let mut rng = rand::thread_rng();
        let initial_x: Vec<f64> = (0..m).map(|_| rng.gen_range(-10.0..10.0)).collect();
        let lambda = 1.0;
        let epsilon = 1e-5;
        let max_iter = 100;

        let (optimal_x, optimal_f) = random_walk(initial_x, lambda, epsilon, max_iter, y);
        assert_ne!(optimal_x[0], 0.0);
        assert_ne!(optimal_x[1], 0.0);
    }
}
