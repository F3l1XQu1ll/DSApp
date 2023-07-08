use crate::crate_prof;
use crate::profession;

crate_prof!();
//Affenhexe
profession!(
    Affenhexe,
    "Affenhexe",
    304,
    vec![VorteilZauberer(25), SonderfertigkeitTradition(Hexe)(135)],
    vec![SprachenSchriften8, Flugsalbe, Vertrautenbindung, Reisende],
    vec![Dolche10, Raufen10],
    vec![],
    vec![
        einZaubertrickausfolgenderListeGluecksgriff,
        Hexenblick,
        Wuerzen,
        Tierpflege,
        Affenarme6,
        Affenruf4,
        GroßeGier6,
        HarmloseGestalt5,
        KraftdesTieres6,
        Motoricus4
    ],
    vec![],
    vec![
        Altersresistenz(nurbeiEigeborenen),
        Flink,
        Gutaussehend,
        SozialeAnpassungsfaehigkeit,
        VerhuellteAura
    ],
    vec![
        KeineFlugsalbe,
        KeinVertrauter,
        SchlechteEigenschaften(Goldgier, Rachsucht)
    ],
    vec![ZaeherHund],
    vec![Angstvor(Hoehen), Haesslich],
    vec![]
);

//Eulenhexe
profession!(
    Eulenhexe,
    "Eulenhexe",
    430,
    vec![VorteilZauberer(25), SonderfertigkeitTradition(Hexe)(135)],
    vec![
        SprachenSchriften4,
        Fluechefuerinsgesamt12AP,
        Flugsalbe,
        Vertrautenbindung,
        Auraverbergen
    ],
    vec![Dolche11, Hiebwaffen10, Raufen11, Wurfwaffen10],
    vec![],
    vec![
        einZaubertrickausfolgenderListeBauchreden,
        Hexenblick,
        Kehrbesen,
        Schnipsen,
        Trocken,
        Wuerze,
        HarmloseGestalt6,
        Hexenholz4,
    ],
    vec![],
    vec![Altersresistenz(nurbeiEigeborenen), VerhuellteAura],
    vec![
        KeineFlugsalbe,
        KeinVertrautentier,
        SchlechteEigenschaften(Jaehzorn, Neugier, Rachsucht)
    ],
    vec![],
    vec![],
    vec![]
);

//Katzenhexe
profession!(
    Katzenhexe,
    "Katzenhexe",
    285,
    vec![VorteilZauberer(25), SonderfertigkeitTradition(Hexe)(135)],
    vec![SprachenSchriften6, Flugsalbe, Vertrautenbindung],
    vec![Dolche10, Raufen10],
    vec![],
    vec![
        einZaubertrickausfolgenderListeBauchreden,
        Duft,
        Gluecksgriff,
        Handwaermer,
        Trocken,
        GroßeGier6,
        HarmloseGestalt5,
        Hexengalle4,
        Katzenaugen6,
        Motoricus4,
        Odem4,
        SatuariasHerrlichkeit5
    ],
    vec![],
    vec![
        Altersresistenz(nurbeiEigeborenen),
        AngenehmerGeruch,
        Flink,
        Gutaussehend,
        SozialeAnpassungsfaehigkeit,
        VerhuellteAura
    ],
    vec![
        KeineFlugsalbe,
        KeinVertrauter,
        Persoenlichkeitsschwaechen(Arroganz, Eitelkeit),
        SchlechteEigenschaften(Goldgier, Rachsucht)
    ],
    vec![ZaeherHund],
    vec![Angstvor(Hoehe), Haesslich],
    vec![]
);

//Krötenhexe
profession!(
    Kroetenhexe,
    "Krötenhexe",
    296,
    vec![VorteilZauberer(25), SonderfertigkeitTradition(Hexe)(135)],
    vec![
        SprachenSchriften6,
        Fluechefuerinsgesamt12,
        Flugsalbe,
        Vertrautenbindung
    ],
    vec![Raufen8],
    vec![],
    vec![
        einZaubertrickausfolgenderListeHandwaermer,
        Regenbogenaugen,
        Schlangenhaende,
        Balsam6,
        GroßeGier5,
        HarmloseGestalt5,
        Hexengalle4,
        Kroetensprung6,
        Radau4,
        Sanftmut4
    ],
    vec![],
    vec![
        Altersresistenz(nurbeiEigeborenen),
        VerbesserteRegeneration(Astralenergie),
        VerhuellteAura
    ],
    vec![
        Angstvor(Feuer),
        Hexenstraehne,
        KeineFlugsalbe,
        KeinVertrauter,
        KoerpergebundeneKraft,
        Persoenlichkeitsschwaechen(Weltfremd)
    ],
    vec![SozialeAnpassungsfaehigkeit, ZaeherHund],
    vec![],
    vec![]
);

//Pardelhexe (Schwesternschaft: Schöne der Nacht)
profession!(
    SchwesternschaftSchoenederNacht,
    "Pardelhexe (Schwesternschaft: Schöne der Nacht)",
    317,
    vec![VorteilZauberer(25), SonderfertigkeitTradition(Hexen)(135)],
    vec![SprachenSchriften6, Flugsalbe, Vertrautenbindung],
    vec![Dolche10, Raufen10],
    vec![
        KoerperFliegen2,
        Klettern6,
        Koerperbeherrschung5,
        Sinnesschaerfe5,
        Verbergen4GesellschaftBetoeren6,
        Etikette3,
        Menschenkenntnis4,
        Überreden4,
        Verkleiden4NaturOrientierung4,
        Tierkunde3,
        Wildnisleben4WissenGoetter & Kulte3,
        Magiekunde4HandwerkAlchimie3
    ],
    vec![
        einZaubertrickausfolgenderListeDuft,
        Haarpracht,
        Schminken,
        GroßeGier6,
        HarmloseGestalt5,
        Hexenkrallen4,
        Katzenaugen6,
        LevthansFeuer6,
        SatuariasHerrlichkeit5,
        Zauberschnurren6
    ],
    vec![],
    vec![
        Altersresistenz(nurbeiEigeborenen),
        AngenehmerGeruch,
        Flink,
        Gutaussehend,
        SozialeAnpassungsfaehigkeit,
        VerhuellteAura
    ],
    vec![
        KeineFlugsalbe,
        KeinVertrauter,
        Persoenlichkeitsschwaeche(Arroganz, Eitelkeit),
        SchlechteEigenschaften(Goldgier, Rachsucht)
    ],
    vec![ZaeherHund],
    vec![Angstvor(Hoehen), Haesslich],
    vec![]
);

//Rabenhexe
profession!(
    Rabenhexe,
    "Rabenhexe",
    295,
    vec![VorteilZauberer(25), SonderfertigkeitTradition(Hexe)(135)],
    vec![
        SprachenSchriften8,
        Fluechefuerinsgesamt8,
        Flugsalbe,
        Vertrautenbindung
    ],
    vec![Raufen8, Stangenwaffen8],
    vec![],
    vec![
        zweiZaubertricksausfolgenderListeAbkuehlung,
        Bauchreden,
        Schnipsen,
        Trocken,
        BlickindieGedanken6,
        GroßeGier4,
        HarmloseGestalt4,
        Hexengalle4,
        Odem6,
        Radau5,
        Sanftmut5
    ],
    vec![],
    vec![Altersresistenz(nurbeiEigeborenen), VerhuellteAura],
    vec![
        KeineFlugsalbe,
        KeinVertrauter,
        Persoenlichkeitsschwaechen(Eitelkeit),
        SchlechteEigenschaften(Neugier, Goldgier, Rachsucht)
    ],
    vec![ZaeherHund],
    vec![Blutrausch],
    vec![]
);

//Schlangenhexe
profession!(
    Schlangenhexe,
    "Schlangenhexe",
    351,
    vec![VorteilZauberer(25), SonderfertigkeitTradition(Hexe)(135)],
    vec![
        SprachenSchriften10,
        Fluechefuerinsgesamt10AP,
        Flugsalbe,
        Vertrautenbindung
    ],
    vec![Dolche8],
    vec![],
    vec![
        einZaubertrickausfolgenderListeDuft,
        Hexenblick,
        Lockruf,
        Kehrbesen,
        Aengstelindern4,
        BlickindieGedanken6,
        Disruptivo4,
        HarmloseGestalt5,
        Hexenknoten4,
        Schlangenruf6,
        Vipernblick5
    ],
    vec![],
    vec![Altersresistenz(nurbeiEigeborenen), VerhuellteAura],
    vec![
        KeineFlugsalbe,
        KeinVertrautentier,
        SchlechteEigenschaften(Jaehzorn, Neugier, Rachsucht)
    ],
    vec![],
    vec![AngstvorSchlangen],
    vec![]
);

//Spinnenhexe
profession!(
    Spinnenhexe,
    "Spinnenhexe",
    334,
    vec![VorteilZauberer(25), SonderfertigkeitTradition(Hexe)(135)],
    vec![
        SprachenSchriften6,
        Flugsalbe,
        Vertrautenbindung,
        RoteJungfern
    ],
    vec![Dolche10],
    vec![],
    vec![
        einZaubertrickausfolgenderListeHaarpracht,
        Hexenblick,
        Insektenbann,
        UnheimlichesLachen,
        GroßeGier6,
        Hexengalle4,
        Hexenspeichel5,
        KrabbelnderSchrecken6,
        Spinnenlauf6,
        Spinnenruf4,
    ],
    vec![],
    vec![
        Altersresistenz(nurbeiEigeborenen),
        AngenehmerGeruch,
        Flink,
        Gutaussehend,
        SozialeAnpassungsfaehigkeit,
        VerhuellteAura
    ],
    vec![
        KeineFlugsalbe,
        KeinVertrauter,
        Persoenlichkeitsschwaeche(Arroganz, Eitelkeit),
        SchlechteEigenschaften(Goldgier, Rachsucht)
    ],
    vec![ZaeherHund],
    vec![Angstvor(Hoehen, Spinnen), Haesslich],
    vec![]
);
