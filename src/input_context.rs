macro_rules! contexts {
	($( $name:ident = $code:literal, )*) => {
		#[derive(Debug)]
		pub enum InputContext {
			$( $name = $code, )*
		}

		impl InputContext {
			pub fn from_u32(value: u32) -> Option<Self> {
				match value {
					$( $code => Some(Self::$name), )*
					_ => None
				}
			}
		}
	}
}

contexts! {
    MainGameplay = 0,
    Menu = 1,
    Console = 2,
    ItemMenu = 3,
    Inventory = 4,
    DebugText = 5,
    Favorites = 6,
    Map = 7,
    Stats = 8,
    Cursor = 9,
    Book = 10,
    DebugOverlay = 11,
    Journal = 12,
    TfcMode = 13,
    DebugMap = 14,
    Lockpicking = 15,
    Favor = 16,
}
