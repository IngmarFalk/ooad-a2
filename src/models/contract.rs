use chrono::Local;

pub trait ContractValidation {
    fn validate_credits() -> bool;
    fn validate_availability() -> bool;
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Contract {
    s_day: chrono::DateTime<Local>,
    e_day: chrono::DateTime<Local>,
    contract_len: u32,
    // item: Item,
    credits: f64,
}

impl Contract {
    pub fn new(
        s_day: chrono::DateTime<Local>,
        e_day: chrono::DateTime<Local>,
        contract_len: u32,
        // item: Item,
        credits: f64,
    ) -> Self {
        Self {
            s_day,
            e_day,
            contract_len,
            // item,
            credits,
        }
    }
}

impl std::fmt::Display for Contract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Start Day:\t{}\nEnd Day:\t{}\nLength:\t{}\nCredits:\t{}",
            self.s_day, self.e_day, self.contract_len, self.credits
        ))
    }
}
