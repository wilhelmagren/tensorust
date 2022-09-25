use tensorust::Tensor;

#[test]
fn test_tensor_new() {
    let t: Tensor = Tensor::new(vec![2, 3], vec![2.0; 6]);
    assert_eq!(*t.dims(), vec![2, 3]);
    assert_eq!(*t.data(), vec![2.0; 6]);
}
