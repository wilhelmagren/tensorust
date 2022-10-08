use tensorust::Tensor;
use tensorust::Size;

#[test]
fn test_tensor_new() {
    let t: Tensor = Tensor::new(Size::from_vec(vec![2, 3]), vec![2.0; 6]);
    assert_eq!(*t.dims().dims(), vec![2, 3]);
    assert_eq!(*t.data(), vec![2.0; 6]);
}

#[test]
fn test_tensor_zeros() {
    let t: Tensor = Tensor::zeros(Size::from_vec(vec![256, 3, 128, 128]));
    assert_eq!(*t.dims().dims(), vec![256, 3, 128, 128]);
    assert_eq!(*t.data(), vec![0.0; 256 * 3 * 128 * 128]);
}

#[test]
fn test_tensor_ones() {
    let t: Tensor = Tensor::ones(Size::from_vec(vec![256, 3, 128, 128]));
    assert_eq!(*t.dims().dims(), vec![256, 3, 128, 128]);
    assert_eq!(*t.data(), vec![1.0; 256 * 3 * 128 * 128]);
}

