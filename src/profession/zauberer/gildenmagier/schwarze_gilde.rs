use crate::profession;

//Schwarzmagier (Halle der Erleuchtung zu Al’Anfa)
profession!(
    HallederErleuchtungzuAlAnfa,
    "Schwarzmagier (Halle der Erleuchtung zu Al’Anfa)",
    345,
    vec![
        VorteilZauberer(25),
        SonderfertigkeitTradition(Gildenmagier)(155)
    ],
    vec![
        SprachenSchriften14,
        BindungdesStabes,
        ScholarderHallederErleuchtungzuAlAnfa
    ],
    vec![Raufen8, Stangenwaffen10],
    vec![
        KoerperKoerperbeherrschung3,
        Selbstbeherrschung4,
        Sinnesschaerfe2,
        Tanzen2GesellschaftBetoeren2,
        Etikette4,
        Menschenkenntnis4,
        Überreden2,
        Verkleiden2,
        Willenskraft5NaturPflanzenkunde3,
        Tierkunde2WissenBrett & Gluecksspiel2,
        Geographie2,
        Geschichtswissen3,
        Goetter & Kulte3,
        Kriegskunst4,
        Magiekunde5,
        Mechanik2,
        Rechnen4,
        Rechtskunde4,
        Sagen & Legenden3,
        Sphaerenkunde3,
        Sternkunde3HandwerkAlchimie5,
        Boote & Schiffe2,
        HeilkundeGift3,
        Malen & Zeichnen4
    ],
    vec![
        einZaubertrickausfolgenderListeAbkuehlung,
        Duft,
        Nagellack,
        Schminken,
        Signatur,
        Abvenenum4,
        Balsam6,
        BlickaufsWesenElfen4,
        Horriphobus7,
        Ignifaxius6,
        Ignisphaero4,
        InvocatioMinima4
    ],
    vec![],
    vec![HoheSeelenkraft, VerbesserteRegeneration(Astralenergie)],
    vec![
        Persoenlichkeitsschwaeche(Arroganz, Eitelkeit, Neid),
        SchlechteEigenschaft(Neugier, Goldgier),
        VerpflichtungenII(Gilde, Akademie, Lehrmeister, Grandenhaus, Imperium)
    ],
    vec![],
    vec![AngstvorWasser, Blutrausch, NiedrigeSeelenkraft],
    vec![]
);

//Schwarzmagier (Halle der Macht zu Lowangen)
profession!(
    HallederMachtzuLowangen,
    "Schwarzmagier (Halle der Macht zu Lowangen)",
    319,
    vec![
        VorteilZauberer(25),
        SonderfertigkeitTradition(Gildenmagier)(155)
    ],
    vec![
        SprachenSchriften10,
        BindungdesStabes,
        ScholarderHallederMachtzuLowangen
    ],
    vec![Dolche10, Raufen10, Stangenwaffen10],
    vec![
        KoerperSelbstbeherrschung5,
        Sinnesschaerfe4GesellschaftEtikette4,
        Menschenkenntnis5,
        Überreden4,
        Willenskraft5NaturOrientierung3WissenGeographie3,
        Geschichtswissen5,
        Goetter & Kulte4,
        Magiekunde6,
        Rechnen5,
        Rechtskunde3,
        Sagen & Legenden5HandwerkAlchimie3
    ],
    vec![
        einZaubertrickausfolgenderListeEiskalterBlick,
        Schnipsen,
        Signatur,
        BandundFessel4,
        Bannbaladin6,
        Einflussbann4,
        Gardianum4,
        Horriphobus6,
        Respondami6,
        Somnigravis4
    ],
    vec![],
    vec![HoheSeelenkraft, VerbesserteRegeneration(Astralenergie)],
    vec![
        Persoenlichkeitsschwaeche(Arroganz, Eitelkeit),
        SchlechteEigenschaft(Neugier),
        VerpflichtungenII(Gilde, Lehrmeister, Spektabilitaet)
    ],
    vec![],
    vec![Blutrausch, NiedrigeSeelenkraft],
    vec![]
);

//Schwarzmagier (Schüler des Demirion Ophenos)
profession!(
    SchuelerdesDemirionOphenos,
    "Schwarzmagier (Schüler des Demirion Ophenos)",
    295,
    vec![
        VorteilZauberer(25),
        SonderfertigkeitTradition(Gildenmagier)(155)
    ],
    vec![
        SprachenSchriften10,
        BindungdesStabes,
        ScholardesDemirionOphenos
    ],
    vec![Dolche8, Stangenwaffen8],
    vec![],
    vec![
        einZaubertrickausfolgenderListeBauchreden,
        Luftstoß,
        Regenbogenaugen,
        Schnipsen,
        UnheimlichesLachen,
        BandundFesselElfen4,
        Imperavi4,
        Incendio5,
        InvocatioMinima10,
        InvocatioMinor6,
        SchwarzerSchrecken6,
        Skelettarius4
    ],
    vec![],
    vec![HoheSeelenkraft, VerbesserteRegeneration(Astralenergie)],
    vec![
        KoerperlicheBesonderheit,
        Persoenlichkeitsschwaeche(Arroganz, Unheimlich),
        SchlechteEigenschaft(Neugier, Rachsucht),
        VerpflichtungenII(Lehrmeister)
    ],
    vec![Keine],
    vec![Angstvor(TotenundUntoten), NiedrigeSeelenkraft],
    vec![]
);

//Schwarzmagier nach Shanada von Ben-Oni
profession!(
    SchwarzmagiernachShanadavonBenOni,
    "Schwarzmagier nach Shanada von Ben-Oni",
    365,
    vec![
        VorteilZauberer(25),
        SonderfertigkeitTradition(Gildenmagier)(155)
    ],
    vec![
        SprachenSchriften8,
        BindungdesSchwerts,
        BindungdesStabes,
        ScholarderShanadavonBenOni
    ],
    vec![Armbrueste10, Schwerter10, Stangenwaffen10],
    vec![],
    vec![
        einZaubertrickausfolgenderListeBauchreden,
        Gluecksgriff,
        Kriesgbemalung,
        Schnipsen,
        Daemonenbann4,
        Daemonenschild5,
        Debilitatio6,
        InvocatioMinor5,
        InvocatioMinima10,
        Nuntiovolo4,
        Pentagramma5
    ],
    vec![],
    vec![
        AffinitaetzuDaemonen,
        HoheSeelenkraft,
        VerbesserteRegeneration(Astralenergie)
    ],
    vec![
        Persoenlichkeitsschwaeche(Arroganz, Eitelkeit),
        PrinzipientreueII(KampfgegenDaemonen),
        VerpflichtungenI(Lehrmeisterin)
    ],
    vec![],
    vec![Blutrausch, NiedrigeSeelenkraft, Zauberanfaellig],
    vec![]
);

//Schwarzmagierin (Akademie der Geistigen Kraft zu Fasar)
profession!(
    AkademiederGeistigenKraftzuFasar,
    "Schwarzmagierin (Akademie der Geistigen Kraft zu Fasar)",
    327,
    vec![
        VorteilZauberer(25),
        SonderfertigkeitTradition(Gildenmagier)(155)
    ],
    vec![
        SprachenSchriften12,
        BindungdesStabes,
        ScholarderAkademiederGeistigenKraftzuFasar
    ],
    vec![Dolche8, Stangenwaffen8],
    vec![],
    vec![
        einZaubertrickausfolgenderListeAbkuehlung,
        Duft,
        EhrfuerchtigesVerhalten,
        Schlangenhaende,
        Signatur,
        Bannbaladin6,
        BlickindieGedanken5,
        Corpofesso4,
        Horriphobus6,
        Odem4,
        Respondami5,
        SomnigravisElfen4
    ],
    vec![],
    vec![HoheSeelenkraft, VerbesserteRegeneration(Astralenergie)],
    vec![
        Persoenlichkeitsschwaeche(Arroganz, Eitelkeit),
        SchlechteEigenschaft(Goldgier, Neugier),
        VerpflichtungenII(Gilde, Akademie, Lehrmeister)
    ],
    vec![],
    vec![
        Blutrausch,
        NiedrigeSeelenkraft,
        SchlechteRegeneration(Astralenergie)
    ],
    vec![]
);

//Schwarzmagierin der Dunklen Halle der Geister zu Brabak
profession!(
    SchwarzmagierinderDunklenHallederGeisterzuBrabak,
    "Schwarzmagierin der Dunklen Halle der Geister zu Brabak",
    307,
    vec![
        VorteilZauberer(25),
        SonderfertigkeitTradition(Gildenmagier)(155)
    ],
    vec![
        SprachenSchriften12,
        BindungdesSchwerts,
        BindungdesStabes,
        ScholarderDunklenHallezuBrabak
    ],
    vec![Stangenwaffen8],
    vec![],
    vec![
        einZaubertrickausfolgenderListeBauchreden,
        Feenfueße,
        LaufendesHaendchen,
        Geistessenz10,
        Geisterruf6,
        Heptagramma4,
        Horriphobus5,
        Nekropathia5,
        Skelettarius6,
        Toteshandle5
    ],
    vec![],
    vec![
        HoheSeelenkraft,
        MagischeEinstimmung(WesenderNacht),
        VerbesserteRegeneration(Astralenergie)
    ],
    vec![
        Persoenlichkeitsschwaeche(Arroganz, Unheimlich),
        SchlechteEigenschaft(Neugier),
        VerpflichtungenII(Akademie)
    ],
    vec![],
    vec![Blutrausch, NiedrigeSeelenkraft],
    vec![]
);

//Schwarzmagierin der Schule der variablen Form zu Mirham
profession!(
    SchwarzmagierinderSchuledervariablenFormzuMirham,
    "Schwarzmagierin der Schule der variablen Form zu Mirham",
    330,
    vec![
        VorteilZauberer(25),
        SonderfertigkeitTradition(Gildenmagier)(155)
    ],
    vec![
        SprachenSchriften12,
        BindungdesStabes,
        ScholarderSchuledervariablenFormzuMirham
    ],
    vec![Raufen8, Stangenwaffen8],
    vec![],
    vec![
        einZaubertrickausfolgenderListeBauchreden,
        LockrufRegenbogenaugen,
        Adamantium4,
        Analys5,
        Arcanovi5,
        Desintegratus6,
        Hartesschmelze6,
        Objectofixo5,
        Weicheserstarre5
    ],
    vec![],
    vec![HoheSeelenkraft, VerbesserteRegeneration(Astralenergie)],
    vec![
        Prinzipientreue(Pazifismus),
        SchlechteEigenschaft(Neugier),
        VerpflichtungenII(Akademie)
    ],
    vec![Waffenbegabung],
    vec![Blutrausch, NiedrigeSeelenkraft],
    vec![]
);
