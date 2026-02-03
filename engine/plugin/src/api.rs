//! Plugin API traits.

/// A loaded plugin instance.
pub trait Plugin: Send + Sync {
    /// Get the plugin name.
    fn name(&self) -> &str;

    /// Get the plugin version.
    fn version(&self) -> &str;

    /// Initialize the plugin.
    fn init(&mut self) -> crate::Result<()>;

    /// Shut down the plugin.
    fn shutdown(&mut self);
}

/// API provided to plugins by the host.
pub trait PluginApi {
    /// Register a command.
    fn register_command(&mut self, name: &str, handler: Box<dyn CommandHandler>);

    /// Register a content type.
    fn register_content_type(&mut self, type_id: &str, handler: Box<dyn ContentHandler>);

    /// Log a message.
    fn log(&self, level: LogLevel, message: &str);
}

/// Command handler.
pub trait CommandHandler: Send + Sync {
    /// Execute the command.
    fn execute(&self, args: &[String]) -> crate::Result<()>;
}

/// Content type handler.
pub trait ContentHandler: Send + Sync {
    /// Render the content.
    fn render(&self, data: &[u8]) -> crate::Result<Vec<u8>>;

    /// Get the content type name.
    fn type_name(&self) -> &str;
}

/// Log level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}
