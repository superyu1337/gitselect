use dialoguer as dlg;

use crate::git;
use crate::Error;

pub trait BranchSelector {
    fn select_branch(&self, branches: Vec<git::Branch>) -> Result<git::Branch, Error>;
}

pub struct DialogueSelector;

impl BranchSelector for DialogueSelector {
    fn select_branch(&self, branches: Vec<git::Branch>) -> Result<git::Branch, Error> {
        let theme = dialoguer::theme::ColorfulTheme::default();
        let select_res = dlg::Select::with_theme(&theme)
            .default(0)
            .items(&branches)
            .interact();

        match select_res {
            Ok(opt) => return Ok(branches[opt].to_owned()),
            Err(e) => return Err(Error::Select(format!("{e}"))),
        }
    }
}