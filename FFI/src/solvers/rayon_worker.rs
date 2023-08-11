use rayon::prelude::*;

pub struct RayonWorker;

impl RayonWorker {
    pub fn sum_of_squares(input: Vec<i32>) -> i32 {
        input.par_iter().map(|&i| i * i).sum()
    }
}
