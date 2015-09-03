
pub use self::keyboard::{NumberSource,Side,Key};

/// An event.
#[derive(Copy,Clone,Debug)]
pub enum Event
{
    Key(Key, Action),
}

#[derive(Copy,Clone,Debug)]
pub enum Action
{
    Press,
    Release,
}

pub mod keyboard
{
    use std::fmt;

    /// The source of a number-related key.
    #[derive(Copy,Clone,Debug)]
    pub enum NumberSource
    {
        /// The horizontal number row.
        Row,
        /// The number pad.
        Pad,
    }

    /// The side of the keyboard that was pressed.
    #[derive(Copy,Clone,Debug)]
    pub enum Side
    {
        Left,
        Right,
    }

    #[derive(Copy,Clone,Debug)]
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
