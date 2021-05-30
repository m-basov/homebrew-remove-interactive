use anyhow::Result;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::{Dfs, Reversed};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct HomebrewInfo {
    pub formulae: Vec<HomebrewFormula>,
}

impl HomebrewInfo {
    pub fn parse(api_json: &str) -> Result<HomebrewInfo> {
        Ok(serde_json::from_str(api_json)?)
    }
}

#[derive(Debug, Deserialize)]
pub struct HomebrewFormula {
    pub name: String,
    pub desc: String,
    pub aliases: Vec<String>,
    pub dependencies: Vec<String>,
    pub installed: Vec<HomebrewFormulaInstalled>,
}

#[derive(Debug, Deserialize)]
pub struct HomebrewFormulaInstalled {
    pub version: String,
}

pub struct FormulaeMap<'a>(HashMap<&'a str, &'a HomebrewFormula>);

impl<'a> FormulaeMap<'a> {
    pub fn build(formulae: &[HomebrewFormula]) -> FormulaeMap {
        let mut map = HashMap::new();

        for formula in formulae {
            map.insert(formula.name.as_str(), formula);
            for alias in &formula.aliases {
                map.insert(alias.as_str(), formula);
            }
        }

        FormulaeMap(map)
    }

    pub fn get(&self, key: &str) -> Option<&HomebrewFormula> {
        self.0.get(key).copied()
    }
}

pub struct HomebrewDependencyGraph<'a> {
    pub graph: Graph<&'a HomebrewFormula, u8>,
    pub node_map: HashMap<&'a str, NodeIndex>,
}

impl<'a> HomebrewDependencyGraph<'a> {
    pub fn build(
        formulae_map: &'a FormulaeMap,
        formulae: &[HomebrewFormula],
    ) -> HomebrewDependencyGraph<'a> {
        let mut graph = Graph::with_capacity(formulae.len(), formulae.len() * 2);
        let mut node_map = HashMap::new();

        for formula in formulae {
            if let Some(formula) = formulae_map.get(formula.name.as_str()) {
                let node = graph.add_node(formula);
                node_map.insert(formula.name.as_str(), node);
            } else {
                eprintln!("formula not found: {}", formula.name);
            }
        }

        let mut dependency_graph = HomebrewDependencyGraph { graph, node_map };
        dependency_graph.build_graph_edges(formulae);
        dependency_graph
    }

    fn build_graph_edges(&mut self, formulae: &[HomebrewFormula]) {
        for formula in formulae {
            if let Some(source_node) = self.node_map.get(formula.name.as_str()).copied() {
                for dependency in &formula.dependencies {
                    if let Some(target_node) = self.node_map.get(dependency.as_str()).copied() {
                        self.graph.add_edge(source_node, target_node, 0);
                    }
                }
            }
        }
    }

    pub fn resolve_dependants(&self, name: &str) -> Vec<&str> {
        let mut dependants = Vec::new();

        if let Some(node) = self.node_map.get(name).copied() {
            let graph = Reversed(&self.graph);
            let mut dfs = Dfs::new(&graph, node);
            dfs.next(&graph);
            while let Some(node) = dfs.next(&graph) {
                dependants.push(self.graph[node].name.as_str());
            }
        }

        dependants
    }
}
