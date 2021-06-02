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

pub struct HomebrewGraph {
    graph: Graph<usize, u8>,
    formulae_map: HashMap<String, usize>,
}

impl HomebrewGraph {
    pub fn build(formulae: &[HomebrewFormula]) -> HomebrewGraph {
        let formulae_map = HomebrewGraph::build_formulae_map(formulae);
        let mut graph = Graph::with_capacity(formulae.len(), formulae.len() * 2);

        for formula in formulae {
            if let Some(idx) = formulae_map.get(&formula.name).copied() {
                graph.add_node(idx);
            } else {
                unimplemented!();
            }
        }

        let mut dependency_graph = HomebrewGraph {
            graph,
            formulae_map,
        };
        dependency_graph.build_graph_edges(formulae);
        dependency_graph
    }

    fn build_formulae_map(formulae: &[HomebrewFormula]) -> HashMap<String, usize> {
        let mut formulae_map = HashMap::new();

        for (idx, formula) in formulae.iter().enumerate() {
            formulae_map.insert(String::from(&formula.name), idx);
            for alias in &formula.aliases {
                formulae_map.insert(String::from(alias), idx);
            }
        }

        formulae_map
    }

    fn build_graph_edges(&mut self, formulae: &[HomebrewFormula]) {
        for (source_idx, formula) in formulae.iter().enumerate() {
            let source_node = NodeIndex::new(source_idx);
            for dependency in &formula.dependencies {
                if let Some(target_idx) = self.formulae_map.get(dependency).copied() {
                    let target_node = NodeIndex::new(target_idx);
                    self.graph.add_edge(source_node, target_node, 0);
                }
            }
        }
    }

    pub fn resolve_dependants(&self, name: &str) -> Vec<usize> {
        let mut dependants = Vec::new();

        if let Some(idx) = self.formulae_map.get(name).copied() {
            let node = NodeIndex::new(idx);
            let graph = Reversed(&self.graph);
            let mut dfs = Dfs::new(&graph, node);
            dfs.next(&graph);
            while let Some(node) = dfs.next(&graph) {
                dependants.push(self.graph[node]);
            }
        }

        dependants
    }
}
