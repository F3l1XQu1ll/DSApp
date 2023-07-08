use crate::crate_prof;
use crate::profession;

crate_prof!();

//Bewahrerin
profession!(
    Bewahrerin,
    "Bewahrerin",
    294,
    vec![SpeziesElf(18), SonderfertigkeitTradition(Elf)(125)],
    vec![
        SprachenSchriften2,
        FertigkeitsspezialisierungHeilkundeWunden,
        Freundschaftslied4,
        Bewahrer
    ],
    vec![Boegen10, Dolche8],
    vec![],
    vec![
        einZaubertrickausfolgenderListeFeuerfinger,
        Handwaermer,
        Lockruf,
        Trocken,
        Armatrutz6,
        Aufwecken3,
        Balsam5,
        Odem3,
        RuheKoerper6,
        Visibili5
    ],
    vec![],
    vec![
        DunkelsichtII,
        Entfernungssinn,
        HoheSeelenkraft,
        Richtungssinn
    ],
    vec![Angstvor(engenRaeumen), SchlechteEigenschaften(Neugier)],
    vec![SozialeAnpassungsfaehigkeit],
    vec![Blutrausch],
    vec![]
);

//Former
profession!(
    Former,
    "Former",
    291,
    vec![SpeziesElf(18), SonderfertigkeitTradition(Elf)(125)],
    vec![
        SprachenSchriften2,
        FertigkeitsspezialisierungHolzbearbeitung,
        Freundschaftslied4,
        Former
    ],
    vec![Boegen10, Dolche8],
    vec![],
    vec![
        einZaubertrickausfolgenderListeFeuerfinger,
        Handwaermer,
        Lockruf,
        Trocken,
        Armatrutz6,
        Blitzdichfind5,
        Fulminictus3,
        Haselbusch6,
        Odem3,
        Silentium5
    ],
    vec![],
    vec![
        DunkelsichtII,
        Entfernungssinn,
        HoheSeelenkraft,
        Richtungssinn
    ],
    vec![Angstvor(engenRaeumen), SchlechteEigenschaften(Neugier)],
    vec![SozialeAnpassungsfaehigkeit],
    vec![Blutrausch],
    vec![]
);

//Kämpferin
profession!(
    Kaempferin,
    "Kämpferin",
    319,
    vec![SpeziesElf(18), SonderfertigkeitTradition(Elf)(125)],
    vec![
        SprachenSchriften2,
        FertigkeitsspezialisierungKriegskunst,
        Freundschaftslied4,
        Kaempfer
    ],
    vec![Boegen12, Dolche12, Fechtwaffen12, Raufen12],
    vec![],
    vec![
        einZaubertrickausfolgenderListeFeuerfinger,
        Handwaermer,
        Lockruf,
        Trocken,
        Armatrutz6,
        Blitzdichfind6,
        Fulminictus5,
        StandhafterWaechter3,
        Silentium4,
        ÜberlegenerKrieger3
    ],
    vec![],
    vec![
        DunkelsichtII,
        Entfernungssinn,
        HoheSeelenkraft,
        Richtungssinn
    ],
    vec![Angstvor(engenRaeumen), SchlechteEigenschaften(Neugier)],
    vec![SozialeAnpassungsfaehigkeit],
    vec![Blutrausch],
    vec![]
);

//Legendensänger
profession!(
    Legendensaenger,
    "Legendensänger",
    193,
    vec![SpeziesElf(18), SonderfertigkeitTradition(Elf)(125)],
    vec![
        SprachenSchriften2,
        FertigkeitsspezialisierungMusizierenoderSingen,
        Freundschaftslied4,
        Legendensaenger
    ],
    vec![Boegen10, Dolche10, Raufen8],
    vec![],
    vec![
        einZaubertrickausfolgenderListeFeuerfinger,
        Handwaermer,
        Hintergrundmusik,
        Lockruf,
        Trocken,
        BandundFessel3,
        Bannbaladin6,
        Blitzdichfind6,
        HilfreichePfote3,
        Silentium5,
    ],
    vec![],
    vec![
        DunkelsichtII,
        Entfernungssinn,
        HoheSeelenkraft,
        Richtungssinn
    ],
    vec![Angstvor(engenRaeumen), SchlechteEigenschaften(Neugier)],
    vec![SozialeAnpassungsfaehigkeit],
    vec![Blutrausch],
    vec![]
);

//Wildnisläuferin
profession!(
    Wildnislaeuferin,
    "Wildnisläuferin",
    256,
    vec![SpeziesElf(18), SonderfertigkeitTradition(Elf)(125)],
    vec![
        SprachenSchriften2,
        FertigkeitsspezialisierungFaehrtensuchen,
        Freundschaftslied4
    ],
    vec![Boegen12, Dolche8],
    vec![],
    vec![
        einZaubertrickausfolgenderListeFeuerfinger,
        Handwaermer,
        Lockruf,
        Trocken,
        Armatrutz6,
        Fulminictus5,
        Odem3,
        Silentium6,
        Visibili5,
        Wasseratem3
    ],
    vec![],
    vec![
        DunkelsichtII,
        Entfernungssinn,
        HoheSeelenkraft,
        Richtungssinn
    ],
    vec![Angstvor(engenRaeumen), SchlechteEigenschaften(Neugier)],
    vec![SozialeAnpassungsfaehigkeit],
    vec![Blutrausch],
    vec![]
);

//Zauberweber
profession!(
    Zauberweber,
    "Zauberweber",
    221,
    vec![SpeziesElf(18), SonderfertigkeitTradition(Elf)(125)],
    vec![
        SprachenSchriften4,
        FertigkeitsspezialisierungMusizieren,
        Zaubermelodie4
    ],
    vec![Boegen8],
    vec![],
    vec![
        zweiZaubertricksausfolgenderListeDuft,
        Feuerfinger,
        Lockruf,
        Signatur,
        Bannbaladin4,
        BlickindieGedanken5,
        Odem6,
        Silentium4,
        Visibili6,
        Wasseratem4
    ],
    vec![],
    vec![
        DunkelsichtII,
        Entfernungssinn,
        HoheSeelenkraft,
        Richtungssinn
    ],
    vec![
        Persoenlichkeitsschwaechen(Arroganz),
        SchlechteEigenschaften(Neugier),
        NiedrigeZaehigkeit
    ],
    vec![SozialeAnpassungsfaehigkeit],
    vec![Blutrausch],
    vec![]
);
