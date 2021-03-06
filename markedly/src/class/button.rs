use nalgebra::{Point2, Vector2};

use class::{ComponentClass, ComponentClassFactory, BackgroundAttributes};
use render::{Renderer};
use scripting::{ScriptRuntime};
use template::{Attributes, Color, EventHook};
use {EventSink, Error, ComponentAttributes, ComponentId};

/// A button component class, raises events on click.
pub struct ButtonClass {
    background: BackgroundAttributes,
    attributes: ButtonAttributes,
    hovering: bool,
}

impl ComponentClassFactory for ButtonClass {
    fn new(attributes: &Attributes, runtime: &ScriptRuntime) -> Result<Self, Error> {
        Ok(ButtonClass {
            background: BackgroundAttributes::load(attributes, runtime)?,
            attributes: ButtonAttributes::load(attributes, runtime)?,
            hovering: false,
        })
    }
}

impl ComponentClass for ButtonClass {
    fn update_attributes(
        &mut self, attributes: &Attributes, runtime: &ScriptRuntime,
    ) -> Result<(), Error> {
        self.background = BackgroundAttributes::load(attributes, runtime)?;
        self.attributes = ButtonAttributes::load(attributes, runtime)?;
        Ok(())
    }

    fn render(
        &self, id: ComponentId,
        attributes: &ComponentAttributes, computed_size: Vector2<f32>, renderer: &mut Renderer,
    ) -> Result<(), Error> {
        self.background.render(id, attributes, computed_size, renderer, self.hovering)?;

        if let Some(ref text) = self.attributes.text {
            renderer.text(
                id, &text, self.attributes.text_font.as_ref(), self.attributes.text_size,
                Point2::new(0.0, 0.0), computed_size, self.attributes.text_color,
            )?;
        }

        Ok(())
    }

    fn is_capturing_cursor(&self) -> bool {
        true
    }

    fn hover_start_event(&mut self, _event_sink: &mut EventSink) -> bool {
        self.hovering = true;
        true
    }

    fn hover_end_event(&mut self, _event_sink: &mut EventSink) -> bool {
        self.hovering = false;
        true
    }

    fn pressed_event(&mut self, event_sink: &mut EventSink) {
        if let Some(ref event) = self.attributes.on_pressed {
            event_sink.raise(event);
        }
    }
}

struct ButtonAttributes {
    text: Option<String>,
    text_color: Color,
    text_font: Option<String>,
    text_size: Option<i32>,
    on_pressed: Option<EventHook>,
}

impl ButtonAttributes {
    pub fn load(attributes: &Attributes, runtime: &ScriptRuntime) -> Result<Self, Error> {
        Ok(ButtonAttributes {
            text: attributes.attribute_optional("text", |v| v.as_string(runtime))?,
            text_color: attributes.attribute(
                "text-color", |v| v.as_color(runtime), Color::new_u8(0, 0, 0, 255)
            )?,
            text_font: attributes.attribute_optional("text-font", |v| v.as_string(runtime))?,
            text_size: attributes.attribute_optional("text-size", |v| v.as_integer(runtime))?,
            on_pressed: attributes.attribute_optional("on-pressed", |v| v.as_event_hook(runtime))?,
        })
    }
}
