use crate::crate_prof;
use crate::profession;

crate_prof!();

//Gildenlose Magierin (Schüler der Kiranya von Kutaki)
profession!( 
    SchuelerderKiranyavonKutaki,
    "Gildenlose Magierin (Schüler der Kiranya von Kutaki)",
    362,
    vec![VorteilZauberer(25),
		SonderfertigkeitTradition(Gildenmagier)(155)],
    vec![SprachenSchriften12,
		BindungdesStabes,
		ScholarderKiranyavonKutaki],
    vec![Raufen8,
		Stangenwaffen8],
    vec![],
    vec![einZaubertrickausfolgenderListeBlickindenLimbus,
		Feuerfinger,
		Signatur,
        BlickindieGedanken4,
		Analys6,
		Gardianum4,
		Invercano4,
		OculusAstralis6,
		Odem4,
		Transversalis7],
    vec![],
    vec![HoheSeelenkraft,
		VerbesserteRegeneration(Astralenergie)],
    vec![Persoenlichkeitsschwaeche(Arroganz,
		Eitelkeit,
		Neid),
		SchlechteEigenschaft(Neugier,
		Goldgier),
		VerpflichtungenII(Lehrmeisterin)],
    vec![],
    vec![AngstvorWasser,
		Blutrausch,
		NiedrigeSeelenkraft],
    vec![]
);

//Gildenlose Magierin (Schülerin der Khelbara ay Baburia)
profession!( 
    SchuelerinderKhelbaraayBaburia,
    "Gildenlose Magierin (Schülerin der Khelbara ay Baburia)",
    353,
    vec![VorteilZauberer(25),
		SonderfertigkeitTradition(Gildenmagier)(155)],
    vec![SprachenSchriften12,
		BindungdesStabes,
		ScholarderKhelbaraayBaburia],
    vec![Dolche8,
		Raufen8,
		Stangenwaffen8],
    vec![KoerperSelbstbeherrschung4,
		Sinnesschaerfe3,
		Verbergen3,
		Zechen1GesellschaftBetoeren3,
		Etikette2,
		Gassenwissen2,
		Menschenkenntnis3,
		Überreden2,
		Verkleiden4,
		Willenskraft4NaturFesseln2,
		Pflanzenkunde2,
		Tierkunde2WissenGeographie2,
		Geschichtswissen5,
		Goetter&Kulte2,
		Magiekunde4,
		Mechanik2,
		Rechnen5,
		Rechtskunde2,
		Sagen&Legenden3,
		Sphaerenkunde4,
		Sternkunde4HandwerkAlchimie3,
		HeilkundeGift4,
		HeilkundeKrankheiten5,
		HeilkundeSeele3,
		HeilkundeWunden6,
		Malen&Zeichnen4],
    vec![EinZaubertrickausfolgenderListeAranischeRasur,
		Duft,
		Regenbogenaugen,
		Schminken,
		Signatur,Balsam6,
		Horriphobus6,
		InvocatioMinima4,
		Leidensbund5,
		RuheKoerper5,
		Schmerzenlindern5,
		VisibiliElfen5],
    vec![],
    vec![Giftresistenz,
		HoheZaehigkeit,
		Immunitaetgegen(Gift),
		Immunitaetgegen(Krankheit),
		Krankheitsresistenz],
    vec![Persoenlichkeitsschwaeche(Arroganz,
		Eitelkeit,
		Neid),
		Prinzipientreue(Friedfertigkeit,
		vegetarischeLebensweise),
		SchlechteEigenschaft(Neugier,
		Goldgier,
		Verschwendungssucht),
		VerpflichtungenII(Lehrmeisterin)],
    vec![Adel],
    vec![Blutrausch,
		Krankheitsanfaellig],
    vec![]
);

//Gildenlose Magierin (Schülerin des Rafim Bey)
profession!( 
    SchuelerindesRafimBey,
    "Gildenlose Magierin (Schülerin des Rafim Bey)",
    296,
    vec![VorteilZauberer(25),
		SonderfertigkeitTradition(Gildenmagier)(155)],
    vec![SprachenSchriften8,
		BindungdesStabes,
		ScholardesRafimBey],
    vec![],
    vec![],
    vec![einZaubertrickausfolgenderListeDaemonling,
		Feuerfinger,
		Signatur,
        Abvenenum3,
		Bannbaladin4,
		ElementarerDiener6,
		Gardianum4,
		InvocatioMinor6,
		InvovatioMinima10,
		Manifesto10],
    vec![],
    vec![HoheSeelenkraft,
		VerbesserteRegeneration(Astralenergie)],
    vec![Persoenlichkeitsschwaeche(Arroganz,
		Eitelkeit,
		Verwoehnt),
		SchlechteEigenschaft(Neugier),
		VerpflichtungenII(Lehrmeister)],
    vec![],
    vec![Blutrausch,
		NiedrigeSeelenkraft],
    vec![]
);

//Gildenlose Magierin der Heptagonakademie zu Yol-Ghurmak
profession!( 
    GildenloseMagierinderHeptagonakademiezuYolGhurmak,
    "Gildenlose Magierin der Heptagonakademie zu Yol-Ghurmak",
    369VoraussetzungenVorteilZauberer(25),SonderfertigkeitTradition(Gildenmagier)(155)SonderfertigkeitenSprachenundSchriftenfürinsgesamt8,BindungdesStabes,ScholarderHeptagonakademiezuYol-GhurmakKampftechnikenDolche10,Raufen8,Stangenwaffen10TalenteKörperSelbstbeherrschung4,Sinnesschärfe4GesellschaftEinschüchtern3,Menschenkenntnis5, Überreden4,Willenskraft6Natur-WissenGeschichtswissen3,Magiekunde5,Rechnen4,Sagen&Legenden4,Sphärenkunde7,Sternkunde5HandwerkAlchimie5,HeilkundeGift3,HeilkundeWunden3,Malen&Zeichnen6Zauber/MagischeHandlungeneinZaubertrickausfolgenderListeAurader Eitelkeit,Feuerfinger,Schlangenhände,Tätowierung,ZauberDämonischesVergessen4,Ecliptifactus5, HerzschlagRuhe6,InvocatioMinima10,Invocatio Minor6,Pandaemonium(Hexen)4,Steinwandle5EmpfohleneVorteileAffinitätzuDämonen, HoheSeelenkraft,VerbesserteRegeneration (Astralenergie)EmpfohleneNachteilePersönlichkeitsschwäche(Arroganz,Unheimlich),VerpflichtungenII(Akademie)UngeeigneteVorteileVertrauenerweckendUngeeigneteNachteileBlutrausch,MagischeEinschränkung(FluchderHelligkeit),NiedrigeSeelenkraft,Prinzipientreue,
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

//Gildenlose Magierin des Magierkollegs zu Honingen
profession!( 
    GildenloseMagierindesMagierkollegszuHoningen,
    "Gildenlose Magierin des Magierkollegs zu Honingen",
    266VoraussetzungenVorteilZauberer(25),SonderfertigkeitTradition(Gildenmagier)(155)SonderfertigkeitenSprachenundSchriftenfürinsgesamt12,BindungdesStabes, ScholardesMagierkollegszuHoningenKampftechnikenStangenwaffen10TalenteKörper-GesellschaftEtikette2,Menschenkenntnis4,Willenskraft5NaturOrientierung2WissenGeographie3,Geschichtswissen3,Götter&Kulte4,Magiekunde6,Rechnen4,Sagen&Legenden5, Sphärenkunde5HandwerkAlchimie3Zauber/MagischeHandlungeneinZaubertrickausfolgenderListeBauchreden,Feuerfinger,Glücksgriff,Signatur,ÜbergangenZauber Analys6,Balsam5,Fulminictus(Elfen)4,Horriphobus4,Motoricus4,Paralysis5,Transversalis6EmpfohleneVorteileHoheSeelenkraft,Verbesserte Regeneration(Astralenergie)EmpfohleneNachteilePersönlichkeitsschwäche (Weltfremd),SchlechteEigenschaft(Neugier),VerpflichtungenI(Lehrmeister)UngeeigneteVorteileUngeeigneteNachteileBlutrausch,NiedrigeSeelenkraft,SchlechteRegeneration(Astralenergie),
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

//Gildenlose Magierin nach Halib abu’l Ketab
profession!( 
    GildenloseMagierinnachHalibabulKetab,
    "Gildenlose Magierin nach Halib abu’l Ketab",
    240VoraussetzungenVorteilZauberer(25),SonderfertigkeitTradition(Gildenmagier)(155)SonderfertigkeitenSprachenundSchriftenfürinsgesamt8,BindungdesStabes, ScholardesHalibabu‘lKetabKampftechnikenStangenwaffen10TalenteKörper-GesellschaftMenschenkenntnis3,Willenskraft4NaturOrientierung3WissenGeschichtswissen4,Götter&Kulte4,Magiekunde6,Rechnen4,Sagen&Legenden5,Sphärenkunde 3,Sternkunde3HandwerkAlchimie3,Malen&Zeichnen5Zauber/MagischeHandlungeneinZaubertrickausfolgenderListeFeuerfinger,Feuerspiel,Feenfüße,HandwärmerZauberElementarer Diener4,Einflussbann5,FlimFlam6,Ignifaxius6, Ignisphaero4,Manifesto10,Visibili(Elfen)4EmpfohleneVorteileAffinitätzuElementaren, HoheSeelenkraft,VerbesserteRegeneration (Astralenergie)EmpfohleneNachteilePersönlichkeitsschwäche(Arroganz),SchlechteEigenschaft(AberglaubeZahlenmystik),VerpflichtungenI(Lehrmeister) UngeeigneteVorteileUngeeigneteNachteileAdel,Blutrausch,Niedrige Seelenkraft,Reich,
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

//Gildenloser Magier (Honinger Collegium)
profession!( 
    HoningerCollegium,
    "Gildenloser Magier (Honinger Collegium)",
    256,
    vec![VorteilZauberer(25),
		SonderfertigkeitTradition(Gildenmagier)(155)],
    vec![SprachenSchriften12,
		BindungdesStabes],
    vec![Stangenwaffen10],
    vec![],
    vec![einZaubertrickausfolgenderListeBauchreden,
		Feuerfinger,
		Gluecksgriff,
		Handwaermer,
		Signatur,
		Analys6,
		Balsam5,
		FulminictusElfen4,
		Horriphobus4,
		Motoricus4,
		Paralysis5,
		Transversalis6],
    vec![],
    vec![HoheSeelenkraft,
		VerbesserteRegeneration(Astralenergie)],
    vec![Persoenlichkeitsschwaeche(Weltfremd),
		SchlechteEigenschaft(Neugier),
		VerpflichtungenI(Lehrmeister)],
    vec![],
    vec![Blutrausch,
		NiedrigeSeelenkraft,
		SchlechteRegeneration(Astralenergie)],
    vec![]
);

//Gildenloser Magier (Kreis der Einfühlung)
profession!( 
    KreisderEinfuehlung,
    "Gildenloser Magier (Kreis der Einfühlung)",
    305,
    vec![VorteilZauberer(25),
		SonderfertigkeitTradition(Gildenmagier)(155)],
    vec![SprachenSchriften8,
		BindungdesStabes,
		ScholardesKreisesderEinfuehlung],
    vec![],
    vec![],
    vec![einZaubertrickausfolgenderListeDuft,
		Feuerfinger,
		Grußworte,
		Verbundenheitsgefuehl,
        Balsam3,
		Bannbaladin4,
		BlickindieGedanken6,
		Gedankenbilder4,
		Odem4,
		Sensibar4,
		Tiergedanken4],
    vec![],
    vec![HoheSeelenkraft,
		VerbesserteRegeneration(Astralenergie)],
    vec![Persoenlichkeitsschwaeche(Weltfremd),
		PrinzipientreueII(friedlicheVoelkerverstaendigung,
		Pazifismus),
		SchlechteEigenschaft(Neugier),
		VerpflichtungenII(Lehrmeister)],
    vec![],
    vec![Blutrausch,
		NiedrigeSeelenkraft],
    vec![]
);

//Gildenloser Magier (Schüler der Sevastana Gevendar)
profession!( 
    SchuelerderSevastanaGevendar,
    "Gildenloser Magier (Schüler der Sevastana Gevendar)",
    375,
    vec![VorteilZauberer(25),
		Sonderfertigkeit],
    vec![SprachenSchriften8,
		BindungdesStabes,
		ScholarderSevastanaGevendar],
    vec![Dolche10,
		Raufen10,
		Stangenwaffen12],
    vec![],
    vec![einZaubertrickausfolgenderListeFeuerfinger,
		Regenbogenaugen,
		Tentakelgriff,Armatrutz4,
		AttributoKoerperkraft3,
		Corpofesso4,
		Fortifex6,
		HagelschlagundSturmgebruell6,
		Ignifaxius5,
		Odem3],
    vec![],
    vec![HoheSeelenkraft,
		VerbesserteRegeneration(Astralenergie)],
    vec![Persoenlichkeitsschwaeche(Arroganz,
		Eitelkeit),
		SchlechteEigenschaft(Neugier),
		VerpflichtungenII(Lehrmeisterin)],
    vec![],
    vec![AngstvorWasser,
		Blutrausch,
		NiedrigeSeelenkraft],
    vec![]
);

//Gildenloser Magier (Schüler des Alrik Dagabor)
profession!( 
    SchuelerdesAlrikDagabor,
    "Gildenloser Magier (Schüler des Alrik Dagabor)",
    237,
    vec![VorteilZauberer(25),
		SonderfertigkeitTradition(Gildenmagier)(155)],
    vec![SprachenSchriften12,
		BindungdesStabes],
    vec![],
    vec![KoerperGesellschaftEtikette2,
		Menschenkenntnis4,
		Willenskraft4NaturOrientierung3WissenGeographie4,
		Geschichtswissen3,
		Goetter&Kulte4,
		Magiekunde5,
		Rechnen4,
		Sagen&Legenden5,
		Sphaerenkunde3HandwerkAlchimie4],
    vec![einZaubertrickausfolgenderListeBauchreden,
		Feuerfinger,
		Gluecksgriff,
		Handwaermer,
		Trocken,
        Armatrutz6,
		BlickindieGedanken5,
		FlimFlam4,
		FulminictusElfen5,
		Odem4,
		Paralysis6,
		Penetrizzel4],
    vec![],
    vec![HoheSeelenkraft,
		VerbesserteRegeneration(Astralenergie)],
    vec![Persoenlichkeitsschwaechen(Eitelkeit,
		Weltfremd),
		SchlechteEigenschaften(Neugier),
		VerpflichtungenII(Lehrmeister)],
    vec![],
    vec![Blutrausch,
		NiedrigeSeelenkraft,
		SchlechteRegeneration(Astralenergie)],
    vec![]
);

//Gildenloser Magier (Schüler des Vadif sal Karim)
profession!( 
    SchuelerdesVadifsalKarim,
    "Gildenloser Magier (Schüler des Vadif sal Karim)",
    375,
    vec![KulturNovadi,
		VorteilZauberer(25),
		SonderfertigkeitTradition(Gildenmagier)(155),Geschlechtmaennlich],
    vec![SprachenSchriften12,
		BindungdesStabes,
		ScholardesVadifsalKarim,
		GelaendekundeWuestenkundig,
		Wettervorhersage],
    vec![Dolche8,
		Stangenwaffen8],
    vec![KoerperSelbstbeherrschung4,
		Sinnesschaerfe4GesellschaftBekehren&Überzeugen2,
		Etikette2,
		Menschenkenntnis4,
		Willenskraft4NaturOrientierung4,
		Pflanzenkunde3,
		Tierkunde5,
		Wildnisleben4WissenBrett&Gluecksspiel4,
		Geschichtswissen3,
		Goetter&Kulte4,
		Magiekunde4,
		Rechnen5,
		Rechtskunde3,
		Sagen&Legenden4,
		Sphaerenkunde4,
		Sternkunde4HandwerkAlchimie2,
		HeilkundeGift4,
		HeilkundeWunden4,
		Malen&Zeichnen5],
    vec![EinZaubertrickausfolgenderListeAbkuehlung,
		Duft,
		Luftstoß,
		Sandfigur,Adlerschwinge5,
		Aeolito4,
		Balsam4,
		HagelschlagundSturmgebruellDruiden5,
		Manifesto4,
		Orcanofaxius4,
		Salander7],
    vec![],
    vec![Hitzeresistenz,
		MagischeEinstimmung(WesenderWueste),
		Immunitaet(Gift),
		Richtungssinn,
		VerbesserteRegeneration(Astralenergie)],
    vec![Persoenlichkeitsschwaeche(VorurteilevorallemgegenAndersglaeubigeund/oderFrauen),
		Prinzipientreue(99Gesetze),
		SchlechteEigenschaft(Aberglaube,
		Neugier),
		VerpflichtungenII(Lehrmeister,
		Sippe)],
    vec![],
    vec![Hitzeempfindlich,
		Lichtempfindlich],
    vec![]
);

//Gildenloser Magier (Seminar der elfischen Verständigung und natürlichen Heilung zu Donnerbach)
profession!( 
    SeminarderelfischenVerstaendigungundnatuerlichenHeilungzuDonnerbach,
    "Gildenloser Magier (Seminar der elfischen Verständigung und natürlichen Heilung zu Donnerbach)",
    301,
    vec![VorteilZauberer(25),
		SonderfertigkeitTradition(Gildenmagier)(155),
		NachteilPrinzipientreueI(Voelkerverstaendigung,
		Friedfertigkeit)(10)],
    vec![SprachenSchriften12,
		BindungdesStabes,
		ScholardesSeminarsderelfischenVerstaendigungundnatuerlichenHeilungzuDonnerbach],
    vec![Stangenwaffen8],
    vec![KoerperKoerperbeherrschung2,
		Selbstbeherrschung2,
		Sinnesschaerfe2GesellschaftEtikette1,
		Menschenkenntnis3,
		Willenskraft3NaturFischen&Angeln2,
		Orientierung3,
		Pflanzenkunde4,
		Tierkunde4,
		Wildnisleben3WissenGeschichtswissen2,
		Goetter&Kulte3,
		Magiekunde5,
		Rechnen4,
		Rechtskunde2,
		Sagen&Legenden3,
		Sternkunde3HandwerkAlchimie3,
		HeilkundeGift3,
		HeilkundeKrankheiten4,
		HeilkundeSeele2,
		HeilkundeWunden4,
		Malen&Zeichnen2,
		Musizieren2],
    vec![einZaubertrickausfolgenderListeGrußwort,
		Handwaermer,
		Lockruf,
		Regenbogenaugen,
		Trocken,
        AttributoKoerperkraft4,
		Balsam6,
		Bannbaladin4,
		BlickaufsWesenElfen5,
		BlickindieGedanken6,
		KlarumPurum5,
		Somnigravis4],
    vec![],
    vec![HoheSeelenkraft,
		VerbesserteRegeneration(Astralenergie)],
    vec![Persoenlichkeitsschwaeche(Eitelkeit,
		Weltfremd),
		SchlechteEigenschaft(Neugier),
		VerpflichtungenII(Akademie,
		Lehrmeister,
		OrdendesAnconius)],
    vec![],
    vec![Blutrausch,
		NiedrigeSeelenkraft,
		SchlechteRegeneration(Astralenergie)],
    vec![]
);

//Gildenloser Magier der Bannakademie von Fasar
profession!( 
    GildenloserMagierderBannakademievonFasar,
    "Gildenloser Magier der Bannakademie von Fasar",
    363VoraussetzungenVorteilZauberer(25),SonderfertigkeitTradition(Gildenmagier)(155)SonderfertigkeitenSprachenundSchriftenfürinsgesamt10,BindungdesStabes, ScholarderBannakademievonFasarKampftechnikenDolche8,Stangenwaffen8 TalenteKörperKörperbeherrschung3,Selbstbeherrschung3, Sinnesschärfe4,Verbergen3GesellschaftBekehren&Überzeugen2,Etikette5, Menschenkenntnis5,Verkleiden3,Überreden3, Willenskraft5NaturTierkunde3WissenGeschichtswissen3,Götter&Kulte3,Magiekunde6,Rechnen5,Rechtskunde4,Sagen&Legenden3,Sphärenkunde3,Sternkunde3HandwerkAlchimie3,HeilkundeGift4,Heilkunde Wunden3,Malen&Zeichnen5Zauber/MagischeHandlungeneinZaubertrickausfolgenderListeBauchreden,Glücksgriff,SchattenspielZauber Einflussbann6, Gardianum6,Heilungsbann6,Hellsichtbann5,Pentagramma5,Telekinesebann5,Verwandlungsbann6EmpfohleneVorteileHoheSeelenkraft,Verbesserte Regeneration(Astralenergie)EmpfohleneNachteilePersönlichkeitsschwäche(Arroganz),VerpflichtungenII(Akademie)UngeeigneteVorteile UngeeigneteNachteileBlutrausch,Niedrige Seelenkraft,
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

//Gildenloser Magier der Halle der Winde zu Olport
profession!( 
    GildenloserMagierderHallederWindezuOlport,
    "Gildenloser Magier der Halle der Winde zu Olport",
    355VoraussetzungenVorteilZauberer(25),SonderfertigkeitTradition(Gildenmagier)(155)SonderfertigkeitenSprachenundSchriftenfürinsgesamt10,BindungdesStabes, ScholarderHallederWindezuOlportKampftechnikenWurfwaffen8,Hiebwaffen10,Raufen8,Stangenwaffen8 TalenteKörperKlettern2,Körperbeherrschung2,Schwimmen 4,Selbstbeherrschung3,Sinnesschärfe3,Zechen2GesellschaftMenschenkenntnis4,Überreden2,Willenskraft4NaturFesseln2,Orientierung3,Tierkunde2WissenGeographie3,Geschichtswissen3,Götter&Kulte3,Magiekunde4,Rechnen4,Rechtskunde2,Sagen &Legenden4,Sphärenkunde4,Sternkunde5HandwerkAlchimie2,Boote&Schiffe3,Heilkunde Krankheiten2,HeilkundeWunden3,Holzbearbeitung3,Lederarbeiten2,Malen&Zeichnen4,Stoffbearbeitung2Zauber/MagischeHandlungeneinZaubertrickausfolgenderListeAbkühlung,Tätowierung,Trocken,WindimHaarZauber Aeolito5, Attributo(Mut)5,ElementarerDiener5,Hexagramma4,Manifesto10,Nebelwand(Druiden)6, Wirbelform6EmpfohleneVorteileAffinitätzuElementaren,Hohe Seelenkraft,Richtungssinn,VerbesserteRegeneration(Astralenergie)EmpfohleneNachteileBlutrausch,Persönlichkeitsschwäche(Arroganz,Vorurteile),SchlechteEigenschaft(Aberglaube),VerpflichtungenII(Akademie)UngeeigneteVorteileUngeeigneteNachteileAngstvor(demMeer),Behäbig,NiedrigeSeelenkraft,
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

//Gildenloser Magier nach Agrimeton
profession!( 
    GildenloserMagiernachAgrimeton,
    "Gildenloser Magier nach Agrimeton",
    358,
    vec![VorteilZauberer(25),
		SonderfertigkeitTradition(Gildenmagier)(155)],
    vec![SprachenSchriften8,
		BindungdesStabes,
		ScholardesAgrimeton],
    vec![Dolche10,
		Raufen8,
		Stangenwaffen10],
    vec![],
    vec![einZaubertrickausfolgenderListeFeuerfinger,
		Flammenhaar,
		Schlangenhaende,
		Taetowierung,DaemonischesVergessen4,
		Ecliptifactus5,
		HerzschlagRuhe6,
		Ignifaxius6,
		Ignisphaero5,
		InvocatioMinima10,
		InvocatioMinor6],
    vec![],
    vec![AffinitaetzuDaemonen,
		HoheSeelenkraft,
		VerbesserteRegeneration(Astralenergie)],
    vec![Persoenlichkeitsschwaeche(Arroganz,
		Unheimlich),
		VerpflichtungenII(Lehrmeister)],
    vec![Vertrauenerweckend],
    vec![Blutrausch,
		NiedrigeSeelenkraft,
		Prinzipientreue],
    vec![]
);

//Gildenloser Magier nach Kalliomathëa Dorikeikos von Sorabis
profession!( 
    GildenloserMagiernachKalliomathëaDorikeikosvonSorabis,
    "Gildenloser Magier nach Kalliomathëa Dorikeikos von Sorabis",
    352,
    vec![VorteilZauberer(25),
		SonderfertigkeitTradition(Gildenmagier)(155)],
    vec![SprachenSchriften12,
		BindungdesStabes],
    vec![Raufen8,
		Stangenwaffen8],
    vec![],
    vec![einZaubertrickausfolgenderListeBlickindenLimbus,
		Feuerfinger,
		Signatur,AnalysArkanstruktur6,
		BlickindieGedanken4,
		Gardianum4,
		Invercano4,
		OculusAstralis6,
		OdemArcanum4,
		Transversalis7],
    vec![],
    vec![HoheSeelenkraft,
		VerbesserteRegeneration(Astralenergie)],
    vec![Persoenlichkeitsschwaeche(Arroganz,
		Eitelkeit,
		Neid),
		SchlechteEigenschaft(Neugier,
		Goldgier),
		VerpflichtungenII(Lehrmeisterin)],
    vec![],
    vec![Angstvor(Wasser),
		Blutrausch,
		NiedrigeSeelenkraft],
    vec![]
);
