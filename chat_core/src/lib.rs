pub mod storage;
struct CoreConfig {
    database_path: str,
}
struct ChatCore {}
pub fn init() {
    if let Err(e) = storage::init() {}
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
