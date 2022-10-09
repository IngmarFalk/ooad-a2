#[cfg(test)]
mod item_tests {
    use crate::models::{
        cdate::CDate,
        domain::{
            contract::Contract,
            item::{Category, Item},
            member::Member,
        },
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
        );
        assert_eq!(*monopoly.get_name(), name);
        assert_eq!(*monopoly.get_description(), description);
        assert_eq!(*monopoly.get_category(), category);
        assert_eq!(*monopoly.get_owner(), owner);
        assert_eq!(*monopoly.get_cost_per_day(), cost_per_day);
        assert_eq!(monopoly.get_history().to_vec(), vec![]);
        assert_eq!(*monopoly.get_is_available(), true);
        assert_eq!(*monopoly.get_day_of_creation(), CDate::now());
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
            .build();

        assert_eq!(*monopoly.get_name(), name);
        assert_eq!(*monopoly.get_description(), description);
        assert_eq!(*monopoly.get_category(), category);
        assert_eq!(*monopoly.get_owner(), owner);
        assert_eq!(*monopoly.get_cost_per_day(), cost_per_day);
        assert_eq!(monopoly.get_history().to_vec(), vec![]);
        assert_eq!(*monopoly.get_is_available(), true);
        assert_eq!(*monopoly.get_day_of_creation(), CDate::now());
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
            .item(item.clone())
            .credits(200f64)
            .from_now_with_days(10)
            .build();

        assert_eq!(item.add_contract(contract).is_ok(), true);
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
            .contract_len(10)
            .build();

        assert_eq!(item.add_contract(contract).is_ok(), true);
    }
}
