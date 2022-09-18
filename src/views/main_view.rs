use crate::types::View;

pub trait MainView {}

pub struct CliMainView {}

impl View for CliMainView {}

impl CliMainView {
    pub fn new() -> CliMainView {
        CliMainView {}
    }
}

impl MainView for CliMainView {}
