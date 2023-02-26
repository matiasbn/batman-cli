use crate::batbelt::metadata::functions_metadata::{FunctionMetadata, FunctionMetadataType};
use crate::batbelt::metadata::trait_impl_metadata::TraitImplMetadata;
use crate::batbelt::parser::ParserError;
use crate::batbelt::sonar::{BatSonar, SonarResult, SonarResultType};
use error_stack::{IntoReport, Report, Result, ResultExt};
use regex::Regex;
use std::fs;

#[derive(Clone, Debug)]
pub struct TraitImplParser {
    pub name: String,
    pub impl_from: String,
    pub impl_to: String,
    pub trait_impl_metadata: TraitImplMetadata,
    pub impl_functions: Vec<FunctionMetadata>,
}

impl TraitImplParser {
    pub fn new_from_metadata(
        trait_impl_metadata: TraitImplMetadata,
        optional_function_metadata_vec: Option<Vec<FunctionMetadata>>,
    ) -> Result<Self, ParserError> {
        let name = trait_impl_metadata.name.clone();
        let mut new_parser = Self {
            name,
            impl_from: "".to_string(),
            impl_to: "".to_string(),
            trait_impl_metadata,
            impl_functions: vec![],
        };
        log::debug!("new_function_parser:\n{:#?}", new_parser);
        new_parser.get_impl_function(optional_function_metadata_vec)?;
        new_parser.get_from_to()?;
        Ok(new_parser)
    }

    fn get_impl_function(
        &mut self,
        optional_function_metadata_vec: Option<Vec<FunctionMetadata>>,
    ) -> Result<(), ParserError> {
        let function_metadata_vec = if optional_function_metadata_vec.is_some() {
            optional_function_metadata_vec.unwrap()
        } else {
            FunctionMetadata::get_filtered_metadata(None, None).change_context(ParserError)?
        };
        let filtered_metadata_vec = function_metadata_vec
            .into_iter()
            .filter(|f_metadata| {
                f_metadata.path == self.trait_impl_metadata.path
                    && f_metadata.start_line_index > self.trait_impl_metadata.start_line_index
                    && f_metadata.end_line_index < self.trait_impl_metadata.end_line_index
            })
            .collect::<Vec<_>>();
        self.impl_functions = filtered_metadata_vec;
        Ok(())
    }

    fn get_from_to(&mut self) -> Result<(), ParserError> {
        let name = self.name.clone();
        let match_regex = Regex::new(r"[A-Za-z0-9]+<[<A-Za-z0-9>]+> for [A-Za-z0-9]+").unwrap();
        let generic_type_regex = Regex::new(r"").unwrap();
        if match_regex.is_match(&name) {
            let mut splitted = name.split(" for ");
            self.impl_from = splitted.next().unwrap().to_string();
            self.impl_to = splitted.next().unwrap().to_string();
        } else {
            self.impl_to = name;
        }
        Ok(())
    }
}