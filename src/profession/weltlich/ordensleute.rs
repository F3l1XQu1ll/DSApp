use crate::crate_prof;
use crate::profession;

crate_prof!();

//Amazone
profession!(
    Amazone,
    "Amazone",
    275,
    vec![
        GE13(fuerFinteIundVorstoß),
        KO13(fuerBelastungsgewoehnungI),
        NachteilPrinzipientreueII(Gehorsam, Loyalitaet, Ehrenhaftigkeit)(20),
        Reiten10(fuerBerittenerKampf),
        VerpflichtungenI(Burg)(10),
        weiblich
    ],
    vec![SprachenSchriften6, BelastungsgewoehnungI, FinteI, Vorstoß],
    vec![Boegen10, Raufen10, Schilde10, Schwerter12, Stangenwaffen10],
    vec![],
    vec![],
    vec![],
    vec![
        Flink,
        HoheLebenskraft,
        HerausragendeKampftechnik,
        Waffenbegabung,
        ZaeherHund
    ],
    vec![Persoenlichkeitsschwaechen(
        Arroganz,
        VorurteilevorallemgegenMaenner,
        Weltfremd
    )],
    vec![],
    vec![Behaebig, ZerbrechlichVarianten],
    vec![]
);

//Draconiterin
profession!(
    Draconiterin,
    "Draconiterin",
    248,
    vec![
        NachteilPrinzipientreueI(Hesindekirche, Ordensregel)(10),
        VerpflichtungenII(Orden, MagisterinderMagister)(20)
    ],
    vec![SprachenSchriften12],
    vec![Armbrueste10, Dolche10, Stangenwaffen10],
    vec![],
    vec![],
    vec![],
    vec![
        GeborenerRedner,
        HoheSeelenkraft,
        HoheZaehigkeit,
        Vertrauenerweckend,
        Zeitgefuehl
    ],
    vec![
        Persoenlichkeitsschwaechen(Arroganz),
        SchlechteEigenschaft(Neugier)
    ],
    vec![],
    vec![Blutrausch],
    vec![]
);

//Golgaritin
profession!(
    Golgaritin,
    "Golgaritin",
    243,
    vec![
        KO13(fuerBelastungsgewoehnungI),
        NachteilPrinzipientreueII(Ordensregeln, Boronkirche)(20),
        VerpflichtungenII(Orden)(20)
    ],
    vec![SprachenSchriften6, BelastungsgewoehnungI],
    vec![
        Dolche8,
        Hiebwaffen12,
        einederfolgendenKampftechnikenauf11Schwerter,
        Stangenwaffen,
        Zweihandhiebwaffen,
        Zweihandschwerter
    ],
    vec![],
    vec![],
    vec![],
    vec![
        Adel,
        HoheSeelenkraft,
        HoheZaehigkeit,
        Reich,
        Waffenbegabung,
        ZaeherHund
    ],
    vec![Pech, Persoenlichkeitsschwaechen(Unheimlich), Taub, Stumm],
    vec![],
    vec![Angstvor(Dunkelheit, TotenundUntoten)],
    vec![]
);

//Rahjasutra-Meister
profession!(
    RahjasutraMeister,
    "Rahjasutra-Meister",
    270,
    vec![],
    vec![
        SprachenSchriften8,
        FertigkeitsspezialisierungBetoeren,
        FertigkeitsspezialisierungKoerperbeherrschung,
        RahjasutraKenntnisse
    ],
    vec![Raufen10],
    vec![],
    vec![],
    vec![],
    vec![
        AngenehmerGeruch,
        AusdauernderLiebhaberin,
        ErotischeAusstrahlung,
        ErotischeStimme,
        Gutaussehend,
        HerausragenderSinn,
        HoheSeelenkraft,
        HoheZaehigkeit,
        HypnotischeBrueste / Gesaeß / Gemaecht,
        StattlicherPenis,
        TraumhafterPo,
        Vertrauenerweckend,
        WohlgeformteBrueste,
        WunderschoeneVagina
    ],
    vec![SchlechteEigenschaft(Neugier)],
    vec![Treu],
    vec![
        Blutrausch,
        EingeschraenkterSinn,
        Farbenblind,
        Frigide,
        Haesslich,
        Homophobie / Heterophobie,
        Impotent,
        Krankheitsanfaellig,
        Persoenlichkeitsschwaeche(Eifersucht),
        Sprachfehler,
        UnerotischeStimme,
        VerfruehterHoehepunkt,
        Verstuemmelt
    ],
    vec![]
);

//Rosenritter
profession!(
    Rosenritter,
    "Rosenritter",
    349,
    vec![
        IN13(fuerdieSFAufmerksamkeit),
        GE13(fuerdieSFFinteIunddieSFVorstoß),
        VerpflichtungenII(MhaharanSchah)
    ],
    vec![
        SprachenSchriften8,
        FertigkeitsspezialisierungEtikette,
        Aufmerksamkeit,
        FinteI,
        Vorstoß
    ],
    vec![
        Dolche10,
        Lanzen10,
        Raufen10,
        einederfolgendenKampftechniken12,
        eineweitereauf11Hiebwaffen,
        Schilde,
        Schwerter,
        Stangenwaffen,
        Zweihandschwerter
    ],
    vec![],
    vec![],
    vec![],
    vec![
        Adel,
        AngenehmerGeruch,
        BegabunginGesellschaftstalenten,
        Flink,
        GeborenerRedner,
        Glueck,
        Gutaussehend,
        HoheLebenskraft,
        Reich,
        VerbesserteRegeneration(Lebensenergie),
        Vertrauenerweckend,
        Waffenbegabung,
        Wohlklang
    ],
    vec![
        Persoenlichkeitsschwaeche(Eitelkeit, Verwoehnt),
        SchlechteEigenschaft(Aberglaube, Autoritaetsglaube, Neugier)
    ],
    vec![],
    vec![
        Arm,
        Behaebig,
        Haesslich,
        Pech,
        SchlechteRegeneration(Lebensenergie),
        Zerbrechlich
    ],
    vec![]
);

//Säbeltänzerin
profession!(
    Saebeltaenzerin,
    "Säbeltänzerin",
    291,
    vec![
        GE13(fuerdieSFFinteI),
        KulturAranieroderMhanadistani,
        PrinzipientreueI(Rahjakirche)(20),
        VerpflichtungenII(Orden)(20)
    ],
    vec![
        Sprachenfuerinsgesamt4,
        FertigkeitsspezialisierungTanzen,
        FinteI
    ],
    vec![Raufen12, Schwerter12],
    vec![],
    vec![],
    vec![],
    vec![
        AngenehmerGeruch,
        ErotischeAusstrahlung,
        ErotischeStimme,
        Gutaussehend,
        Rahjagekuesst,
        Wohlklang
    ],
    vec![
        Persoenlichkeitsschwaeche(Eitelkeit, Weltfremd),
        SchlechteEigenschaft(Goldgier, Naiv)
    ],
    vec![Treu],
    vec![
        Blutrausch,
        Haesslich,
        Impotent,
        SchlechterGeschmack,
        UnerotischeStimme,
        VerfruehterHoehepunkt
    ],
    vec![]
);
