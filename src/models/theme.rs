#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub description: Option<String>,
    pub directory: Option<String>,
    pub repository: Option<String>,
    pub commit: Option<String>,
    pub version: Option<String>,
    pub leftwm_versions: Option<String>,
    pub dependencies: Option<Vec<String>>,
    pub current: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct TempThemes {
    pub theme: Vec<Theme>,
}

impl Theme {
    pub fn new(name: String, description: Option<String>, directory: Option<String>) -> Self {
        Theme {
            name,
            description,
            directory,
            repository: None,
            commit: None,
            version: Some("0.0.0".to_string()),
            leftwm_versions: Some("*".to_string()),
            dependencies: None,
            current: Some(false),
        }
    }

    pub fn is_installed(&self) -> bool {
        match &self.directory {
            Some(_dir) => true,
            None => false,
        }
    }

    pub fn find(themes: &mut Vec<Theme>, name: String) -> Option<&mut Theme> {
        themes.iter_mut().find(|p| name == p.name)
    }

    pub fn directory(&mut self, dir: Option<&str>) {
        self.directory = match dir {
            Some(dir) => Some(dir.to_string()),
            None => None,
        };
    }
}
