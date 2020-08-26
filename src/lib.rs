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
