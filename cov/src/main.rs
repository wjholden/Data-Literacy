use std::collections::HashSet;

fn main() -> Result<(), ()> {
    // df = data.frame(x=c(5., 7., 3., 6., 8., 1.), y=c(65., 80., 50., 70., 90., 100.))
    let x = vec![5., 7., 3., 6., 8., 1.];
    let y = vec![65., 80., 50., 70., 90., 100.];
    println!("Covariance: {}", cov(&x, &y).unwrap());
    println!("Correlation: {}", cor(&x, &y).unwrap());
    println!("Xi Cor: {}", xicor(&x, &y));
    println!("Xi dups: {}", xicor(&vec![1.,1.,1.], &vec![1.,2.,3.]));

    let x = [-5.,-4.,-3.,-2.,-1.,0.,1.,2.,3.,4.,5.];
    let y = [25.,16.,9.,4.,1.,0.,1.,4.,9.,16.,25.];
    println!("Correlation for (-5:5)^2: {}", cor(&x, &y)?);
    println!("Same but xicor:           {}", xicor(&x, &y));
    //println!("sortperm: {:?}", sortperm(&x));
    //println!("sortperm: {:?}", sortperm(&y));
    //println!("sortperm: {:?}", sortperm(&vec![1.,2.,3.,4.,5.]));
    //println!("sortperm: {:?}", sortperm(&vec![5.,4.,3.,2.,1.]));
    Ok(())
}

fn cov(x: &[f64], y: &[f64]) -> Result<f64, ()> {
    if x.len() != y.len() {
        return Err(())
    }
    let n = x.len() as f64;
    let xm = mean(x);
    let ym = mean(y);
    let covariance = x.iter().zip(y.iter()).map(|(a,b)| {
        (a - xm) * (b - ym) / (n - 1.0)
    }).sum::<f64>();
    Ok(covariance)
}

fn cor(x: &[f64], y: &[f64]) -> Result<f64, ()> {
    cov(&scale(x), &scale(y))
}

fn scale(v: &[f64]) -> Vec<f64> {
    let mu = mean(v);
    let sigma = sd(v);
    v.iter().map(|x| {
        (x - mu) / sigma
    }).collect()
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / (v.len() as f64)
}

fn sd(v: &[f64]) -> f64 {
    let mu = mean(v);
    let variance = v.iter().map(|x| {
        (x - mu).powi(2)
    });
    let n = v.len() as f64;
    (variance.sum::<f64>() / (n - 1.0)).sqrt()
}

#[allow(dead_code)]
fn sortperm(v: &[f64]) -> Vec<usize> {
    let mut i: Vec<usize> = (0..v.len()).collect();
    i.sort_by(|&a,&b| (v[a]).total_cmp(&v[b]));
    i
}

#[allow(dead_code)]
fn isunique(v: &[f64]) -> bool {
    let mut h = HashSet::new();
    for &i in v.iter() {
        if h.contains(&i.to_bits()) {
            return false;
        }
        h.insert(i.to_bits());
    }
    true
}

// https://arxiv.org/pdf/1909.10140
// https://towardsdatascience.com/a-new-coefficient-of-correlation-64ae4f260310
// 
fn xicor_distinct(x: &[f64], y: &[f64]) -> f64 {
    // This implementation does not handle the case of duplicate values in y.
    let n = x.len();

    // 1) Sort y by x.
    let mut i: Vec<_> = (0..n).collect();
    i.sort_by(|&a, &b| x[a].total_cmp(&x[b]));
    let y: Vec<f64> = i.iter().map(|&i| y[i]).collect();

    // 2) Order y by sorting 1:n by y.
    let mut order: Vec<_> = (0..n).collect();
    order.sort_by(|&a, &b| y[a].total_cmp(&y[b]));

    // 3) Rank y by sorting 1:n by order.
    let mut r: Vec<_> = (0..n).collect();
    r.sort_by(|&a, &b| order[a].cmp(&order[b]));

    // Sum of absolute distances in successive y ranks.
    let mut r_consec_abs_dist = 0.0;
    for i in 1..n {
        r_consec_abs_dist += (r[i] as f64 - r[i-1] as f64).abs();
    }
    1.0 - 3.0 * r_consec_abs_dist / (n.pow(2) as f64 - 1.0)
}

#[allow(dead_code)]
fn xicor2_original(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len();
    
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a,&b| x[a].total_cmp(&x[b]));

    let r: Vec<usize> = order.iter().map(|&i| {
        order.iter().filter(|&&j| y[j] <= y[i]).count()
    }).collect();

    let l: Vec<usize> = order.iter().map(|&i| {
        order.iter().filter(|&&j| y[j] >= y[i]).count()
    }).collect();

    let mut r_consec_abs_dist = 0.0;
    for i in 1..n {
        r_consec_abs_dist += (r[i] as f64 - r[i-1] as f64).abs();
    }

    let mut l_term = 0.0;
    for i in 0..n {
        l_term += (l[i] * (n - l[i])) as f64;
    }
    1.0 - (n as f64 * r_consec_abs_dist) / (2.0 * l_term)
}

/// This function implements the Chatterjee correlation coefficient where
/// duplicated x values are allowed ((https://arxiv.org/pdf/1909.10140).
/// 
/// This function is written for clarity and is not intended to be optimal.
fn xicor(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len();
    
    // Order of x values. This function does not use randomness.
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a,&b| x[a].total_cmp(&x[b]));

    // r values are the ranks of the y values. The ith y value is the number of
    // j such that y[j] <= y[i]. The order of r values corresponds to the order
    // of x.
    let r: Vec<_> = order.iter().map(|&i| {
        (0..n).filter(|&j| y[j] <= y[i]).count() as f64
    }).collect();

    // l values are just like the r values, only it is y[j] >= y[i].
    let l: Vec<_> = order.iter().map(|&i| {
        (0..n).filter(|&j| y[j] >= y[i]).count() as f64
    }).collect();

    // Sum of absolute differences in consecutive r values.
    let rsum = &r.windows(2).map(|ri| (ri[1] - ri[0]).abs()).sum();

    // Sum of l terms for the denominator.
    let lsum = l.iter().map(|&li| li * (n as f64 - li)).sum::<f64>();

    1. - (n as f64 * rsum) / (2. * lsum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quadratic() {
        let x = vec![-5.0_f64,-4.,-3.,-2.,-1.,0.,1.,2.,3.,4.,5.];
        let y = x.iter().map(|&i| i.powi(2)).collect::<Vec<_>>();
        assert_eq!(0.0, cor(&x, &y).unwrap().round());
    }

    #[test]
    fn rbloggers() {
        // https://www.r-bloggers.com/2023/07/covariance-in-r-with-the-cov-function/
        let x = vec![5., 7., 3., 6., 8.];
        let y = vec![65., 80., 50., 70., 90.];
        assert_eq!(29.0, cov(&x,&y).unwrap());
    }

    #[test]
    fn xicor_test_1() {
        let x = vec![5., 7., 3., 6., 8.];
        let y = vec![65., 80., 50., 70., 90.];
        assert_eq!(0.5, xicor_distinct(&x, &y));
        assert_eq!(0.5, xicor(&x, &y));
    }

    #[test]
    fn xicor_test_2_mtcars() {
        let mpg = vec![21.0, 21.0, 22.8, 21.4, 18.7, 18.1, 14.3,
        24.4, 22.8, 19.2, 17.8, 16.4, 17.3, 15.2, 10.4, 10.4, 14.7, 32.4, 30.4, 
        33.9, 21.5, 15.5, 15.2, 13.3, 19.2, 27.3, 26.0, 30.4, 15.8, 19.7,
        15.0, 21.4];
        let cyl = vec![6.0, 6.0, 4.0, 6.0, 8.0, 6.0, 8.0, 4.0, 4.0,
        6.0, 6.0, 8.0, 8.0, 8.0, 8.0, 8.0, 8.0, 4.0, 4.0, 4.0, 4.0, 8.0, 8.0,
        8.0, 8.0, 4.0, 4.0, 4.0, 8.0, 6.0, 8.0, 4.0];
        // We've learned that anything with duplicates is tricky because of randomness.
        // The official XICOR library (by Chatterjee himself!) produces non-deterministic
        // outputs. So, this statistic should be somewhere in this general range.
        let xi = xicor(&mpg, &cyl);
        assert!(0.7 <= xi && xi <= 0.85);
    }

    #[test]
    fn xicor_test_3() {
        let x = vec![21.0, 21.0, 22.8, 21.4, 18.7, 18.1];
        let y = vec![2.620, 2.875, 2.320, 3.215, 3.440, 3.460];
        assert_eq!(xicor_distinct(&x, &y), xicor(&x, &y));
        assert_eq!(0.22857142857142853654, xicor(&x, &y));
    }

    #[test]
    fn xicor_test_4_unsuccessful_laminator() {
        // from https://github.com/UnsuccessfulLaminator/xicor/blob/127962345556f58c80e896f4af5b85d55feb28f0/src/tests.rs#L26
        let x = [1., 4., -9., -6., -5., -8., -1., 0., -4., -5.];
        let y = [9., 8., 5., -10., 7., -6., -2., -8., 4., 3.];
        // R equivalent:
        // t4 <- data.frame(x = c(1., 4., -9., -6., -5., -8., -1., 0., -4., -5.), y = c(9., 8., 5., -10., 7., -6., -2., -8., 4., 3.))
        assert!((xicor(&x, &y) - 0.0909090909).abs() < 0.0001)
    }

}