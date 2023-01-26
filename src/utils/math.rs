use std::iter::zip;

pub fn hadamard_product(v1: &Vec<f32>, v2: &Vec<f32>) -> Vec<f32> {
    zip(v1, v2).map(|x| x.0 * x.1).collect()
}

pub fn dot_product(v1: &Vec<f32>, v2: &Vec<f32>) -> f32 {
    assert_eq!(v1.len(), v2.len());

    zip(v1, v2).map(|x| x.0 * x.1).sum::<f32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot_product() {
        let res = dot_product(&vec![4.0, 5.0], &vec![10.0, 20.0]);
        assert_eq!(res, 140.0);
    }

    #[test]
    fn test_hadamard_product() {
        let res = hadamard_product(&vec![2.0, 3.0], &vec![4.0, 5.0]);
        assert_eq!(res, vec![8.0, 15.0]);
    }
}
