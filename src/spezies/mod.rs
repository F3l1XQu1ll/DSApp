pub mod achaz;
pub mod elfen;
pub mod halbelfen;
pub mod halborks;
pub mod holberker;
pub mod menschen;
pub mod orks;
pub mod zwerge;

pub mod prelude {
    pub use super::achaz;
    pub use super::elfen;
    pub use super::halbelfen;
    pub use super::halborks;
    pub use super::holberker;
    pub use super::menschen;
    pub use super::orks;
    pub use super::zwerge;
    pub use super::Spezies;
    pub use super::SpeziesBase;
}

pub trait SpeziesBase<'de>: Default + Clone + serde::Deserialize<'de> + serde::Serialize {
    fn name() -> &'static str;
    fn le() -> u8;
    fn sk() -> i8;
    fn zk() -> i8;
    fn gs() -> u8 {
        8
    }
    fn eigenschaften_mod(&self, eigenschaften: &mut crate::data::Attributes);
    fn vorteile_mod(&self, charakter: &mut crate::data::Character);
    fn cost() -> u8;
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub enum Spezies {
    Achaz(achaz::Achaz),
    Elfen(elfen::Elfen),
    Halbelfen(halbelfen::Halbelfen),
    Halborks(halborks::Halborks),
    Holberker(holberker::Holberker),
    Menschen(menschen::Menschen),
    Orks(orks::Orks),
    Zwerge(zwerge::Zwerge),
}

impl PartialEq for Spezies {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Spezies::Achaz(_), &Spezies::Achaz(_))
            | (&Spezies::Elfen(_), &Spezies::Elfen(_))
            | (&Spezies::Halbelfen(_), &Spezies::Halbelfen(_))
            | (&Spezies::Halborks(_), &Spezies::Halborks(_))
            | (&Spezies::Holberker(_), &Spezies::Holberker(_))
            | (&Spezies::Menschen(_), &Spezies::Menschen(_))
            | (&Spezies::Orks(_), &Spezies::Orks(_))
            | (&Spezies::Zwerge(_), &Spezies::Zwerge(_)) => true,
            (_, _) => false,
        }
    }
}

impl Default for Spezies {
    fn default() -> Self {
        Self::Menschen(menschen::Menschen::default())
    }
}

macro_rules! impl_Spezies_fn {
    ($self:ident, $fn:ident()) => {
        match $self {
            Spezies::Achaz(_) => achaz::Achaz::$fn(),
            Spezies::Elfen(_) => elfen::Elfen::$fn(),
            Spezies::Halbelfen(_) => halbelfen::Halbelfen::$fn(),
            Spezies::Halborks(_) => halborks::Halborks::$fn(),
            Spezies::Holberker(_) => holberker::Holberker::$fn(),
            Spezies::Menschen(_) => menschen::Menschen::$fn(),
            Spezies::Orks(_) => orks::Orks::$fn(),
            Spezies::Zwerge(_) => zwerge::Zwerge::$fn(),
        }
    };

    ($self:ident, @member $fn:ident()) => {
        match $self {
            Spezies::Achaz(t) => t.$fn(),
            Spezies::Elfen(t) => t.$fn(),
            Spezies::Halbelfen(t) => t.$fn(),
            Spezies::Halborks(t) => t.$fn(),
            Spezies::Holberker(t) => t.$fn(),
            Spezies::Menschen(t) => t.$fn(),
            Spezies::Orks(t) => t.$fn(),
            Spezies::Zwerge(t) => t.$fn(),
        }
    };
}

impl Spezies {
    pub fn name(&self) -> &'static str {
        match self {
            Spezies::Achaz(_) => achaz::Achaz::name(),
            Spezies::Elfen(_) => elfen::Elfen::name(),
            Spezies::Halbelfen(_) => halbelfen::Halbelfen::name(),
            Spezies::Halborks(_) => halborks::Halborks::name(),
            Spezies::Holberker(_) => holberker::Holberker::name(),
            Spezies::Menschen(_) => menschen::Menschen::name(),
            Spezies::Orks(_) => orks::Orks::name(),
            Spezies::Zwerge(_) => zwerge::Zwerge::name(),
        }
    }

    pub fn le(&self) -> u8 {
        impl_Spezies_fn!(self, le())
    }
    pub fn sk(&self) -> i8 {
        impl_Spezies_fn!(self, sk())
    }
    pub fn zk(&self) -> i8 {
        impl_Spezies_fn!(self, zk())
    }
    pub fn cost(&self) -> u8 {
        impl_Spezies_fn!(self, cost())
    }
    pub fn gs(&self) -> u8 {
        impl_Spezies_fn!(self, gs())
    }
}

// macro_rules! impl_Spezies {
//     ($visibility:vis $enum_type:ident $name:ident {$( $s:ident($p:path) ),*,}) => {
//         #[derive(Clone, serde::Deserialize, serde::Serialize)]
//         $visibility $enum_type $name{
//             $(
//                 $s($p),
//             )*
//         }

//         impl<'de> SpeziesBase<'de> for $name {
//             fn name(&self) -> &'static str {
//                 match self {
//                     $(
//                         $name::$s(t) => t::name(),
//                     )*
//                 }
//             }

//             fn le(&self) -> u8 {
//                 match self {
//                     $(
//                         $name::$s(t) => t.le(),
//                     )*
//                 }
//             }

//             fn sk(&self) -> i8 {
//                 match self {
//                     $(
//                         $name::$s(t) => t.sk(),
//                     )*
//                 }
//             }

//             fn zk(&self) -> i8 {
//                 match self {
//                     $(
//                         $name::$s(t) => t.zk(),
//                     )*
//                 }
//             }

//             fn eigenschaften_mod(&self, eigenschaften: &mut crate::data::Attributes) {
//                 match self {
//                     $(
//                         $name::$s(t) => t.eigenschaften_mod(eigenschaften),
//                     )*
//                 }
//             }

//             fn vorteile_mod(&self, charakter: &mut crate::data::Character) {
//                 match self {
//                     $(
//                         $name::$s(t) => t.vorteile_mod(charakter),
//                     )*
//                 }
//             }

//             fn cost(&self) -> u8 {
//                 match self {
//                     $(
//                         $name::$s(t) => t.cost(),
//                     )*
//                 }
//             }
//         }
//     };
// }

// impl_Spezies! {
//     pub enum Spezies {
//         Achaz(achaz::Achaz),
//         Elfen(elfen::Elfen),
//         Halbelfen(halbelfen::Halbelfen),
//         Halborks(halborks::Halborks),
//         Holberker(holberker::Holberker),
//         Menschen(menschen::Menschen),
//         Orks(orks::Orks),
//         Zwerge(zwerge::Zwerge),
//     }
// }
