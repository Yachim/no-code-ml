use std::iter::zip;

pub fn dot_product(v1: &Vec<f32>, v2: &Vec<f32>) -> f32 {
    assert_eq!(v1.len(), v2.len());

    return zip(v1, v2).map(|x| x.0 * x.1).sum::<f32>();
}
