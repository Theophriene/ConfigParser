use std::collections::*;

#[derive(Deserialize, Default)]
pub struct Pair {
    pub name: String,
    pub value: String,
}
type Param = Pair;
type Variable = Pair;

#[derive(Deserialize, Default)]
pub struct Params {
    pub param: Vec<Param>,
}

#[derive(Deserialize, Default)]
pub struct Variables {
    pub variable: Vec<Variable>,
}

#[derive(Deserialize, Default)]
pub struct User {
    #[serde(flatten)]
    pub attrs: HashMap<String, String>,
    pub params: Option<Params>,
    pub variables: Option<Variables>,
}

#[derive(Deserialize, Default)]
pub struct Users {
    pub user: Vec<User>,
}

#[derive(Deserialize, Default)]
pub struct Group {
    #[serde(flatten)]
    pub attrs: HashMap<String, String>,
    pub params: Option<Params>,
    pub variables: Option<Variables>,
    pub users: Option<Users>,
}

#[derive(Deserialize, Default)]
pub struct Groups {
    pub group: Vec<Group>,
}

#[derive(Deserialize, Default)]
pub struct Domain {
    #[serde(flatten)]
    pub attrs: HashMap<String, String>,
    pub params: Option<Params>,
    pub variables: Option<Variables>,
    pub users: Option<Users>,
    pub groups: Option<Groups>,
}

impl Into<HashMap<String, String>> for Variables {
    fn into(self) -> HashMap<String, String> {
        self.variable
            .into_iter()
            .map(|p| (p.name, p.value))
            .collect()
    }
}

impl Into<HashMap<String, String>> for Params {
    fn into(self) -> HashMap<String, String> {
        self.param.into_iter().map(|p| (p.name, p.value)).collect()
    }
}
