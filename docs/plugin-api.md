# Plugin API

Wolia supports plugins for extending functionality.

## Overview

Plugins can:

- Add new node types to the document model
- Register toolbar items and menu entries
- Handle custom file formats
- Provide rendering extensions
- Add keyboard shortcuts

## Creating a Plugin

### 1. Implement the Plugin Trait

```rust
use wolia_plugin::{Plugin, PluginContext, PluginError};

pub struct MyPlugin {
    name: String,
    version: String,
}

impl Plugin for MyPlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn initialize(&mut self, context: &PluginContext) -> Result<(), PluginError> {
        // Register capabilities
        context.register_node_type("my-custom-node", MyNodeRenderer)?;
        context.register_command("my-plugin.action", my_action)?;
        Ok(())
    }

    fn shutdown(&mut self) {
        // Cleanup
    }
}
```

### 2. Export the Plugin

```rust
#[no_mangle]
pub extern "C" fn create_plugin() -> Box<dyn Plugin> {
    Box::new(MyPlugin {
        name: "My Plugin".to_string(),
        version: "1.0.0".to_string(),
    })
}
```

### 3. Create a Manifest

`manifest.toml`:

```toml
[plugin]
name = "my-plugin"
version = "1.0.0"
description = "A sample plugin"
author = "Your Name"

[capabilities]
node_types = ["my-custom-node"]
commands = ["my-plugin.action"]

[dependencies]
wolia = "0.1"
```

## Plugin Context API

### Registering Node Types

```rust
impl PluginContext {
    /// Register a custom node type
    pub fn register_node_type<R: NodeRenderer>(
        &self,
        type_name: &str,
        renderer: R,
    ) -> Result<(), PluginError>;
}

pub trait NodeRenderer: Send + Sync {
    fn render(&self, node: &Node, context: &RenderContext);
    fn layout(&self, node: &Node, constraints: Size) -> Size;
}
```

### Registering Commands

```rust
impl PluginContext {
    /// Register a command
    pub fn register_command<F>(
        &self,
        command_id: &str,
        handler: F,
    ) -> Result<(), PluginError>
    where
        F: Fn(&CommandContext) -> Result<(), CommandError> + Send + Sync + 'static;
}
```

### Registering Menu Items

```rust
impl PluginContext {
    /// Add a menu item
    pub fn register_menu_item(
        &self,
        menu: &str,
        item: MenuItem,
    ) -> Result<(), PluginError>;
}

pub struct MenuItem {
    pub label: String,
    pub command: String,
    pub shortcut: Option<KeyBinding>,
}
```

### Registering Toolbar Items

```rust
impl PluginContext {
    /// Add a toolbar button
    pub fn register_toolbar_item(
        &self,
        toolbar: &str,
        item: ToolbarItem,
    ) -> Result<(), PluginError>;
}

pub struct ToolbarItem {
    pub icon: Icon,
    pub tooltip: String,
    pub command: String,
}
```

## Built-in Plugins

### LaTeX Plugin

Renders mathematical equations:

```rust
// Usage in document
{
  "kind": "latex",
  "content": "E = mc^2"
}
```

### Diagrams Plugin

Renders flowcharts and UML:

```rust
// Usage in document
{
  "kind": "diagram",
  "diagram_type": "flowchart",
  "content": "A --> B --> C"
}
```

### Code Blocks Plugin

Syntax highlighting for code:

```rust
// Usage in document
{
  "kind": "code-block",
  "language": "rust",
  "content": "fn main() { println!(\"Hello\"); }"
}
```

## Plugin Distribution

Plugins are distributed as:

1. **Dynamic libraries** (`.so`, `.dylib`, `.dll`)
2. **Wasm modules** (for sandboxed execution)

### Installation

```bash
# From registry
wolia plugin install my-plugin

# From file
wolia plugin install ./my-plugin.wolia-plugin

# From URL
wolia plugin install https://example.com/my-plugin.wolia-plugin
```

## Security

Plugins run in a sandboxed environment:

- Limited file system access
- No network access by default
- Memory limits enforced
- API access controlled by capabilities

Plugins must declare capabilities in their manifest, and users approve them on installation.
