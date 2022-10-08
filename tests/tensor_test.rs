use tensorust::Tensor;

#[test]
fn test_tensor_new() {
    let t: Tensor = Tensor::new(vec![2, 3], vec![2.0; 6]);
    assert_eq!(*t.dims(), vec![2, 3]);
    assert_eq!(*t.data(), vec![2.0; 6]);
}

#[test]
fn test_tensor_zeros() {
    let t: Tensor = Tensor::zeros(vec![256, 3, 128, 128]);
    assert_eq!(*t.dims(), vec![256, 3, 128, 128]);
    assert_eq!(*t.data(), vec![0.0; 256 * 3 * 128 * 128]);
}

#[test]
fn test_tensor_ones() {
    let t: Tensor = Tensor::ones(vec![256, 3, 128, 128]);
    assert_eq!(*t.dims(), vec![256, 3, 128, 128]);
    assert_eq!(*t.data(), vec![1.0; 256 * 3 * 128 * 128]);
}

