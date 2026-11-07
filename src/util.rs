//! starship model — cross-shell prompt.

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub enabled: bool,
    pub format: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub modules: Vec<Module>,
}

impl Default for Model {
    fn default() -> Self { Self { modules: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_module(&mut self, module: Module) { self.modules.push(module); }

    pub fn toggle(&mut self, name: &str) -> bool {
        if let Some(m) = self.modules.iter_mut().find(|m| m.name == name) {
            m.enabled = !m.enabled;
            true
        } else { false }
    }

    pub fn render(&self) -> String {
        self.modules.iter().filter(|m| m.enabled).map(|m| m.format.clone()).collect::<Vec<_>>().join("")
    }

    pub fn list(&self) -> &[Module] { &self.modules }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn module_toggle_render() {
        let mut s = Model::new();
        s.add_module(Module { name: "dir".into(), enabled: true, format: "[dir] ".into() });
        s.add_module(Module { name: "git".into(), enabled: true, format: "[git] ".into() });
        assert!(s.toggle("git"));
        assert!(s.render().contains("dir"));
        assert!(!s.render().contains("git"));
    }
}
