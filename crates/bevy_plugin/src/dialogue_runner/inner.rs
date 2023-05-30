use crate::prelude::*;
use bevy::prelude::*;

pub(crate) fn inner_dialogue_runner_plugin(_app: &mut App) {}

pub struct InnerDialogue<'a>(pub(crate) &'a Dialogue);
pub struct InnerDialogueMut<'a>(pub(crate) &'a mut Dialogue);

impl<'a> InnerDialogue<'a> {
    pub fn node_names(&self) -> impl Iterator<Item = &str> {
        self.0.node_names().unwrap()
    }

    #[must_use]
    pub fn get_line_id_for_node(&self, node_name: &str) -> Option<LineId> {
        self.0.get_line_id_for_node(node_name)
    }

    #[must_use]
    pub fn analyse(&self, context: &mut YarnAnalysisContext) -> &Self {
        self.0.analyse(context);
        self
    }
}

impl<'a> InnerDialogueMut<'a> {
    pub fn node_names(&self) -> impl Iterator<Item = &str> {
        self.0.node_names().unwrap()
    }

    #[must_use]
    pub fn get_line_id_for_node(&self, node_name: &str) -> Option<LineId> {
        self.0.get_line_id_for_node(node_name)
    }

    pub fn set_node(&mut self, name: impl Into<String>) -> Result<&mut Self> {
        self.0.set_node(name)?;
        Ok(self)
    }

    pub fn set_node_to_start(&mut self) -> Result<&mut Self> {
        self.0.set_node_to_start()?;
        Ok(self)
    }

    #[must_use]
    pub fn analyse(&self, context: &mut YarnAnalysisContext) -> &Self {
        self.0.analyse(context);
        self
    }
}
