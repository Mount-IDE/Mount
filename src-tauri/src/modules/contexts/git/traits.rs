use crate::modules::shared::kernel::values::Path;

pub type EmptyType = Result<(), ()>;

pub trait TGitService {
    fn has_git(&self, path: Path) -> bool;

    fn init_git(&self, path: Path) -> EmptyType;

    fn is_merged(&self, from: String, to: String, path: Path) -> bool;

    fn git_remove(&self, path: Path) -> EmptyType;

    fn log(&self, path: Path) -> ();
}

pub trait TGitCommitService {
    fn index_add(&self, files: Vec<Path>, path: Path) -> EmptyType;

    fn index_remove(&self, files: Vec<Path>, path: Path) -> EmptyType;

    fn commit(&self, path: Path, amend: bool) -> EmptyType;
}

pub trait TGitBranchService {
    fn branch_create(&self, name: String, path: Path) -> EmptyType;

    fn branch_checkout(&self, name: String, path: Path) -> EmptyType;

    fn branch_remove(&self, name: String, path: Path) -> EmptyType;

    fn merge(&self, from: String, to: String, path: Path) -> EmptyType;
}
