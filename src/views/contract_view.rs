use super::{
    console::{Console, Ui},
    Options,
};
use crate::{models::domain::contract::Contract, types::View};
use shared::COptions;
use std::str::FromStr;

#[derive(Debug, COptions)]
pub enum ContractOption {
    DisplayContractSimple,
    DisplayContractVerbose,
    #[other]
    Other,
}

pub trait ContractDisplay {
    fn contract_menu(&self) -> ContractOption;
    fn display_contract_simple(&self, contract: Contract);
    fn display_contract_verbose(&self, contract: Contract);
}

pub struct ContractView {
    console: Console,
}

impl ContractDisplay for ContractView {
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

impl View for ContractView {}
