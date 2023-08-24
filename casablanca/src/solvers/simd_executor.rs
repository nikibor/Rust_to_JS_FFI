use simd_json;

pub struct SimdExecutor;

impl SimdExecutor {
    pub fn parse_json_pre() -> anyhow::Result<()> {
        let mut d = br#"{"some": ["key", "value", 2]}"#.to_vec();
        let v: simd_json::OwnedValue = simd_json::to_owned_value(&mut d).unwrap();
        println!("{v:?}");
        Ok(())
    }

    pub fn parse_json(val: &str) -> anyhow::Result<()> {
        let mut d = val.as_bytes().to_vec();
        let v: simd_json::OwnedValue = simd_json::to_owned_value(&mut d).unwrap();
        println!("{v:?}");
        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::SimdExecutor;

//     #[test]
//     fn simd_test() {
//         let x = SimdExecutor::parse_json();
//     }
// }
