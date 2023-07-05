use crate::profession;

//Intuitiver Zauberer (Magischer Taugenichts)
profession!(
    IntuitiverZauberer(MagischerTaugenichts),
    "Intuitiver Zauberer (Magischer Taugenichts)",
    197,
    vec![
        VorteilZauberer(25),
        SonderfertigkeitTradition(IntuitiveZauberer)(50)
    ],
    vec![SprachenSchriften6],
    vec![Dolche8, Hiebwaffen10, Raufen10],
    vec![
        KoerperKlettern6,
        Koerperbeherrschung4,
        Kraftakt2,
        Sinnesschaerfe4,
        Taschendiebstahl4,
        Verbergen4GesellschaftEinschuechtern4,
        Gassenwissen4,
        Ueberreden3,
        Verkleiden2,
        Willenskraft2NaturFesseln2WissenMagiekunde1,
        Mechanik3,
        Rechnen2,
        Rechtskunde2HandwerkHandel2,
        Schloesserknacken4
    ],
    vec![
        EinZaubertrickausfolgenderListeBauchreden,
        Gluecksgriff,
        Lockruf,
        Schlangenhaende,
        Schnipsen,
        Axxeleratus4,
        Ignorantia4,
        Spinnenlauf5
    ],
    vec![],
    vec![
        DunkelsichtI,
        Fuchssinn,
        Schlangenmensch,
        VerhuellteAura,
        ZwergennaseEmpfohleneNachteileKoerperlicheAuffaelligkeit,
        MagischeEinschraenkung,
        SchwacherAstralkoerper,
        Stigma,
        WildeMagie
    ],
    vec![],
    vec![],
    vec![Artefaktgebunden],
    vec![]
);
