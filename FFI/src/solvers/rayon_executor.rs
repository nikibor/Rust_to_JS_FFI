use super::parser::ConfigParser;
use rayon::prelude::*;
use std::fs;

pub struct RayonWorker;

impl RayonWorker {
    pub fn sum_of_squares(input: Vec<i32>) -> i32 {
        input.par_iter().map(|&i| i * i).sum()
    }

    pub fn read_configs(dir_path: &str) -> anyhow::Result<Vec<String>> {
        let mut path_str = vec![];
        let paths = fs::read_dir(dir_path)?;
        for path in paths {
            path_str.push(path?.path().to_str().unwrap().to_string());
        }
        let parser = ConfigParser {
            path_dir: dir_path.to_string(),
        };

        let result = path_str
            .par_iter()
            .map(|file_path| parser.read_config(file_path).unwrap().name)
            .collect::<Vec<String>>();

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::RayonWorker;

    #[test]
    fn config_reader_test() {
        let dir_path = "../test_configs";
        let configs = RayonWorker::read_configs(dir_path).unwrap();
        assert!(configs.len() > 0)
    }
}
