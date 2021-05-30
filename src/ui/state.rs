use crate::homebrew::{HomebrewClient, HomebrewDependencyGraph, HomebrewFormula};
use anyhow::Result;
use std::collections::HashSet;
use tui::widgets::ListState;

pub struct State<'a> {
    pub selected_formula: ListState,
    pub formulae: Vec<&'a HomebrewFormula>,
    pub graph: &'a HomebrewDependencyGraph<'a>,
    all_formulae: &'a [HomebrewFormula],
    filter_selected: bool,
    formulae_to_delete: HashSet<&'a str>,
}

impl<'a> State<'a> {
    pub fn new(formulae: &'a [HomebrewFormula], graph: &'a HomebrewDependencyGraph) -> State<'a> {
        State {
            selected_formula: ListState::default(),
            formulae: formulae.iter().collect(),
            graph,
            all_formulae: formulae,
            filter_selected: false,
            formulae_to_delete: HashSet::new(),
        }
    }

    pub fn select_next_formula(&mut self) {
        let next_idx = if let Some(selected) = self.selected_formula.selected() {
            if selected == self.formulae.len() - 1 {
                0
            } else {
                selected + 1
            }
        } else {
            0
        };
        self.selected_formula.select(Some(next_idx));
    }

    pub fn select_prev_formula(&mut self) {
        let prev_idx = if let Some(selected) = self.selected_formula.selected() {
            if selected == 0 {
                self.formulae.len() - 1
            } else {
                selected - 1
            }
        } else {
            0
        };
        self.selected_formula.select(Some(prev_idx));
    }

    pub fn get_selected_formula(&self) -> Option<&'a HomebrewFormula> {
        self.selected_formula
            .selected()
            .map(|selected| self.formulae[selected])
    }

    pub fn is_formula_selected(&self, name: &str) -> bool {
        self.formulae_to_delete.contains(name)
    }

    pub fn select_formulae_to_delete(&mut self) {
        if let Some(name) = self.get_selected_formula().map(|f| f.name.as_str()) {
            if self.formulae_to_delete.contains(name) {
                self.formulae_to_delete.remove(name);
            } else {
                for dependant in self.graph.resolve_dependants(name) {
                    self.formulae_to_delete.insert(dependant);
                }

                self.formulae_to_delete.insert(name);
            }
        }
    }

    pub fn filter_selected(&mut self) {
        self.filter_selected = !self.filter_selected;
        self.selected_formula.select(None);

        self.formulae = self
            .all_formulae
            .iter()
            .filter(|formula| {
                if self.filter_selected {
                    self.is_formula_selected(&formula.name)
                } else {
                    true
                }
            })
            .collect();
    }

    pub fn remove_selected_formulae(&mut self) -> Result<()> {
        let formulae_names = self.formulae_to_delete.iter().copied().collect::<Vec<_>>();

        HomebrewClient::remove_formulae(&formulae_names)
    }
}
