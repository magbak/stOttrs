use std::collections::HashMap;
use oxrdf::Variable;
use super::Triplestore;
use polars::prelude::{col, Expr};
use spargebra::algebra::GraphPattern;
use log::{debug, warn};
use crate::triplestore::sparql::errors::SparqlError;
use crate::triplestore::sparql::query_context::{Context, PathEntry};
use crate::triplestore::sparql::solution_mapping::SolutionMappings;

impl Triplestore {
    pub(crate) fn lazy_project(
        &self,
        inner: &GraphPattern,
        variables: &Vec<Variable>,
        solution_mappings: Option<SolutionMappings>,
        context: &Context,
    ) -> Result<SolutionMappings, SparqlError> {
        debug!("Processing project graph pattern");
        let SolutionMappings{ mut mappings, mut datatypes ,.. } = self.lazy_graph_pattern(
            inner,
            solution_mappings,
            &context.extension_with(PathEntry::ProjectInner),
        )?;
        let cols: Vec<Expr> = variables.iter().map(|c| col(c.as_str())).collect();
        mappings = mappings.select(cols.as_slice());
        let mut new_datatypes = HashMap::new();
        for v in variables {
            if !datatypes.contains_key(v) {
                warn!("Datatypes does not contain {}", v);
            } else {
                new_datatypes.insert(v.clone(), datatypes.remove(v).unwrap());
            }
        }
        Ok(SolutionMappings::new(mappings, variables.iter().map(|x|x.as_str().to_string()).collect(), new_datatypes))
    }
}
