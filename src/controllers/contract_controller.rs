use super::app::App;
use crate::{
    models::domain::{contract::Contract, item::Item, system::LendingSystem},
    types::{Model, View},
    views::{
        contract_view::{ContractOption, ContractView},
        item_view::{CliItemView, ItemView},
        member_view::{CliMemberView, MemberView},
    },
};
use shared::controller;

/// The Contract controller.
#[derive(Debug, Clone)]
#[controller(ContractView)]
pub struct ContractController<M, V>
where
    M: Model + LendingSystem,
    V: View + ContractView,
{
    model: M,
    view: V,
}

impl<M, V> ContractController<M, V>
where
    M: Model + LendingSystem + Clone,
    V: View + ContractView,
{
    fn ret(&self, display: &str) -> M {
        self.view.wait(display);
        self.model.clone()
    }

    fn fetch_item(&self) -> Option<Item> {
        let item_view: CliItemView = CliItemView::new();
        let items = self.model.get_items();
        let item = item_view.select_item(items);
        match item {
            Some(i) => Some(i.clone()),
            None => None,
        }
    }

    fn fetch_contract<F>(&self, fun: F) -> M
    where
        F: Fn(&Contract),
    {
        let item = self.fetch_item();
        match item {
            Some(i) => {
                let contracts = i.get_history().iter().collect::<Vec<&Contract>>();
                let contract = self.view.select_contract(contracts);
                if let Some(c) = contract {
                    fun(c);
                    self.view.wait("");
                };
                self.model.clone()
            }
            None => self.model.clone(),
        }
    }

    fn display_contract_simple(&self) -> M {
        self.fetch_contract(|c: &Contract| self.view.display_contract_simple(c))
    }

    fn display_contract_verbose(&self) -> M {
        self.fetch_contract(|c: &Contract| self.view.display_contract_verbose(c))
    }

    fn create_contract(&mut self) -> M {
        let mview = CliMemberView::new();
        let iview = CliItemView::new();
        let item = self.fetch_item();
        match item {
            Some(i) => match mview.select_member(self.model.get_members()) {
                Some(lendee) => {
                    if lendee == i.get_owner() {
                        return self.ret("Cannot lend to yourself.");
                    }
                    match iview.select_date(&i) {
                        Some(start_date) => {
                            let data = self.view.get_contract_info();
                            let contract = Contract::new(
                                lendee.clone(),
                                start_date,
                                i.clone(),
                                *data.get_contract_len(),
                            );
                            let mut temp = i.clone();
                            match temp.add_contract(contract, self.model.now()) {
                                Ok(_) => match self.model.update_item(&i, &temp) {
                                    Ok(_) => self.ret("Successfully created contract."),
                                    Err(_) => self.ret("Failed to create contrct."),
                                },
                                Err(_) => self.model.clone(),
                            }
                        }
                        None => self.model.clone(),
                    }
                }
                None => self.model.clone(),
            },
            None => self.model.clone(),
        }
    }

    fn edit_contract(&mut self) -> M {
        let item = self.fetch_item();
        match item {
            Some(i) => {
                let mut history = i.get_history().clone();
                let contract = self
                    .view
                    .select_contract(history.iter().collect::<Vec<&Contract>>());
                match contract {
                    Some(c) => {
                        let new_contract_info = self.view.edit_contract(c);
                        match new_contract_info {
                            Some(new_contract) => {
                                let idx = history.index_of(&new_contract).unwrap();
                                history.set(idx, &new_contract);
                                self.view.wait("Updated contract successfully.")
                            }
                            None => {
                                self.view.wait("Couldnt edit contract.");
                            }
                        };

                        self.model.clone()
                    }
                    None => self.model.clone(),
                }
            }
            None => self.model.clone(),
        }
    }
}

impl<M, V> App<M> for ContractController<M, V>
where
    M: Model + LendingSystem + Clone,
    V: View + ContractView,
{
    fn run(&mut self, sys: M) -> M {
        let choice = self.view.contract_menu();
        let state = match choice {
            ContractOption::DisplayContractSimple => self.display_contract_simple(),
            ContractOption::DisplayContractVerbose => self.display_contract_verbose(),
            ContractOption::CreateContract => self.create_contract(),
            ContractOption::EditContract => self.edit_contract(),
            ContractOption::Quit => std::process::exit(0),
            _ => return sys,
        };
        self.run(state)
    }
}
