use std::path::{self, Path};

use anyhow::{Context, Result};
use git2::Repository;

use gblib::{
    database, gb_repository, paths::DataDir, project_repository, projects, sessions, storage, users,
};

pub struct App {
    path: path::PathBuf,
    local_data_dir: DataDir,
    project: projects::Project,
    sessions_db: sessions::Database,
    user: Option<users::User>,
}

impl App {
    pub fn new() -> Result<Self> {
        let path = find_git_directory().context("failed to find project directory")?;
        let local_data_dir = find_local_data_dir();

        let storage = storage::Storage::from(&local_data_dir);
        let users_storage = users::Controller::from(&storage);
        let projects_storage = projects::Controller::from(&local_data_dir);
        let projects = projects_storage.list().context("failed to list projects")?;

        let project = projects
            .into_iter()
            .find(|p| p.path == path)
            .context("failed to find project")?;

        let user = users_storage.get_user().context("failed to get user")?;
        let database =
            database::Database::try_from(&local_data_dir).context("failed to open database")?;
        let sessions_db = sessions::Database::from(database);

        Ok(Self {
            path,
            local_data_dir,
            project,
            sessions_db,
            user,
        })
    }

    pub fn user(&self) -> Option<&users::User> {
        self.user.as_ref()
    }

    pub fn path(&self) -> &path::PathBuf {
        &self.path
    }

    pub fn local_data_dir(&self) -> &DataDir {
        &self.local_data_dir
    }

    pub fn project(&self) -> &projects::Project {
        &self.project
    }

    pub fn sessions_db(&self) -> &sessions::Database {
        &self.sessions_db
    }

    pub fn project_repository(&self) -> project_repository::Repository {
        project_repository::Repository::open(&self.project).unwrap()
    }

    pub fn gb_repository(&self) -> gb_repository::Repository {
        let project_repository = project_repository::Repository::open(&self.project)
            .expect("failed to open project repository");
        let gb_repository = gb_repository::Repository::open(
            &self.local_data_dir,
            &project_repository,
            self.user.as_ref(),
        )
        .expect("failed to open repository");
        gb_repository
    }
}

fn find_git_directory() -> Option<path::PathBuf> {
    match Repository::discover("./") {
        Ok(repo) => repo.workdir().map(Path::to_path_buf),
        Err(_) => None,
    }
}

fn find_local_data_dir() -> DataDir {
    let mut path = dirs::config_dir().unwrap();
    path.push("com.gitbutler.app.dev");
    path::PathBuf::from(path.to_string_lossy().to_string()).into()
}
