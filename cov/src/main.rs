use std::collections::HashSet;

fn main() {
    let x = vec![5., 7., 3., 6., 8., 1.];
    let y = vec![65., 80., 50., 70., 90., 100.];
    println!("Covariance: {}", cov(&x, &y).unwrap());
    println!("Correlation: {}", cor(&x, &y).unwrap());
    println!("Xi Cor: {}", xicor(&x, &y));
    println!("Xi dups: {}", xicor(&vec![1.,1.,1.], &vec![1.,2.,3.]));
    //println!("sortperm: {:?}", sortperm(&x));
    //println!("sortperm: {:?}", sortperm(&y));
    //println!("sortperm: {:?}", sortperm(&vec![1.,2.,3.,4.,5.]));
    //println!("sortperm: {:?}", sortperm(&vec![5.,4.,3.,2.,1.]));
}

fn cov(x: &Vec<f64>, y: &Vec<f64>) -> Result<f64, ()> {
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

fn cor(x: &Vec<f64>, y: &Vec<f64>) -> Result<f64, ()> {
    cov(&scale(x), &scale(y))
}

fn scale(v: &Vec<f64>) -> Vec<f64> {
    let mu = mean(v);
    let sigma = sd(v);
    v.iter().map(|x| {
        (x - mu) / sigma
    }).collect()
}

fn mean(v: &Vec<f64>) -> f64 {
    v.iter().sum::<f64>() / (v.len() as f64)
}

fn sd(v: &Vec<f64>) -> f64 {
    let mu = mean(v);
    let variance = v.iter().map(|x| {
        (x - mu).powi(2)
    });
    let n = v.len() as f64;
    (variance.sum::<f64>() / (n - 1.0)).sqrt()
}

#[allow(dead_code)]
fn sortperm(v: &Vec<f64>) -> Vec<usize> {
    let mut i: Vec<usize> = (0..v.len()).collect();
    i.sort_by(|&a,&b| (v[a]).total_cmp(&v[b]));
    i
}

#[allow(dead_code)]
fn isunique(v: &Vec<f64>) -> bool {
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
fn xicor(x: &Vec<f64>, y: &Vec<f64>) -> f64 {
    // This implementation does not handle the case of duplicate values.
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

fn xicor_duplicates(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len();
    
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a,&b| x[a].total_cmp(&x[b]));

    let r: Vec<usize> = order.iter().map(|&i| {
        order.iter().filter(|&&j| y[j] <= y[i]).count()
    }).collect();

    let l: Vec<usize> = order.iter().map(|&i| {
        order.iter().filter(|&&j| y[j] >= y[i]).count()
    }).collect();

    let rsum: f64 = (1..n).map(|i| (r[i] as f64 - r[i-1] as f64).abs()).sum();
    let lsum: f64 = (0..n).map(|i| (l[i] * (n - l[i])) as f64).sum();

    1.0 - (n as f64 * rsum) / (2.0 * lsum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quadratic() {
        let x = vec![-5.0_f64,-4.,-3.,-2.,-1.,0.,1.,2.,3.,4.,5.];
        let y = x.iter().map(|&i| i.powi(2)).collect();
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
    fn xicor1() {
        let x = vec![5., 7., 3., 6., 8.];
        let y = vec![65., 80., 50., 70., 90.];
        assert_eq!(0.5, xicor(&x, &y));
        assert_eq!(0.5, xicor_duplicates(&x, &y));
    }

    #[test]
    fn xicor2() {
        let x = vec![21.0, 21.0, 22.8, 21.4, 18.7, 18.1, 14.3,
            24.4, 22.8, 19.2, 17.8, 16.4, 17.3, 15.2, 10.4, 10.4, 14.7, 32.4,
            30.4, 33.9, 21.5, 15.5, 15.2, 13.3, 19.2, 27.3, 26.0, 30.4, 15.8,
            19.7, 15.0,21.4
        ];
        let y = vec![2.620, 2.875, 2.320, 3.215, 3.440, 3.460,
            3.570, 3.190, 3.150, 3.440, 3.440, 4.070, 3.730, 3.780, 5.250,
            5.424, 5.345, 2.200, 1.615, 1.835, 2.465, 3.520, 3.435, 3.840,
            3.845, 1.935, 2.140, 1.513, 3.170, 2.770, 3.570, 2.780
        ];
        // We don't quite reproduce the reference value from the XICOR R package.
        assert_eq!(0.5416058, xicor_duplicates(&x, &y));
    }

    #[test]
    fn xicor3() {
        let x = vec![21.0, 21.0, 22.8, 21.4, 18.7, 18.1];
        let y = vec![2.620, 2.875, 2.320, 3.215, 3.440, 3.460];
        assert_eq!(xicor(&x, &y), xicor_duplicates(&x, &y));
        assert_eq!(0.22857142857142853654, xicor_duplicates(&x, &y));
    }

    #[test]
    fn xicor_mtcars() {
        let mpg = vec![21.0, 21.0, 22.8, 21.4, 18.7, 18.1, 14.3,
        24.4, 22.8, 19.2, 17.8, 16.4, 17.3, 15.2, 10.4, 10.4, 14.7, 32.4, 30.4, 
        33.9, 21.5, 15.5, 15.2, 13.3, 19.2, 27.3, 26.0, 30.4, 15.8, 19.7,
        15.0, 21.4];
        let cyl = vec![6.0, 6.0, 4.0, 6.0, 8.0, 6.0, 8.0, 4.0, 4.0,
        6.0, 6.0, 8.0, 8.0, 8.0, 8.0, 8.0, 8.0, 4.0, 4.0, 4.0, 4.0, 8.0, 8.0,
        8.0, 8.0, 4.0, 4.0, 4.0, 8.0, 6.0, 8.0, 4.0];
        assert_eq!(0.32729384436701514094, xicor_duplicates(&mpg, &cyl));
        // Extremely different from what R gives us :-( 
    }

}