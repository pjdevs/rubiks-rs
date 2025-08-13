use core::fmt;

use crate::rubiks::location::CubePieceLocation;
use crate::rubiks::twist::Twist;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct CubeStickerLocation {
    pub piece_location: CubePieceLocation,
    pub twist: Twist
}

impl CubeStickerLocation {
    // pub const UFR: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::UFR), twist: Twist::SOLVED };
    // pub const FRU: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::UFR), twist: Twist::CW_120 };
    // pub const RUF: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::UFR), twist: Twist::CW_240 };
    // pub const UFL: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::UFL), twist: Twist::SOLVED };
    // pub const FLU: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::UFL), twist: Twist::CW_120 };
    // pub const LUF: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::UFL), twist: Twist::CW_240 };
    // pub const UBL: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::UBL), twist: Twist::SOLVED };
    // pub const BLU: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::UBL), twist: Twist::CW_120 };
    // pub const LUB: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::UBL), twist: Twist::CW_240 };
    // pub const UBR: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::UBR), twist: Twist::SOLVED };
    // pub const BRU: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::UBR), twist: Twist::CW_120 };
    // pub const RUB: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::UBR), twist: Twist::CW_240 };
    // pub const DFR: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::DFR), twist: Twist::SOLVED };
    // pub const FRD: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::DFR), twist: Twist::CW_120 };
    // pub const RDF: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::DFR), twist: Twist::CW_240 };
    // pub const DFL: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::DFL), twist: Twist::SOLVED };
    // pub const FLD: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::DFL), twist: Twist::CW_120 };
    // pub const LDF: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::DFL), twist: Twist::CW_240 };
    // pub const DBL: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::DBL), twist: Twist::SOLVED };
    // pub const BLD: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::DBL), twist: Twist::CW_120 };
    // pub const LDB: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::DBL), twist: Twist::CW_240 };
    // pub const DBR: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::DBR), twist: Twist::SOLVED };
    // pub const BRD: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::DBR), twist: Twist::CW_120 };
    // pub const RDB: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::DBR), twist: Twist::CW_240 };

    // pub const UF: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::UF), twist: Twist::SOLVED };
    // pub const FU: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::UF), twist: Twist::FLIPPED };
    // pub const UL: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::UL), twist: Twist::SOLVED };
    // pub const LU: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::UL), twist: Twist::FLIPPED };
    // pub const UB: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::UB), twist: Twist::SOLVED };
    // pub const BU: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::UB), twist: Twist::FLIPPED };
    // pub const UR: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::UR), twist: Twist::SOLVED };
    // pub const RU: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::UR), twist: Twist::FLIPPED };
    // pub const DF: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::DF), twist: Twist::SOLVED };
    // pub const FD: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::DF), twist: Twist::FLIPPED };
    // pub const DL: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::DL), twist: Twist::SOLVED };
    // pub const LD: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::DL), twist: Twist::FLIPPED };
    // pub const DB: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::DB), twist: Twist::SOLVED };
    // pub const BD: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::DB), twist: Twist::FLIPPED };
    // pub const DR: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::DR), twist: Twist::SOLVED };
    // pub const RD: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::DR), twist: Twist::FLIPPED };
    // pub const FR: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::FR), twist: Twist::SOLVED };
    // pub const RF: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::FR), twist: Twist::FLIPPED };
    // pub const FL: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::FL), twist: Twist::SOLVED };
    // pub const LF: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::FL), twist: Twist::FLIPPED };
    // pub const BR: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::BR), twist: Twist::SOLVED };
    // pub const RB: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::BR), twist: Twist::FLIPPED };
    // pub const BL: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::BL), twist: Twist::SOLVED };
    // pub const LB: CubeStickerLocation = CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::BL), twist: Twist::FLIPPED };

    pub fn to_sticker_name(&self) -> String {
        let number_of_twists = self.twist.number_of_twists();
        let mut faces = self.piece_location.mask().faces();
        faces.rotate_right(number_of_twists as usize);

        faces.iter().map(|f| f.to_string()).collect()
    }
}

impl fmt::Debug for CubeStickerLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_sticker_name())
    }
}