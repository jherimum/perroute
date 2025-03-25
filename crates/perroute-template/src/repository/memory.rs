use std::collections::HashMap;
use crate::template::{NotRenderedTemplateState, Template};
use super::{TemplateId, TemplateLookup, TemplateRepositoryError};

pub struct InMemoryTemplateRepository<'a> {
    map: HashMap<TemplateId<'a>, Template<NotRenderedTemplateState>>,
}

#[async_trait::async_trait]
impl<'a> TemplateLookup for InMemoryTemplateRepository<'a> {
    async fn get<'b>(
        &self,
        id: &'b TemplateId<'b>,
    ) -> Result<
        Option<Template<NotRenderedTemplateState>>,
        TemplateRepositoryError,
    > {
        todo!()
    }
}
