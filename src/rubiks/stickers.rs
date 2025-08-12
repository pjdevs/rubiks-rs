// use enum_iterator::Sequence;


// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub enum CubeSticker {
//     CornerSticker(CornerSticker),
//     EdgeSticker(EdgeSticker),
// }

// impl CubeElement for CubeSticker {
//     fn get_face_mask(&self) -> Faces {
//         match self {
//             CubeSticker::CornerSticker(corner_sticker) => corner_sticker.get_face_mask(),
//             CubeSticker::EdgeSticker(edge_sticker) => edge_sticker.get_face_mask(),
//         }
//     }
// }

// impl CubeSticker {
//     pub fn is_same_cubie(&self, other: &CubeSticker) -> bool {
//         self.get_face_mask() == other.get_face_mask()
//     }
// }

// impl TryInto<CornerSticker> for CubeSticker {
//     type Error = ();

//     fn try_into(self) -> Result<CornerSticker, Self::Error> {
//         match self {
//             CubeSticker::CornerSticker(corner_sticker) => Result::Ok(corner_sticker),
//             CubeSticker::EdgeSticker(_) => Result::Err(()),
//         }
//     }
// }

// impl TryInto<EdgeSticker> for CubeSticker {
//     type Error = ();

//     fn try_into(self) -> Result<EdgeSticker, Self::Error> {
//         match self {
//             CubeSticker::CornerSticker(_) => Result::Err(()),
//             CubeSticker::EdgeSticker(edge_sticker) => Result::Ok(edge_sticker),
//         }
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Sequence)]
// pub enum CornerSticker {
//     UFR, FRU, RFU,
//     UFL, FLU, LUF,
//     UBR, BRU, RUB,
//     UBL, BLU, LUB,
//     DFR, FRD, RDF,
//     DFL, FLD, LDF,
//     DBR, BRD, RDB,
//     DBL, BLD, LDB,
// }

// impl CubeElement for CornerSticker {
//     fn get_face_mask(&self) -> Faces {
//         use CornerSticker::*;
//         match self {
//             UFR => Faces::UFR,
//             FRU => Faces::UFR,
//             RFU => Faces::UFR,
//             UFL => Faces::UFL,
//             FLU => Faces::UFL,
//             LUF => Faces::UFL,
//             UBR => Faces::UBR,
//             BRU => Faces::UBR,
//             RUB => Faces::UBR,
//             UBL => Faces::UBL,
//             BLU => Faces::UBL,
//             LUB => Faces::UBL,
//             DFR => Faces::DFR,
//             FRD => Faces::DFR,
//             RDF => Faces::DFR,
//             DFL => Faces::DFL,
//             FLD => Faces::DFL,
//             LDF => Faces::DFL,
//             DBR => Faces::DBR,
//             BRD => Faces::DBR,
//             RDB => Faces::DBR,
//             DBL => Faces::DBL,
//             BLD => Faces::DBL,
//             LDB => Faces::DBL,
//         }
//     }
// }

// impl CornerSticker {
//     pub fn is_same_cubie(&self, other: &CornerSticker) -> bool {
//         self.get_face_mask() == other.get_face_mask()
//     }

// pub fn twisted(&self, twist: CornerOrientation) -> Self {
//     use CornerSticker::*;
//     use CornerOrientation::*;
//     match self {
//         UFR => match twist {
//             Solved => UFR,
//             Twisted120 => FRU,
//             Twisted240 => RFU,
//         },
//         FRU => match twist {
//             CornerOrientation::Solved => FRU,
//             Twisted120 => RFU,
//             Twisted240 => UFR,
//         },
//         RFU => match twist {
//             Solved => RFU,
//             Twisted120 => UFR,
//             Twisted240 => FRU,
//         },
//         UFL => match twist {
//             Solved => UFL,
//             Twisted120 => FLU,
//             Twisted240 => LUF,
//         },
//         FLU => match twist {
//             Solved => FLU,
//             Twisted120 => LUF,
//             Twisted240 => UFL,
//         },
//         LUF => match twist {
//             Solved => LUF,
//             Twisted120 => UFL,
//             Twisted240 => FLU,
//         },
//         UBR => match twist {
//             Solved => UBR,
//             Twisted120 => BRU,
//             Twisted240 => RUB,
//         },
//         BRU => match twist {
//             Solved => BRU,
//             Twisted120 => RUB,
//             Twisted240 => UBR,
//         },
//         RUB => match twist {
//             Solved => RUB,
//             Twisted120 => UBR,
//             Twisted240 => BRU,
//         },
//         UBL => match twist {
//             Solved => UBL,
//             Twisted120 => BLU,
//             Twisted240 => LUB,
//         },
//         BLU => match twist {
//             Solved => BLU,
//             Twisted120 => LUB,
//             Twisted240 => UBL,
//         },
//         LUB => match twist {
//             Solved => LUB,
//             Twisted120 => UBL,
//             Twisted240 => BLU,
//         },
//         DFR => match twist {
//             Solved => DFR,
//             Twisted120 => FRD,
//             Twisted240 => RDF,
//         },
//         FRD => match twist {
//             Solved => FRD,
//             Twisted120 => RDF,
//             Twisted240 => DFR,
//         },
//         RDF => match twist {
//             Solved => RDF,
//             Twisted120 => DFR,
//             Twisted240 => FRD,
//         },
//         DFL => match twist {
//             Solved => DFL,
//             Twisted120 => FLD,
//             Twisted240 => LDF,
//         },
//         FLD => match twist {
//             Solved => FLD,
//             Twisted120 => LDF,
//             Twisted240 => DFL,
//         },
//         LDF => match twist {
//             Solved => LDF,
//             Twisted120 => DFL,
//             Twisted240 => FLD,
//         },
//         DBR => match twist {
//             Solved => DBR,
//             Twisted120 => BRD,
//             Twisted240 => RDB,
//         },
//         BRD => match twist {
//             Solved => BRD,
//             Twisted120 => RDB,
//             Twisted240 => DBR,
//         },
//         RDB => match twist {
//             Solved => RDB,
//             Twisted120 => DBR,
//             Twisted240 => BRD,
//         },
//         DBL => match twist {
//             Solved => DBL,
//             Twisted120 => BLD,
//             Twisted240 => LDB,
//         },
//         BLD => match twist {
//             Solved => BLD,
//             Twisted120 => LDB,
//             Twisted240 => DBL,
//         },
//         LDB => match twist {
//             Solved => LDB,
//             Twisted120 => DBL,
//             Twisted240 => BLD,
//         },
//     }
// }
// }

// impl TryFrom<Faces> for CornerSticker {
//     type Error = ();

//     fn try_from(value: Faces) -> Result<Self, Self::Error> {
//         use CornerSticker::*;
//         match value {
//             Faces::UFR => Ok(UFR),
//             Faces::UFL => Ok(UFL),
//             Faces::UBR => Ok(UBR),
//             Faces::UBL => Ok(UBL),
//             Faces::DFR => Ok(DFR),
//             Faces::DFL => Ok(DFL),
//             Faces::DBR => Ok(DBR),
//             Faces::DBL => Ok(DBL),
//             _ => Err(())
//         }
//     }
// }

// impl Into<CubeSticker> for EdgeSticker {
//     fn into(self) -> CubeSticker {
//         CubeSticker::EdgeSticker(self)
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Sequence)]
// pub enum EdgeSticker {
//     UF, FU,
//     UL, LU,
//     UB, BU,
//     UR, RU,
//     DF, FD,
//     DL, LD,
//     DB, BD,
//     DR, RD,
//     FR, RF,
//     FL, LF,
//     BR, RB,
//     BL, LB,
// }

// impl TryFrom<Faces> for EdgeSticker {
//     type Error = ();

//     fn try_from(value: Faces) -> Result<Self, Self::Error> {
//         use EdgeSticker::*;
//         match value {
//             Faces::UF => Ok(UF),
//             Faces::UL => Ok(UL),
//             Faces::UB => Ok(UB),
//             Faces::UR => Ok(UR),
//             Faces::DF => Ok(DF),
//             Faces::DL => Ok(DL),
//             Faces::DB => Ok(DB),
//             Faces::DR => Ok(DR),
//             Faces::FR => Ok(FR),
//             Faces::FL => Ok(FL),
//             Faces::BR => Ok(BR),
//             Faces::BL => Ok(BL),
//             _ => Err(())
//         }
//     }
// }

// impl CubeElement for EdgeSticker {
//     fn get_face_mask(&self) -> Faces {
//         use EdgeSticker::*;
//         match self {
//             UF => Faces::UF,
//             FU => Faces::UF,
//             UL => Faces::UL,
//             LU => Faces::UL,
//             UB => Faces::UB,
//             BU => Faces::UB,
//             UR => Faces::UR,
//             RU => Faces::UR,
//             DF => Faces::DF,
//             FD => Faces::DF,
//             DL => Faces::DL,
//             LD => Faces::DL,
//             DB => Faces::DB,
//             BD => Faces::DB,
//             DR => Faces::DR,
//             RD => Faces::DR,
//             FR => Faces::FR,
//             RF => Faces::FR,
//             FL => Faces::FL,
//             LF => Faces::FL,
//             BR => Faces::BR,
//             RB => Faces::BR,
//             BL => Faces::BL,
//             LB => Faces::BL,
//         }
//     }
// }

// impl EdgeSticker {
//     pub fn is_same_cubie(&self, other: &EdgeSticker) -> bool {
//         self.get_face_mask() == other.get_face_mask()
//     }

//     pub fn inverted(&self) -> Self {
//         use EdgeSticker::*;
//         match self {
//             UF => FU,
//             FU => UF,
//             UL => LU,
//             LU => UL,
//             UB => BU,
//             BU => UB,
//             UR => RU,
//             RU => UR,
//             DF => FD,
//             FD => DF,
//             DL => LD,
//             LD => DL,
//             DB => BD,
//             BD => DB,
//             DR => RD,
//             RD => DR,
//             FR => RF,
//             RF => FR,
//             FL => LF,
//             LF => FL,
//             BR => RB,
//             RB => BR,
//             BL => LB,
//             LB => BL,
//         }
//     }
// }

// impl Into<CubeSticker> for CornerSticker {
//     fn into(self) -> CubeSticker {
//         CubeSticker::CornerSticker(self)
//     }
// }