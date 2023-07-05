use crate::profession;

//Hazaqi
profession!(
    Hazaqi,
    "Hazaqi",
    280,
    vec![
        VorteilZauberer(25),
        SonderfertigkeitTradition(Zaubertaenzer)(75),
        KulturZahori
    ],
    vec![
        SprachenSchriften6,
        BindungderZauberkleidung,
        GewandderBeeinflussung
    ],
    vec![Dolche10, Raufen10, Wurfwaffen10],
    vec![
        KoerperGaukeleien2,
        Koerperbeherrschung6,
        Selbstbeherrschung3,
        Singen2,
        Sinnesschaerfe3,
        Tanzen7,
        Taschendiebstahl2,
        Zechen3GesellschaftBetoeren6,
        Gassenwissen2,
        Menschenkenntnis4,
        Überreden4,
        Verkleiden4,
        Willenskraft3NaturWildnisleben3WissenBrett & Gluecksspiel2,
        Rechtskunde3,
        Sagen & Legenden4HandwerkMusizieren4,
        Stoffbearbeitung4
    ],
    vec![],
    vec![],
    vec![Begabung(Tanzen), Gutaussehend, VerhuellteAura],
    vec![
        Persoenlichkeitsschwaeche(Arroganz, Eitelkeit),
        SchlechteEigenschaft(Jaehzorn, Rachsucht),
        VerpflichtungenI(Lehrmeisterin, Sippe)
    ],
    vec![Adel],
    vec![Behaebig, Haesslich, Taub, Verstuemmelt(Einbeinig)],
    vec![]
);

//Majuna
profession!(
    Majuna,
    "Majuna",
    291,
    vec![
        KulturAranien,
        VorteilZauberer(25),
        SonderfertigkeitTradition(Zaubertaenzer)(75),
        GeschlechtMaennlich
    ],
    vec![
        SprachenSchriften8,
        BindungderZauberkleidung,
        GewandderHeilung
    ],
    vec![Dolche10, Raufen10, Stangenwaffen10],
    vec![
        KoerperKoerperbeherrschung7,
        Kraftakt3,
        Reiten2,
        Selbstbeherrschung3,
        Singen2,
        Sinnesschaerfe3,
        Tanzen7GesellschaftBetoeren4,
        Etikette4,
        Gassenwissen4,
        Menschenkenntnis4,
        Überreden2,
        Verkleiden4,
        Willenskraft3NaturWissenGeschichtswissen1,
        Goetter & Kulte4,
        Rechnen3,
        Rechtskunde2,
        Sagen & Legenden4HandwerkMusizieren4,
        Stoffbearbeitung4
    ],
    vec![],
    vec![],
    vec![Begabung(Tanzen), Flink, Gutaussehend],
    vec![
        Persoenlichkeitsschwaeche(Arroganz, Eitelkeit),
        SchlechteEigenschaft(Naiv),
        VerpflichtungenI(Lehrmeister, Tanzgruppe)
    ],
    vec![],
    vec![Behaebig, Haesslich, Taub, Verstuemmelt(Einbeinig)],
    vec![]
);

//Rahkisa
profession!(
    Rahkisa,
    "Rahkisa",
    275,
    vec![
        KulturNovadi,
        VorteilZauberer(25),
        SonderfertigkeitTradition(Zaubertaenzer)(75),
        GeschlechtWeiblich
    ],
    vec![SprachenSchriften6, BindungderZauberkleidung, LangerTanz],
    vec![Dolche10, Raufen8, Schwerter10],
    vec![
        KoerperKoerperbeherrschung6,
        Reiten3,
        Selbstbeherrschung5,
        Singen2,
        Sinnesschaerfe3,
        Tanzen7GesellschaftBetoeren5,
        Etikette4,
        Menschenkenntnis4,
        Überreden2,
        Verkleiden4,
        Willenskraft3NaturWissenBrett & Gluecksspiel1,
        Geschichtswissen1,
        Goetter & Kulte3,
        Rechnen3,
        Rechtskunde2,
        Sagen & Legenden4HandwerkMusizieren4,
        Stoffbearbeitung4
    ],
    vec![],
    vec![],
    vec![Begabung(Tanzen), Gutaussehend],
    vec![
        Persoenlichkeitsschwaeche(Arroganz, Eitelkeit),
        Prinzipientreue(99Gesetze),
        SchlechteEigenschaft(Neugier, Vorurteile),
        VerpflichtungenII(Lehrmeisterin, Sippe)
    ],
    vec![],
    vec![Behaebig, Haesslich, Taub, Verstuemmelt(Einbeinig)],
    vec![]
);

//Sharisad
profession!(
    Sharisad,
    "Sharisad",
    268,
    vec![
        KulturTulamidenlandeoderMhanadistan,
        VorteilZauberer(25),
        SonderfertigkeitTradition(Zaubertaenzer)(75)
    ],
    vec![SprachenSchriften6, BindungderZauberkleidung, Besitzanspruch],
    vec![Dolche10, Raufen10, Schwerter10],
    vec![],
    vec![],
    vec![],
    vec![Begabung(Tanzen), Gutaussehend],
    vec![
        Persoenlichkeitsschwaeche(Arroganz, Eitelkeit),
        SchlechteEigenschaft(Neugier),
        Verpflichtungen(Lehrmeisterin, Geldgeber)
    ],
    vec![Keine],
    vec![Behaebig, Haesslich, Taub, Verstuemmelt(Einbeinig)],
    vec![]
);
