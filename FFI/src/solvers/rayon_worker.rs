use rayon::prelude::*;

pub(crate) struct RayonWorker;

impl RayonWorker {
    pub(crate) fn sum_of_squares(input: Vec<i32>) -> i32 {
        input.par_iter()
            .map(|&i| i * i)
            .sum()
    }
}
