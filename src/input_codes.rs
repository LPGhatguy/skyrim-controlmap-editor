use std::fmt::Debug;

pub trait InputCode: Debug + Sized {
    fn from_u32(value: u32) -> Option<Self>;
}

macro_rules! input_code {
    ($struct_name:ident { $($name:ident = $code:literal,)* }) => {
        #[derive(Debug)]
        pub enum $struct_name {
            $( $name = $code, )*
        }

        impl InputCode for $struct_name {
            fn from_u32(value: u32) -> Option<Self> {
                match value {
                    $( $code => Some(Self::$name), )*
                    _ => None
                }
            }
        }
    };
}

input_code!(Keyboard {
    Escape = 0x01,
    One = 0x02,
    Two = 0x03,
    Three = 0x04,
    Four = 0x05,
    Five = 0x06,
    Six = 0x07,
    Seven = 0x08,
    Eight = 0x09,
    Nine = 0x0A,
    Zero = 0x0B,
    Minus = 0x0C,
    Equals = 0x0D,
    Backspace = 0x0E,
    Tab = 0x0F,
    Q = 0x10,
    W = 0x11,
    E = 0x12,
    R = 0x13,
    T = 0x14,
    Y = 0x15,
    U = 0x16,
    I = 0x17,
    O = 0x18,
    P = 0x19,
    LeftBracket = 0x1A,
    RightBracket = 0x1B,
    Enter = 0x1C,
    LeftControl = 0x1D,
    A = 0x1E,
    S = 0x1F,
    D = 0x20,
    F = 0x21,
    G = 0x22,
    H = 0x23,
    J = 0x24,
    K = 0x25,
    L = 0x26,
    Semicolon = 0x27,
    Apostrophe = 0x28,
    Backtick = 0x29,
    LeftShift = 0x2A,
    BackSlash = 0x2B,
    Z = 0x2C,
    X = 0x2D,
    C = 0x2E,
    V = 0x2F,
    B = 0x30,
    N = 0x31,
    M = 0x32,
    Comma = 0x33,
    Period = 0x34,
    ForwardSlash = 0x35,
    RightShift = 0x36,
    NumpadStar = 0x37,
    LeftAlt = 0x38,
    Spacebar = 0x39,
    CapsLock = 0x3A,
    F1 = 0x3B,
    F2 = 0x3C,
    F3 = 0x3D,
    F4 = 0x3E,
    F5 = 0x3F,
    F6 = 0x40,
    F7 = 0x41,
    F8 = 0x42,
    F9 = 0x43,
    F10 = 0x44,
    NumLock = 0x45,
    ScrollLock = 0x46,
    Num7 = 0x47,
    Num8 = 0x48,
    Num9 = 0x49,
    NumMinus = 0x4A,
    Num4 = 0x4B,
    Num5 = 0x4C,
    Num6 = 0x4D,
    NumPlus = 0x4E,
    Num1 = 0x4F,
    Num2 = 0x50,
    Num3 = 0x51,
    Num0 = 0x52,
    NumPeriod = 0x53,
    F11 = 0x57,
    F12 = 0x58,
    NumEnter = 0x9C,
    RightControl = 0x9D,
    NumSlash = 0xB5,
    PrintScreen = 0xB7,
    RightAlt = 0xB8,
    Pause = 0xC5,
    Home = 0xC7,
    UpArrow = 0xC8,
    PageUp = 0xC9,
    LeftArrow = 0xCB,
    RightArrow = 0xCD,
    End = 0xCF,
    DownArrow = 0xD0,
    PageDown = 0xD1,
    Insert = 0xD2,
    Delete = 0xD3,
});

input_code!(Mouse {
    Button1 = 0x00,
    Button2 = 0x01,
    // Button3 = 0x02,
    // Button4 = 0x03,
    // Button5 = 0x04,
    Move = 0x0a,
    WheelUp = 0x08,
    WheelDown = 0x09,
});

input_code!(Gamepad {
    LeftStick = 0x000b,
    RightStick = 0x000c,
    LeftThumb = 0x0040,
    RightThumb = 0x0080,
    RightShoulder = 0x0100,
    LeftShoulder = 0x0200,
    A = 0x1000,
    B = 0x2000,
    X = 0x4000,
    Y = 0x8000,
    LeftTrigger = 0x0009,
    RightTrigger = 0x000a,
    DpadUp = 0x0001,
    DpadDown = 0x0002,
    DpadLeft = 0x0004,
    DpadRight = 0x0008,
    Back = 0x0020,
    Start = 0x0010,
});
