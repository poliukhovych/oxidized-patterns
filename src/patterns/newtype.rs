pub struct UserId(u64);

impl UserId {
    pub fn new(id: u64) -> Self {
        UserId(id)
    }
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl From<u64> for UserId {
    fn from(id: u64) -> Self {
        UserId(id)
    }
}

impl Into<u64> for UserId {
    fn into(self) -> u64 {
        self.0
    }
}

pub fn demo() {
    let uid = UserId::new(1001);
    println!("[Newtype demo] UserId value = {}", uid.value());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_newtype_new_and_value() {
        let uid = UserId::new(42);
        assert_eq!(uid.value(), 42);
    }

    #[test]
    fn test_from_into() {
        let uid: UserId = 99u64.into();
        let id: u64 = uid.into();
        assert_eq!(id, 99);
    }
}
