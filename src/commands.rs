use git2::Repository;

pub fn open_repo() -> Result<Repository, git2::Error> {
    Repository::discover(".")
}
