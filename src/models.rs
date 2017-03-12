#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientInput {
    pub urls: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Category {
    pub name: String,
    pub urls: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Categories {
    pub results: Vec<Category>
}
