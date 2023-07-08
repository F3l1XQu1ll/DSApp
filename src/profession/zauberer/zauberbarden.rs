use crate::crate_prof;
use crate::profession;

crate_prof!();

//Ceoladir
profession!(
    Ceoladir,
    "Ceoladir",
    261,
    vec![
        KulturMittelreich(ausAlbernia),
        VorteilZauberer(25),
        SonderfertigkeitTradition(Zauberbarden)(80)
    ],
    vec![
        SprachenSchriften16,
        BindungdesZauberinstruments,
        freiwaehlbareFertigkeitsspezialisierungMusizieren,
        Zugabe
    ],
    vec![Dolche8, Raufen8],
    vec![],
    vec![],
    vec![],
    vec![Gutaussehend, SozialeAnpassungsfaehigkeit, Wohlklang],
    vec![
        Persoenlichkeitsschwaeche(Arroganz, Eitelkeit),
        SchlechteEigenschaft(Neugier),
        VerpflichtungenI(Lehrmeister, Goenner)
    ],
    vec![],
    vec![Sprachfehler, Stumm, Taub],
    vec![]
);

//Derwisch
profession!(
    Derwisch,
    "Derwisch",
    316,
    vec![
        KulturNovadi,
        VorteilZauberer(25),
        SonderfertigkeitTradition(Zauberbarden)(80),
        NachteilPrinzipientreueI(99Gesetze)(10),
        VerpflichtungenII(OrdenderDerwische)(20),
        Geschlechtmaennlich
    ],
    vec![
        SprachenSchriften8,
        BindungdesZauberinstruments,
        FertigkeitsspezialisierungMusizieren(Trommeln),
        WeiterKlang
    ],
    vec![Dolche8, Raufen10, Schwerter8, Wurfwaffen8],
    vec![
        KoerperKoerperbeherrschung4,
        Reiten2,
        Selbstbeherrschung4,
        Sinnesschaerfe2,
        Tanzen7GesellschaftBekehren & Überzeugen2,
        Einschuechtern4,
        Menschenkenntnis5,
        Überreden5NaturOrientierung4,
        Pflanzenkunde1,
        Tierkunde1,
        Wildnisleben5WissenBrett & Gluecksspiel2,
        Geschichtswissen2,
        Goetter & Kulte5,
        Kriegskunst2,
        Magiekunde2,
        Rechnen3,
        Rechtskunde4,
        Sagen & Legenden5,
        Sternkunde4HandwerkHeilkundeGift3,
        HeilkundeWunden3,
        Holzbearbeitung1,
        Lederbearbeitung1,
        Musizieren7,
        Stoffbearbeitung1
    ],
    vec![],
    vec![],
    vec![Hitzeresistenz, HoheSeelenkraft, HoheZaehigkeit, ZaeherHund],
    vec![
        KoerpergebundeneKraft,
        Persoenlichkeitsschwaeche(Arroganz, VorurteilegegenEchsenoderUnglaeubige)
    ],
    vec![MagischeEinstimmung(WesenderNacht), VerhuellteAura],
    vec![
        Behaebig,
        Fettleibig,
        NiedrigeSeelenkraft,
        Taub,
        Zerbrechlich
    ],
    vec![]
);

//Sangara
profession!(
    Sangara,
    "Sangara",
    317,
    vec![
        KulturThorwal,
        VorteilZauberer(25),
        SonderfertigkeitTradition(Zauberbarden)(80)
    ],
    vec![SprachenSchriften16, FertigkeitsspezialisierungSingen],
    vec![Dolche8, Hiebwaffen10, Raufen8],
    vec![],
    vec![],
    vec![],
    vec![
        Glueck,
        MagischeEinstimmung(WesendesWaldes),
        SozialeAnpassungsfaehigkeit,
        Wohlklang
    ],
    vec![
        KoerperlicheAuffaelligkeit,
        Persoenlichkeitsschwaeche(Arroganz, Unheimlich, Weltfremd),
        Prinzipientreue(ÖgnirGlaube),
        SchlechteEigenschaft(Neugier, Rachsucht),
        Stigma
    ],
    vec![],
    vec![Kaelteempfindlich, Sprachfehler, Stumm, Taub],
    vec![]
);

//Satuduraki
profession!(
    Satuduraki,
    "Satuduraki",
    249,
    vec![
        KulturZyklopeninseln,
        VorteilZauberer(25),
        SonderfertigkeitTradition(Zauberbarden)(80)
    ],
    vec![
        SprachenSchriften8,
        FertigkeitsspezialisierungMusizieren,
        BindungdesZauberinstruments,
        Hilferuf
    ],
    vec![Dolche8, Raufen8],
    vec![],
    vec![],
    vec![],
    vec![Gutaussehend, SozialeAnpassungsfaehigkeit, Wohlklang],
    vec![
        Persoenlichkeitsschwaeche(Arroganz, Eitelkeit),
        SchlechteEigenschaft(Neugier),
        VerpflichtungenI(Lehrmeister, Goenner)
    ],
    vec![],
    vec![Sprachfehler, Stumm, Taub],
    vec![]
);
