use crate::profession;

//Kristallomant
profession!(
    Kristallomant,
    "Kristallomant",
    300,
    vec![
        SpeziesAchaz,
        KulturAchazRha,
        VorteilZauberer(25),
        SonderfertigkeitTradition(Kristallomanten)(115)
    ],
    vec![
        SprachenSchriften6,
        FertigkeitsspezialisierungGoetter & Kulte,
        BindungdesSchuppenbeutels
    ],
    vec![Dolche8, Stangenwaffen8],
    vec![
        KoerperSelbstbeherrschung2,
        Sinnesschaerfe3GesellschaftBekehren & Überzeugen4,
        Etikette2,
        Menschenkenntnis2,
        Willenskraft6NaturPflanzenkunde4,
        Tierkunde3WissenGeographie4,
        Geschichtswissen5,
        Goetter & Kulte7,
        Magiekunde4,
        Mechanik4,
        Rechnen5,
        Rechtskunde3,
        Sagen & Legenden5,
        Sphaerenkunde5,
        Sternkunde4HandwerkAlchimie4,
        Steinbearbeitung5
    ],
    vec![
        einZaubertrickausfolgenderListeGluecksgriff,
        SprechendesKaestchen,
        WarmesBlut,
        BandundFessel3,
        Gifthaut3,
        Nebelwand4,
        Serpentialis5,
        TempusStasis5,
        UnberuehrtvonSatinav5,
        ZornderElemente4
    ],
    vec![],
    vec![
        HoheSeelenkraft,
        Mystiker,
        Pragmatiker,
        Vertrauenerweckend,
        Zeitgefuehl
    ],
    vec![SchlechteEigenschaften(Neugier)],
    vec![],
    vec![],
    vec![]
);
