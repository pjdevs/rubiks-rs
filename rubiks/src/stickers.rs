use core::fmt;

use crate::faces::FaceMask;
use crate::location::CubePieceLocation;
use crate::twist::Twist;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct CubeStickerLocation {
    pub piece_location: CubePieceLocation,
    pub twist: Twist,
}

impl CubeStickerLocation {
    pub const URF: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::URF,
        twist: Twist::SOLVED,
    };
    pub const RFU: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::URF,
        twist: Twist::CW_120,
    };
    pub const FUR: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::URF,
        twist: Twist::CW_240,
    };
    pub const UFL: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::UFL,
        twist: Twist::SOLVED,
    };
    pub const FLU: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::UFL,
        twist: Twist::CW_120,
    };
    pub const LUF: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::UFL,
        twist: Twist::CW_240,
    };
    pub const ULB: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::ULB,
        twist: Twist::SOLVED,
    };
    pub const LBU: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::ULB,
        twist: Twist::CW_120,
    };
    pub const BUL: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::ULB,
        twist: Twist::CW_240,
    };
    pub const UBR: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::UBR,
        twist: Twist::SOLVED,
    };
    pub const BRU: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::UBR,
        twist: Twist::CW_120,
    };
    pub const RUB: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::UBR,
        twist: Twist::CW_240,
    };
    pub const DFR: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::DFR,
        twist: Twist::SOLVED,
    };
    pub const FRD: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::DFR,
        twist: Twist::CW_120,
    };
    pub const RDF: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::DFR,
        twist: Twist::CW_240,
    };
    pub const DLF: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::DLF,
        twist: Twist::SOLVED,
    };
    pub const LFD: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::DLF,
        twist: Twist::CW_120,
    };
    pub const FDL: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::DLF,
        twist: Twist::CW_240,
    };
    pub const DBL: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::DBL,
        twist: Twist::SOLVED,
    };
    pub const BLD: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::DBL,
        twist: Twist::CW_120,
    };
    pub const LDB: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::DBL,
        twist: Twist::CW_240,
    };
    pub const DRB: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::DRB,
        twist: Twist::SOLVED,
    };
    pub const RBD: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::DRB,
        twist: Twist::CW_120,
    };
    pub const BDR: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::DRB,
        twist: Twist::CW_240,
    };

    pub const UF: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::UF,
        twist: Twist::SOLVED,
    };
    pub const FU: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::UF,
        twist: Twist::FLIPPED,
    };
    pub const UL: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::UL,
        twist: Twist::SOLVED,
    };
    pub const LU: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::UL,
        twist: Twist::FLIPPED,
    };
    pub const UB: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::UB,
        twist: Twist::SOLVED,
    };
    pub const BU: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::UB,
        twist: Twist::FLIPPED,
    };
    pub const UR: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::UR,
        twist: Twist::SOLVED,
    };
    pub const RU: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::UR,
        twist: Twist::FLIPPED,
    };
    pub const DF: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::DF,
        twist: Twist::SOLVED,
    };
    pub const FD: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::DF,
        twist: Twist::FLIPPED,
    };
    pub const DL: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::DL,
        twist: Twist::SOLVED,
    };
    pub const LD: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::DL,
        twist: Twist::FLIPPED,
    };
    pub const DB: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::DB,
        twist: Twist::SOLVED,
    };
    pub const BD: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::DB,
        twist: Twist::FLIPPED,
    };
    pub const DR: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::DR,
        twist: Twist::SOLVED,
    };
    pub const RD: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::DR,
        twist: Twist::FLIPPED,
    };
    pub const FR: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::FR,
        twist: Twist::SOLVED,
    };
    pub const RF: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::FR,
        twist: Twist::FLIPPED,
    };
    pub const FL: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::FL,
        twist: Twist::SOLVED,
    };
    pub const LF: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::FL,
        twist: Twist::FLIPPED,
    };
    pub const BR: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::BR,
        twist: Twist::SOLVED,
    };
    pub const RB: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::BR,
        twist: Twist::FLIPPED,
    };
    pub const BL: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::BL,
        twist: Twist::SOLVED,
    };
    pub const LB: CubeStickerLocation = CubeStickerLocation {
        piece_location: CubePieceLocation::BL,
        twist: Twist::FLIPPED,
    };

    pub fn to_sticker_name(&self) -> String {
        let number_of_twists = self.twist.number_of_twists();
        let mut faces = self.piece_location.get_faces();
        faces.rotate_left(number_of_twists as usize);

        faces.iter().map(|f| f.to_string()).collect()
    }
}

impl fmt::Debug for CubeStickerLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_sticker_name())
    }
}
