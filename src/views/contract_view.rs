use super::{
    console::{Console, Ui},
    Options,
};
use crate::models::domain::contract::Contract;
use shared::{COptions, View};
use std::str::FromStr;

/// Enum that contains all valid operations for the contract page.
#[derive(Debug, COptions)]
pub enum ContractOption {
    /// Displays a single contract in a simple format.
    /// ! Current implementation does not work and since it is not required
    /// ! It will not be available in the release version.
    DisplayContractSimple,
    /// Displays a single contract in a vernose format.
    CreateContract,
    /// Asks user for input and returns a new instance of the previous contract
    /// with (possibly) updated attributes.
    EditContract,
    /// Returns to the previous page.
    Back,
    /// Quits the entire application.
    Quit,
    /// Any other operation the user tried to execute.
    #[other]
    Other,
}

/// Defines the methods a specific contract view needs to implement, be it tui/gui.
pub trait ContractView {
    /// Selecting a contract from a list of possible ones..
    fn select_contract<'a>(&'a self, contracts: Vec<&'a Contract>) -> Option<&Contract>;
    /// Edit a certain contract.
    fn edit_contract(&self, c: &Contract) -> Option<Contract>;
    /// Show all possible choices for the contract view.
    fn contract_menu(&self) -> ContractOption;
    /// Get information for a new contract.
    fn get_contract_info(&self) -> Contract;
    /// Displays a contract in a simple format.
    fn display_contract_simple(&self, contract: &Contract);
    /// Displays a message to the user and waits till the user acknowledges the message
    /// before continuing with the regular flow.
    fn wait(&self, display: &str);
}

/// A concrete implementation of the ContractView.
#[derive(View)]
pub struct CliContractView {
    console: Console,
}

impl ContractView for CliContractView {
    fn select_contract<'a>(&'a self, contracts: Vec<&'a Contract>) -> Option<&Contract> {
        self.console.select_model(contracts)
    }

    fn edit_contract(&self, c: &Contract) -> Option<Contract> {
        self.console.edit_model_info::<Contract>(c)
    }

    fn contract_menu(&self) -> ContractOption {
        self.console.title();
        let choice: ContractOption = self.console.show_menu(ContractOption::options());
        match choice {
            ContractOption::Other => self.contract_menu(),
            _ => choice,
        }
    }

    fn get_contract_info(&self) -> Contract {
        let new_contract = Contract::default();
        self.console.get_model_info(new_contract)
    }

    fn display_contract_simple(&self, contract: &Contract) {
        let out = format!(
            "Owner:\t{}\nLendee:\t{}\nCredits:\t{}\nStatus:\t{}\nStart Date:\t{}\nEnd Date:\t{}",
            contract.get_owner().get_name(),
            contract.get_lendee(),
            contract.get_credits(),
            contract.get_status(),
            contract.get_start_date(),
            contract.get_end_date(),
        );

        self.console.clear();
        self.console.title();
        self.console.write(out.as_str());
    }

    fn wait(&self, display: &str) {
        self.console.wait(display)
    }
}
