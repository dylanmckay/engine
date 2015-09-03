
extern crate glfw;

use gfx;
use gfx::gl;
use gfx::input::Event;

use std::sync::mpsc::Receiver;
use std::collections::LinkedList;

pub struct Backend
{
    glfw: glfw::Glfw,
    window: glfw::Window,
    events: Receiver<(f64,glfw::WindowEvent)>,
}

impl Backend
{
    pub fn new() -> Backend {
        use self::glfw::Context;


        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut window, events) = glfw.create_window(500,500,
                                                      gl::backends::DEFAULT_TITLE,
                                                      glfw::WindowMode::Windowed)
                                       .expect("Failed to create GLFW window");

        gfx::gl::gl::load_with(|s| window.get_proc_address(s));

        window.set_key_polling(true);
        window.set_mouse_button_polling(true);

        window.make_current();

        Backend {
            glfw: glfw,
            window: window,
            events: events,
        }
    }
}

impl gfx::gl::Backend for Backend
{
    fn run(&mut self, events: &mut LinkedList<Event>) {
        self.glfw.poll_events();

        for (_,event) in glfw::flush_messages(&self.events) {
            match self::util::into_event(event, &self.window) {
                Some(e) => {
                    events.push_back(e);
                },
                None => (),
            }
        }
    }

    fn end(&mut self) {
        use self::glfw::Context;
        self.window.swap_buffers()
    }

    fn is_open(&self) -> bool {
        !self.window.should_close()
    }

    fn dimensions(&self) -> (u32,u32) {
        let (width,height) = self.window.get_size();
        (width as u32, height as u32)
    }

    fn set_title(&mut self, title: &str) {
        self.window.set_title(title)
    }
}

/// Useful utilities.
pub mod util
{
    use super::glfw;
    use gfx::input::{self,Event};

    pub fn into_event(event: glfw::WindowEvent,
                      window: &glfw::Window)
        -> Option<Event> {

        match event {
            glfw::WindowEvent::Key(gkey, _, gaction, _) => {
                let action = match into_action(gaction) {
                    Some(a) => a,
                    None => { return None; },
                };

                let key = match into_key(gkey) {
                    Some(key) => key,
                    None => { return None; },
                };

                Some(Event::Key(key, action))

            },
            glfw::WindowEvent::MouseButton(gbutton, gaction, _) => {
                let button = match into_mouse_button(gbutton) {
                    Some(b) => b,
                    None => { return None; },
                };

                let action = match into_action(gaction) {
                    Some(a) => a,
                    None => { return None; },
                };

                let kind = input::mouse::Kind::Button(button,action);
                let mouse_event = create_mouse_event(kind, &window);

                Some(input::Event::Mouse(mouse_event))

            },
            _ => unimplemented!(),
        }
    }

    pub fn into_action(action: glfw::Action) -> Option<input::Action> {
        match action {
            glfw::Action::Press => Some(input::Action::Press),
            glfw::Action::Release => Some(input::Action::Release),
            _ => None, // we don't handle this
        }
    }

    // TODO: implement all keys
    pub fn into_key(key: glfw::Key) -> Option<input::Key> {
        use gfx::input::{Key,Side};

        match key {
            glfw::Key::Space => Some(Key::Space),
            glfw::Key::Apostrophe => Some(Key::Apostrophe),
            glfw::Key::Comma => Some(Key::Comma),
            glfw::Key::Minus => Some(Key::Minus),
            glfw::Key::Period => Some(Key::Period),
            glfw::Key::Slash => Some(Key::Slash),
            glfw::Key::Num0 => Some(Key::Num(0, input::NumberSource::Row)),
            glfw::Key::Num1 => Some(Key::Num(1, input::NumberSource::Row)),
            glfw::Key::Num2 => Some(Key::Num(2, input::NumberSource::Row)),
            glfw::Key::Num3 => Some(Key::Num(3, input::NumberSource::Row)),
            glfw::Key::Num4 => Some(Key::Num(4, input::NumberSource::Row)),
            glfw::Key::Num5 => Some(Key::Num(5, input::NumberSource::Row)),
            glfw::Key::Num6 => Some(Key::Num(6, input::NumberSource::Row)),
            glfw::Key::Num7 => Some(Key::Num(7, input::NumberSource::Row)),
            glfw::Key::Num8 => Some(Key::Num(8, input::NumberSource::Row)),
            glfw::Key::Num9 => Some(Key::Num(9, input::NumberSource::Row)),
            glfw::Key::Semicolon => Some(Key::Semicolon),
            glfw::Key::Equal => Some(Key::Equal),
            glfw::Key::A => Some(Key::A),
            glfw::Key::B => Some(Key::B),
            glfw::Key::C => Some(Key::C),
            glfw::Key::D => Some(Key::D),
            glfw::Key::E => Some(Key::E),
            glfw::Key::F => Some(Key::F),
            glfw::Key::G => Some(Key::G),
            glfw::Key::H => Some(Key::H),
            glfw::Key::I => Some(Key::I),
            glfw::Key::J => Some(Key::J),
            glfw::Key::K => Some(Key::K),
            glfw::Key::L => Some(Key::L),
            glfw::Key::M => Some(Key::M),
            glfw::Key::N => Some(Key::N),
            glfw::Key::O => Some(Key::O),
            glfw::Key::P => Some(Key::P),
            glfw::Key::Q => Some(Key::Q),
            glfw::Key::R => Some(Key::R),
            glfw::Key::S => Some(Key::S),
            glfw::Key::T => Some(Key::T),
            glfw::Key::U => Some(Key::U),
            glfw::Key::V => Some(Key::V),
            glfw::Key::W => Some(Key::W),
            glfw::Key::X => Some(Key::X),
            glfw::Key::Y => Some(Key::Y),
            glfw::Key::Z => Some(Key::Z),
            glfw::Key::LeftBracket => Some(Key::LeftBracket),
            glfw::Key::Backslash => Some(Key::Backslash),
            glfw::Key::RightBracket => Some(Key::RightBracket),
            glfw::Key::GraveAccent => Some(Key::GraveAccent),
            glfw::Key::World1 => None, // do not handle
            glfw::Key::World2 => None, // do not handle
            glfw::Key::Escape => Some(Key::Escape),
            glfw::Key::Enter => Some(Key::Enter),
            glfw::Key::Tab => Some(Key::Tab),
            glfw::Key::Backspace => Some(Key::Backspace),
            glfw::Key::Insert => Some(Key::Insert),
            glfw::Key::Delete => Some(Key::Delete),
            glfw::Key::Right => Some(Key::Right),
            glfw::Key::Left => Some(Key::Left),
            glfw::Key::Down => Some(Key::Down),
            glfw::Key::Up => Some(Key::Up),
            glfw::Key::PageUp => Some(Key::PageUp),
            glfw::Key::PageDown => Some(Key::PageDown),
            glfw::Key::Home => Some(Key::Home),
            glfw::Key::End => Some(Key::End),
            glfw::Key::CapsLock => Some(Key::CapsLock),
            glfw::Key::ScrollLock => Some(Key::ScrollLock),
            glfw::Key::NumLock => Some(Key::NumLock),
            glfw::Key::PrintScreen => Some(Key::PrintScreen),
            glfw::Key::Pause => Some(Key::Pause),

            glfw::Key::F1 => Some(Key::Function(1)),
            glfw::Key::F2 => Some(Key::Function(2)),
            glfw::Key::F3 => Some(Key::Function(3)),
            glfw::Key::F4 => Some(Key::Function(4)),
            glfw::Key::F5 => Some(Key::Function(5)),
            glfw::Key::F6 => Some(Key::Function(6)),
            glfw::Key::F7 => Some(Key::Function(7)),
            glfw::Key::F8 => Some(Key::Function(8)),
            glfw::Key::F9 => Some(Key::Function(9)),
            glfw::Key::F10 => Some(Key::Function(10)),
            glfw::Key::F11 => Some(Key::Function(11)),
            glfw::Key::F12 => Some(Key::Function(12)),
            glfw::Key::F13 => Some(Key::Function(13)),
            glfw::Key::F14 => Some(Key::Function(14)),
            glfw::Key::F15 => Some(Key::Function(15)),
            glfw::Key::F16 => Some(Key::Function(16)),
            glfw::Key::F17 => Some(Key::Function(17)),
            glfw::Key::F18 => Some(Key::Function(18)),
            glfw::Key::F19 => Some(Key::Function(19)),
            glfw::Key::F20 => Some(Key::Function(20)),
            glfw::Key::F21 => Some(Key::Function(21)),
            glfw::Key::F22 => Some(Key::Function(22)),
            glfw::Key::F23 => Some(Key::Function(23)),
            glfw::Key::F24 => Some(Key::Function(24)),
            glfw::Key::F25 => Some(Key::Function(25)),

            glfw::Key::Kp0 => Some(Key::Num(0, input::NumberSource::Pad)),
            glfw::Key::Kp1 => Some(Key::Num(1, input::NumberSource::Pad)),
            glfw::Key::Kp2 => Some(Key::Num(2, input::NumberSource::Pad)),
            glfw::Key::Kp3 => Some(Key::Num(3, input::NumberSource::Pad)),
            glfw::Key::Kp4 => Some(Key::Num(4, input::NumberSource::Pad)),
            glfw::Key::Kp5 => Some(Key::Num(5, input::NumberSource::Pad)),
            glfw::Key::Kp6 => Some(Key::Num(6, input::NumberSource::Pad)),
            glfw::Key::Kp7 => Some(Key::Num(7, input::NumberSource::Pad)),
            glfw::Key::Kp8 => Some(Key::Num(8, input::NumberSource::Pad)),
            glfw::Key::Kp9 => Some(Key::Num(9, input::NumberSource::Pad)),

            glfw::Key::KpDecimal => Some(Key::Decimal),
            glfw::Key::KpDivide => Some(Key::Divide),
            glfw::Key::KpMultiply => Some(Key::Multiply),
            glfw::Key::KpSubtract => Some(Key::Subtract),
            glfw::Key::KpAdd => Some(Key::Add),
            glfw::Key::KpEnter => Some(Key::Enter),
            glfw::Key::KpEqual => Some(Key::Equal),
            glfw::Key::LeftShift => Some(Key::Shift(Side::Left)),
            glfw::Key::LeftControl => Some(Key::Control(Side::Left)),
            glfw::Key::LeftAlt => Some(Key::Control(Side::Left)),
            glfw::Key::LeftSuper => Some(Key::Super(Side::Left)),
            glfw::Key::RightShift => Some(Key::Shift(Side::Right)),
            glfw::Key::RightControl => Some(Key::Control(Side::Right)),
            glfw::Key::RightAlt => Some(Key::Alt(Side::Right)),
            glfw::Key::RightSuper => Some(Key::Super(Side::Right)),
            glfw::Key::Menu => Some(Key::Menu),
        }
    }

    pub fn into_mouse_button(button: glfw::MouseButton) -> Option<input::mouse::Button> {
        match button {
            glfw::MouseButton::Button1 => Some(input::mouse::Button::Left),
            glfw::MouseButton::Button2 => Some(input::mouse::Button::Right),
            glfw::MouseButton::Button3 => Some(input::mouse::Button::Middle),
            glfw::MouseButton::Button4 => None,
            glfw::MouseButton::Button5 => None,
            glfw::MouseButton::Button6 => None,
            glfw::MouseButton::Button7 => None,
            glfw::MouseButton::Button8 => None,
        }
    }

    pub fn create_mouse_event(kind: input::mouse::Kind,
                              window: &glfw::Window)
        -> input::mouse::Event {
        // get cursor pos as relatie position
        let (x,y) = window.get_cursor_pos();

        let info = input::mouse::Info {
            pos: (x as f32, y as f32),
        };

        (kind, info)
    }
}
