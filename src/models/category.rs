#[derive(Debug, Clone, PartialEq)]
pub enum Category {
    Tool,
    Vehicle,
    Game,
    Toy,
    Sport,
    Other,
}

impl Default for Category {
    fn default() -> Self {
        Category::Other
    }
}
