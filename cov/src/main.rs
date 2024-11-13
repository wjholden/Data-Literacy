fn main() {
    let x = vec![5., 7., 3., 6., 8.];
    let y = vec![65., 80., 50., 70., 90.];
    println!("Covariance: {}", cov(&x, &y).unwrap());
    println!("Correlation: {}", cor(&x, &y).unwrap());
    println!("sortperm: {:?}", sortperm(&x));
    println!("sortperm: {:?}", sortperm(&y));
    println!("sortperm: {:?}", sortperm(&vec![1.,2.,3.,4.,5.]));
    println!("sortperm: {:?}", sortperm(&vec![5.,4.,3.,2.,1.]));
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

fn sortperm(v: &Vec<f64>) -> Vec<usize> {
    let mut i: Vec<usize> = (1..=v.len()).collect();
    i.sort_by(|&a,&b| (v[a-1]).total_cmp(&v[b-1]));
    i
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
}