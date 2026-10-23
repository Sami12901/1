use crate::error::{CoreError, Result};
use crate::timeline::Timeline;
use std::collections::VecDeque;

/// Snapshot-based or action-based history manager for undo/redo
#[derive(Debug, Clone)]
pub struct HistoryManager {
    undo_stack: VecDeque<Timeline>,
    redo_stack: VecDeque<Timeline>,
    max_history: usize,
}

impl Default for HistoryManager {
    fn default() -> Self {
        Self::new(50)
    }
}

impl HistoryManager {
    pub fn new(max_history: usize) -> Self {
        Self {
            undo_stack: VecDeque::with_capacity(max_history),
            redo_stack: VecDeque::with_capacity(max_history),
            max_history,
        }
    }

    /// Record a timeline state before performing a mutation
    pub fn push_state(&mut self, state: Timeline) {
        if self.undo_stack.len() >= self.max_history {
            self.undo_stack.pop_front();
        }
        self.undo_stack.push_back(state);
        self.redo_stack.clear();
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Undo: restores the previous timeline state, pushing the current state onto redo
    pub fn undo(&mut self, current: Timeline) -> Result<Timeline> {
        let prev = self.undo_stack.pop_back().ok_or(CoreError::NothingToUndo)?;
        if self.redo_stack.len() >= self.max_history {
            self.redo_stack.pop_front();
        }
        self.redo_stack.push_back(current);
        Ok(prev)
    }

    /// Redo: restores the next timeline state, pushing the current state onto undo
    pub fn redo(&mut self, current: Timeline) -> Result<Timeline> {
        let next = self.redo_stack.pop_back().ok_or(CoreError::NothingToRedo)?;
        if self.undo_stack.len() >= self.max_history {
            self.undo_stack.pop_front();
        }
        self.undo_stack.push_back(current);
        Ok(next)
    }

    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_undo_redo_flow() {
        let mut history = HistoryManager::new(10);
        let mut t1 = Timeline::new_default();
        t1.playhead_frame = 10;

        let mut t2 = t1.clone();
        t2.playhead_frame = 20;

        // User changes from t1 to t2:
        history.push_state(t1.clone());
        assert!(history.can_undo());
        assert!(!history.can_redo());

        // Perform undo:
        let restored = history.undo(t2.clone()).expect("Undo should succeed");
        assert_eq!(restored.playhead_frame, 10);
        assert!(history.can_redo());

        // Perform redo:
        let redone = history.redo(restored).expect("Redo should succeed");
        assert_eq!(redone.playhead_frame, 20);
        assert!(!history.can_redo());
    }
}
