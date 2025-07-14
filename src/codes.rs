#[cfg(target_os = "windows")]
/// Windows virtual key codes
pub enum VirtualKey {
    /// Left mouse button
    LMouseButton = 0x01,
    /// Right mouse button
    RMouseButton = 0x02,
    /// Control-break processing
    Cancel = 0x03,
    /// Middle mouse button (three-button mouse)
    MiddleMouseButton = 0x04,
    /// X1 mouse button
    XMouseButton1 = 0x05,
    /// X2 mouse button
    XMouseButton2 = 0x06,
    /// BACKSPACE key
    Backspace = 0x08,
    /// TAB key
    Tab = 0x09,
    /// CLEAR key
    Clear = 0x0C,
    /// ENTER key
    Enter = 0x0D,
    /// SHIFT key
    Shift = 0x10,
    /// CTRL key
    Control = 0x11,
    /// ALT key
    Alt = 0x12,
    /// PAUSE key
    Pause = 0x13,
    /// CAPS LOCK key
    CapsLock = 0x14,
    /// IME Kana mode or HANGUL/HANGUEL mode
    KanaHangulHanguel = 0x15,
    /// IME Junja mode
    Junja = 0x17,
    /// IME final mode
    Final = 0x18,
    /// IME Hanja or Kanji mode
    HanjaKanji = 0x19,
    /// ESC key
    Escape = 0x1B,
    /// IME convert
    Convert = 0x1C,
    /// IME nonconvert
    NonConvert = 0x1D,
    /// IME accept
    Accept = 0x1E,
    /// IME mode change request
    ModeChange = 0x1F,
    /// SPACEBAR
    Space = 0x20,
    /// PAGE UP key
    PageUp = 0x21,
    /// PAGE DOWN key
    PageDown = 0x22,
    /// END key
    End = 0x23,
    /// HOME key
    Home = 0x24,
    /// LEFT ARROW key
    Left = 0x25,
    /// UP ARROW key
    Up = 0x26,
    /// RIGHT ARROW key
    Right = 0x27,
    /// DOWN ARROW key
    Down = 0x28,
    /// SELECT key
    Select = 0x29,
    /// PRINT key
    Print = 0x2A,
    /// EXECUTE key
    Execute = 0x2B,
    /// PRINT SCREEN key
    PrintScreen = 0x2C,
    /// INS key
    Insert = 0x2D,
    /// DEL key
    Delete = 0x2E,
    /// HELP key
    Help = 0x2F,
    /// 0 key
    N0 = 0x30,
    /// 1 key
    N1 = 0x31,
    /// 2 key
    N2 = 0x32,
    /// 3 key
    N3 = 0x33,
    /// 4 key
    N4 = 0x34,
    /// 5 key
    N5 = 0x35,
    /// 6 key
    N6 = 0x36,
    /// 7 key
    N7 = 0x37,
    /// 8 key
    N8 = 0x38,
    /// 9 key
    N9 = 0x39,
    /// A key
    A = 0x41,
    /// B key
    B = 0x42,
    /// C key
    C = 0x43,
    /// D key
    D = 0x44,
    /// E key
    E = 0x45,
    /// F key
    F = 0x46,
    /// G key
    G = 0x47,
    /// H key
    H = 0x48,
    /// I key
    I = 0x49,
    /// J key
    J = 0x4A,
    /// K key
    K = 0x4B,
    /// L key
    L = 0x4C,
    /// M key
    M = 0x4D,
    /// N key
    N = 0x4E,
    /// O key
    O = 0x4F,
    /// P key
    P = 0x50,
    /// Q key
    Q = 0x51,
    /// R key
    R = 0x52,
    /// S key
    S = 0x53,
    /// T key
    T = 0x54,
    /// U key
    U = 0x55,
    /// V key
    V = 0x56,
    /// W key
    W = 0x57,
    /// X key
    X = 0x58,
    /// Y key
    Y = 0x59,
    /// Z key
    Z = 0x5A,
    /// Left Windows key (Natural keyboard)
    LWin = 0x5B,
    /// Right Windows key (Natural keyboard)
    RWin = 0x5C,
    /// Applications key (Natural keyboard)
    Apps = 0x5D,
    /// Computer Sleep key
    Sleep = 0x5F,
    /// Numeric keypad 0 key
    Numpad0 = 0x60,
    /// Numeric keypad 1 key
    Numpad1 = 0x61,
    /// Numeric keypad 2 key
    Numpad2 = 0x62,
    /// Numeric keypad 3 key
    Numpad3 = 0x63,
    /// Numeric keypad 4 key
    Numpad4 = 0x64,
    /// Numeric keypad 5 key
    Numpad5 = 0x65,
    /// Numeric keypad 6 key
    Numpad6 = 0x66,
    /// Numeric keypad 7 key
    Numpad7 = 0x67,
    /// Numeric keypad 8 key
    Numpad8 = 0x68,
    /// Numeric keypad 9 key
    Numpad9 = 0x69,
    /// Multiply key
    Multiply = 0x6A,
    /// Add key
    Add = 0x6B,
    /// Separator key
    Seperator = 0x6C,
    /// Subtract key
    Subtract = 0x6D,
    /// Decimal key
    Decimal = 0x6E,
    /// Divide key
    Divide = 0x6F,
    /// F1 key
    F1 = 0x70,
    /// F2 key
    F2 = 0x71,
    /// F3 key
    F3 = 0x72,
    /// F4 key
    F4 = 0x73,
    /// F5 key
    F5 = 0x74,
    /// F6 key
    F6 = 0x75,
    /// F7 key
    F7 = 0x76,
    /// F8 key
    F8 = 0x77,
    /// F9 key
    F9 = 0x78,
    /// F10 key
    F10 = 0x79,
    /// F11 key
    F11 = 0x7A,
    /// F12 key
    F12 = 0x7B,
    /// F13 key
    F13 = 0x7C,
    /// F14 key
    F14 = 0x7D,
    /// F15 key
    F15 = 0x7E,
    /// F16 key
    F16 = 0x7F,
    /// F17 key
    F17 = 0x80,
    /// F18 key
    F18 = 0x81,
    /// F19 key
    F19 = 0x82,
    /// F20 key
    F20 = 0x83,
    /// F21 key
    F21 = 0x84,
    /// F22 key
    F22 = 0x85,
    /// F23 key
    F23 = 0x86,
    /// F24 key
    F24 = 0x87,
    /// NUM LOCK key
    Numlock = 0x90,
    /// SCROLL LOCK key
    Scroll = 0x91,
    /// Left SHIFT key
    LShift = 0xA0,
    /// Right SHIFT key
    RShift = 0xA1,
    /// Left CONTROL key
    LControl = 0xA2,
    /// Right CONTROL key
    RControl = 0xA3,
    /// Left MENU key
    LMenu = 0xA4,
    /// Right MENU key
    RMenu = 0xA5,
    /// Browser Back key
    BrowserBack = 0xA6,
    /// Browser Forward key
    BrowserForward = 0xA7,
    /// Browser Refresh key
    BrowserRefresh = 0xA8,
    /// Browser Stop key
    BrowserStop = 0xA9,
    /// Browser Search key
    BrowserSearch = 0xAA,
    /// Browser Favorites key
    BrowserFavorites = 0xAB,
    /// Browser Start and Home key
    BrowserHome = 0xAC,
    /// Volume Mute key
    VolumeMute = 0xAD,
    /// Volume Down key
    VolumeDown = 0xAE,
    /// Volume Up key
    VolumeUp = 0xAF,
    /// Next Track key
    MediaNextTrack = 0xB0,
    /// Previous Track key
    MediaPrevTrack = 0xB1,
    /// Stop Media key
    MediaStop = 0xB2,
    /// Play/Pause Media key
    MediaPlayPause = 0xB3,
    /// Start Mail key
    LaunchMail = 0xB4,
    /// Select Media key
    LaunchMediaSelect = 0xB5,
    /// Start Application 1 key
    LaunchApp1 = 0xB6,
    /// Start Application 2 key
    LaunchApp2 = 0xB7,
    /// Used for miscellaneous characters, it can vary by keyboard.
    Oem1 = 0xBA,
    /// For any country/region, the '+' key
    OemPlus = 0xBB,
    /// For any country/region, the ',' key
    OemComma = 0xBC,
    /// For any country/region, the '-' key
    OemMinus = 0xBD,
    /// For any country/region, the '.' key
    OemPeriod = 0xBE,
    /// Used for miscellaneous characters, it can vary by keyboard.
    Oem2 = 0xBF,
    /// Used for miscellaneous characters, it can vary by keyboard.
    Oem3 = 0xC0,
    /// Used for miscellaneous characters, it can vary by keyboard.
    Oem4 = 0xDB,
    /// Used for miscellaneous characters, it can vary by keyboard.
    Oem5 = 0xDC,
    /// Used for miscellaneous characters, it can vary by keyboard.
    Oem6 = 0xDD,
    /// Used for miscellaneous characters, it can vary by keyboard.
    Oem7 = 0xDE,
    /// Used for miscellaneous characters, it can vary by keyboard.
    Oem8 = 0xDF,
    /// Either the angle bracket key or the backslash key on the RT 102-key keyboard
    Oem102 = 0xE2,
    /// IME PROCESS key
    ProcessKey = 0xE5,
    /// Used to pass Unicode characters as if they were keystrokes.
    Packet = 0xE7,
    /// Attn key
    Attn = 0xF6,
    /// CrSel key
    CrSel = 0xF7,
    /// ExSel key
    ExSel = 0xF8,
    /// Erase EOF key
    ErEOF = 0xF9,
    /// Play key
    Play = 0xFA,
    /// Zoom key
    Zoom = 0xFB,
    /// PA1 key
    Pa1 = 0xFD,
    /// Clear key
    OemClear = 0xFE,
}
