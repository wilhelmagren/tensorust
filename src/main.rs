#[derive(Debug)]
struct Function<'a> {
    parents: Vec<&'a Tensor<'a>>,
    saved_tensors: Vec<&'a Tensor<'a>>,
    requires_grad: bool
}

impl<'a> Function<'a> {
    fn none() -> Self {
        Self { parents: vec![], saved_tensors: vec![], requires_grad: false }
    }

    fn add(a: &'a Tensor, b: &'a Tensor) -> Self {
        Self { parents: vec![a, b], saved_tensors: vec![], requires_grad: false }
    }
}

#[derive(Debug)]
struct Tensor<'a> {
    dims: Vec<usize>,
    data: Vec<f32>,
    ctx: Function<'a>,
    requires_grad: bool
}

impl<'a> Tensor<'a> {
    fn zeros(dims: Vec<usize>) -> Self {
        let nsamples: usize = dims.iter().product();
        Self { dims: dims, data: vec![0.0; nsamples],
        requires_grad: false, ctx: Function::none() }
    }

    fn add(&'a self, other: &'a Tensor) -> Self {
        let nsamples: usize = self.dims.iter().product();
        Self { dims: vec![3, 4], data: vec![0.0; nsamples],
        requires_grad: false, ctx: Function::add(self, other) }
    }
}


fn main() {
    let a: Tensor = Tensor::zeros(vec![3, 4]);
    let b: Tensor = Tensor::zeros(vec![3, 4]);
    let c: Tensor = a.add(&b);
    println!("Resulting Tensor: {:?}", c);
}
