use crate::crate_prof;
use crate::profession;

crate_prof!();

//Weißmagier (Halle des Lebens zu Norburg)
profession!(
    HalledesLebenszuNorburg,
    "Weißmagier (Halle des Lebens zu Norburg)",
    336,
    vec![
        VorteilZauberer(25),
        SonderfertigkeitTradition(Gildenmagier)(155)
    ],
    vec![
        SprachenSchriften10,
        BindungdesStabes,
        ScholarderHalledesLebenszuNorburg
    ],
    vec![],
    vec![],
    vec![
        einZaubertrickausfolgenderListeAbkuehlung,
        Feuerfinger,
        HeilsameBeruehrung,
        Ängstelindern5,
        Balsam5,
        BlickaufsWesen5,
        KlarumPurum5,
        Paralyssis4,
        RuheKoerper6,
        Verwandlungbeenden4
    ],
    vec![],
    vec![HoheSeelenkraft, VerbesserteRegeneration(Astralenergie)],
    vec![
        Prinzipientreue(HilfefuerKranke, Pazifismus),
        SchlechteEigenschaften(Neugier, Weltfremd),
        VerpflichtungenII(Akademie, Gilde,)
    ],
    vec![],
    vec![
        Angstvor(Dunkelheit),
        Blutrausch,
        Lichtempfindlich,
        Nachtblind,
        NiedrigeSeelenkraft,
        Persoenlichkeitsschwaechen(Arroganz, Eitelkeit),
        SchlechteEigenschaften(Goldgier),
        SchlechteRegeneration(Astralenergie)
    ],
    vec![]
);

//Weißmagier (Schule des magischen Wissens zu Methumis)
profession!(
    SchuledesmagischenWissenszuMethumis,
    "Weißmagier (Schule des magischen Wissens zu Methumis)",
    266,
    vec![
        VorteilZauberer(25),
        SonderfertigkeitTradition(Gildenmagier)(155)
    ],
    vec![
        SprachenSchriften14,
        BindungdesStabes.ScholarderSchuledesmagischenWissenszuMethumis
    ],
    vec![],
    vec![],
    vec![
        einZaubertrickausfolgenderListeAbkuehlung,
        AstralesLeuchten,
        Feuerfinger,
        Adlerauge5,
        Analys5,
        BlickaufsWesen5,
        Hellsichtbann5,
        Illusionsbann4,
        Penetrizzel6,
        Sensibar4
    ],
    vec![],
    vec![HoheSeelenkraft, VerbesserteRegeneration(Astralenergie)],
    vec![
        Prinzipientreue(Gesetzestreue),
        SchlechteEigenschaften(Neugier, Weltfremd),
        VerpflichtungenII(Akademie, Gilde, Horasreich)
    ],
    vec![],
    vec![
        Angstvor(Dunkelheit),
        Blutrausch,
        Lichtempfindlich,
        Nachtblind,
        NiedrigeSeelenkraft,
        Persoenlichkeitsschwaechen(Arroganz, Eitelkeit),
        SchlechteEigenschaften(Goldgier),
        SchlechteRegeneration(Astralenergie)
    ],
    vec![]
);

//Weißmagier (Schwert & Stab-Akademie zu Gareth)
profession!(
    SchwertundStabAkademiezuGareth,
    "Weißmagier (Schwert & Stab-Akademie zu Gareth)",
    331,
    vec![
        VorteilZauberer(25),
        SonderfertigkeitTradition(Gildenmagier)(155),
        NachteilPrinzipientreueII(
            SchutzdesMittelreichesundseinesAdels,
            GlaubeandieZwoelfgoetter,
            Wahrheitsliebe
        )(20)
    ],
    vec![
        SprachenSchriften10,
        BindungdesStabes,
        ScholarderAkademieSchwertundStabzuGareth
    ],
    vec![Dolche8, Schwerter10, Stangenwaffen10],
    vec![],
    vec![
        einZaubertrickausfolgenderListeAbkuehlung,
        Feuerfinger,
        Rohrstock,
        Schnipsen,
        Signatur,
        Trocken,
        Armatrutz5,
        Blitzdichfind6,
        FulminictusElfen5,
        Gardianum4,
        Ignifaxius6,
        Paralysis4,
        Psychostabilis4
    ],
    vec![],
    vec![HoheSeelenkraft, VerbesserteRegeneration(Astralenergie)],
    vec![
        Persoenlichkeitsschwaeche(Eitelkeit),
        SchlechteEigenschaft(Neugier),
        VerpflichtungenII(Gilde, Akademie, Lehrmeister, Mittelreich)
    ],
    vec![],
    vec![
        Blutrausch,
        NiedrigeSeelenkraft,
        SchlechteRegeneration(Astralenergie)
    ],
    vec![]
);

//Weißmagier (Schüler des Hesindius Lichtblick)
profession!(
    SchuelerdesHesindiusLichtblick,
    "Weißmagier (Schüler des Hesindius Lichtblick)",
    276,
    vec![
        VorteilZauberer(25),
        SonderfertigkeitTradition(Gildenmagier)(155),
        PrinzipientreueI(Hilfsbereitschaft, BannungdesBoesen)(10)
    ],
    vec![
        SprachenSchriften10,
        BindungdesStabes,
        ScholardesHesindiusLichtblick
    ],
    vec![Stangenwaffen8],
    vec![],
    vec![
        einZaubertrickausfolgenderListeBauchreden,
        Feuerfinger,
        Gluecksgriff,
        GutenMorgen,
        Handwaermer,
        Trocken,
        Armatrutz4,
        Balsam6,
        Gardianum4,
        Motoricus4,
        Penetrizzel5,
        Psychostabilis5,
        SomnigravisElfen6
    ],
    vec![],
    vec![HoheSeelenkraft, VerbesserteRegeneration(Astralenergie)],
    vec![
        Persoenlichkeitsschwaeche(Weltfremd),
        SchlechteEigenschaft(Autoritaetsglaube, Naiv, Neugier),
        VerpflichtungenII(Gilde, Lehrmeister, Horasreich)
    ],
    vec![],
    vec![
        Blutrausch,
        NiedrigeSeelenkraft,
        Persoenlichkeitsschwaeche(Arroganz, Eitelkeit, Streitsucht, Unheimlich),
        SchlechteEigenschaft(Jaehzorn, Rachsucht),
        SchlechteRegeneration(Astralenergie)
    ],
    vec![]
);

//Weißmagier des Informations-Instituts zu Rommilys
profession!(
    WeißmagierdesInformationsInstitutszuRommilys,
    "Weißmagier des Informations-Instituts zu Rommilys",
    328,
    vec![
        VorteilZauberer(25),
        SonderfertigkeitTradition(Gildenmagier)(155)
    ],
    vec![
        SprachenSchriften12,
        BindungdesStabes,
        ScholardesInformationsInstitutszuRommilys
    ],
    vec![Dolche8, Stangenwaffen8],
    vec![],
    vec![
        einZaubertrickausfolgenderListeBauchreden,
        Gluecksgriff,
        Signatur,
        UnauffaelligerStab,
        Analys5,
        BlickindieGedanken5,
        Einflussbann4,
        ObjectovocoElfen5,
        Penetrizzel4,
        Sensibar6,
        Xenographus4
    ],
    vec![],
    vec![
        Adlig,
        HoheSeelenkraft,
        SozialeAnpassungsfaehigkeit,
        VerbesserteRegeneration(Astralenergie),
        VerhuellteAura,
        Zeitgefuehl
    ],
    vec![
        Persoenlichkeitsschwaeche(Arroganz),
        PrinzipientreueII(Zwoelfgoetterglaube),
        VerpflichtungenII(Akademie, Kaiserreich)
    ],
    vec![],
    vec![
        Angstvor,
        Blutrausch,
        EingeschraenkterSinn,
        Farbenblind,
        MagischeEinschraenkung(FluchderFinsternis),
        NiedrigeSeelenkraft,
        SchlechteEigenschaft(Goldgier, Rachsucht),
        Stigma
    ],
    vec![]
);

//Weißmagierin (Akademie der Herrschaft zu Elenvina)
profession!(
    AkademiederHerrschaftzuElenvina,
    "Weißmagierin (Akademie der Herrschaft zu Elenvina)",
    1,
    vec![
        VorteilZauberer(25),
        SonderfertigkeitTradition(Gildenmagier)(155)
    ],
    vec![
        SprachenSchriften10,
        BindungdesStabes,
        ScholarderAkademiederHerrschaftzuElenvina
    ],
    vec![Dolche8, Raufen8, Stangenwaffen8],
    vec![],
    vec![
        einZaubertrickausfolgenderListeFeuerfinger,
        Signatur,
        Verneigung,
        BandundFessel4,
        Bannbaladin6,
        Einflussbann4,
        Horriphobus4,
        Ignifaxius4,
        Respondami6,
        Somnigravis6
    ],
    vec![],
    vec![HoheSeelenkraft, VerbesserteRegeneration(Astralenergie)],
    vec![
        Persoenlichkeitsschwaeche(Arroganz),
        SchlechteEigenschaft(Neugier),
        VerpflichtungenII(Gilde, Lehrmeister, Mittelreich)
    ],
    vec![],
    vec![Blutrausch, NiedrigeSeelenkraft],
    vec![]
);

//Weißmagierin (Akademie der magischen Rüstung zu Gareth)
profession!(
    AkademiedermagischenRuestungzuGareth,
    "Weißmagierin (Akademie der magischen Rüstung zu Gareth)",
    294,
    vec![
        VorteilZauberer(25),
        SonderfertigkeitTradition(Gildenmagier)(155)
    ],
    vec![
        SprachenSchriften14,
        BindungdesStabes,
        ScholarderAkademiedermagischenRuestungzuGareth
    ],
    vec![Dolche8, Raufen8, Stangenwaffen10],
    vec![],
    vec![
        einZaubertrickausfolgenderListeAbkuehlung,
        AuffaelligerStil,
        Armatrutz5,
        Daemonenschild5,
        Fulminictus4,
        Gardianum6,
        Illusionsbann4,
        Invercano5,
        Verwandlungsbann4
    ],
    vec![],
    vec![HoheSeelenkraft, VerbesserteRegeneration(Astralenergie)],
    vec![
        Persoenlichkeitsschwaechen(Arroganz, Eitelkeit),
        Prinzipientreue(SchutzdesMittelreiches),
        SchlechteEigenschaften(Neugier),
        VerpflichtungenII(Akademie, Gilde, Mittelreich)
    ],
    vec![],
    vec![
        Blutrausch,
        NiedrigeSeelenkraft,
        SchlechteRegeneration(Astralenergie)
    ],
    vec![]
);

//Weißmagierin (Akademie von Licht und Dunkelheit zu Nostria)
profession!(
    AkademievonLichtundDunkelheitzuNostria,
    "Weißmagierin (Akademie von Licht und Dunkelheit zu Nostria)",
    336,
    vec![
        VorteilZauberer(25),
        SonderfertigkeitTradition(Gildenmagier)(155)
    ],
    vec![
        SprachenSchriften10,
        BindungdesStabes,
        ScholarderAkademievonLichtundDunkelheitzuNostria
    ],
    vec![Dolche8, Raufen8, Stangenwaffen8],
    vec![],
    vec![
        einZaubertrickausfolgenderListeAbkuehlung,
        Feuerfinger,
        Leselampe,
        Analys5,
        Armatrutz5,
        Caldofrigo5,
        Corpofrigo5,
        Dunkelheit6,
        FlimFlam4,
        NebelwandElfen4
    ],
    vec![],
    vec![HoheSeelenkraft, VerbesserteRegeneration(Astralenergie)],
    vec![
        SchlechteEigenschaften(Neugier, Weltfremd),
        VerpflichtungenII(
            Akademie,
            Gilde,
            KoenigreichNostriaoderOrdenderSchlangederErkenntnis
        )
    ],
    vec![],
    vec![
        Angstvor(Dunkelheit),
        Blutrausch,
        Lichtempfindlich,
        Nachtblind,
        NiedrigeSeelenkraft,
        Persoenlichkeitsschwaechen(Arroganz, Eitelkeit),
        SchlechteEigenschaften(Goldgier),
        SchlechteRegeneration(Astralenergie)
    ],
    vec![]
);

//Weißmagierin (Schule der Austreibung zu Perricum)
profession!(
    SchulederAustreibungzuPerricum,
    "Weißmagierin (Schule der Austreibung zu Perricum)",
    333,
    vec![
        VorteilZauberer(25),
        SonderfertigkeitTradition(Gildenmagier)(155)
    ],
    vec![
        SprachenSchriften10,
        BindungdesBannschwerts(klein),
        BindungdesStabes,
        ScholarderSchulederAustreibungzuPerricum
    ],
    vec![Dolche10, Raufen10, Stangenwaffen8],
    vec![],
    vec![
        einZaubertrickausfolgenderListeAbkuehlung,
        Beruhigung,
        Gluecksgriff,
        Signatur,
        Trocken,
        ÄngstelindernHexen4,
        Einflussbann5,
        Heptagramma5,
        Illusionsbann5,
        InvocatioMinima4,
        Pentagramma7,
        Verwandlungsbann5
    ],
    vec![],
    vec![HoheSeelenkraft, VerbesserteRegeneration(Astralenergie)],
    vec![
        Persoenlichkeitsschwaeche(Arroganz),
        SchlechteEigenschaft(Neugier),
        Verpflichtungen(Gilde, Akademie, LehrmeisterOrdenderNoioniten, Mittelreich)
    ],
    vec![],
    vec![
        Angstvor(TotenundUntoten),
        Blutrausch,
        Persoenlichkeitsschwaeche(Streitsucht)
    ],
    vec![]
);

//Weißmagierin der Anatomischen Akademie zu Vinsalt
profession!(
    WeißmagierinderAnatomischenAkademiezuVinsalt,
    "Weißmagierin der Anatomischen Akademie zu Vinsalt",
    345,
    vec![
        VorteilZauberer(25),
        SonderfertigkeitTradition(Gildenmagier)(155)
    ],
    vec![
        SprachenSchriften8,
        BindungdesStabes,
        ScholarderAnatomischenAkademiezuVinsalt
    ],
    vec![Stangenwaffen8],
    vec![],
    vec![
        einZaubertrickausfolgenderListeDuft,
        Koerperpflege,
        Signatur,
        Taetowierung,
        Ängstelindern4,
        Balsam6,
        Heptagramma5,
        KlarumPurum6,
        Psychostabilis4,
        RuheKoerper5,
        Sensibar5
    ],
    vec![],
    vec![
        HoheSeelenkraft,
        Krankheitsresistenz,
        VerbesserteRegeneration(Astralenergie)
    ],
    vec![
        Persoenlichkeitsschwaeche(Arroganz, Eitelkeit),
        PrinzipientreueII(Seuchenbekaempfung),
        VerpflichtungenII(Akademie)
    ],
    vec![],
    vec![
        Angstvor(Blut, Tote & Untote),
        Blutrausch,
        NiedrigeSeelenkraft
    ],
    vec![]
);

//Weißmagierin der Halle der Metamorphosen zu Kuslik
profession!(
    WeißmagierinderHallederMetamorphosenzuKuslik,
    "Weißmagierin der Halle der Metamorphosen zu Kuslik",
    291,
    vec![
        VorteilZauberer(25),
        NachteilPrinzipientreueII(Hesindekirche),
        SonderfertigkeitTradition(Gildenmagier)(155)
    ],
    vec![
        SprachenSchriften12,
        BindungdesStabes,
        ScholarderHallederMetamorphosenzuKuslik
    ],
    vec![Dolche8, Fechtwaffen8, Stangenwaffen8],
    vec![],
    vec![
        einZaubertrickausfolgenderListeEinfacheTelekinese,
        Feenfueße,
        Schlangenhaende,
        Signatur,
        Adlerschwinge5,
        AttributoKlugheit6,
        Memorans4,
        Psychostabilis5,
        Salander6,
        Verwandlungsbann5,
        VisibiliElfen4
    ],
    vec![],
    vec![HoheSeelenkraft, VerbesserteRegeneration(Astralenergie)],
    vec![
        Persoenlichkeitsschwaeche(Arroganz),
        VerpflichtungenII(Akademie)
    ],
    vec![],
    vec![Blutrausch, NiedrigeSeelenkraft],
    vec![]
);
