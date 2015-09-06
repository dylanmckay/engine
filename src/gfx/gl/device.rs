
use gfx::{self,gl,util};
use gfx::input::{self,Event};
use libgl;
use geom;

use std::collections::LinkedList;

pub struct Device<B: gl::Backend>
{
    backend: B,
    event_queue: LinkedList<Event>,
    input_state: input::State,
}

impl<B: gl::Backend> Device<B>
{
    pub fn new(backend: B) -> Self {
        Device {
            backend: backend,
            event_queue: LinkedList::new(),
            input_state: Default::default(),
        }
    }

    pub fn run(&mut self) {

        let mut tmp = Vec::new();
        self.backend.run(&mut tmp);

        for event in tmp {
            self.input_state.process(&event);
            self.event_queue.push_front(event);
        }
    }

    pub fn is_open(&self) -> bool {
        self.backend.is_open()
    }

    pub fn begin(&self) -> gl::Canvas {
        use gfx::Viewport;
        self.area().begin()
    }

    pub fn end(&mut self) {
        self.backend.end()
    }

    /// Gets the viewport containing the entire area.
    pub fn area(&self) -> gl::Viewport {
        use gfx::Viewport;

        let (width,height) = self.dimensions();
        let half_extents = (width/2,height/2);

        gl::Viewport::new(half_extents, half_extents)
    }

    /// Gets the dimensions in pixels.
    pub fn dimensions(&self) -> (u32,u32) {
        self.backend.dimensions()
    }

    /// Enables culling.
    pub fn enable_culling(&mut self) {
        unsafe {
            libgl::Enable(libgl::CULL_FACE);
            // Use clockwise vertices as front,
            libgl::FrontFace(libgl::CW);
        }
    }

    /// Disables culling.
    pub fn disable_culling(&mut self) {
        unsafe {
            libgl::Disable(libgl::CULL_FACE);
        }
    }

    /// Sets the culling mode.
    /// This enables culling if it is disabled.
    pub fn set_culling_mode(&mut self, mode: gfx::CullingMode) {
        let mode_enum = gl::util::culling_mode(mode);

        self.enable_culling();
        unsafe {
            libgl::CullFace(mode_enum);
        }
    }

    /// Loads mesh data.
    pub fn load_mesh_data<I,V>(&mut self, data: &geom::mesh::Data<I,V>)
        -> gl::mesh::Data
        where I: gl::Type, V: gl::Vertex {
        gl::mesh::Data::new().load(data, libgl::STATIC_DRAW)
    }

    /// Sets the title.
    /// This operation is not always supported. If this is the case, this
    /// function does nothing.
    pub fn set_title(&mut self, title: &str) {
        self.backend.set_title(title);
    }

    /// Maps from normalized window coordinates to pixel coordinates.
    pub fn map_point_to_pixel(&self, point: (f32,f32)) -> (u32,u32) {
        use num::Cast;

        let dimensions: (f32,f32) = self.dimensions().cast();
        util::map_point_to_pixel(point, dimensions).cast()
    }

    /// Maps from pixel coordinates to normalized window coordinates.
    pub fn map_pixel_to_point(&self, point: (u32,u32)) -> (f32,f32) {
        use num::Cast;

        util::map_pixel_to_point(point, self.dimensions()).cast()
    }

    /// Gets an iterator of events.
    pub fn events(&mut self) -> Events {
        use std::mem;

        let mut queue = LinkedList::new();
        mem::swap(&mut queue, &mut self.event_queue);

        Events {
            queue: queue,
        }
    }

    /// Gets the current input state.
    pub fn input_state<'a>(&'a self) -> &'a input::State {
        &self.input_state
    }
}

pub struct Events
{
    queue: LinkedList<Event>,
}

impl Iterator for Events
{
    type Item = Event;

    fn next(&mut self) -> Option<Event> {
        self.queue.pop_front()
    }
}
