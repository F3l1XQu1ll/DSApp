mod nachteile;
mod vorteile;

pub trait Vorteil {
    fn name() -> &'static str;
    fn regel_text() -> &'static str;
    fn voraussetzungen_text() -> &'static str;
    fn ap_wert(&self) -> u16;
    fn nachteil() -> bool;
}

pub trait VorteilGroup {
    fn name() -> &'static str;
    fn ap_text() -> &'static str;
    fn members() -> &'static [&'static Self];
}

#[macro_export]
macro_rules! vorteil {
    ($name: ident, $name_text: expr, $voraussetzungen_text: expr, $regel_text: expr, $ap: expr, $nachteil: expr) => {
        pub struct $name;

        vorteil!(@noauto $name, $name_text, $voraussetzungen_text, $regel_text, $ap, $nachteil);
    };
    (@noauto $name: ident, $name_text: expr, $voraussetzungen_text: expr, $regel_text: expr, $ap: expr, $nachteil: expr) => {
        impl Vorteil for $name {
            fn name() -> &'static str {
                $name_text
            }

            fn regel_text() -> &'static str {
                $regel_text
            }

            fn voraussetzungen_text() -> &'static str {
                $voraussetzungen_text
            }

            fn ap_wert(&self) -> u16 {
                $ap
            }

            fn nachteil() -> bool {
                $nachteil
            }
        }
    };

    (@noauto $name: ident, $name_text: expr, $voraussetzungen_text: expr, $regel_text: expr, @call $ap: expr, $nachteil: expr) => {
        impl Vorteil for $name {
            fn name() -> &'static str {
                $name_text
            }

            fn regel_text() -> &'static str {
                $regel_text
            }

            fn voraussetzungen_text() -> &'static str {
                $voraussetzungen_text
            }

            fn ap_wert(&self) -> u16 {
                $ap(self)
            }

            fn nachteil() -> bool {
                $nachteil
            }
        }
    };
}
