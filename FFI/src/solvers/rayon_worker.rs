use std::fs::{self, DirEntry};

use anyhow::Error;
use rayon::prelude::*;

pub struct RayonWorker;

impl RayonWorker {
    pub fn sum_of_squares(input: Vec<i32>) -> i32 {
        input.par_iter().map(|&i| i * i).sum()
    }

    pub fn read_configs(dir_path: String) -> anyhow::Result<Vec<String>>{
        // todo: 
        // Read dir_path - get list of files
        // iterative read it in parallel
        // return list of 'name's
        // write test
        let mut path_str = vec![];
        let paths = fs::read_dir(dir_path)?;
        for path in paths {
             path_str.push(path?.path().to_str().unwrap().to_string());
        }
        let result = path_str.par_iter().map(|file_path|{
            // some action
            unimplemented!()
        }).collect::<Vec<String>>();
    
        
        Ok(result)
    }
}
