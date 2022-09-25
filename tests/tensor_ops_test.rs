use tensorust::Tensor;

#[test]
fn test_tensor_add() {
    let tone: Tensor = Tensor::new(vec![3, 1, 4], vec![1.0; 12]);
    let ttwo: Tensor = Tensor::new(vec![3, 1, 4], vec![3.0; 12]);
    let tadd: Tensor = tone + ttwo;
    assert_eq!(*tadd.dims(), vec![3, 1, 4]);
    assert_eq!(*tadd.data(), vec![4.0; 12]);
}
