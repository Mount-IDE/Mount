use crate::modules::app::FS_READ_SERVICE;
use crate::modules::contexts::filesystem::app::traits::TFSReadService;
use crate::modules::contexts::filesystem::app::utils::path_from;
use crate::modules::contexts::filesystem::app::utils::PathPart;
use crate::modules::contexts::git::traits::{EmptyType, TGitService};
use crate::modules::shared::kernel::values::Path;
use std::process::Command;
use which::which;

pub struct GitService();
pub struct GitCommitService();
pub struct GitBranchService();

impl TGitService for GitService {
    fn has_git(&self, path: Path) -> bool {
        let path = path_from![path, ".git"];
        let ext = FS_READ_SERVICE.exists(path);
        ext
    }

    fn init_git(&self, path: Path) -> EmptyType {
        let prog = which("git");
        if let Err(_) = prog {
            return Err(());
        }
        let mut res = Command::new("git")
            .arg("init")
            .arg(path.get())
            .spawn()
            .unwrap();

        let code = res.wait().unwrap().code();

        if let None = code {
            return Err(());
        }
        Ok(())
    }

    fn is_merged(&self, from: String, to: String, path: Path) -> bool {
        todo!()
    }

    fn git_remove(&self, path: Path) -> EmptyType {
        todo!()
    }

    fn log(&self, path: Path) -> () {
        todo!()
    }
}
