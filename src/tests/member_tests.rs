#[cfg(test)]
mod member_test {

    use crate::{models::domain::member::Member, types::Validate};

    #[test]
    fn test_new_creation() {
        let name = "Bob".to_owned();
        let email = "bob@gmail.com".to_owned();
        let phone_nr = "40123456789".to_owned();
        let bob = Member::new(name, email, phone_nr, 0).ok().unwrap();
        assert_eq!(bob.get_name(), "Bob");
        assert_eq!(bob.get_email(), "bob@gmail.com");
        assert_eq!(bob.get_phone_nr(), "40123456789");
        assert_eq!(bob.get_credits(), &0f64);
    }

    #[test]
    fn test_builder_creation() {
        let name = "Bob".to_owned();
        let email = "bob@gmail.com".to_owned();
        let phone_nr = "40123456789".to_owned();
        let bob = Member::default()
            .name(name)
            .email(email)
            .phone_nr(phone_nr)
            .build();

        assert_eq!(bob.get_name(), "Bob");
        assert_eq!(bob.get_email(), "bob@gmail.com");
        assert_eq!(bob.get_phone_nr(), "40123456789");
        assert_eq!(bob.get_credits(), &0f64);
    }

    #[test]
    fn test_partial_builder_creation() {
        let name = "Bob".to_owned();
        let email = "bob@gmail.com".to_owned();
        let phone_nr = "40123456789".to_owned();
        let bob = Member::default().name(name).build();

        assert_eq!(bob.get_name(), "Bob");
        assert_eq!(bob.get_email(), "");
        assert_eq!(bob.get_phone_nr(), "");
        assert_eq!(bob.get_credits(), &0f64);

        let bob1 = Member::default().email(email).build();

        assert_eq!(bob1.get_name(), "");
        assert_eq!(bob1.get_email(), "bob@gmail.com");
        assert_eq!(bob1.get_phone_nr(), "");
        assert_eq!(bob1.get_credits(), &0f64);

        let bob2 = Member::default().phone_nr(phone_nr).build();

        assert_eq!(bob2.get_name(), "");
        assert_eq!(bob2.get_email(), "");
        assert_eq!(bob2.get_phone_nr(), "40123456789");
        assert_eq!(bob2.get_credits(), &0f64);
    }
    #[test]
    fn test_default_creation() {
        let member = Member::default();
        assert_eq!(member.get_name(), &String::new());
        assert_eq!(member.get_email(), &String::new());
        assert_eq!(member.get_phone_nr(), &String::new());
        assert_eq!(member.get_credits(), &0f64);
        assert_eq!(member.get_uuid().get_len(), &6);
    }

    #[test]
    fn test_invalid_email() {
        let allan = Member::default()
            .name("Allan".to_owned())
            .email("abc".to_owned())
            .phone_nr("4612312312".to_owned())
            .validate_and_build();

        assert_eq!(allan.is_err(), true);
    }

    #[test]
    fn test_phone_nr_contains_str() {
        let allan = Member::default()
            .name("Allan".to_owned())
            .email("some.email@gmail.com".to_owned())
            .phone_nr("abc1298374".to_owned())
            .validate_and_build();

        assert_eq!(allan.is_err(), true);
    }

    #[test]
    fn test_phone_nr_to_long() {
        let allan2 = Member::default()
            .name("Allan".to_owned())
            .email("some.email@gmail.com".to_owned())
            .phone_nr("098765789876567898765678".to_owned())
            .validate_and_build();

        assert_eq!(allan2.is_err(), true);
    }

    #[test]
    fn test_phone_nr_to_short() {
        let allan3 = Member::default()
            .name("Allan".to_owned())
            .email("some.email@gmail.com".to_owned())
            .phone_nr("1234567".to_owned())
            .validate_and_build();

        assert_eq!(allan3.is_err(), true);
    }

    #[test]
    fn test_adding_credits() {
        let mut allan = Member::default()
            .name("Allan".to_owned())
            .email("allan@turing.com".to_owned())
            .phone_nr("4609876543".to_owned())
            .validate_and_build()
            .ok()
            .unwrap();

        assert_eq!(*allan.get_credits(), 0f64);
        allan.add_credits(100f64).ok().unwrap();
        assert_eq!(*allan.get_credits(), 100f64);
    }

    #[test]
    fn test_deducing_credits() {
        let mut allan = Member::default()
            .name("Allan".to_owned())
            .email("allan@turing.com".to_owned())
            .phone_nr("4609876543".to_owned())
            .validate_and_build()
            .ok()
            .unwrap();

        assert_eq!(*allan.get_credits(), 0f64);
        allan.add_credits(100f64).ok().unwrap();
        assert_eq!(*allan.get_credits(), 100f64);
        allan.deduce_credits(100f64).ok().unwrap();
        assert_eq!(*allan.get_credits(), 0f64);
    }

    #[test]
    fn test_invalid_credit_input() {
        let mut allan = Member::default()
            .name("Allan".to_owned())
            .email("allan@turing.com".to_owned())
            .phone_nr("4609876543".to_owned())
            .validate_and_build()
            .ok()
            .unwrap();

        assert_eq!(allan.add_credits(-100f64).is_err(), true);
        assert_eq!(allan.deduce_credits(-100f64).is_err(), true);
        assert_eq!(allan.deduce_credits(100f64).is_err(), true);
    }
}
