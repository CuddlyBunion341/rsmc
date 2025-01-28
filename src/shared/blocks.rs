use serde::{Deserialize, Serialize};

macro_rules! enum_from_u8 {
    ($name:ident { $( $variant:ident ),* $(,)? }) => {
        #[repr(u8)]
        #[derive(Debug, PartialEq, Copy, Clone, Deserialize, Serialize)]
        pub enum $name {
            $( $variant ),*
        }

        impl $name {
            pub fn from_u8(value: u8) -> Option<$name> {
                match value {
                    $(x if x == $name::$variant as u8 => Some($name::$variant),)*
                        _ => None,
                }
            }

            pub fn to_u8(&self) -> u8 {
                self.clone() as u8
            }
        }
    };
}

enum_from_u8! {
    BlockId {
        Air,
        Grass,
        Dirt,
        Stone,
        Bedrock,
        RedSand,
        BrownTerracotta,
        CyanTerracotta,
        GrayTerracotta,
        LightGrayTerracotta,
        OrangeTerracotta,
        RedTerracotta,
        Terracotta,
        YellowTerracotta,
    }
}
