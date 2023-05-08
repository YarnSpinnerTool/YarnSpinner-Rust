use crate::prelude::*;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use yarn_slinger_core::prelude::*;

#[derive(Debug, Clone)]
pub(crate) struct SharedState {
    program: Arc<RwLock<Option<Program>>>,
    current_node_name: Arc<RwLock<Option<String>>>,
    language_code: Arc<RwLock<Option<String>>>,
    variable_storage: Arc<RwLock<Box<dyn VariableStorage + Send + Sync>>>,
    state: Arc<RwLock<State>>,
    execution_state: Arc<RwLock<ExecutionState>>,
    current_node: Arc<RwLock<Option<Node>>>,
}

impl Default for SharedState {
    fn default() -> Self {
        Self {
            program: Default::default(),
            current_node_name: Default::default(),
            language_code: Default::default(),
            variable_storage: Arc::new(RwLock::new(Box::<MemoryVariableStore>::default())),
            state: Default::default(),
            execution_state: Default::default(),
            current_node: Default::default(),
        }
    }
}

impl SharedStateHolder for SharedState {
    fn shared_state(&self) -> &SharedState {
        self
    }
}

pub(crate) trait SharedStateHolder {
    fn shared_state(&self) -> &SharedState;

    fn program(&self) -> RwLockReadGuard<Option<Program>> {
        self.shared_state().program.read().unwrap()
    }

    fn program_mut(&mut self) -> RwLockWriteGuard<Option<Program>> {
        self.shared_state().program.write().unwrap()
    }

    fn current_node_name(&self) -> RwLockReadGuard<Option<String>> {
        self.shared_state().current_node_name.read().unwrap()
    }

    fn current_node_name_mut(&mut self) -> RwLockWriteGuard<Option<String>> {
        self.shared_state().current_node_name.write().unwrap()
    }

    fn language_code(&self) -> RwLockReadGuard<Option<String>> {
        self.shared_state().language_code.read().unwrap()
    }

    fn language_code_mut(&mut self) -> RwLockWriteGuard<Option<String>> {
        self.shared_state().language_code.write().unwrap()
    }

    fn variable_storage(&self) -> RwLockReadGuard<Box<dyn VariableStorage + Send + Sync>> {
        self.shared_state().variable_storage.read().unwrap()
    }

    fn variable_storage_shared(&self) -> Arc<RwLock<Box<dyn VariableStorage + Send + Sync>>> {
        self.shared_state().variable_storage.clone()
    }

    fn variable_storage_mut(&mut self) -> RwLockWriteGuard<Box<dyn VariableStorage + Send + Sync>> {
        self.shared_state().variable_storage.write().unwrap()
    }

    fn state(&self) -> RwLockReadGuard<State> {
        self.shared_state().state.read().unwrap()
    }

    fn state_mut(&mut self) -> RwLockWriteGuard<State> {
        self.shared_state().state.write().unwrap()
    }

    fn execution_state(&self) -> RwLockReadGuard<ExecutionState> {
        self.shared_state().execution_state.read().unwrap()
    }

    fn execution_state_mut(&mut self) -> RwLockWriteGuard<ExecutionState> {
        self.shared_state().execution_state.write().unwrap()
    }

    fn current_node(&self) -> RwLockReadGuard<Option<Node>> {
        self.shared_state().current_node.read().unwrap()
    }

    fn current_node_mut(&mut self) -> RwLockWriteGuard<Option<Node>> {
        self.shared_state().current_node.write().unwrap()
    }
}
