//! Rendering functions and backend traits.

use nalgebra::{Point2, Vector2};
use template::{Color};
use {ComponentId, Ui, Error, ComponentFlow};

/// A renderer backend, implements how individual rendering operations are done.
pub trait Renderer {
    fn render_cache_to_target(&mut self, id: ComponentId) -> Result<(), Error>;

    /// Returns true if the cache is empty.
    fn create_resize_cache(
        &mut self, id: ComponentId, size: Vector2<u32>
    ) -> Result<bool, Error>;

    fn clear_cache(&mut self, id: ComponentId) -> Result<(), Error>;

    fn render_cache(
        &mut self, id: ComponentId,
        source_id: ComponentId, position: Point2<f32>
    ) -> Result<(), Error>;

    /// Renders text centered in an area to the component's cache.
    /// Font is a string identifier that should be resolved by the renderer's font cache.
    fn text(
        &mut self, id: ComponentId,
        text: &String, text_font: Option<&String>, text_size: Option<i32>,
        position: Point2<f32>, size: Vector2<f32>, color: Color,
    ) -> Result<(), Error>;

    /// Renders vertices to the component's cache.
    fn vertices(
        &mut self, id: ComponentId,
        vertices: &[Point2<f32>], indices: &[u16], color: Color,
    ) -> Result<(), Error>;
}

/// Renders a UI using a renderer backend.
pub fn render<R: Renderer>(
    renderer: &mut R, ui: &mut Ui
) -> Result<(), Error> {
    // TODO: Clear the cache of elements that don't exist anymore

    let root_id = ui.root_id();

    // Update the components' caches recursively, then render the final cache to the target
    let size = ui.target_size();
    update_component_cache(renderer, ui, root_id, size)?;
    renderer.render_cache_to_target(root_id)?;

    // Mark all components all not needing updating anymore
    ui.mark_all_rendered();

    Ok(())
}

fn update_component_cache<R: Renderer>(
    renderer: &mut R, ui: &Ui, component_id: ComponentId, parent_size: Vector2<f32>,
) -> Result<bool, Error> {
    let component = ui.get(component_id).unwrap();
    let computed_size = component.attributes().compute_size(parent_size);

    // Make sure this component's cache is created and of the correct size
    let cache_empty = renderer.create_resize_cache(component_id, Vector2::new(
        computed_size.x.ceil() as u32,
        computed_size.y.ceil() as u32,
    ))?;

    // Make sure all children's caches are up-to-date
    let mut child_updated = false;
    for child_id in component.children() {
        child_updated |= update_component_cache(renderer, ui, *child_id, computed_size)?;
    }

    // Only render if we need to
    if cache_empty || child_updated || component.needs_rendering() {
        renderer.clear_cache(component_id)?;

        // Let the component's class render itself to the component's cache
        component.render(component_id, computed_size, renderer)?;

        // Render all children caches in sequence to this component
        let mut flow = ComponentFlow::new(computed_size);
        for child_id in component.children() {
            let child = ui.get(*child_id).unwrap();
            let computed_position = child.attributes().compute_position(computed_size, &mut flow);
            renderer.render_cache(component_id, *child_id, computed_position)?;
        }

        Ok(true)
    } else {
        Ok(false)
    }
}
