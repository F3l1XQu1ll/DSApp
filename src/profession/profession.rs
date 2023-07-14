use crate::{
    data::{CharakterTalentBase, Kampftechnik, Liturgy, Talent, Zauber},
    spezies::Spezies,
    vorteile::Vorteil,
};

//Voraussetzungen allg implentiern
pub enum Voraussetzung {
    Spezies(Spezies),
    Kultur(),
    Vorteil(dyn Vorteil),
    Nachteil(Vorteil),
    Sonderfertigkeit(),
}

pub struct Sonderfertigkeit {}

// Un-/Geeignete Vor-/nachteile einfügen
pub struct Profession {
    name: &'static str,
    ap: u16,
    voraussetzung: Vec<Voraussetzung>,
    sonderfertigkeiten: Vec<Sonderfertigkeit>,
    kampftechniken: Vec<(Kampftechnik, u8)>,
    talente: Vec<(CharakterTalentBase, u8)>,
    zauber: Vec<(Zauber, u8)>,
    liturgien: Vec<(Liturgy, u8)>,
    e_vorteile: Vec<Vorteil>,
    e_nachteile: Vec<Vorteil>,
    u_vorteile: Vec<Vorteil>,
    u_nachteile: Vec<Vorteil>,
    varianten: Vec<Profession>,
}

#[macro_export]
macro_rules! profession {
    ($name_ident: ident,
    $name: expr,
    $ap: expr,
    $voraussetzung: expr,
    $sonderfertigkeiten: expr,
    $kampftechniken: expr,
    $talent: expr,
    $zauber: expr,
    $liturgien: expr,
    $e_vorteile: expr,
    $e_nachteile: expr,
    $u_vorteile: expr,
    $u_nachteile: expr,
    $varianten: expr) => {

        impl crate::profession::profession::Profession {
            pub fn $name_ident() -> crate::profession::profession::Profession {
                crate::profession::profession::Profession {
                    name: $name,
                    ap: $ap,
                    voraussetzung: $voraussetzung,
                    sonderfertigkeiten: $sonderfertigkeiten,
                    kampftechniken: $kampftechniken,
                    talente: $talent,
                    zauber: $zauber,
                    liturgien: $liturgien,
                    e_vorteile: $e_vorteile,
                    e_nachteile: $e_nachteile,
                    u_vorteile: $u_vorteile,
                    u_nachteile: $u_nachteile,
                    varianten: $varianten,
                }
            }
        }
    };
}

#[macro_export]
macro_rules! crate_prof {
    () => {
        use crate::character_talents::*;
        use crate::data::CharakterTalentBase;
        use crate::spezies::Spezies;
        use crate::vorteile::nachteile::{
            AngstvorIBisIII, ArmIBisIII, Artefaktgebunden, Behaebig, Blind, Blutrausch,
            BoeserNamensvetter, EingeschraenkterSinn, EmpfindlichkeitunedleMetalle, Farbenblind,
            Fettleibig, Friedlos, GiftanfaelligIBisII, Glaesern, HaesslichIBisII, Hitzeempfindlich,
            HitzeempfindlichII, InstabilerZauberer, Jagdwildgeruch, Kaelteempfindlich,
            Kaeltestarre, KeinVertrauter, KeineFlugsalbe, KleineZauberauswahlIBisII,
            KoerpergebundeneKraft, KoerperlicheAuffaelligkeit, KrankheitsanfaelligIBisII,
            LaecherlicherName, LaestigeBluetenfeen, LaestigeMindergeister, Lernfaul,
            Lichtempfindlich, LichtempfindlichII, MagischeEinschraenkung, MisslungeneReifepruefung,
            Nachtblind, NiedrigeAstralkraftIBisVII, NiedrigeKarmalkraftIBisVII,
            NiedrigeLebenskraftIBisVII, NiedrigeSeelenkraft, NiedrigeZaehigkeit, PechIBisIII,
            Pechmagnet, Persoenlichkeitsschwaeche, PrinzipientreueIBisIII, Raubtiergeruch,
            Schlafwandler, SchlechteAngewohnheit, SchlechteEigenschaft,
            SchlechteRegenerationAstralenergieIBisIII, SchlechteRegenerationKarmaenergieIBisIII,
            SchlechteRegenerationLebensenergieIBisIII, SchlechtesNamensgedaechtnis, Schurkenname,
            SchwacheZaubermelodien, SchwacheZaubertaenze, SchwacherAstralkoerper,
            SchwacherKarmalkoerper, SensiblerGeruchssinn, Sprachfehler, StechenderOrkgeruch,
            Stigma, Stumm, Taub, Unfaehig, Unfrei, UnpassenderName,
            UnvertraeglichkeitgegenueberAlkohol, VerminderteZaubermelodienIBisIII,
            VerminderteZaubertaenzeIBisIII, VerpflichtungenIBisIII, Verstuemmelt, Verweichlicht,
            WahrerName, WenigePredigtenIBisII, WenigeVisionenIBisII, WildeMagie, Yurach,
            ZauberanfaelligIBisII, Zerbrechlich,
        };
        use crate::vorteile::vorteile::{
            AdelIBisIII, AffinitaetzuDaemonen, AffinitaetzuElementaren, Akoluth, Allerweltsname,
            Altersresistenz, AngenehmerGeruch, AnimalischeKraft, Basiliskentoeter,
            BegabterAufreisserin, Begabung, Beidhaendig, BissIBisIII, Blumenduft, Blutzauberer,
            DickesFell, Drachentoeter, DunkelsichtIBisIII, EinblickeinsVerborgene, EinkommenIBisVI,
            EisenaffineAura, Eisern, Entfernungssinn, FeenblutBluetenfee, FeenblutDryade,
            FeenblutGrosserEberbiestinger, FeenblutNixer, Feenfreund, Flink, Fuchssinn,
            GeborenerRedner, Geweihter, GiftresistenzIBisII, GlatteHaut, GlueckIBisIII, Gon,
            Greifschwanz, GrosseZauberauswahlIBisII, GutaussehendIBisII, GutesNamensgedaechtnis,
            HarmoniedesWaldes, HartimNehmen, Hass, Hauttasche, HeldenhafterName,
            HerausragendeFertigkeit, HerausragendeKampftechnik, HerausragenderSinn, Hitzeresistenz,
            HoheAstralkraftIBisVII, HoheKarmalkraftIBisVII, HoheLebenskraftIBisVII,
            HoheSeelenkraft, HoheZaehigkeit, ImmunitaetgegenGift, ImmunitaetgegenKrankheit,
            Kaelteresistenz, KeinBanndesEisens, Koboldfreund, KontaktBettler, KontaktBotenreiter,
            KontaktFreundausKindheitstagen, KontaktGeweihte, KontaktHandwerksmeister,
            KontaktHehler, KontaktJunkerin, KontaktMagier, KontaktSchmuggler, KontaktSklavin,
            KontaktSoeldnerhauptmann, KontaktSoldat, KontaktStadtmagistrat, KontaktStadtwache,
            KontaktTempelvorsteherin, KontaktUnterweltkoenig, KontaktWirt, KraftderBaeume,
            KrankheitsresistenzIBisII, LeichterGang, MachtvolleZaubermelodien,
            MachtvolleZaubertaenze, MagischeEinstimmung, Mystiker, NatuerlicheWaffeBiss,
            NatuerlicheWaffeSchwanz, NatuerlicherRuestungsschutzIBisII, Nichtschlaefer,
            Pragmatiker, Prediger, ReichIBisX, Richtungssinn, Sagaheld, SaumagenIBisII,
            Schlangenmensch, SchnellwiederaufdenBeinen, Schutzgeist, Schwerzuverzaubern,
            Seesprache, SozialeAnpassungsfaehigkeit, Tierfreund, TiergeistauraPatron,
            TierischeKampfgefaehrten, UnbeschwertesZaubern, Unscheinbar,
            VerbesserteRegenerationAstralenergieIBisIII,
            VerbesserteRegenerationKarmaenergieIBisIII,
            VerbesserteRegenerationLebensenergieIBisIII, VerfuehrerischeGestalt, VerhuellteAura,
            Vertrauenerweckend, Visionaer, Waffenbegabung, WalwutSwafskari, WasserdesLebens,
            Wasserlebewesen, WeitreichendeZaubermelodienIBisIII, WeitreichendeZaubertaenzeIBisIII,
            Wohlklang, Wolfsblut, Wolfsgebiss, Wolfskind, ZaeherHund, ZahlreichePredigtenIBisII,
            ZahlreicheVisionenIBisII, Zauberer, Zeitgefuehl, ZusaetzlicheGliedmassenSchwanz,
            ZweistimmigerGesang, Zwergennase,
        };
    };
}
