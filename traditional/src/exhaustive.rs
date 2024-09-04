fn y(x1: f64) -> f64 {
    2.0 * (x1 * x1) - (x1 * x1 * x1)
}

pub fn main() -> f64 {
    let mut x_min: f64 = 0.0;
    let mut x_max: f64 = 2.0;
    let n = 10.0;
    let delta = (x_max - x_min) / n;

    let mut x1 = x_min;
    let mut x2 = x_min + delta;
    let mut x3 = x2 + delta;

    let mut did_reach_optima = false;

    loop {
        let f1 = y(x1);
        let f2 = y(x2);
        let f3 = y(x3);

        if (f2 > f1 && f2 > f3) {
            //reached maxima
            did_reach_optima = true;
            break;
        } else {
            //update the values
            x1 = x2;
            x2 = x3;
            x3 = x2 + delta;
        }

        if (x3 > x_max) {
            println!("The maximum does not lie in the range..!");
            break;
        }
    }

    if (did_reach_optima) {
        x2
    } else {
        -1.0
    }
}

mod tests {
    use super::*;
    #[test]
    pub fn test_base() {
        assert_eq!(main(), 1.4);
    }
}
