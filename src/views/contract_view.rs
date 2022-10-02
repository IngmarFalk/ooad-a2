use super::{
    console::{Console, Ui},
    Options,
};
use crate::models::domain::contract::Contract;
use shared::{COptions, View};
use std::str::FromStr;

#[derive(Debug, COptions)]
pub enum ContractOption {
    DisplayContractSimple,
    DisplayContractVerbose,
    CreateContract,
    EditContract,
    Quit,
    Back,
    #[other]
    Other,
}

pub trait ContractView {
    fn select_contract<'a>(&'a self, contracts: Vec<&'a Contract>) -> Option<&Contract>;
    fn contract_menu(&self) -> ContractOption;
    fn display_contract_simple(&self, contract: Contract);
    fn display_contract_verbose(&self, contract: Contract);
}

#[derive(View)]
pub struct CliContractView {
    console: Console,
}

impl ContractView for CliContractView {
    fn select_contract<'a>(&'a self, contracts: Vec<&'a Contract>) -> Option<&Contract> {
        self.console.select_model(contracts)
    }

    fn contract_menu(&self) -> ContractOption {
        self.console.title();
        let choice: ContractOption = self.console.show_menu(ContractOption::options());
        match choice {
            ContractOption::Other => self.contract_menu(),
            _ => choice,
        }
    }

    fn display_contract_simple(&self, _contract: Contract) {
        // let info = contract.unpack();
        todo!()
    }

    fn display_contract_verbose(&self, _contract: Contract) {
        todo!()
    }
}
