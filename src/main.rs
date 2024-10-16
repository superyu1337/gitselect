use clap::Parser;

mod git;
mod select;
mod cli;

#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum Error {
    #[error("gitselect: {0}")]
    Cli(String),
    #[error("gitselect: {0}")]
    Config(String),
    #[error("gitselect: {0}")]
    Git(String),
    #[error("gitselect: {0}")]
    Regex(String),
    #[error("gitselect: {0}")]
    Select(String),
    #[error("gitselect: {0}")]
    Terminal(String),
}

fn main() {
    let args = cli::Args::parse();

    let getter = git::fs::FsBranchGetter {
        repo_dir: std::env::current_dir().expect("Failed to get current directory")
    };
    let selector = select::DialogueSelector;

    bselect(&args, getter, selector, &mut std::io::stdout()).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });
}

fn bselect(
    args: &cli::Args,
    branch_getter: impl git::BranchGetter,
    selector: impl select::BranchSelector,
    stdout: &mut dyn std::io::Write,
) -> Result<(), Error> {
    let branches = filter(branch_getter.branches()?, args.local)?;
    let selected_branch = selector.select_branch(branches)?;
    
    writeln!(stdout, "{}", selected_branch.name)
        .map_err(|e| Error::Terminal(format!("cannot write to stdout: {e}")))
}

fn filter(
    branches: Vec<git::Branch>,
    local_only: bool,
) -> Result<Vec<git::Branch>, Error> {
    if branches.is_empty() {
        return Err(Error::Git(String::from("This is not a git repository.")))
    }

    let out: Vec<git::Branch> = branches
        .into_iter()
        .filter(|b| {
            !local_only || b.branch_type == git::BranchType::Local
        })
        .collect();

    if out.is_empty() {
        return Err(Error::Select(String::from("No matching branches")))
    }

    Ok(out)
}