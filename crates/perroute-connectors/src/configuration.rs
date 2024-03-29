use derive_builder::Builder;
use perroute_commons::types::properties::{Properties, PropertiesError};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};

pub trait Configuration: Debug + Send + Sync {
    fn validate(&self, props: &Properties) -> Result<(), PropertiesError>;
    fn properties(&self) -> Vec<&ConfigurationProperty>;
}

#[derive(Debug, Deserialize, Default)]
pub struct NilConfiguration;

impl validator::Validate for NilConfiguration {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct DefaultConfiguration<T>
where
    T: DeserializeOwned + validator::Validate,
{
    properties: ConfigurationProperties,
    _marker: std::marker::PhantomData<T>,
}

impl Default for DefaultConfiguration<NilConfiguration> {
    fn default() -> Self {
        Self {
            properties: Default::default(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> DefaultConfiguration<T>
where
    T: DeserializeOwned + validator::Validate,
{
    pub fn new(properties: ConfigurationProperties, _marker: std::marker::PhantomData<T>) -> Self {
        Self {
            properties,
            _marker,
        }
    }
}

impl<T> Configuration for DefaultConfiguration<T>
where
    T: DeserializeOwned + validator::Validate + Debug + Send + Sync,
{
    fn validate(&self, props: &Properties) -> Result<(), PropertiesError> {
        props.from_value::<T>().map(|_| ())
    }

    fn properties(&self) -> Vec<&ConfigurationProperty> {
        self.properties.0.iter().collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub enum ConfigurationPropertyType {
    String,
    Number,
    Boolean,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Builder)]
pub struct ConfigurationProperty {
    name: &'static str,
    required: bool,
    description: &'static str,
    property_type: ConfigurationPropertyType,
    multiple: bool,
}

#[derive(Debug, Default)]
pub struct ConfigurationProperties(Vec<ConfigurationProperty>);

impl<const N: usize> From<[ConfigurationProperty; N]> for ConfigurationProperties {
    fn from(value: [ConfigurationProperty; N]) -> Self {
        Self(
            value
                .into_iter()
                .map(|p| (p.name, p))
                .collect::<HashMap<_, _>>()
                .values()
                .cloned()
                .collect(),
        )
    }
}
