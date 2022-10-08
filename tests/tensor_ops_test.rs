use tensorust::Tensor;
use tensorust::Size;

#[test]
fn test_tensor_add() {
    let tone: Tensor = Tensor::new(Size::from_vec(vec![3, 1, 4]), vec![1.0; 12]);
    let ttwo: Tensor = Tensor::new(Size::from_vec(vec![3, 1, 4]), vec![3.0; 12]);
    let tadd: Tensor = tone.add(&ttwo);
    assert_eq!(*tadd.dims().dims(), vec![3, 1, 4]);
    assert_eq!(*tadd.data(), vec![4.0; 12]);
    println!("tone func: {:?}", tone.ctx());
    println!("ttwo func: {:?}", ttwo.ctx());
    println!("tadd func: {:?}", tadd.ctx());
}

