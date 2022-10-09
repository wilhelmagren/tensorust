use tensorust::Tensor;

#[test]
fn tensor_add() {
    let a: Tensor = Tensor::new(vec![3, 1, 4], vec![1.0; 12]);
    let b: Tensor = Tensor::new(vec![3, 1, 4], vec![3.0; 12]);
    let c: Tensor = a.add(&b);
    assert_eq!(*c.dims(), vec![3, 1, 4]);
    assert_eq!(*c.data(), vec![4.0; 12]);
}

