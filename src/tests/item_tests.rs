#[cfg(test)]
mod item_tests {
    use crate::models::{
        cdate::CDate,
        domain::{
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
        assert_eq!(*monopoly.get_day_of_creation(), CDate::new());
    }
}
