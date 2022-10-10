#[cfg(test)]
mod uuid_tests {
    use crate::models::uuid::Uuid;

    #[test]
    fn test_len() {
        let uuid = Uuid::default();
        assert_eq!(uuid.get_len(), &6);

        let uuid2 = Uuid::with_len(15);
        assert_eq!(uuid2.get_len(), &15);
    }

    #[test]
    fn test_alphanumeric() {
        let uuid = Uuid::new();
        assert_eq!(
            uuid.get_value()
                .chars()
                .into_iter()
                .all(|c| c.is_alphanumeric()),
            true
        )
    }
}
