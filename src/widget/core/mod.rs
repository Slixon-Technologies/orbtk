use theme::Selector;

pub use self::context::Context;
pub use self::key_chain::{WidgetKey, KeyChain};
pub use self::property::{SharedProperty, PropertyResult};
pub use self::state::State;
pub use self::template::Template;
pub use self::widget_container::WidgetContainer;

mod context;
mod key_chain;
mod property;
mod state;
mod template;
mod widget_container;

/// The `Widget` trait is used to define a new widget.
pub trait Widget {
    /// Returns the template of the implemented widget.
    fn create() -> Template;
}

/// Adds the given `pseudo_class` to the css selector of the given `wiget`.
pub fn add_selector_to_widget(pseudo_class: &str, widget: &mut WidgetContainer) {
    if let Ok(selector) = widget.borrow_mut_property::<Selector>() {
        selector.pseudo_classes.insert(String::from(pseudo_class));
    }
}

/// Removes the given `pseudo_class` from the css selector of the given `wiget`.
pub fn remove_selector_from_widget(pseudo_class: &str, widget: &mut WidgetContainer) {
    if let Ok(selector) = widget.borrow_mut_property::<Selector>() {
        selector.pseudo_classes.remove(pseudo_class);
    }
}