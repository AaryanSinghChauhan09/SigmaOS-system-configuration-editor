// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-system-configuration-editor: system configuration editor component model.
//!
//! Real implementation: editors, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Editor {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemConfigurationEditorError {
    EmptyEditorName,
    DuplicateEditor(String),
    EmptyEventSubject,
}

impl fmt::Display for SystemConfigurationEditorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyEditorName => write!(f, "editor name must not be empty"),
            Self::DuplicateEditor(id) => write!(f, "duplicate editor {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SystemConfigurationEditorError {}

pub struct SystemConfigurationEditor {
    pub name: String,
    editors: BTreeMap<String, Editor>,
    events: BTreeMap<String, Event>,
}

impl SystemConfigurationEditor {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            editors: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_editor(&mut self, item: Editor) -> Result<(), SystemConfigurationEditorError> {
        if item.name.trim().is_empty() {
            return Err(SystemConfigurationEditorError::EmptyEditorName);
        }
        let id = item.id.clone();
        if self.editors.contains_key(&id) {
            return Err(SystemConfigurationEditorError::DuplicateEditor(id.clone()));
        }
        self.editors.insert(id, item);
        Ok(())
    }

    pub fn editor_count(&self) -> usize {
        self.editors.len()
    }

    pub fn get_editor(&self, id: &str) -> Option<&Editor> {
        self.editors.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SystemConfigurationEditorError> {
        if event.subject.trim().is_empty() {
            return Err(SystemConfigurationEditorError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_editor(id: &str, name: &str) -> Editor {
        Editor {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SystemConfigurationEditor::new("SystemConfigurationEditor");
        assert_eq!(m.name, "SystemConfigurationEditor");
        assert_eq!(m.editor_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_editor() {
        let mut m = SystemConfigurationEditor::new("SystemConfigurationEditor");
        m.add_editor(sample_editor("a1", "Alpha")).unwrap();
        assert_eq!(m.editor_count(), 1);
        assert!(m.get_editor("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SystemConfigurationEditor::new("SystemConfigurationEditor");
        assert!(m.add_editor(sample_editor("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SystemConfigurationEditor::new("SystemConfigurationEditor");
        m.add_editor(sample_editor("x", "One")).unwrap();
        assert!(m.add_editor(sample_editor("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SystemConfigurationEditor::new("SystemConfigurationEditor");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
