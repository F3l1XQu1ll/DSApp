use crate::crate_prof;
use crate::profession;

crate_prof!();

//Scharlatan
profession!(
    Scharlatan,
    "Scharlatan",
    269,
    vec![
        VorteilZauberer(25),
        SonderfertigkeitTradition(Scharlatane)(125)
    ],
    vec![SprachenSchriften10, BindungderZauberkugel],
    vec![Raufen8, Stangenwaffen8, Wurfwaffen8],
    vec![
        KoerperGaukeleien5,
        Koerperbeherrschung2,
        Selbstbeherrschung4,
        Singen2,
        Sinnesschaerfe4,
        Tanzen2,
        Zechen2GesellschaftBekehren & Überzeugen1,
        Betoeren1,
        Gassenwissen4,
        Menschenkenntnis4,
        Überreden4,
        Verkleiden4NaturFesseln4,
        Wildnisleben2WissenGeographie2,
        Goetter & Kulte3,
        Magiekunde1,
        Rechnen2,
        Rechtskunde1,
        Sagen & Legenden4HandwerkAlchimie2,
        Fahrzeuge4,
        Holzbearbeitung2,
        Malen & Zeichen4,
        Stoffbearbeitung4
    ],
    vec![EinZaubertrickausfolgenderListeBauchreden,
		Duft,
		Feenfuesse,
		Regenbogenaugen,
		Schminken,
		Attributo(Charisma)6,
		AurisIllusionis6,
		Duplicatus3,
		ManusIllusionis4,
		ManusMiracula7,
		Menetekel5,
		OculusIllusionis7],
    vec![],
    vec![
        GeborenerRedner,
        HerausragenderSinn,
        HoheAstralkraft,
        SozialeAnpassungsfaehigkeit
    ],
    vec![
        MagischeEinschraenkung(WesenderBuehne),
        Persoenlichkeitsschwaeche(Eitelkeit),
        SchlechteEigenschaft(Goldgier, Neugier)
    ],
    vec![Unscheinbar],
    vec![Blind, Farbenblind, Sprachfehler, Stumm, Taub],
    vec![]
);
