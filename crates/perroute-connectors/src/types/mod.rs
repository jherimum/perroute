use perroute_commons::types::Configuration;

#[derive(Debug)]
pub struct Property {
    name: String,
}

#[derive(Debug, Default)]
pub struct Properties {
    props: Vec<Property>,
}

impl Properties {
    pub fn validate(&self, cfg: &Configuration) -> Result<(), Vec<String>> {
        todo!()
    }
}
