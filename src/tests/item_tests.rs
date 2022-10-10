#[cfg(test)]
mod item_tests {

    use crate::models::domain::{
        contract::Contract,
        item::{Category, Item},
        member::Member,
    };

    #[test]
    fn test_new_creation() {
        let name = "Monopoly".to_owned();
        let description = "A Family Game".to_owned();
        let category = Category::Game;
        let cost_per_day = 20f64;
        let owner = Member::default()
            .name("Allan".to_owned())
            .email("allan@turing.com".to_owned())
            .phone_nr("4601234567".to_owned())
            .build();

        let monopoly = Item::new(
            name.clone(),
            description.clone(),
            category.clone(),
            owner.clone(),
            cost_per_day,
            0,
        );
        assert_eq!(*monopoly.get_name(), name);
        assert_eq!(*monopoly.get_description(), description);
        assert_eq!(*monopoly.get_category(), category);
        assert_eq!(*monopoly.get_owner(), owner);
        assert_eq!(*monopoly.get_cost_per_day(), cost_per_day);
        assert_eq!(monopoly.get_history().to_vec(), vec![]);
        assert_eq!(*monopoly.get_is_available(), true);
        assert_eq!(*monopoly.get_day_of_creation(), 0);
    }

    #[test]
    fn test_builder_creation() {
        let name = "Monopoly".to_owned();
        let description = "A Family Game".to_owned();
        let category = Category::Game;
        let cost_per_day = 20f64;
        let owner = Member::default()
            .name("Allan".to_owned())
            .email("allan@turing.com".to_owned())
            .phone_nr("4601234567".to_owned())
            .build();

        let monopoly = Item::default()
            .name(name.clone())
            .description(description.clone())
            .category(category.clone())
            .owner(owner.clone())
            .cost_per_day(cost_per_day)
            .day_of_creation(0)
            .build();

        assert_eq!(*monopoly.get_name(), name);
        assert_eq!(*monopoly.get_description(), description);
        assert_eq!(*monopoly.get_category(), category);
        assert_eq!(*monopoly.get_owner(), owner);
        assert_eq!(*monopoly.get_cost_per_day(), cost_per_day);
        assert_eq!(monopoly.get_history().to_vec(), vec![]);
        assert_eq!(*monopoly.get_is_available(), true);
        assert_eq!(*monopoly.get_day_of_creation(), 0);
    }

    #[test]
    fn test_add_contract() {
        let allan = Member::default()
            .name("Allan".to_owned())
            .email("allan@turing.com".to_owned())
            .phone_nr("4602134567".to_owned())
            .build();
        let bob = Member::default()
            .name("Bob".to_owned())
            .email("bob@gmail.com".to_owned())
            .phone_nr("46291328475".to_owned())
            .credits(500f64)
            .build();
        let mut item = Item::default()
            .name("Monopoly".to_owned())
            .description("A Family Game".to_owned())
            .category(Category::Game)
            .cost_per_day(20f64)
            .owner(allan.clone());

        let contract = Contract::default()
            .owner(allan.clone())
            .lendee(bob.clone())
            .credits(200f64)
            .from_date(0, 10)
            .build();

        assert_eq!(item.add_contract(contract).is_ok(), true);
        let history = item.get_history().to_vec();
        let c = history.first().unwrap();
        assert_eq!(c.get_owner(), &allan);
        assert_eq!(c.get_lendee(), &bob);
        assert_eq!(c.get_credits(), &200f64);
        assert_eq!(c.get_start_date(), &0);
        assert_eq!(c.get_end_date(), &10);
        assert_eq!(c.get_contract_len(), &10);
    }

    #[test]
    fn test_add_contract_in_taken_period() {
        let allan = Member::default()
            .name("Allan".to_owned())
            .email("allan@turing.com".to_owned())
            .phone_nr("4602134567".to_owned())
            .build();
        let bob = Member::default()
            .name("Bob".to_owned())
            .email("bob@gmail.com".to_owned())
            .phone_nr("46291328475".to_owned())
            .credits(500f64)
            .build();
        let mut monopoly = Item::default()
            .name("Monopoly".to_owned())
            .description("A Family Game".to_owned())
            .category(Category::Game)
            .cost_per_day(20f64)
            .owner(allan.clone());

        let c1 = Contract::default()
            .owner(allan.clone())
            .lendee(bob.clone())
            .credits(200f64)
            .from_date(0, 10)
            .build();

        let c2 = Contract::default()
            .owner(allan.clone())
            .lendee(bob.clone())
            .credits(100f64)
            .from_date(3, 5)
            .build();

        assert_eq!(monopoly.add_contract(c1).is_ok(), true);
        assert_eq!(monopoly.add_contract(c2).is_ok(), false);
    }

    #[test]
    fn test_contract_not_enough_funds() {
        let allan = Member::default()
            .name("Allan".to_owned())
            .email("allan@turing.com".to_owned())
            .phone_nr("4602134567".to_owned())
            .build();
        let bob = Member::default()
            .name("Bob".to_owned())
            .email("bob@gmail.com".to_owned())
            .phone_nr("46291328475".to_owned())
            .credits(200f64)
            .build();
        let mut monopoly = Item::default()
            .name("Monopoly".to_owned())
            .description("A Family Game".to_owned())
            .category(Category::Game)
            .cost_per_day(20f64)
            .owner(allan.clone());

        let c1 = Contract::default()
            .owner(allan.clone())
            .lendee(bob.clone())
            .credits(200f64)
            .from_date(0, 10)
            .build();

        let c2 = Contract::default()
            .owner(allan.clone())
            .lendee(bob.clone())
            .credits(400f64)
            .from_date(11, 15)
            .build();

        assert_eq!(monopoly.add_contract(c1).is_ok(), true);
        assert_eq!(monopoly.add_contract(c2).is_ok(), false);
    }
}
