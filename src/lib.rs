pub use std::ops::Range;

pub fn search(
    range: Range<f64>, init: f64, eps: f64,
    der: impl Fn(f64) -> f64, der2: impl Fn(f64) -> f64) -> f64 {
    
    assert!((range.start..=range.end).contains(&init));
    assert!(eps > 0.0);
    
    let mut x = init;
    let mut derx = der(x);
    while derx.abs() > eps {
        x -= derx / der2(x);
        derx = der(x);
    }
    
    x
}

pub fn search_with_n(
    range: Range<f64>, init: f64, n: usize,
    der: impl Fn(f64) -> f64, der2: impl Fn(f64) -> f64) -> f64 {
    
    assert!((range.start..=range.end).contains(&init));
    assert!(n > 0);
    
    let mut x = init;
    for _ in 0..n {
        x -= der(x) / der2(x);
    }
    
    x
}
