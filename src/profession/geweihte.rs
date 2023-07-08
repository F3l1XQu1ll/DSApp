use crate::crate_prof;
use crate::profession;

crate_prof!();

//Achazschamane
profession!(
    Achazschamane,
    "Achazschamane",
    322,
    vec![
        Spezies::Achaz,
        Kultur::Stammesachaz,
        Vorteil::Geweihter(25AP),
        Nachteil::PrinzipientreueI,  /*(Stammesregeln)(-10AP)*/
        Nachteil::VerpflichtungenII, /*(Stamm)(-20AP)*/
        Sonderfertigkeit::Tradition(Achazschamanen)(135AP),
    ],
    vec![SprachenSchriften4],
    vec![Hiebwaffen11, Raufen10],
    vec![
        Koerperbeherrschung,
        4,
        Kraftakt4,
        Selbstbeherrschung4,
        Sinnesschaerfe4,
        BekehrenundUeberzeugen2,
        Einschuechtern2,
        Etikette4,
        Menschenkenntnis4,
        Willenskraft4,
        Orientierung4,
        Pflanzenkunde4,
        Tierkunde4,
        Wildnisleben4,
        Geschichtswissen4,
        GoetterundKulte5,
        Rechnen2,
        Rechtskunde5,
        SagenundLegenden6
    ],
    vec![],
    vec![
        Bannzone6,
        BefehldesSchamanen5,
        HauchdesElements5,
        RatderAhnen7,
        einederfolgendenLiturgienauf5,
        DienerderErde,
        DienerderWellen,
        DienerderWolken
    ],
    vec![HoheSeelenkraft, HoheZaehigkeit, Vertrauenerweckend],
    vec![Persoenlichkeitsschwaechen(Arroganz, Eitelkeit)],
    vec![],
    vec![Behaebig, Unfaehig],
    vec![]
);

//Angroschpriester
profession!(
    Angroschpriester,
    "Angroschpriester",
    329,
    vec![
        Vorteil::Geweihter(25AP),
        Nachteil::PrinzipientreueII(Angroschkirche)(-20AP),
        VerpflichtungenII(Tempel)(-20AP),
        Sonderfertigkeit::Tradition(Angroschkirche)(125AP)
    ],
    vec![SprachenSchriften6, HueterderEsse],
    vec![Hiebwaffen10, Raufen10],
    vec![],
    vec![],
    vec![
        DieZwoelfSegnungen,
        BlickindieFlammen4,
        GebieterderFlammen5,
        LaeuterungdesErzes5,
        Objektsegen7,
        ZwergischeVerbruederung5
    ],
    vec![
        HoheLebenskraft,
        HoheSeelenkraft,
        HoheZaehigkeit,
        Mystiker,
        Vertrauenerweckend,
        ZaeherHund
    ],
    vec![Persoenlichkeitsschwaechen(Arroganz, Eitelkeit)],
    vec![],
    vec![Unfrei, Zerbrechlich],
    vec![]
);

//Ardarit
profession!(
    Ardarit,
    "Ardarit",
    227,
    vec![
        KO13(fuerBelastungsgewoehnungI),
        Nachteil::PrinzipientreueII(Ordensregeln, Rondrakirche)(-20AP),
        VerpflichtungenII(Tempel, Orden)(-20AP)
    ],
    vec![SprachenSchriften6, BelastungsgewoehnungI],
    vec![Raufen10,
        Schwerter |,
        Fechtwaffen12,
        Zweihandschwerter10],
    vec![],
    vec![],
    vec![],
    vec![
        HerausragendeKampftechnik,
        HoheLebenskraft,
        HoheSeelenkraft,
        HoheZaehigkeit,
        Mystiker,
        Reich,
        Vertrauenerweckend,
        Waffenbegabung,
        ZaeherHund
    ],
    vec![Persoenlichkeitsschwaechen(Arroganz, Eitelkeit)],
    vec![],
    vec![Behaebig, Unfaehig(Koerpertalente), Unfrei, Zerbrechlich],
    vec![]
);

//Avesgeweihte
profession!(
    Avesgeweihte,
    "Avesgeweihte",
    342,
    vec![
        Vorteil::Geweihter(25AP),
        Nachteil::PrinzipientreueI(Aveskirche)(-10AP),
        VerpflichtungenII(Kirche)(-20AP),
        Sonderfertigkeit::Tradition(Aveskirche)(110AP)
    ],
    vec![
        SprachenSchriften16,
        FertigkeitsspezialisierungGoetterundKulte,
        Kartographie
    ],
    vec![Dolche8, Raufen8, Schleudern10, Stangenwaffen10],
    vec![],
    vec![],
    vec![DieZwoelfSegnungen(außerEidsegen,Grabsegen und Weisheitssegen),
        GeschwinderSchritt6,
        GoettlichesZeichen6,
        Reisesegen5,
        RufderHeimat4,
        UnbeschwerteWanderung5,
        Wegweiser5],
    vec![
        Entfernungssinn,
        Glueck,
        HoheSeelenkraft,
        HoheZaehigkeit,
        Mystiker,
        Pragmatiker,
        Richtungssinn,
        SozialeAnpassungsfaehigkeit
    ],
    vec![SchlechteEigenschaft(Neugier)],
    vec![],
    vec![
        Blutrausch,
        Pechmagnet,
        Persoenlichkeitsschwaechen(Verwoehnt),
        SchlechteEigenschaft(Autoritaetsglaeubig),
        Unfrei
    ],
    vec![]
);

//Badilakanerin
profession!(
    Badilakanerin,
    "Badilakanerin",
    237,
    vec![
        Nachteil::PrinzipientreueII(Ordensregeln, Traviakirche)(-20AP),
        VerpflichtungenII(Tempel, Orden)(-20AP)
    ],
    vec![
        SprachenSchriften6,
        FertigkeitsspezialisierungLebensmittelbearbeitung
    ],
    vec![],
    vec![],
    vec![],
    vec![],
    vec![
        GeborenerRedner,
        HoheSeelenkraft,
        HoheZaehigkeit,
        Vertrauenerweckend
    ],
    vec![],
    vec![],
    vec![SchlechteEigenschaften(Goldgier, Jaehzorn)],
    vec![]
);

//Bannstrahler
profession!(
    Bannstrahler,
    "Bannstrahler",
    330,
    vec![
        KO13(fuerBelastungsgewoehnungI),
        KK13(fuerWuchtschlagI),
        Nachteil::PrinzipientreueIII(Ordensregeln, Praioskirche)(-30AP),
        VerpflichtungenIII(Orden)(-30AP)
    ],
    vec![SprachenSchriften6, BelastungsgewoehnungI, WuchtschlagI],
    vec![Hiebwaffen11, Kettenwaffen11, Raufen10, Schwerter10],
    vec![],
    vec![],
    vec![],
    vec![
        HerausragendeKampftechnik,
        HoheLebenskraft,
        HoheSeelenkraft,
        HoheZaehigkeit,
        Waffenbegabung,
        ZaeherHund
    ],
    vec![
        Persoenlichkeitsschwaechen(Arroganz, Eitelkeit),
        SchlechteEigenschaften(Jaehzorn, Rachsucht)
    ],
    vec![],
    vec![Behaebig, Unfaehig(Koerpertalente), Unfrei, Zerbrechlich],
    vec![]
);

//Borongeweihter
profession!(
    Borongeweihter,
    "Borongeweihter",
    285,
    vec![
        Vorteil::Geweihter(25AP),
        Nachteil::PrinzipientreueII(Boronkirche)(-20AP),
        VerpflichtungenII(Tempel, Kirche)(-20AP),
        Sonderfertigkeit::Tradition(Boronkirche)(130AP)
    ],
    vec![
        SprachenSchriften8,
        FertigkeitsspezialisierungGoetterundKulte
    ],
    vec![Dolche8, Hiebwaffen8],
    vec![],
    vec![],
    vec![
        DieZwoelfSegnungen,
        BannderFurcht3,
        BanndesLichts5,
        Exorzismus3,
        KleinerBannwiderUntote7,
        Objektsegen8,
        Schlaf6
    ],
    vec![
        Giftresistenz,
        Immunitaetgegen(Gift),
        Immunitaetgegen(Krankheit),
        Krankheitsresistenz,
        Mystiker,
        Pragmatiker,
        Zeitgefuehl
    ],
    vec![Lichtempfindlich],
    vec![Vertrauenerweckend],
    vec![Angstvor(Toten und Untoten),
        Giftanfaellig,
        Krankheitsanfaellig,],
    vec![]
);

//BrenochDûn(Gjalskerschamane)
profession!(
    BrenochDûn(Gjalskerschamane),
    "Brenoch-Dûn (Gjalskerschamane)",
    300,
    vec![
        Kultur(Gjalsker),
        Vorteil::Geweihter(25AP),
        Nachteil::PrinzipientreueI(Stammesregeln)(-10AP),
        VerpflichtungenII(Stamm)(-20AP),
        Sonderfertigkeit::Tradition(Gjalskerschamanen)(100AP)
    ],
    vec![SprachenSchriften8],
    vec![Hiebwaffen11, Raufen10],
    vec![],
    vec![],
    vec![Aufnahme(Initiation)4,
        Geistheilung5,
        Heldenkraft5,
        RatderAhnen7,
        Tiereberuhigen6],
    vec![HoheSeelenkraft, HoheZaehigkeit, Vertrauenerweckend],
    vec![Persoenlichkeitsschwaechen(Arroganz, Eitelkeit)],
    vec![],
    vec![Behaebig, Unfaehig(Koerpertalente), Unfrei, Zerbrechlich],
    vec![]
);

//BruderdesFeuers
profession!(
    BruderdesFeuers,
    "Bruder des Feuers",
    316,
    vec![
        Vorteil::Geweihter(25AP),
        Nachteil::PrinzipientreueII(Ingerimmkirche, Ordensregel)(-20AP),
        VerpflichtungenII(Orden, Kirche)(-20AP),
        Sonderfertigkeit::Tradition(Ingerimmkirche)(125AP)
    ],
    vec![
        SprachenSchriften8,
        FertigkeitsspezialisierungGoetterundKulte
    ],
    vec![Hiebwaffen10, Raufen8],
    vec![],
    vec![],
    vec![
        DieZwoelfSegnungen,
        BlickindieFlammen7,
        GebieterderFlammen6,
        GoettlichesZeichen7,
        HerrderFlammen6
    ],
    vec![
        Begabung(Handwerkstalent),
        HerausragendeFertigkeit(Handwerkstalent),
        Hitzeresistenz,
        HoheZaehigkeit,
        Mystiker,
        ZaeherHund
    ],
    vec![
        Persoenlichkeitsschwaechen(Arroganz, Unheimlich),
        SchlechteEigenschaft(Aberglaube, Neugier),
        Stigma(Brandmale)
    ],
    vec![AngenehmerGeruch],
    vec![
        Angstvor(Feuer),
        Hitzeempfindlich,
        Unfaehig(Handwerkstalente),
        Zerbrechlich
    ],
    vec![]
);

//BuendlerindesWahrenGlaubens
profession!(
    BuendlerindesWahrenGlaubens,
    "Bündlerin des Wahren Glaubens",
    226,
    vec![
        Nachteil::PrinzipientreueIII(Ordensregeln)(-30AP),
        VerpflichtungenII(Orden)(-20AP)
    ],
    vec![SprachenSchriften8,],
    vec![Raufen10],
    vec![],
    vec![],
    vec![],
    vec![GeborenerRedner, HoheSeelenkraft, Vertrauenerweckend,],
    vec![Persoenlichkeitsschwaechen(Arroganz, Eitelkeit)],
    vec![],
    vec![],
    vec![]
);

//ChrSsirSsrPriester
profession!(
    ChrSsirSsrPriester,
    "Chr’Ssir’Ssr-Priester",
    365,
    vec![
        Spezies::Achaz,
        Vorteil::Geweihter(25AP),
        Nachteil::PrinzipientreueII(ChrSsirSsrKult)(-20AP),
        VerpflichtungenII(Sippe)(-20AP),
        Sonderfertigkeit::Tradition(ChrSsirSsrKult)(130AP)
    ],
    vec![
        SprachenSchriften2,
        FertigkeitsspezialisierungGoetterundKulte
    ],
    vec![Dolche8, Stangenwaffen10, Wurfwaffen10],
    vec![
        Fliegen3,
        Klettern4,
        Koerperbeherrschung4,
        Kraftakt2,
        Schwimmen7,
        Selbstbeherrschung2,
        Sinnesschaerfe3,
        BekehrenundÜberzeugen5,
        Einschuechtern4,
        Etikette2,
        Menschenkenntnis4,
        Überreden2,
        Willenskraft5,
        FischenundAngeln7,
        Orientierung5,
        Pflanzenkunde3,
        Tierkunde4,
        Wildnisleben5,
        Geographie3,
        Geschichtswissen2,
        GoetterundKulte6,
        Rechnen4,
        Rechtskunde2,
        SagenundLegenden4,
        Sternkunde4,
        BooteundSchiffe6,
        HeilkundeKrankheiten3,
        HeilkundeWunden2
    ],
    vec![],
    vec![Eidsegen,
        Glueckssegen,
        KleinerHeilsegen,
        Staerkungssegen,
        Tranksegen,
        Weisheitssegen;
        Blitzschlag3,
        Flugechsenruf5,
        KleineWindhose5,
        Nebelschwaden7,
        Objektsegen6,
        Regenkontrolle6],
    vec![
        Begabung(Schwimmen, BooteundSchiffe),
        DunkelsichtI,
        Entfernungssinn,
        GeborenerRedner,
        Kaelteresistenz,
        Richtungssinn
    ],
    vec![
        Persoenlichkeitsschwaeche(Jaehzorn),
        SchlechteEigenschaft(Aberglaube)
    ],
    vec![Persoenlichkeitsschwaeche(Rachsucht)],
    vec![AngstvordemMeer, Kaelteempfindlich, Nachtblind, Unfrei],
    vec![]
);

//Efferdgeweihte
profession!(
    Efferdgeweihte,
    "Efferdgeweihte",
    381,
    vec![
        Vorteil::Geweihter(25AP),
        Nachteil::PrinzipientreueII(Efferdkirche)(-20AP),
        VerpflichtungenII(Tempel, Kirche)(-20AP),
        Sonderfertigkeit::Tradition(Efferdkirche)(130AP)
    ],
    vec![
        SprachenSchriften12,
        FertigkeitsspezialisierungGoetterundKulte,
        Analytiker,
        GelaendekundeMeereskundig
    ],
    vec![Dolche8, Raufen10, Stangenwaffen10, Wurfwaffen10],
    vec![],
    vec![],
    vec![
        DieZwoelfSegnungen,
        GoettlichesZeichen6,
        GuterFang7,
        HeilsameQuelle3,
        KleinerWindstoß5,
        Objektsegen6,
        Unterwasseratmung5
    ],
    vec![
        Begabung(Schwimmen, BooteundSchiffe),
        DunkelsichtI,
        Entfernungssinn,
        GeborenerRedner,
        Kaelteresistenz,
        Richtungssinn
    ],
    vec![
        Persoenlichkeitsschwaeche(Jaehzorn),
        SchlechteEigenschaft(Aberglaube)
    ],
    vec![Persoenlichkeitsschwaeche(Rachsucht)],
    vec![Angstvor(demMeer), Kaelteempfindlich, Nachtblind, Unfrei],
    vec![]
);

//EfferdgeweihterderSiebenwindkueste
profession!(
    EfferdgeweihterderSiebenwindkueste,
    "Efferdgeweihter der Siebenwindküste",
    381,
    vec![
        Vorteil::Geweihter(25AP),
        Nachteil::PrinzipientreueII(Efferdkirche)(-20AP),
        VerpflichtungenII(Tempel, Kirche)(-20AP),
        Sonderfertigkeit::Tradition(Efferdkirche)(130AP)
    ],
    vec![
        SprachenSchriften12,
        FertigkeitsspezialisierungGoetterundKulte,
        Gelaendekunde(Meereskundig)
    ],
    vec![Dolche8, Raufen10, Stangenwaffen10, Wurfwaffen10],
    vec![],
    vec![],
    vec![],
    vec![
        Begabung(Schwimmen, BooteundSchiffe),
        DunkelsichtI,
        Entfernungssinn,
        GeborenerRedner,
        Kaelteresistenz,
        Richtungssinn
    ],
    vec![
        Persoenlichkeitsschwaeche(Jaehzorn),
        SchlechteEigenschaft(Aberglaube)
    ],
    vec![],
    vec![
        Angstvor(demMeer),
        Kaelteempfindlich,
        Nachtblind,
        Persoenlichkeitsschwaeche(Rachsucht),
        Unfrei
    ],
    vec![]
);

//Etilianer
profession!(
    Etilianer,
    "Etilianer",
    221,
    vec![
        Nachteil::PrinzipientreueII(Ordensregeln, Boronkirche)(-20AP),
        VerpflichtungenII(Orden)(-20AP)
    ],
    vec![SprachenSchriften8, FertigkeitsspezialisierungHeilkundeSeele],
    vec![Raufen10],
    vec![],
    vec![],
    vec![],
    vec![HoheSeelenkraft, Vertrauenerweckend],
    vec![],
    vec![],
    vec![],
    vec![]
);

//Ferkinaschamane
profession!(
    Ferkinaschamane,
    "Ferkinaschamane",
    299,
    vec![
        Kultur(Ferkinas),
        Vorteil::Geweihter(25AP),
        Nachteil::PrinzipientreueI(Stammesregeln)(-10AP),
        VerpflichtungenII(Stamm)(-20AP),
        Sonderfertigkeit::Tradition(Ferkinaschamanen)(100AP)
    ],
    vec![SprachenSchriften6],
    vec![Hiebwaffen11, Raufen10],
    vec![],
    vec![],
    vec![
        Geistheilung5,
        Kriegsfarben6,
        Magiesicht5,
        RatderAhnen7,
        Schutzsegen5
    ],
    vec![HoheSeelenkraft, HoheZaehigkeit, Vertrauenerweckend],
    vec![Persoenlichkeitsschwaechen(Arroganz, Eitelkeit)],
    vec![],
    vec![Behaebig, Unfaehig(Koerpertalente), Unfrei, Zerbrechlich],
    vec![]
);

//Firungeweihter
profession!(
    Firungeweihter,
    "Firungeweihter",
    374,
    vec![
        Vorteil::Geweihter(25AP),
        Nachteil::PrinzipientreueII(Firunkirche)(-20AP),
        VerpflichtungenII(Tempel, Kirche)(-20AP),
        Sonderfertigkeit::Tradition(Firunkirche)(140AP)
    ],
    vec![ SprachenSchriften8,
        FertigkeitsspezialisierungGoetterundKulte,
        Gelaendekunde(Eis und Schnee,Gebirgs,Steppen,Sumpf | Waldkundig),
        Jaeger,
        Wettervorhersage],
    vec![Boegen12, Dolche10, Raufen8, Stangenwaffen10],
    vec![],
    vec![],
    vec![
        DieZwoelfSegnungen,
        AugedesJaegers6,
        Frostschutz6,
        Jagdglueck7,
        SichererTritt6,
        Zuflucht5
    ],
    vec![
        HoheLebenskraft,
        HoheSeelenkraft,
        HoheZaehigkeit,
        Kaelteresistenz,
        Mystiker,
        Pragmatiker,
        Richtungssinn,
        ZaeherHund
    ],
    vec![Persoenlichkeitsschwaechen(Unheimlich, Weltfremd)],
    vec![SozialeAnpassungsfaehigkeit],
    vec![
        Angstvor,
        Behaebig,
        Kaelteempfindlich,
        Persoenlichkeitsschwaechen(Verwoehnt),
        SchlechteEigenschaft(Goldgier, Verschwendungssucht),
        Zerbrechlich
    ],
    vec![]
);

//Graveshpriester
profession!(
    Graveshpriester,
    "Graveshpriester",
    309,
    vec![
        Vorteil::Geweihter(25AP),
        Nachteil::PrinzipientreueII(Graveshkult)(-20AP),
        VerpflichtungenII(Stammesregeln)(-20AP),
        Sonderfertigkeit::Tradition(Graveshkult)(120AP)
    ],
    vec![Sprachenfuerinsgesamt6],
    vec![Hiebwaffen10, Raufen10],
    vec![Koerperbeherrschung4,
        Kraftakt6,
        Selbstbeherrschung6,
        Sinnesschaerfe4,
        BekehrenundÜberzeugen4,
        Einschuechtern2,
        Etikette4,
        Menschenkenntnis4,
        Willenskraft6,
        Wildnisleben3,
        Geschichtswissen7,
        GoetterundKulte4,
        Rechnen6,
        Rechtsk und e6,
        SagenundLegenden6,
        HandwerkMetallbearbeitung6],
    vec![],
    vec![
        Feuerwall3,
        GebieterderFlammen4,
        HauchdesElements5,
        HerrderFlammen5,
        Objektsegen6,
        Objektweihe4,
        Steinhaut4
    ],
    vec![
        HoheLebenskraft,
        HoheSeelenkraft,
        HoheZaehigkeit,
        Mystiker,
        Vertrauenerweckend,
        ZaeherHund
    ],
    vec![Persoenlichkeitsschwaechen(Arroganz, Eitelkeit)],
    vec![],
    vec![Unfrei, Zerbrechlich],
    vec![]
);

//Hesindegeweihte
profession!(
    Hesindegeweihte,
    "Hesindegeweihte",
    310,
    vec![
        Vorteil::Geweihter(25AP),
        Nachteil::PrinzipientreueI(Hesindekirche)(-10AP),
        VerpflichtungenII(Tempel, Kirche)(-20AP),
        Sonderfertigkeit::Tradition(Hesindekirche)(130AP)
    ],
    vec![
        SprachenSchriften16,
        FertigkeitsspezialisierungGoetterundKulte
    ],
    vec![Dolche8, Stangenwaffen8],
    vec![],
    vec![],
    vec![
        DieZwoelfSegnungen,
        Entzifferung5,
        FriedvolleAura6,
        GoettlicherFingerzeig3,
    ],
    vec![
        HoheSeelenkraft,
        Mystiker,
        Pragmatiker,
        Vertrauenerweckend,
        Zeitgefuehl
    ],
    vec![SchlechteEigenschaften(Neugier)],
    vec![],
    vec![Blutrausch],
    vec![]
);

//HSzintPriester
profession!(
    HSzintPriester,
    "H’Szint-Priester",
    317,
    vec![
        Spezies::Achaz,
        Vorteil::Geweihter(25AP),
        Nachteil::PrinzipientreueI(HSzintKult)(-10AP),
        VerpflichtungenII(Sippe)(-20AP),
        Sonderfertigkeit::Tradition(HSzintKult)(130AP)
    ],
    vec![
        SprachenSchriften6,
        FertigkeitsspezialisierungGoetterundKulte
    ],
    vec![Dolche8, Stangenwaffen8],
    vec![
        Koerperbeherrschung2,
        Selbstbeherrschung2,
        Sinnesschaerfe5,
        BekehrenundÜberzeugen6,
        Etikette2,
        Menschenkenntnis2,
        Willenskraft6,
        Pflanzenkunde4,
        Tierkunde3,
        Geographie4,
        Geschichtswissen5,
        GoetterundKulte7,
        Magiekunde4,
        Mechanik4,
        Rechnen5,
        Rechtskunde3,
        SagenundLegenden5,
        Sphaerenkunde5,
        Sternkunde3,
        Alchimie4,
        HeilkundeWunden2
    ],
    vec![],
    vec![Harmoniesegen,
        KleinerSchutzsegen,
        Speisesegen,
        Staerkungssegen,
        Tranksegen,
        Weisheitssegen;
        FriedvolleAura6,
        Magieanalyse5,
        Magieschutz7,
        Magiesicht6,
        Objektsegen5,
        Schlangenruf3],
    vec![
        HoheSeelenkraft,
        Mystiker,
        Pragmatiker,
        Vertrauenerweckend,
        Zeitgefuehl
    ],
    vec![SchlechteEigenschaften(Neugier)],
    vec![],
    vec![Blutrausch],
    vec![]
);

//Ifirngeweihte
profession!(
    Ifirngeweihte,
    "Ifirngeweihte",
    373,
    vec![
        Vorteil::Geweihter(25AP),
        Nachteil::PrinzipientreueI(Ifirnkirche)(-10AP),
        VerpflichtungenII(Tempel, Kirche)(-20AP),
        Sonderfertigkeit::Tradition(Ifirnkirche)(105AP)
    ],
    vec![ SprachenSchriften8,
        FertigkeitsspezialisierungGoetterundKulte,
        Gelaendekunde(Eis und Schnee,Gebirgs,Steppen | Waldkundig),
        Jaeger],
    vec![Boegen12, Dolche10, Raufen8, Stangenwaffen10],
    vec![],
    vec![],
    vec![DieZwoelfSegnungen(außerGlueckssegen,Tranksegen und Weisheitssegen),
        FriedvolleAura5,
        HilfeinderNot6,
        Tierleidlindern6,
        Tiereberuhigen5,
        Zuflucht6],
    vec![
        HoheLebenskraft,
        HoheSeelenkraft,
        HoheZaehigkeit,
        Kaelteresistenz,
        Mystiker,
        Pragmatiker,
        Richtungssinn,
        Vertrauenerweckend,
        ZaeherHund
    ],
    vec![
        Persoenlichkeitsschwaechen(Weltfremd),
        SchlechteEigenschaft(Naiv)
    ],
    vec![SozialeAnpassungsfaehigkeit],
    vec![
        Kaelteempfindlich,
        Nachtblind,
        Persoenlichkeitsschwaechen(Verwoehnt),
        SchlechteEigenschaft(Jaehzorn, Rachsucht),
        Zerbrechlich
    ],
    vec![]
);

//Ingerimmgeweihte
profession!(
    Ingerimmgeweihte,
    "Ingerimmgeweihte",
    267,
    vec![
        Vorteil::Geweihter(25AP),
        Nachteil::PrinzipientreueII(Ingerimmkirche)(-20AP),
        VerpflichtungenII(Tempel, Kirche)(-20AP),
        Sonderfertigkeit::Tradition(Ingerimmkirche)(125AP)
    ],
    vec![
        SprachenSchriften8,
        FertigkeitsspezialisierungGoetterundKulte
    ],
    vec![Hiebwaffen10, Raufen8],
    vec![],
    vec![],
    vec![
        DieZwoelfSegnungen,
        ErzeneOpfergabe6,
        GebieterderFlammen6,
        GeweihterPanzer5,
        HerrderFlammen5,
        Wiederherstellung6
    ],
    vec![
        Begabung(Handwerkstalent),
        HerausragendeFertigkeit(Handwerkstalent),
        Hitzeresistenz,
        Mystiker,
        Pragmatiker,
        SozialeAnpassungsfaehigkeit,
        Vertrauenerweckend,
        ZaeherHund
    ],
    vec![
        Persoenlichkeitsschwaechen(Arroganz),
        SchlechteEigenschaft(Neugier, Jaehzorn)
    ],
    vec![AngenehmerGeruch],
    vec![
        Angstvor(Feuer),
        Hitzeempfindlich,
        Unfaehig(Handwerkstalente),
        Zerbrechlich
    ],
    vec![]
);

//Korgeweihter
profession!(
    Korgeweihter,
    "Korgeweihter",
    360,
    vec![
        KK13(fuerWuchtschlagI),
        KO13(fuerBelastungswoehnungI),
        Vorteil::Geweihter(25AP),
        Nachteil::PrinzipientreueII(Korkirche)(-20AP),
        VerpflichtungenI(Tempel, Kirche)(-10AP),
        Sonderfertigkeit::Tradition(Korkirche)(130AP)
    ],
    vec![
        SprachenSchriften8,
        FertigkeitsspezialisierungGoetterundKulte,
        BelastungsgewoehnungI,
        WuchtschlagI
    ],
    vec![Armbrueste10, Hiebwaffen10, Raufen10, Zweihandhiebwaffen12],
    vec![],
    vec![],
    vec![DieZwoelfSegnungen(außerGeburtssegen,Harmoniesegen und KleinerHeilsegen),
        Angriffslust6,
        BlutigerZorn5,
        EhrlicherVertrag5,
        MaechtigerAngriff6,
        Schmerzresistenz6,
        ZaeheHaut5],
    vec![
        HoheLebenskraft,
        HerausragendeKampftechnik,
        HoheSeelenkraft,
        HoheZaehigkeit,
        Mystiker,
        Pragmatiker,
        Reich,
        SozialeAnpassungsfaehigkeit,
        Waffenbegabung,
        ZaeherHund
    ],
    vec![
        Persoenlichkeitsschwaechen(Arroganz, Unheimlich),
        SchlechteEigenschaft(Jaehzorn)
    ],
    vec![Zauberer],
    vec![Angstvor, Behaebig, Unfaehig(Koerpertalente), Zerbrechlich],
    vec![]
);

//Korgeweihter
profession!(
    Korgeweihter,
    "Levthanpriester",
    319,
    vec![
        Vorteil::Geweihter(25AP),
        Nachteil::PrinzipientreueI(Levthankult)(-10AP),
        Sonderfertigkeit::Tradition(Levthankult)(125AP)
    ],
    vec![SprachenSchriften4, FertigkeitsspezialisierungBetoeren],
    vec![Hiebwaffen10, Raufen10, Zweihandhiebwaffen10],
    vec![],
    vec![],
    vec![DieZwoelfSegnungen(außerEidsegen,Schutzsegen und Weisheitssegen);
        Einfluesterung4,
        EntstelltesAntlitz5,
        ErregenderRausch5,
        FesselndesBand5,
        Lusterzeugen4,
        VerstecktesBegehren4],
    vec![
        Gutaussehend,
        Haesslich,
        HerausragenderSinn,
        HoheSeelenkraft,
        HoheZaehigkeit,
        Mystiker
    ],
    vec![
        Persoenlichkeitsschwaeche(Eitelkeit, Unheimlich, Verwoehnt),
        SchlechteEigenschaft(Neugier, Goldgier, Spielsucht, Verschwendungssucht)
    ],
    vec![Adel],
    vec![Krankheitsanfaellig],
    vec![]
);

//adaBasariOrdensmitglied
profession!(
    MadaBasariOrdensmitglied,
    "Mada Basari-Ordensmitglied",
    262,
    vec![
        Nachteil::PrinzipientreueI(Phexkirche)(-10AP),
        VerpflichtungenII(Tempel, Kirche)(-20AP)
    ],
    vec![SprachenSchriften12, FertigkeitsspezialisierungHandel],
    vec![Dolche10, Raufen8, Wurfwaffen8],
    vec![
        Klettern4,
        Koerperbeherrschung4,
        Selbstbeherrschung3,
        Sinnesschaerfe5,
        Taschendiebstahl2,
        Verbergen4,
        BekehrenundÜberzeugen4,
        Etikette4,
        Gassenwissen4,
        Menschenkenntnis5,
        Überreden4,
        Willenskraft5,
        Orientierung3,
        Wildnisleben2,
        Geschichtswissen3,
        GoetterundKulte5,
        Magiekunde3,
        Mechanik3,
        Rechnen6,
        Rechtskunde3,
        SagenundLegenden3,
        Sternkunde4HandwerkHandel7,
        Schloesserknacken3
    ],
    vec![],
    vec![],
    vec![
        Beidhaendig,
        Entfernungssinn,
        Fuchssinn,
        Glueck,
        Unscheinbar,
        Zwergennase
    ],
    vec![SchlechteEigenschaft(Neugier, Goldgier)],
    vec![],
    vec![Blutrausch, Nachtblind, Pech],
    vec![]
);

//Marbopriester
profession!(
    Marbopriester,
    "Marbopriester",
    297,
    vec![
        Vorteil::Geweihter,
        Nachteil::PrinzipientreueII(Ordensregeln, Boronkirche)(-20AP),
        VerpflichtungenII(Orden)(-20AP),
        Sonderfertigkeit::Tradition(Marbokult)(120AP)
    ],
    vec![
        SprachenSchriften6,
        FertigkeitsspezialisierungGoetterundKulte
    ],
    vec![Dolche12, Raufen10],
    vec![],
    vec![],
    vec![DieZwoelfSegnungen(außerGeburtssegen,Glueckssegen und Speisesegen);
        HeilungvonSeelenkranken4,
        Lebenstausch5,
        Objektsegen7,
        Schattenfessel6,
        VampirischeKraefte5],
    vec![HoheSeelenkraft, Vertrauenerweckend],
    vec![],
    vec![],
    vec![],
    vec![]
);

//MitglieddesDreischwesternordens
profession!(
    MitglieddesDreischwesternordens,
    "Mitglied des Dreischwesternordens",
    299,
    vec![
        Nachteil::PrinzipientreueII(Ordensregeln)(-20AP),
        VerpflichtungenII(Orden)(-20AP)
    ],
    vec![SprachenSchriften8],
    vec![],
    vec![],
    vec![],
    vec![],
    vec![
        HoheLebenskraft,
        HoheSeelenkraft,
        HoheZaehigkeit,
        Vertrauenerweckend,
    ],
    vec![Persoenlichkeitsschwaechen(Naiv)],
    vec![],
    vec![],
    vec![]
);

//NamenloserGeweihter
profession!(
    NamenloserGeweihter,
    "Namenloser-Geweihter",
    303,
    vec![
        Vorteil::Geweihter(25AP),
        Nachteil::PrinzipientreueI(Namenloser)(-10AP),
        VerpflichtungenII(Verschwoerung, Kult)(-20AP),
        Sonderfertigkeit::Tradition(Namenloser)(150AP)
    ],
    vec![
        SprachenSchriften10,
        FertigkeitsspezialisierungGoetterundKulte
    ],
    vec![Dolche10, Raufen10],
    vec![Koerperbeherrschung2,
        Selbstbeherrschung7,
        Singen2,
        Verbergen4,
        Zechen2,
        BekehrenundÜberzeugen6,
        Einschuechtern 6,
        Menschenkenntnis5,
        Überreden6,
        Verkleiden4,
        Willenskraft7,
        Fesseln2,
        Geschichtswissen2,
        GoetterundKulte6,
        Magiekunde 2,
        SagenundLegenden5,
        Sphaerenkunde4,
        HandwerkAlchimie2,
        Handel2,
        HeilkundeGift4],
    vec![],
    vec![DesEinenbezaubernderSphaerenklang7,
        NamenloseKaelte6,
        NamenloseRaserei5,
        Namenlose Zweifel6,
        NamenlosesVergessen4,
        Pech und  Schwefel5],
    vec![
        GeborenerRedner,
        Giftresistenz,
        HoheSeelenkraft,
        HoheZaehigkeit,
        Mystiker,
        Pragmatiker,
        SozialeAnpassungsfaehigkeit,
        Vertrauenerweckend
    ],
    vec![
        Haesslich,
        Pech,
        Pechmagnet,
        Persoenlichkeitsschwaechen(Arroganz, Unheimlich),
        SchlechteEigenschaft(Goldgier, Jaehzorn, Rachsucht),
        Stigma,
        Verstuemmelt
    ],
    vec![],
    vec![SchlechteEigenschaft(Naiv)],
    vec![]
);

//Nandusgeweihte
profession!(
    Nandusgeweihte,
    "Nandusgeweihte",
    265,
    vec![
        Vorteil::Geweihter(25AP),
        Nachteil::PrinzipientreueI(Nanduskirche)(-10AP),
        VerpflichtungenII(Tempel, Kirche)(-20AP),
        Sonderfertigkeit::Tradition(Nanduskirche)(130AP)
    ],
    vec![
        SprachenSchriften14,
        FertigkeitsspezialisierungGoetterundKulte
    ],
    vec![Armbrust8, Stangenwaffen8],
    vec![],
    vec![],
    vec![DieZwoelfSegnungen(außerFeuersegen,KleinerSchutzsegen und Staerkungssegen),
        BildfuerdieEwigkeit7,
        Entzifferung5,
        GoettlicheErkenntnis6,
        GoettlicherFingerzeig5,
        MitDummheitschlagen4,
        OffenlegungdesGeistes5],
    vec![
        GeborenerRedner,
        HoheSeelenkraft,
        Mystiker,
        Pragmatiker,
        SozialeAnpassungsfaehigkeit,
        Vertrauenerweckend
    ],
    vec![
        Persoenlichkeitsschwaechen(Arroganz, Eitelkeit),
        SchlechteEigenschaft(Neugier)
    ],
    vec![],
    vec![Blutrausch, Persoenlichkeitsschwaechen(Autoritaetsglaeubig)],
    vec![]
);

//Nivesenschamane
profession!(
    Nivesenschamane,
    "Nivesenschamane",
    319,
    vec![
        Kultur(Nivesen),
        Vorteil::Geweihter(25AP),
        Nachteil::PrinzipientreueI(Stammesregeln)(-10AP),
        VerpflichtungenII(Sippe)(-20AP),
        Sonderfertigkeit::Tradition(Nivesenschamanen)(100AP)
    ],
    vec![SprachenSchriften8],
    vec![Hiebwaffen11, Raufen10],
    vec![],
    vec![],
    vec![
        Geisterfalle5,
        Geistersprache4,
        Geistheilung6,
        RatderAhnen7,
        Wolfsgestalt5
    ],
    vec![HoheSeelenkraft, HoheZaehigkeit, Vertrauenerweckend],
    vec![Persoenlichkeitsschwaechen(Arroganz, Eitelkeit)],
    vec![],
    vec![Behaebig, Unfaehig(Koerpertalente), Unfrei, Zerbrechlich],
    vec![]
);

//Noionit
profession!(
    Noionit,
    "Noionit",
    291,
    vec![
        Vorteil::Geweihter,
        Nachteil::PrinzipientreueII(Ordensregeln, Boronkirche)(-20AP),
        VerpflichtungenII(Orden)(-20AP),
        Sonderfertigkeit::Tradition(Boronkirche)
    ],
    vec![
        SprachenSchriften8,
        FertigkeitsspezialisierungGoetterundKulte,
        FertigkeitsspezialisierungHeilkundeSeele
    ],
    vec![Dolche10, Raufen10],
    vec![],
    vec![],
    vec![
        DieZwoelfSegnungen,
        BannderFurcht3,
        BanndesLichts5,
        Exorzismus3,
        KleinerBannwiderUntote7,
        Objektsegen8,
        Schlaf6
    ],
    vec![HoheSeelenkraft, Vertrauenerweckend],
    vec![],
    vec![],
    vec![],
    vec![]
);

//Numinorupriester
profession!(
    Numinorupriester,
    "Numinorupriester",
    264,
    vec![
        Vorteil::Geweihter(25AP),
        Nachteil::PrinzipientreueII(Numinorukult)(-20AP),
        VerpflichtungenII(Kult)(-20AP),
        Sonderfertigkeit::Tradition(Numinorukult)(125AP)
    ],
    vec![
        SprachenSchriften10,
        FertigkeitsspezialisierungGoetterundKulte
    ],
    vec![Stangenwaffen8],
    vec![],
    vec![],
    vec![
        SechsSegnungen,
        BlickaufdenMeeresgrund4,
        GoettlicherFingerzeig4,
        LeitendeStroemung4,
        NuminorusFesseln6,
        Unterwasseratmung6
    ],
    vec![
        Entfernungssinn,
        Fuchssinn,
        Kaelteresistenz,
        Mystiker,
        Pragmatiker,
        Richtungssinn,
        Vertrauenerweckend,
        Zeitgefuehl
    ],
    vec![
        Persoenlichkeitsschwaechen(Arroganz, Unheimlich),
        SchlechteEigenschaft(Aberglaube)
    ],
    vec![],
    vec![AngstvordemMeer, AngstvorMeerestieren],
    vec![]
);

//OrdenskriegerderTarisharim
profession!(
    OrdenskriegerderTarisharim,
    "Ordenskrieger der Tarisharim",
    205,
    vec![
        Nachteil::PrinzipientreueII(Ordensregeln)(-20AP),
        VerpflichtungenII(Orden)(-20AP)
    ],
    vec![SprachenSchriften6],
    vec![Raufen10, Schwerter12],
    vec![],
    vec![],
    vec![],
    vec![
        HerausragendeKampftechnik,
        HoheLebenskraft,
        HoheSeelenkraft,
        HoheZaehigkeit,
        Waffenbegabung,
        ZaeherHund
    ],
    vec![],
    vec![],
    vec![Behaebig, Unfaehig(Koerpertalente), Unfrei, Zerbrechlich],
    vec![]
);

//OrdenskriegerinderAlDrakorhim
profession!(
    OrdenskriegerinderAlDrakorhim,
    "Ordenskriegerin der Al’Drakorhim",
    439,
    vec![
        GE13(fuerFinteI),
        KO13(fuerBelastungsgewoehnungI),
        Nachteil::PrinzipientreueI(Ordensregeln)(-10AP),
        VerpflichtungenII(Orden)(-20AP)
    ],
    vec![
        SprachenSchriften8,
        BelastungsgewoehnungI,
        FinteI,
        FertigkeitsspezialisierungTierkunde
    ],
    vec![
        Boegen10,
        Dolche11,
        Raufen11,
        Schilde11,
        Schwerter12,
        Stangenwaffen12
    ],
    vec![],
    vec![],
    vec![],
    vec![
        HerausragendeKampftechnik,
        HoheLebenskraft,
        HoheSeelenkraft,
        HoheZaehigkeit,
        Waffenbegabung,
        ZaeherHund
    ],
    vec![
        Persoenlichkeitsschwaechen(Arroganz),
        SchlechteEigenschaft(Aberglaube)
    ],
    vec![],
    vec![Behaebig, Unfaehig(Koerpertalente), Unfrei, Zerbrechlich],
    vec![]
);

//OrdenskriegerinderBeniUchakâni
profession!(
    OrdenskriegerinderBeniUchakâni,
    "Ordenskriegerin der Beni Uchakâni",
    306,
    vec![
        KK13(fuerWuchtschlagI),
        Nachteil::PrinzipientreueII(Ordensregeln)(-20AP),
        VerpflichtungenII(Orden)(-20AP)
    ],
    vec![SprachenSchriften4, WuchtschlagI],
    vec![Raufen12, Schwerter12, Wurfwaffen12],
    vec![],
    vec![],
    vec![],
    vec![
        HerausragendeKampftechnik,
        HoheLebenskraft,
        HoheZaehigkeit,
        Waffenbegabung,
        ZaeherHund
    ],
    vec![
        Persoenlichkeitsschwaeche(Weltfremd),
        SchlechteEigenschaften(Aberglaube)
    ],
    vec![],
    vec![Behaebig, Unfaehig(Koerpertalente), Unfrei, Zerbrechlich],
    vec![]
);

//OrdensmitgliedderBeniFessiri
profession!(
    OrdensmitgliedderBeniFessiri,
    "Ordensmitglied der Beni Fessiri",
    309,
    vec![
        GE13(fuerFinteI),
        Nachteil::PrinzipientreueI(Ordensregeln, Phexkirche)(-10AP),
        VerpflichtungenII(Orden)(-20AP)
    ],
    vec![
        SprachenSchriften6,
        FinteI,
        FertigkeitsspezialisierungVerbergen
    ],
    vec![Dolche11, Raufen11, Wurfwaffen12],
    vec![],
    vec![],
    vec![],
    vec![
        GeborenerRedner,
        HoheSeelenkraft,
        HoheZaehigkeit,
        Vertrauenerweckend
    ],
    vec![
        Persoenlichkeitsschwaeche(Eitelkeit),
        SchlechteEigenschaften(Goldgier, Neugier)
    ],
    vec![Adel, Reich],
    vec![Behaebig, Unfaehig(Koerpertalente)],
    vec![]
);

//Perainegeweihter
profession!(
    Perainegeweihter,
    "Perainegeweihter",
    290,
    vec![
        Vorteil::Geweihter(25AP),
        Nachteil::PrinzipientreueI(Perainekirche)(-10AP),
        VerpflichtungenII(Tempel, Kirche)(-20AP),
        Sonderfertigkeit::Tradition(Perainekirche)(110AP)
    ],
    vec![
        SprachenSchriften8,
        FertigkeitsspezialisierungGoetterundKulte
    ],
    vec![Hiebwaffen8],
    vec![],
    vec![],
    vec![
        DieZwoelfSegnungen,
        FriedvolleAura3,
        Giftbann5,
        Heilsegen7,
        Krankheitsbann6,
        Objektsegen7,
        Pflanzenwuchs4
    ],
    vec![
        Giftresistenz,
        HoheSeelenkraft,
        Immunitaetgegen(Gift),
        Immunitaetgegen(Krankheit),
        Krankheitsresistenz,
        Mystiker,
        Pragmatiker,
        SozialeAnpassungsfaehigkeit,
        Vertrauenerweckend
    ],
    vec![],
    vec![Reich],
    vec![
        Blutrausch,
        Giftanfaellig,
        Krankheitsanfaellig,
        SchlechteEigenschaften(Goldgier)
    ],
    vec![]
);

//Phexgeweihter
profession!(
    Phexgeweihter,
    "Phexgeweihter",
    304,
    vec![
        Vorteil::Geweihter(25AP),
        Nachteil::PrinzipientreueI(Phexkirche)(-10AP),
        VerpflichtungenII(Tempel, Kirche)(-20AP),
        Sonderfertigkeit::Tradition(Phexkirche)(150AP)
    ],
    vec![
        SprachenSchriften8,
        FertigkeitsspezialisierungGoetterundKulte
    ],
    vec![Dolche10],
    vec![],
    vec![],
    vec![
        DieZwoelfSegnungen,
        FallinsNichts5,
        Lautlos6,
        Mondsicht5,
        Objektsegen5,
        Wieselflink8
    ],
    vec![
        Beidhaendig,
        Entfernungssinn,
        Flink,
        Glueck,
        Mystiker,
        Pragmatiker,
        Unscheinbar,
        Zwergennase
    ],
    vec![SchlechteEigenschaften(Neugier, Goldgier)],
    vec![],
    vec![Blutrausch, Nachtblind, Pech],
    vec![]
);

//Praiosgeweihte
profession!(
    Praiosgeweihte,
    "Praiosgeweihte",
    242,
    vec![
        Vorteil::Geweihter(25),
        Nachteil::PrinzipientreueII(Praioskirche)(20),
        VerpflichtungenII(Tempel, Kirche)(20),
        Sonderfertigkeit::Tradition(Praioskirche)(130)
    ],
    vec![
        SprachenundSchriftenfuerinsgesamt10Abenteuerpunkte,
        FertigkeitsspezialisierungGoetterundKulte
    ],
    vec![Hiebwaffen8],
    vec![],
    vec![],
    vec![
        DieZwoelfSegnungen,
        BannderDunkelheit4,
        Blendstrahl6,
        Magieschutz6,
        Objektsegen5,
        Wahrheit3
    ],
    vec![
        HoheSeelenkraft,
        Mystiker,
        Pragmatiker,
        Schwerzuverzaubern,
        Vertrauenerweckend,
        Zeitgefuehl
    ],
    vec![Persoenlichkeitsschwaechen(Arroganz, Eitelkeit)],
    vec![Zauberer],
    vec![Blutrausch],
    vec![]
);

//Rabengardistin
profession!(
    Rabengardistin,
    "Rabengardistin",
    337,
    vec![
        KO13(fuerBelastungsgewoehnungI),
        KK13(fuerWuhctschlagI),
        Nachteil::PrinzipientreueII(Ordensregeln, Boronkirche)(20),
        VerpflichtungenII(Orden)(20)
    ],
    vec![
        SprachenundSchriftenfuerinsgesamt8Abenteuerpunkte,
        BelastungsgewoehnungI,
        WuchtschlagI
    ],
    vec![Dolche12, Hiebwaffen12, Raufen10, Schilde12],
    vec![],
    vec![],
    vec![],
    vec![
        HerausragendeKampftechnik,
        HoheLebenskraft,
        HoheSeelenkraft,
        HoheZaehigkeit,
        Reich,
        Vertrauenerweckend,
        Waffenbegabung,
        ZaeherHund
    ],
    vec![Persoenlichkeitsschwaechen(Arroganz, Eitelkeit)],
    vec![keine],
    vec![Behaebig, Unfaehig(Koerpertalente), Unfrei, Zerbrechlich],
    vec![]
);

//Rahjageweihte
profession!(
    Rahjageweihte,
    "Rahjageweihte",
    303,
    vec![
        Vorteil::Geweihter(25),
        Nachteil::PrinzipientreueI(Rahjakirche)(10),
        VerpflichtungenII(Tempel, Kirche)(20),
        Sonderfertigkeit::Tradition(Rahjakirche)(125)
    ],
    vec![
        SprachenundSchriftenfuerinsgesamt8Abenteuerpunkte,
        FertigkeitsspezialisierungGoetterundKulte
    ],
    vec![Raufen8],
    vec![],
    vec![],
    vec![
        DieZwoelfSegnungen,
        BegnadeterReiter5,
        BerauschenderWein6,
        FestderFreude5,
        FriedvollerRausch6,
        HeiligesLiebesspiel6,
        Objektsegen5
    ],
    vec![
        AngenehmerGeruch,
        Gutaussehend,
        HerausragenderSinn,
        HoheSeelenkraft,
        HoheZaehigkeit,
        Mystiker,
        Pragmatiker,
        Wohlklang,
        Vertrauenerweckend
    ],
    vec![
        Persoenlichkeitsschwaechen(Eitelkeit, Verwoehnt),
        SchlechteEigenschaft(Neugier),
        Zerbrechlich
    ],
    vec![keine],
    vec![
        Blutrausch,
        EingeschraenkterSinn,
        Farbenblind,
        Haesslich,
        Krankheitsanfaellig,
        Sprachfehler,
        Verstuemmelt
    ],
    vec![]
);

//Rahjakavalier
profession!(
    Rahjakavalier,
    "Rahjakavalier",
    253,
    vec![
        GE13(fuerFinteI),
        Nachteil::PrinzipientreueII(Ordensregeln, Rahjakirche)(20),
        VerpflichtungenII(Orden)(20)
    ],
    vec![SprachenundSchriftenfuerinsgesamt8Abenteuerpunkte, FinteI],
    vec![
        Dolche12,
        Raufen10,
        einederfolgendenKampftechniken12Fechtwaffen,
        Schwerter
    ],
    vec![],
    vec![],
    vec![],
    vec![
        HerausragendeKampftechnik,
        HoheLebenskraft,
        HoheZaehigkeit,
        Vertrauenerweckend,
        Waffenbegabung
    ],
    vec![Persoenlichkeitsschwaechen(Arroganz, Eitelkeit)],
    vec![keine],
    vec![Haesslich, Unfaehig(Reiten)],
    vec![]
);

//Rhodensteinerin
profession!(
    Rhodensteinerin,
    "Rhodensteinerin",
    425,
    vec![
        GE13(fuerFinteI),
        KO13(fuerBelastungsgewoehnungI),
        Vorteil::Geweihter(25),
        Nachteil::PrinzipientreueII(Ordensregeln, Rondrakirche)(20),
        VerpflichtungenII(Tempel, Orden)(20)
    ],
    vec![
        SprachenundSchriftenfuerinsgesamt12Abenteuerpunkte,
        BelastungsgewoehnungI,
        FinteI,
        FertigkeitsspezialisierungGoetterundKulte
    ],
    vec![
        Dolche10,
        Hiebwaffen11,
        Raufen11,
        Schwerter12,
        Zweihandschwerter10
    ],
    vec![],
    vec![],
    vec![
        DieZwoelfSegnungen,
        GoettlicheKlinge6,
        Heldenkraft6,
        Opfergang4,
        WeihedesHeims6
    ],
    vec![
        HerausragendeKampftechnik,
        HoheLebenskraft,
        HoheSeelenkraft,
        HoheZaehigkeit,
        Mystiker,
        Reich,
        Vertrauenerweckend,
        Waffenbegabung,
        ZaeherHund
    ],
    vec![Persoenlichkeitsschwaechen(Arroganz, Eitelkeit)],
    vec![keine],
    vec![Behaebig, Unfaehig(Koerpertalente), Unfrei, Zerbrechlich],
    vec![]
);

//Rikaipriester
profession!(
    Rikaipriester,
    "Rikaipriester",
    203,
    vec![
        Spezies::HalborkoderOrk,
        Kultur(OrklandoderSvellttal),
        Nachteil::PrinzipientreueI(Rikaikult)(10),
        VerpflichtungenII(Stammesregeln)(20)
    ],
    vec![
        Sprachenfuerinsgesamt4Abenteuerpunkte,
        FertigkeitsspezialisierungHeilkundeWunden
    ],
    vec![Dolche8, Hiebwaffen8],
    vec![
        KoerperSelbstbeherrschung6,
        Sinnesschaerfe4GesellschaftEtikette4,
        Menschenkenntnis5,
        Überreden4NaturFesseln4,
        Pflanzenkunde6,
        Tierkunde6WissenGoetterundKulte4,
        Rechnen5,
        Rechtskunde3,
        SagenundLegenden2HandwerkHeilkundeGift5,
        HeilkundeKrankheiten6,
        HeilkundeWunden7
    ],
    vec![],
    vec![],
    vec![
        Giftresistenz,
        Immunitaetgegen(Gift),
        Immunitaetgegen(Krankheit),
        Krankheitsresistenz
    ],
    vec![],
    vec![keine],
    vec![
        Angstvor(Blut, TotenundUntoten),
        Giftanfaellig,
        Krankheitsanfaellig
    ],
    vec![]
);

//Rondrageweihter
profession!(
    Rondrageweihter,
    "Rondrageweihter",
    283,
    vec![
        Vorteil::Geweihter(25),
        Nachteil::PrinzipientreueII(Rondrakirche)(20),
        VerpflichtungenII(Tempel, Kirche)(20),
        Sonderfertigkeit::Tradition(Rondrakirche)(150)
    ],
    vec![
        SprachenundSchriftenfuerinsgesamt8Abenteuerpunkte,
        FertigkeitsspezialisierungGoetterundKulte
    ],
    vec![
        Raufen10,
        Schwerter12,
        Zweihandschwerter10,
        eineKampftechnikausfolgenderListe10Fechtwaffen,
        Hiebwaffen,
        Kettenwaffen,
        Zweihandhiebwaffen
    ],
    vec![],
    vec![],
    vec![
        DieZwoelfSegnungen,
        Ehrenhaftigkeit6,
        Objektsegen5,
        Schmerzresistenz6,
        SchutzderWehrlosen4
    ],
    vec![
        HoheLebenskraft,
        HerausragendeKampftechnik,
        HoheSeelenkraft,
        HoheZaehigkeit,
        Mystiker,
        Pragmatiker,
        Reich,
        Vertrauenerweckend,
        Waffenbegabung,
        ZaeherHund
    ],
    vec![Persoenlichkeitsschwaechen(Arroganz, Eitelkeit)],
    vec![keine],
    vec![Angstvor, Behaebig, UnfaehiginKoerpertalenten, Zerbrechlich],
    vec![]
);

//Rur-und-Gror-Priesterin
profession!(
    RurundGrorPriesterin,
    "Rur-und-Gror-Priesterin",
    227,
    vec![
        Nachteil::PrinzipientreueII(RurundGrorKirche)(20),
        VerpflichtungenII(Tempel)(20)
    ],
    vec![SprachenundSchriftenfuerinsgesamt10Abenteuerpunkte],
    vec![Diskusse10, Raufen10, Stangenwaffen10],
    vec![],
    vec![],
    vec![],
    vec![HoheSeelenkraft, Vertrauenerweckend],
    vec![Persoenlichkeitsschwaechen(Arroganz, Eitelkeit)],
    vec![keine],
    vec![keine],
    vec![]
);

//Skuldrun (Fjarningerschamanin)
profession!(
    Fjarningerschamanin,
    "Skuldrun (Fjarningerschamanin)",
    313,
    vec![
        Kultur(Fjarninger),
        Vorteil::Geweihter(25),
        Nachteil::PrinzipientreueI(Stammesregeln)(10),
        VerpflichtungenII(Stamm)(20),
        Sonderfertigkeit::Tradition(Fjarningerschamanen)(100)
    ],
    vec![SprachenundSchriftenfuerinsgesamt4Abenteuerpunkte],
    vec![Hiebwaffen11, Raufen10, Stangenwaffen10, Wurfwaffen8],
    vec![],
    vec![],
    vec![
        Bannzone5,
        Jagdglueck6,
        Magiesicht5,
        RatderAhnen7,
        Tiersprache6
    ],
    vec![HoheSeelenkraft, HoheZaehigkeit, Vertrauenerweckend],
    vec![Persoenlichkeitsschwaechen(Arroganz, Eitelkeit)],
    vec![keine],
    vec![Behaebig, Unfaehig(Koerpertalente), Unfrei, Zerbrechlich],
    vec![]
);

//Sonnenlegionärin
profession!(
    Sonnenlegionaerin,
    "Sonnenlegionärin",
    375,
    vec![
        KO13(fuerBelastungsgewoehnungI),
        KK13(fuerWuchtschlagI),
        Nachteil::PrinzipientreueII(Ordensregeln, Praioskirche)(20),
        VerpflichtungenII(Orden)(20)
    ],
    vec![
        SprachenundSchriftenfuerinsgesamt6Abenteuerpunkte,
        BelastungsgewoehnungI,
        WuchtschlagI
    ],
    vec![
        Hiebwaffen12,
        Raufen10,
        Schilde12,
        Schwerter12,
        Stangenwaffen12
    ],
    vec![],
    vec![],
    vec![],
    vec![
        HoheLebenskraft,
        HerausragendeKampftechnik,
        HoheSeelenkraft,
        HoheZaehigkeit,
        Reich,
        Vertrauenerweckend,
        Waffenbegabung,
        ZaeherHund
    ],
    vec![Persoenlichkeitsschwaechen(Arroganz, Eitelkeit)],
    vec![keine],
    vec![Behaebig, Unfaehig(Koerpertalente), Unfrei, Zerbrechlich],
    vec![]
);

//Swafnirgeweihte
profession!(
    Swafnirgeweihte,
    "Swafnirgeweihte",
    286,
    vec![
        Kultur(Thorwaler),
        Vorteil::Geweihter(25),
        Nachteil::PrinzipientreueII(Swafnirkirche)(20),
        VerpflichtungenII(Thorwal, Tempel, Gemeinschaft)(20),
        Sonderfertigkeit::Tradition(Swafnirkirche)(115)
    ],
    vec![
        SprachenundSchriftenfuerinsgesamt8Abenteuerpunkte,
        FertigkeitsspezialisierungGoetterundKulte,
        Gelaendekunde(Meereskundig)
    ],
    vec![Hiebwaffen10, Raufen8, Schilde10, Wurfwaffen8],
    vec![],
    vec![],
    vec![
        DieZwoelfSegnungen(außerFeuersegen, GeburtssegenundSpeisesegen),
        Besaenftigung6,
        BlutigerZorn6,
        Ermutigung6,
        HerrderMeere5,
        RufderHeimat5
    ],
    vec![
        HoheSeelenkraft,
        HoheZaehigkeit,
        Mystiker,
        Pragmatiker,
        Richtungssinn,
        Vertrauenerweckend,
        ZaeherHund
    ],
    vec![
        Persoenlichkeitsschwaechen(Vorurteile),
        SchlechteEigenschaft(Jaehzorn, Neugier)
    ],
    vec![Adel, SozialeAnpassungsfaehigkeit],
    vec![Angstvor(demMeer), Blutrausch, Unfrei],
    vec![]
);

//Säbeltänzerin
profession!(
    Saebeltaenzerin,
    "Säbeltänzerin",
    306,
    vec![
        GE13(fuerdieSFFinteI),
        Kultur(AranieroderMhanadistani),
        PrinzipientreueI(Rahjakirche)(20),
        VerpflichtungenII(Orden)(20)
    ],
    vec![
        Sprachenfuerinsgesamt4Abenteuerpunkte,
        FertigkeitsspezialisierungTanzen,
        FinteI
    ],
    vec![Raufen12, Schwerter12],
    vec![],
    vec![],
    vec![],
    vec![AngenehmerGeruch, Gutaussehend, Wohlklang],
    vec![
        Persoenlichkeitsschwaeche(Eitelkeit, Weltfremd),
        SchlechteEigenschaft(Goldgier, Naiv)
    ],
    vec![],
    vec![Blutrausch, Haesslich],
    vec![]
);

//Tahayaschamanin
profession!(
    Tahayaschamanin,
    "Tahayaschamanin",
    316,
    vec![
        Kultur(WaldmenschenoderUtulu),
        Vorteil::Geweihter(25),
        Nachteil::PrinzipientreueI(Stammesregeln)(10),
        VerpflichtungenII(Stamm)(20),
        Sonderfertigkeit::Tradition(Tahayaschamanen)(100)
    ],
    vec![SprachenundSchriftenfuerinsgesamt4Abenteuerpunkte],
    vec![Hiebwaffen11, Raufen10],
    vec![],
    vec![],
    vec![
        Bannzone6,
        HauchdesElements5,
        Jaguarruf5,
        RatderAhnen7,
        Tabuzone5
    ],
    vec![HoheSeelenkraft, HoheZaehigkeit, Vertrauenerweckend],
    vec![Persoenlichkeitsschwaechen(Arroganz, Eitelkeit)],
    vec![keine],
    vec![Behaebig, Unfaehig(Koerpertalente), Unfrei, Zerbrechlich],
    vec![]
);

//Tairachschamane
profession!(
    Tairachschamane,
    "Tairachschamane",
    318,
    vec![
        Spezies::HalborkoderOrk,
        Kultur(OrklandoderSvellttal),
        Vorteil::Geweihter(25),
        Nachteil::PrinzipientreueI(Stammesregeln)(10),
        VerpflichtungenII(Sippe)(20),
        Sonderfertigkeit::Tradition(Tairachkult)(135)
    ],
    vec![Sprachenfuerinsgesamt8Abenteuerpunkte],
    vec![Hiebwaffen11, Raufen10],
    vec![
        KoerperKoerperbeherrschung5,
        Kraftakt3,
        Selbstbeherrschung5,
        Sinnesschaerfe4GesellschaftBekehrenundÜberzeugen2,
        Etikette4,
        Menschenkenntnis4,
        Willenskraft4NaturOrientierung4,
        Pflanzenkunde4,
        Tierkunde4,
        Wildnisleben4WissenGeschichtswissen4,
        GoetterundKulte5,
        Rechnen2,
        Rechtskunde5,
        SagenundLegenden6HandwerkHeilkundeKrankheiten4,
        HeilkundeSeele4,
        HeilkundeWunden3
    ],
    vec![],
    vec![
        BefehldesSchamanen5,
        FurchteinfloeßendeTiergeister5,
        GnadedesVergessens3,
        HauchdesElements5,
        Kriegsfarben4,
        Magiesicht4,
        Untotenerhebung5
    ],
    vec![HoheSeelenkraft, HoheZaehigkeit, Vertrauenerweckend],
    vec![Persoenlichkeitsschwaechen(Arroganz, Eitelkeit)],
    vec![keine],
    vec![Behaebig, Unfaehig(Koerpertalente), Unfrei, Zerbrechlich],
    vec![]
);

//Therbûnit
profession!(
    Therbûnit,
    "Therbûnit",
    231,
    vec![
        Nachteil::PrinzipientreueI(Perainekirche, Ordensregel)(10),
        VerpflichtungenII(Orden, Kloster, Siechenhaus)(20)
    ],
    vec![
        SprachenundSchriftenfuerinsgesamt8Abenteuerpunkte,
        MeisterderImprovisation
    ],
    vec![Raufen8, Dolche8, Stangenwaffen8],
    vec![],
    vec![],
    vec![],
    vec![
        Begabung(HeilkundeGiftoderHeilkundeKrankheiten),
        Giftresistenz,
        HoheSeelenkraft,
        HerausragendeFertigkeit(HeilkundeGiftoderHeilkundeKrankheiten),
        Immunitaetgegen(Gift),
        Immunitaetgegen(Krankheit),
        Krankheitsresistenz,
        Pragmatiker,
        SozialeAnpassungsfaehigkeit,
        Vertrauenerweckend
    ],
    vec![keine],
    vec![Reich],
    vec![
        Angstvor(TotenundUntoten, Krankheiten),
        Blutrausch,
        Giftanfaellig,
        Krankheitsanfaellig,
        SchlechteEigenschaft(Goldgier, Rachsucht)
    ],
    vec![]
);

//Traviageweihter
profession!(
    Traviageweihter,
    "Traviageweihter",
    224,
    vec![
        Vorteil::Geweihter(25),
        Nachteil::PrinzipientreueII(Traviakirche)(20),
        VerpflichtungenII(Tempel, Kirche)(20),
        Sonderfertigkeit::Tradition(Traviakirche)(110)
    ],
    vec![
        SprachenundSchriftenfuerinsgesamt6Abenteuerpunkte,
        FertigkeitsspezialisierungGoetterundKulte
    ],
    vec![Hiebwaffen8, Raufen8],
    vec![],
    vec![],
    vec![
        DieZwoelfSegnungen,
        Friedfertigkeit5,
        FriedvolleAura6,
        HelfendeHand5,
        SegnungdesHeims6,
        Speisung5,
        WeihedesHeims5
    ],
    vec![HoheSeelenkraft, Mystiker, Pragmatiker, Vertrauenerweckend],
    vec![Arm, SchlechteEigenschaft(Naiv)],
    vec![keine],
    vec![
        Persoenlichkeitsschwaechen(Eitelkeit, Streitsucht),
        Blutrausch,
        SchlechteEigenschaft(Geiz)
    ],
    vec![]
);

//Trollzackerschamanin
profession!(
    Trollzackerschamanin,
    "Trollzackerschamanin",
    332,
    vec![
        Kultur(Trollzacker),
        Vorteil::Geweihter(25),
        Nachteil::PrinzipientreueI(Stammesregeln)(10),
        VerpflichtungenII(Stamm)(20),
        Sonderfertigkeit::Tradition(Trollzackerschamanen)(100)
    ],
    vec![SprachenundSchriftenfuerinsgesamt6Abenteuerpunkte],
    vec![Hiebwaffen11, Raufen10, Zweihandhiebwaffen10],
    vec![],
    vec![],
    vec![
        Erdbeben5,
        Heldenkraft6,
        RatderAhnen7,
        Steinhaut5,
        Versteinerung4
    ],
    vec![HoheSeelenkraft, HoheZaehigkeit, Vertrauenerweckend],
    vec![Persoenlichkeitsschwaechen(Arroganz, Eitelkeit)],
    vec![keine],
    vec![Behaebig, Unfaehig(Koerpertalente), Unfrei, Zerbrechlich],
    vec![]
);

//Tsageweihte
profession!(
    Tsageweihte,
    "Tsageweihte",
    292,
    vec![
        Vorteil::Geweihter(25),
        Nachteil::PrinzipientreueII(Tsakirche)(20),
        VerpflichtungenII(Tempel, Kirche)(20),
        Sonderfertigkeit::Tradition(Tsakirche)(140)
    ],
    vec![
        SprachenundSchriftenfuerinsgesamt8Abenteuerpunkte,
        FertigkeitsspezialisierungGoetterundKulte,
        MeisterderImprovisation
    ],
    vec![],
    vec![],
    vec![],
    vec![
        DieZwoelfSegnungen,
        BefreiungdesGeistes6,
        Entfesselung6,
        FriedvolleAura5,
        Fruchtbarkeit5,
        Lebensschutz5,
        Motivation6
    ],
    vec![
        Altersresistenz,
        Glueck,
        HoheSeelenkraft,
        HoheZaehigkeit,
        Mystiker,
        Pragmatiker,
        SozialeAnpassungsfaehigkeit,
        Vertrauenerweckend
    ],
    vec![
        Persoenlichkeitsschwaechen(Weltfremd),
        SchlechteEigenschaft(Naiv, Neugier, Verschwendungssucht)
    ],
    vec![HerausragendeKampftechnik, Waffenbegabung],
    vec![
        Blutrausch,
        Persoenlichkeitsschwaechen(Streitsucht),
        SchlechteEigenschaft(Autoritaetsglaeubig, Rachsucht)
    ],
    vec![]
);

//Ucuriatin
profession!(
    Ucuriatin,
    "Ucuriatin",
    302,
    vec![
        Nachteil::PrinzipientreueII(Ordensregeln)(20),
        VerpflichtungenII(Orden)(20)
    ],
    vec![
        SprachenundSchriftenfuerinsgesamt12Abenteuerpunkte,
        FertigkeitsspezialisierungEtikette
    ],
    vec![Dolche10, Hiebwaffen10, Raufen10],
    vec![],
    vec![],
    vec![],
    vec![
        HoheLebenskraft,
        HoheSeelenkraft,
        HoheZaehigkeit,
        Reich,
        Vertrauenerweckend,
    ],
    vec![Persoenlichkeitsschwaechen(Arroganz, Eitelkeit)],
    vec![keine],
    vec![Behaebig, Unfaehig(Koerpertalente), Unfrei, Zerbrechlich],
    vec![]
);

//Yppolitanerin
profession!(
    Yppolitanerin,
    "Yppolitanerin",
    391,
    vec![
        GE13(fuerFinteI),
        KO13(fuerBelastungsgewoehnungI),
        KK13(fuerWuchtschlagI),
        Vorteil::Geweihter(25),
        Nachteil::PrinzipientreueII(Ordensregeln, Rondrakirche)(20),
        VerpflichtungenII(Tempel, Orden)(20),
        Sonderfertigkeit::Tradition(Rondrakirche)
    ],
    vec![
        SprachenundSchriftenfuerinsgesamt6Abenteuerpunkte,
        BelastungsgewoehnungI,
        FinteI,
        WuchtschlagI,
        FertigkeitsspezialisierungGoetterundKulte
    ],
    vec![Raufen12, Schwerter12, Zweihandschwerter12],
    vec![],
    vec![],
    vec![
        DieZwoelfSegnungen,
        GoettlicheKlinge5,
        Heldenkraft4,
        Opfergang6,
        WeihedesHeims6
    ],
    vec![
        HerausragendeKampftechnik,
        HoheLebenskraft,
        HoheSeelenkraft,
        HoheZaehigkeit,
        Mystiker,
        Reich,
        Vertrauenerweckend,
        Waffenbegabung,
        ZaeherHund
    ],
    vec![Persoenlichkeitsschwaechen(Arroganz, Eitelkeit)],
    vec![keine],
    vec![Behaebig, Unfaehig(Koerpertalente), Unfrei, Zerbrechlich],
    vec![]
);

//Zsahh-Priesterin
profession!(
    ZsahhPriesterin,
    "Zsahh-Priesterin",
    313,
    vec![
        Spezies::Achaz,
        Vorteil::Geweihter(25),
        Nachteil::{PrinzipientreueII(ZsaahKult)(20),
        VerpflichtungenII(Sippe)(20)},
        Sonderfertigkeit::Tradition(ZsahhKult)(140)
    ],
    vec![
        SprachenundSchriftenfuerinsgesamt2Abenteuerpunkte,
        FertigkeitsspezialisierungGoetterundKulte
    ],
    vec![Stangenwaffen8],
    vec![
        KoerperKoerperbeherrschung2,
        Schwimmen2,
        Selbstbeherrschung2,
        Sinnesschaerfe6,
        Verbergen3GesellschaftBekehrenundÜberzeugen4,
        Menschenkenntnis4,
        Überreden2,
        Willenskraft4NaturOrientierung2,
        Pflanzenkunde5,
        Tierkunde5,
        Wildnisleben2WissenGoetterundKulte6,
        SagenundLegenden5,
        Sphaerenkunde3HandwerkHeilkundeGift2,
        HeilkundeKrankheiten4,
        HeilkundeSeele4,
        HeilkundeWunden7,
        Lebensmittelbearbeitung2,
        MalenundZeichnen4,
        Musizieren2
    ],
    vec![],
    vec![
        Geburtssegen,
        Harmoniesegen,
        KleinerHeilsegen,
        KleinerSchutzsegen,
        Speisesegen,
        Tranksegen,
        Eidechsengestalt6,
        Eidechsenregeneration6,
        FriedvolleAura5,
        Fruchtbarkeit5,
        Lebensschutz5,
        Objektsegen6
    ],
    vec![
        Altersresistenz,
        Glueck,
        HoheSeelenkraft,
        HoheZaehigkeit,
        Mystiker,
        Pragmatiker,
        SozialeAnpassungsfaehigkeit,
        Vertrauenerweckend
    ],
    vec![
        Persoenlichkeitsschwaechen(Weltfremd),
        SchlechteEigenschaft(Naiv, Neugier, Verschwendungssucht)
    ],
    vec![HerausragendeKampftechnik, Waffenbegabung],
    vec![
        Blutrausch,
        Persoenlichkeitsschwaechen(Streitsucht),
        SchlechteEigenschaft(Autoritaetsglaeubig, Rachsucht)
    ],
    vec![]
);

//Zyklopäischer Avesgeweihter
profession!(
    ZyklopaeischerAvesgeweihter,
    "Zyklopäischer Avesgeweihter",
    1,
    vec![],
    vec![],
    vec![],
    vec![],
    vec![],
    vec![],
    vec![],
    vec![],
    vec![],
    vec![],
    vec![]
);
