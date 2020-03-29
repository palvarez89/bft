#[derive(Debug)]
pub struct VirtualMachine<Number> {
    memory: Vec<Number>,
    head: usize,
}

impl<Number: Clone + num_traits::Num> VirtualMachine<Number> {
    pub fn new(size: usize, elastic: bool) -> VirtualMachine<Number> {
        let size = match size {
            0 => 30000,
            n => n,
        };

        VirtualMachine {
            memory: vec![Number::zero(); size],
            head: 0,
        }
    }
}

#[cfg(test)]
mod tests {}
