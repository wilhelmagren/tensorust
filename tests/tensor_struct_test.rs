use tensorust::Tensor;

#[test]
fn tensor_new() {
    let t: Tensor = Tensor::new(vec![64, 2], vec![-2.0; 64 * 2]);
    assert_eq!(*t.dims(), vec![64, 2]);
    assert_eq!(*t.data(), vec![-2.0; 64 * 2]);
    assert_eq!(t.requires_grad(), false);
}

#[test]
fn tensor_zeros() {
    let t: Tensor = Tensor::zeros(vec![128, 3, 256, 256]);
    assert_eq!(*t.dims(), vec![128, 3, 256, 256]);
    assert_eq!(*t.data(), vec![0.0; 128 * 3 * 256 * 256]);
}

#[test]
fn tensor_ones() {
    let t: Tensor = Tensor::ones(vec![128, 3, 256, 256]);
    assert_eq!(*t.dims(), vec![128, 3, 256, 256]);
    assert_eq!(*t.data(), vec![1.0; 128 * 3 * 256 * 256]);
}

