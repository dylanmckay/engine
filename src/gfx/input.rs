
pub use self::keyboard::{NumberSource,Side,Key};

/// An event.
#[derive(Copy,Clone,Debug)]
pub enum Event
{
    Key(Key, Action),
    /// A mouse event.
    Mouse(mouse::Event),
}

/// The current input device states.
#[derive(Clone,Default,Debug)]
pub struct State
{
    keyboard: keyboard::State,
    mouse: mouse::State,
}

impl State
{
    /// Gets the state of the keyboard.
    pub fn keyboard<'a>(&'a self) -> &'a keyboard::State {
        &self.keyboard
    }

    /// Gets the state of the mouse.
    pub fn mouse<'a>(&'a self) -> &'a mouse::State {
        &self.mouse
    }

    /// Processes an event and updates the state accordingly.
    pub fn process(&mut self, event: &Event) {
        match *event {
            Event::Mouse(e) => self.mouse.process(e),
            Event::Key(key, action) => unimplemented!(),
        }
    }
}

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum Action
{
    Press,
    Release,
}

pub mod keyboard
{
    use std;
    use std::fmt;
    use std::collections::LinkedList;

    /// Holds keyboard state.
    #[derive(Clone,Default,Debug,PartialEq,Eq)]
    pub struct State
    {
        pressed: LinkedList<Key>,
    }

    impl State
    {
        /// Checks if a key is pressed.
        pub fn pressed(&self, key: Key) -> bool {
            self.pressed.iter().any(|&a| a==key)
        }

        /// Gets an iterator over the pressed keys.
        pub fn pressed_keys<'a>(&'a self)
            -> std::collections::linked_list::Iter<'a, Key> {
            self.pressed.iter()
        }
    }

    /// The source of a number-related key.
    #[derive(Copy,Clone,Debug,PartialEq,Eq)]
    pub enum NumberSource
    {
        /// The horizontal number row.
        Row,
        /// The number pad.
        Pad,
    }

    /// The side of the keyboard that was pressed.
    #[derive(Copy,Clone,Debug,PartialEq,Eq)]
    pub enum Side
    {
        Left,
        Right,
    }

    #[derive(Copy,Clone,Debug,PartialEq,Eq)]
    pub enum Key
    {
        Space,
        Apostrophe,
        Comma,
        Minus,
        Period,
        Slash,
        Backslash,
        Semicolon,
        Equal,
        GraveAccent,

        /// A number from the horizontal number row.
        Num(u8, NumberSource),

        A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P,
        Q, R, S, T, U, V, W, X, Y, Z,

        LeftBracket, RightBracket,

        Decimal,
        Divide,
        Multiply,
        Subtract,
        Add,

        Escape,
        Enter,
        Tab,
        Backspace,
        Insert,
        Delete,
        
        Up, Down, Left, Right,

        PageUp, PageDown,
        Home,
        End,
        CapsLock,
        ScrollLock,
        NumLock,
        PrintScreen,
        Pause,

        Function(u8),

        Shift(Side),
        Control(Side),
        Alt(Side),
        Super(Side),
        Menu,
    }

    impl fmt::Display for Key
    {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Key::Space => ' '.fmt(fmt),
                Key::Apostrophe => '\''.fmt(fmt),
                Key::Comma => ','.fmt(fmt),
                Key::Minus => '-'.fmt(fmt),
                Key::Period => '.'.fmt(fmt),
                Key::Slash => '/'.fmt(fmt),
                Key::Backslash => '\''.fmt(fmt),
                Key::Semicolon => ';'.fmt(fmt),
                Key::Equal => '='.fmt(fmt),
                Key::GraveAccent => '`'.fmt(fmt),
                Key::Num(n,_) => n.fmt(fmt),

                Key::A => 'a'.fmt(fmt),
                Key::B => 'b'.fmt(fmt),
                Key::C => 'c'.fmt(fmt),
                Key::D => 'd'.fmt(fmt),
                Key::E => 'e'.fmt(fmt),
                Key::F => 'f'.fmt(fmt),
                Key::G => 'g'.fmt(fmt),
                Key::H => 'h'.fmt(fmt),
                Key::I => 'i'.fmt(fmt),
                Key::J => 'j'.fmt(fmt),
                Key::K => 'k'.fmt(fmt),
                Key::L => 'l'.fmt(fmt),
                Key::M => 'm'.fmt(fmt),
                Key::N => 'n'.fmt(fmt),
                Key::O => 'o'.fmt(fmt),
                Key::P => 'p'.fmt(fmt),
                Key::Q => 'q'.fmt(fmt),
                Key::R => 'r'.fmt(fmt),
                Key::S => 's'.fmt(fmt),
                Key::T => 't'.fmt(fmt),
                Key::U => 'u'.fmt(fmt),
                Key::V => 'v'.fmt(fmt),
                Key::W => 'w'.fmt(fmt),
                Key::X => 'x'.fmt(fmt),
                Key::Y => 'y'.fmt(fmt),
                Key::Z => 'z'.fmt(fmt),

                Key::LeftBracket => '('.fmt(fmt),
                Key::RightBracket => ')'.fmt(fmt),
                Key::Decimal => '.'.fmt(fmt),
                Key::Divide => '/'.fmt(fmt),
                Key::Multiply => '*'.fmt(fmt),
                Key::Subtract => '-'.fmt(fmt),
                Key::Add => '+'.fmt(fmt),

                Key::Escape => "Esc".fmt(fmt),
                Key::Enter => "Enter".fmt(fmt),
                Key::Tab => "Tab".fmt(fmt),
                Key::Backspace => "Backspace".fmt(fmt),
                Key::Insert => "Insert".fmt(fmt),
                Key::Delete => "Delete".fmt(fmt),

                Key::Up => "Up".fmt(fmt),
                Key::Down => "Down".fmt(fmt),
                Key::Left => "Left".fmt(fmt),
                Key::Right => "Right".fmt(fmt),

                Key::PageUp => "Page Up".fmt(fmt),
                Key::PageDown => "Page Down".fmt(fmt),
                Key::Home => "Home".fmt(fmt),
                Key::End => "End".fmt(fmt),
                Key::CapsLock => "Caps Lock".fmt(fmt),
                Key::ScrollLock => "Scroll Lock".fmt(fmt),
                Key::NumLock => "NumLock".fmt(fmt),
                Key::PrintScreen => "Print Screen".fmt(fmt),
                Key::Pause => "Pause".fmt(fmt),

                Key::Function(num) => format!("F{}", num).fmt(fmt),
             
                Key::Shift(..) => "Shift".fmt(fmt),
                Key::Control(..) => "Ctrl".fmt(fmt),
                Key::Alt(..) => "Alt".fmt(fmt),
                Key::Super(..) => "Super".fmt(fmt),
                Key::Menu => "Menu".fmt(fmt),
            }
        }
    }
}

pub mod mouse
{
    use super::Action;
    use std;

    /// Holds mouse state.
    #[derive(Clone,Default,Debug)]
    pub struct State
    {
        position: (f32,f32),
        pressed_buttons: Vec<Button>,
    }

    impl State
    {
        /// Gets the position of the mouse.
        pub fn position(&self) -> (f32,f32) { self.position }

        /// Checks if a button is pressed.
        pub fn pressed(&self, button: Button) -> bool {
            self.pressed_buttons.contains(&button)
        }

        /// Gets the buttons that are pressed.
        pub fn pressed_buttons<'a>(&'a self) -> std::slice::Iter<'a,Button> {
            self.pressed_buttons.iter()
        }

        pub fn process(&mut self, (kind,info): Event) {
            self.position = info.position();
            match kind {
                Kind::Move => (), // do nothing, we already set the position
                Kind::Button(button, action) => {
                    match action {
                        Action::Press => {
                            debug_assert!(!self.pressed_buttons.contains(&button),
                                          "button cannot be pressed while it already is");

                            self.pressed_buttons.push(button);
                        },
                        Action::Release => {
                            debug_assert!(self.pressed_buttons.contains(&button),
                                          "button should already be pressed");

                            let idx = self.pressed_buttons.iter().position(|&b| b==button).unwrap();
                            self.pressed_buttons.remove(idx);
                        },
                    }
                },
            }

        }
    }

    /// A mouse event.
    #[derive(Copy,Clone,Debug,PartialEq,Eq)]
    pub enum Kind
    {
        /// A button was pressed or released.
        Button(Button, Action),
        Move,
    }

    pub type Event = (Kind,Info);

    /// Information about a mouse event.
    #[derive(Copy,Clone,Debug)]
    pub struct Info {
        pub position: (f32,f32),
    }

    impl Info
    {
        pub fn position(&self) -> (f32,f32) { self.position }
    }

    /// Specifies a mouse button.
    #[derive(Copy,Clone,Debug,Eq,PartialEq)]
    pub enum Button {
        Left,
        Middle,
        Right,
    }
}
