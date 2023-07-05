use crate::profession;

//Graumagier (Halle der Antimagie zu Kuslik)
profession!( 
    HallederAntimagiezuKuslik,
    "Graumagier (Halle der Antimagie zu Kuslik)",
    320,
    vec![VorteilZauberer(25),
		SonderfertigkeitTradition(Gildenmagier)(155)],
    vec![SprachenSchriften16,
		BindungdesStabes,
		ScholarderHallederAntimagiezuKuslik],
    vec![Fechtwaffen9,
		Raufen8,
		Stangenwaffen10],
    vec![KoerperReiten2,
		Selbstbeherrschung3,
		Sinnesschaerfe3,
		Tanzen2,
		Zechen1GesellschaftBetoeren1,
		Einschuechtern1,
		Etikette6,
		Menschenkenntnis3,
		Überreden2,
		Willenskraft5NaturPflanzenkunde1,
		Tierkunde1WissenBrett&Gluecksspiel2,
		Geographie2,
		Geschichtswissen3,
		Goetter&Kulte3,
		Magiekunde5,
		Mechanik1,
		Rechnen4,
		Rechtskunde2,
		Sagen&Legenden2,
		Sphaerenkunde5,
		Sternkunde2HandwerkAlchimie2,
		HeilkundeGift2,
		Malen&Zeichnen4],
    vec![EinZaubertrickausfolgenderListeAbkuehlung,
		Duft,
		Ordentlich,
		Schminken,
		Signatur,
        Disruptivo5,
		Einflussbann7,
		Gardianum6,
		Hellsichtbann5,
		Illusionsbann5,
		Sensibar6,
		Telekinesebann4],
    vec![],
    vec![Adel,
		HoheSeelenkraft,
		Reich],
    vec![Persoenlichkeitsschwaeche(Arroganz,
		Eitelkeit),
		SchlechteEigenschaft(Neugier,
		Verwoehnt),
		VerpflichtungenII(Gilde,
		Akademie,
		Lehrmeister,
		Adelshaus)],
    vec![],
    vec![Haesslich,
		Persoenlichkeitsschwaeche(Weltfremd),
		Unfrei],
    vec![]
);

//Graumagier (Halle des Quecksilbers zu Festum)
profession!( 
    HalledesQuecksilberszuFestum,
    "Graumagier (Halle des Quecksilbers zu Festum)",
    297,
    vec![VorteilZauberer(25),
		SonderfertigkeitTradition(Gildenmagier)(155)],
    vec![SprachenSchriften10,
		BindungdesStabes,
		ScholarderHalledesQuecksilberszuFestum],
    vec![Raufen10,
		Stangenwaffen8],
    vec![],
    vec![einZaubertrickausfolgenderListeFeuerfinger,
		KonstanteTemperatur,
		Signatur,
        Adamantium4,
		Caldogrigo5,
		Manifesto6,
		Motoricus4,
		Nebelwand5,
		Objektofixo5,
		Silentium5],
    vec![],
    vec![HoheSeelenkraft,
		VerbesserteRegeneration(Astralenergie)],
    vec![Persoenlichkeitsschwaeche(Arroganz,
		Eitelkeit),
		SchlechteEigenschaft(Neugier),
		VerpflichtungenII(Gilde,
		Lehrmeister,
		Spektabilitaet,
		Stoerrebrandt)],
    vec![],
    vec![Blutrausch,
		NiedrigeSeelenkraft],
    vec![]
);

//Graumagier (Kampfseminar zu Andergast)
profession!( 
    KampfseminarzuAndergast,
    "Graumagier (Kampfseminar zu Andergast)",
    368,
    vec![VorteilZauberer(25),
		SonderfertigkeitTradition(Gildenmagier)(155)],
    vec![SprachenSchriften10,
		BindungdesStabes],
    vec![Dolche8,
		Raufen10,
		Schwerter8,
		Stangenwaffen11],
    vec![],
    vec![einenZaubertrickausfolgenderListeFeuerfinger,
		Gluecksgriff,
		Waffensaeuberung,
		Armatrutz5,
		AttributoKoerperkraft4,
		Balsam4,
		Blitz6,
		Duplicatus5,
		FulminictusElfen4,
		Ignisphaero6],
    vec![],
    vec![Entfernungssinn,
		HoheSeelenkraft,
		Richtungssinn,
		VerbesserteRegeneration(Astralenergie),
		ZaeherHund],
    vec![SchlechteEigenschaften(Neugier,
		VorurteilevorallemgegenNostrierundFrauen,
		Weltfremd),
		VerpflichtungenII(Akademie,
		Gilde,
		KoenigreichAndergast)],
    vec![],
    vec![Blutrausch,
		NiedrigeSeelenkraft,
		SchlechteRegeneration(Astralenergie)],
    vec![]
);

//Graumagier (Schule der Hellsicht zu Thorwal)
profession!( 
    SchulederHellsichtzuThorwal,
    "Graumagier (Schule der Hellsicht zu Thorwal)",
    336,
    vec![VorteilZauberer(25),
		SonderfertigkeitTradition(Gildenmagier)(155)],
    vec![SprachenSchriften14,
		Analytiker,
		BindungdesStabes,
		ScholarderSchulederHellsichtzuThorwal],
    vec![Raufen7,
		Schwerter8,
		Stangenwaffen9],
    vec![KoerperKlettern2,
		Koerperbeherrschung2,
		Schwimmen2,
		Selbstbeherrschung2,
		Sinnesschaerfe2,
		Zechen1GesellschaftBekehren&Überzeugen3,
		Menschenkenntnis4,
		Überreden3,
		Willenskraft4NaturOrientierung2,
		Pflanzenkunde3,
		Tierkunde3,
		Wildnisleben2WissenGeographie5,
		Geschichtswissen3,
		Goetter&Kulte4,
		Magiekunde5,
		Rechnen4,
		Rechtskunde2,
		Sagen&Legenden3,
		Sternkunde3HandwerkAlchimie3,
		Boote&Schiffe1,
		HeilkundeKrankheiten2,
		HeilkundeWunden2,
		Holzbearbeitung2,
		Lederbearbeitung2,
		Malen&Zeichnen4,
		Stoffbearbeitung2],
    vec![einZaubertrickausfolgenderListeFeuerfinger,
		Luftstoß,
		Taetowierung,
		Trocken,
		Wasserhoehe,
        Analys6,
		Balsam4,
		OculusAstralis5,
		Odem4,
		Penetrizzel4,
		Sensibar5,
		Xenographus4],
    vec![],
    vec![GeborenerRedner,
		HoheSeelenkraft,
		VerbesserteRegeneration(Astralenergie),],
    vec![Persoenlichkeitsschwaeche(Arroganz),
		Prinzipientreue(VerteidigungderLehrfreiheit,
		politischeNeutralitaet,
		SchutzderwegenihrerMeinungVerfolgten),
		SchlechteEigenschaft(Neugier),
        VerpflichtungenII(Gilde,
		Akademie,
		Lehrmeister)],
    vec![],
    vec![AngstvorWasser,
		Persoenlichkeitsschwaeche(Vorurteile,
		Weltfremd),
		SchlechteEigenschaft(Verwoehnt)],
    vec![]
);

//Graumagier (Schule des Direkten Weges zu Gerasim)
profession!( 
    SchuledesDirektenWegeszuGerasim,
    "Graumagier (Schule des Direkten Weges zu Gerasim)",
    308,
    vec![VorteilZauberer(25),
		SonderfertigkeitTradition(Gildenmagier)(155),
		PrinzipientreueI(Voelkerverstaendigung,
		Friedfertigkeit)(10)],
    vec![SprachenSchriften10,
		BindungdesStabes,
		ScholarderSchuledesDirektenWegeszuGerasim],
    vec![Raufen8,
		HiebwaffenoderStangenwaffen8],
    vec![KoerperKlettern2,
		Koerperbeherrschung3,
		Schwimmen1,
		Selbstbeherrschung1,
		Sinnesschaerfe4,
		Verbergen2GesellschaftEtikette1,
		Menschenkenntnis2,
		Willenskraft3NaturFaehrtensuchen2,
		Fischen&Angeln2,
		Orientierung2,
		Pflanzenkunde4,
		Tierkunde4,
		Wildnisleben3WissenGeographie3,
		Geschichtswissen2,
		Goetter&Kulte1,
		Magiekunde3,
		Rechnen3,
		Rechtskunde1,
		Sagen&Legenden4,
		Sternkunde2HandwerkHeilkundeWunden2,
		Holzbearbeitung2,
		Lebensmittelbearbeitung2,
		Lederbearbeitung1,
		Malen&Zeichnen4,
		Musizieren2,
		Stoffbearbeitung2],
    vec![EinZaubertrickausfolgenderListeBluetenduft,
		Feuerfinger,
		Handwaermer,
		Lockruf,
		Trocken,
        AttributoKlugheit6,
		AxxeleratusElfen7,
		Balsam6,
		Motoricus5,
		Movimento6,
		Transversalis4,
		Wirbelform5],
    vec![],
    vec![Flink,
		Richtungssinn,
		SozialeAnpassungsfaehigkeit],
    vec![Persoenlichkeitsschwaeche(Arroganz,
		Eitelkeit),
		SchlechteEigenschaft(Neugier,
		Naiv),
		VerpflichtungenII(Gilde,
		Akademie,
		Lehrmeister),
		WahrerName],
    vec![],
    vec![Blutrausch,
		Persoenlichkeitsschwaeche(Vorurteile)],
    vec![]
);

//Graumagier (Stoerrebrandt-Kolleg zu Riva)
profession!( 
    StoerrebrandtKollegzuRiva,
    "Graumagier (Stoerrebrandt-Kolleg zu Riva)",
    319,
    vec![VorteilZauberer(25),
		SonderfertigkeitTradition(Gildenmagier)(155)],
    vec![SprachenSchriften10,
		BindungdesStabes,
		ScholardesStoerrebrandtKollegszuRiva],
    vec![Dolche10,
		Raufen10,
		Stangenwaffen12],
    vec![],
    vec![einZaubertrickausfolgenderListeKleingeld,
		Schnipsen,
		Signatur,
        AttributoKoerperkraft4,
		Armatrutz4,
		Balsam6,
		Blitzdichfind4,
		Custodosigil6,
		Gardianum4,
		Horriphobus6],
    vec![],
    vec![HoheSeelenkraft,
		VerbesserteRegeneration(Astralenergie)],
    vec![Persoenlichkeitsschwaeche(Arroganz,
		Eitelkeit),
		SchlechteEigenschaft(Neugier),
		VerpflichtungenII(Gilde,
		Lehrmeister,
		Spektabilitaet,
		Stoerrebrandt)],
    vec![],
    vec![Blutrausch,
		NiedrigeSeelenkraft],
    vec![]
);

//Graumagier der Drachenei-Akademie zu Khunchom
profession!( 
    GraumagierderDracheneiAkademiezuKhunchom,
    "Graumagier der Drachenei-Akademie zu Khunchom",
    360,
    vec![VorteilZauberer(25),
		SonderfertigkeitTradition(Gildenmagier)(155)],
    vec![SprachenSchriften8,
		BindungdesStabes,
		ScholarderDracheneiAkademiezuKhunchom],
    vec![Stangenwaffen8],
    vec![KoerperSelbstbeherrschung5,
		Sinnesschaerfe3GesellschaftEtikette4,
		Menschenkenntnis5,
		Überreden3,
		Willenskraft5NaturWissenBrett&Gluecksspiel4,
		Geschichtswissen4,
		Goetter&Kulte5,
		Magiekunde6,
		Rechnen4,
		Rechtskunde4,
		Sagen&Legenden4,
		Sphaerenkunde5,
		Sternkunde5HandwerkAlchimie5,
		Handel7,
		Malen&Zeichnen4],
    vec![einZaubertrickausfolgenderListeAbkuehlung,
		Feuerfinger,
		KleinerFliegenderTeppich,
		Trocken,
        Analys5,
		Animatio5,
		Applicatus6,
		Arcanovi7,
		Destructibo4,
		Foramen5,
		Motoricus6],
    vec![],
    vec![HoheSeelenkraft,
		VerbesserteRegeneration(Astralenergie)],
    vec![Persoenlichkeitsschwaeche(Arroganz,
		VorurteilegegenAndersglaeubige),
		SchlechteEigenschaft(AberglaubeZahlenmystik),
		VerpflichtungenII(Akademie,
		Kalifat)],
    vec![],
    vec![Blutrausch,
		NiedrigeSeelenkraft],
    vec![]
);

//Graumagier der Zauberschule des Kalifen zu Mherwed
profession!( 
    GraumagierderZauberschuledesKalifenzuMherwed,
    "Graumagier der Zauberschule des Kalifen zu Mherwed",
    316VoraussetzungenVorteilZauberer(25),NachteilPrinzipientreueII(99Gesetze)(-20),SonderfertigkeitTradition(Gildenmagier)(155),Geschlecht männlichSonderfertigkeitenSprachenundSchriftenfürinsgesamt8,BindungdesStabes,ScholarderZauberschuledesKalifenzuMherwedKampftechnikenDolche8,Stangenwaffen8TalenteKörperSelbstbeherrschung5,Sinnesschärfe3GesellschaftBekehren&Überzeugen3,Etikette4,Menschenkenntnis5,Überreden3,Willenskraft4Natur-WissenBrett-&Glücksspiel2,Geschichtswissen4,Götter &Kulte5,Magiekunde6,Rechnen4,Rechtskunde4, Sagen&Legenden4,Sphärenkunde5,Sternkunde5HandwerkAlchimie4,Malen&Zeichnen4Zauber/MagischeHandlungeneinZaubertrickausfolgenderListeAbkühlung,Feuerfinger,KleinerFliegenderTeppich,TrockenZauberAdamantium6,Arcanovi5,Caldofrigo5, ElementarerDiener6,KlarumPurum6,Manifesto 10,Pentagramma5EmpfohleneVorteileAffinitätzuElementaren, HoheSeelenkraft,VerbesserteRegeneration (Astralenergie)EmpfohleneNachteilePersönlichkeitsschwäche(Arroganz,VorurteilegegenAndersgläubige),Schlechte Eigenschaft(AberglaubeZahlenmystik),VerpflichtungenII(Akademie,Kalifat)UngeeigneteVorteileUngeeigneteNachteileBlutrausch,Niedrige Seelenkraft,
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

//Graumagier des Konzils der Elemente zu Drakonia
profession!( 
    GraumagierdesKonzilsderElementezuDrakonia,
    "Graumagier des Konzils der Elemente zu Drakonia",
    317 VoraussetzungenVorteilZauberer(25),SonderfertigkeitTradition(Gildenmagier)(155)SonderfertigkeitenSprachenundSchriftenfürinsgesamt12,BindungdesStabes,ScholardesKonzilsderElementezuDrakoniaKampftechnikenStangenwaffen8TalenteKörperKlettern3,Körperbeherrschung2,Selbstbeherrschung4,Sinnesschärfe4GesellschaftEtikette2,Willenskraft5NaturTierkunde2,Wildnisleben2WissenBrett-&Glücksspiel2,Geschichtswissen5,Götter&Kulte3,Magiekunde6,Rechnen4,Rechtskunde2,Sagen&Legenden3,Sphärenkunde6,Sternkunde3HandwerkAlchimie4,Malen&Zeichnen3,Steinbearbeitung5Zauber/MagischeHandlungeneinZaubertrickausfolgenderListeAbkühlung,Feuerfinger,SiegelderElemente,TrockenZauberAdamantium6,Archofaxius5,ElementarerDiener6,ErhabenheitdesMarmors4,Felsenform4,Manifesto10,Steinwand6EmpfohleneVorteileAffinitätzuElementaren,HoheSeelenkraft,VerbesserteRegeneration(Astralenergie)EmpfohleneNachteilePersönlichkeitsschwäche(Arroganz,Weltfremd),Unfähig(Gesellschaftstalente),VerpflichtungenII(Akademie)UngeeigneteVorteileAffinitätzuDämonen,SozialeAnpassungsfähigkeitUngeeigneteNachteileAngstvor(Höhe),Blutrausch,Fettleibig,NiedrigeSeelenkraftVariantenEiselementarist(307)EmpfohlenerVorteilKälteresistenz,Steinbearbeitung0statt5,ZauberCaldofrigo5,Corpofrigo6,Eiswand4,ElementarerDiener6,Frigifaxius5,Gletscherform4,Manifesto10Feuerelementarist(314)EmpfohlenerVorteilHitzeresistenz,Steinbearbeitung0statt5,ZauberDrachenleib4,ElementarerDiener6,Flammenwand6,Ignifaxius5,Ignisphaero5,Incendio4,Manifesto10Humuselementarist(335)HeilkundeGift3statt0,HeilkundeKrankheiten3statt0,HeilkundeWunden4statt0,Steinbearbeitung0statt5,Dornenwand5,ElementarerDiener6,Humofaxius5,KlarumPurum6,Manifesto10,Pflanzenform4,RuheKörper4Luftelementarist(318)Fliegen3statt0,Steinbearbeitung0statt5,Aeolito6,ElementarerDiener6,Himmelslauf4,Manifesto10,Orcanofaxius5,Stillstand4,Wirbelform5Wasserelementarist(308)Schwimmen3statt0,Steinbearbeitung0statt5,Aquafaxius5,ElementarerDiener6,Manifesto10,Wasseratem4,Wellenlauf4,Wellenwand5,Wogenform6,
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

//Graumagierin (Akademie der Hohen Magie zu Punin)
profession!( 
    AkademiederHohenMagiezuPunin,
    "Graumagierin (Akademie der Hohen Magie zu Punin)",
    323,
    vec![VorteilZauberer(25),
		SonderfertigkeitTradition(Gildenmagier)(155)],
    vec![SprachenSchriften18,
		Analytiker,
		BindungdesStabes,
		ScholarderAkademiederHohenMagiezuPunin],
    vec![],
    vec![KoerperSelbstbeherrschung4,
		Sinnesschaerfe2GesellschaftEtikette2,
		Willenskraft4NaturPflanzenkunde4,
		Tierkunde4WissenGeographie2,
		Geschichtswissen4,
		Goetter&Kulte4,
		Magiekunde8,
		Mechanik2,
		Rechnen6,
		Rechtskunde6,
		Sagen&Legenden2,
		Sphaerenkunde6,
		Sternkunde4HandwerkAlchimie6,
		Malen&Zeichnen4],
    vec![einZaubertrickausfolgenderListeBauchreden,
		Handwaermer,
		Schnipsen,
		Signatur,
		Zauberfeder,
        Analys8,
		InvocatioMinima8,
		Manifesto8,
		OculusAstralis7,
		Odem7,
		Pentagramma6,
		Xenographus4],
    vec![],
    vec![Adel,
		Begabung(einodermehrereWissenstalente),
		VerbesserteRegeneration(Astralenergie)],
    vec![Behaebig,
		Fettleibig,
		Persoenlichkeitsschwaeche(Arroganz,
		Weltfremd),
		SchlechteEigenschaft(Neugier,
		Verwoehnt),
		VerpflichtungenII(Gilde,
		Akademie,
		Lehrmeister)],
    vec![Flink,
		HerausragendeKampftechnik,
		Schlangenmensch,
		ZaeherHund],
    vec![Blutrausch,
		SchlechteEigenschaft(Jaehzorn)],
    vec![]
);

//Graumagierin (Akademie der Verformungen zu Lowangen)
profession!( 
    AkademiederVerformungenzuLowangen,
    "Graumagierin (Akademie der Verformungen zu Lowangen)",
    310,
    vec![VorteilZauberer(25),
		SonderfertigkeitTradition(Gildenmagier)(155)],
    vec![SprachenSchriften12,
		BindungdesStabes],
    vec![Stangenwaffen8],
    vec![KoerperKoerperbeherrschung2,
		Selbstbeherrschung3,
		Sinnesschaerfe3GesellschaftEtikette2,
		Menschenkenntnis4,
		Willenskraft3NaturOrientierung3,
		Wildnisleben2WissenGeographie4,
		Geschichtswissen4,
		Goetter&Kulte4,
		Magiekunde6,
		Rechnen4,
		Sagen&Legenden3,
		Sphaerenkunde3HandwerkAlchimie4,
		HeilkundeGift4,
		HeilkundeKrankheit5,
		HeilkundeWunden4,
		Malen&Zeichnen4],
    vec![einZaubertrickausfolgenderListeGluecksgriff,
		Handwaermer,
		Lockruf,
		Regenbogenaugen,
		Signatur,
        Armatrutz5,
		Bannbaladin4,
		Corpofesso6,
		Paralysis6,
		Penetrizzel4,
		Salander5,
		VisibiliElfen4],
    vec![],
    vec![HoheSeelenkraft,
		VerbesserteRegeneration(Astralenergie)],
    vec![Persoenlichkeitsschwaeche(Eitelkeit,
		Weltfremd),
		PrinzipientreueI(VerletztenungeachtetvonStand,
		KulturundSpezieshelfen,
		Pazifismus,
		elfischnaturverbundeneWeltsicht),
		SchlechteEigenschaft(Neugier),
		VerpflichtungenII(AkademieoderStadtLowangen)],
    vec![],
    vec![Angstvor(TotenundUntoten),
		Blutrausch,
		NiedrigeSeelenkraft,
		SchlechteRegeneration(Astralenergie)],
    vec![]
);

//Graumagierin (Akademie der Erscheinungen zu Grangor)
profession!( 
    AkademiederErscheinungenzuGrangor,
    "Graumagierin (Akademie der Erscheinungen zu Grangor)",
    301,
    vec![VorteilZauberer(25),
		SonderfertigkeitTradition(Gildenmagier)(155)],
    vec![SprachenSchriften10,
		BindungdesStabes,
		ScholarderAkademiederErscheinungenzuGrangor],
    vec![Raufen10,
		Stangenwaffen8],
    vec![],
    vec![einZaubertrickausfolgenderListeDuft,
		Geistergeraeusche,
		Haarpracht,
		Regenbogenaugen,
        AurisIllusionis4,
		Geisteressenz10,
		Geisterruf5,
		Ignorantia5,
		Menetekel6,
		OculusIllusionis5,
		Projektimago4],
    vec![],
    vec![HoheSeelenkraft,
		VerbesserteRegeneration(Astralenergie)],
    vec![Persoenlichkeitsschwaeche(Arroganz,
		Eitelkeit),
		SchlechteEigenschaft(Neugier),
		VerpflichtungenII(Gilde,
		Lehrmeister,
		Spektabilitaet)],
    vec![],
    vec![Blutrausch,
		NiedrigeSeelenkraft],
    vec![]
);

//Graumagierin (Akademie der Geistreisen zu Belhanka)
profession!( 
    AkademiederGeistreisenzuBelhanka,
    "Graumagierin (Akademie der Geistreisen zu Belhanka)",
    303,
    vec![VorteilZauberer(25),
		SonderfertigkeitTradition(Gildenmagier)(155)],
    vec![SprachenSchriften10,
		BindungdesStabes,
		ScholarAkademiederGeistreisenzuBelhanka],
    vec![Raufen10,
		Stangenwaffen8],
    vec![],
    vec![einZaubertrickausfolgenderListeBereitzumAufbruch,
		Duft,
		Feuerfinger,
        Animatio4,
		Axxeleratus5,
		Blitzdichfind5,
		Foramen6,
		Motoricus4,
		Telekinesebann5,
		Transversalis5],
    vec![],
    vec![HoheSeelenkraft,
		VerbesserteRegeneration(Astralenergie)],
    vec![Persoenlichkeitsschwaeche(Eitelkeit),
		SchlechteEigenschaft(Neugier),
		VerpflichtungenII(Gilde,
		Lehrmeister,
		Spektabilitaet)],
    vec![],
    vec![Blutrausch,
		NiedrigeSeelenkraft],
    vec![]
);

//Graumagierin (Halle des vollendeten Kampfes zu Bethana)
profession!( 
    HalledesvollendetenKampfeszuBethana,
    "Graumagierin (Halle des vollendeten Kampfes zu Bethana)",
    360,
    vec![VorteilZauberer(25),
		SonderfertigkeitTradition(Gildenmagier)(155)],
    vec![SprachenSchriften10,
		BindungdesStabes,
		ScholarderHalledesvollendetenKampfeszuBethana],
    vec![Raufen12,
		Stangenwaffen12],
    vec![],
    vec![einZaubertrickausfolgenderListeFeuerfinger,
		RuhigeAusstrahlung,
		Signatur,
        Armatrutz4,
		Balsam5,
		Corpofesso6,
		Desintegratus4,
		Fulminictus5,
		Ignifaxius5,
		Ignisphaero5],
    vec![],
    vec![HoheSeelenkraft,
		VerbesserteRegeneration(Astralenergie)],
    vec![Persoenlichkeitsschwaeche(Arroganz,
		Eitelkeit),
		Prinzipientreu(Wahrheitsliebe,
		SchutzdesGutenundRechtschaffenen,
		TreuezudenZwoelfgoetternundihrenDienern),
		SchlechteEigenschaft(Neugier),
		VerpflichtungenII(Gilde,
		Lehrmeister,
		Spektabilitaet)],
    vec![],
    vec![Blutrausch,
		NiedrigeSeelenkraft],
    vec![]
);

//Graumagierin (Schule der Vierfachen Verwandlung zu Sinoda)
profession!( 
    SchulederVierfachenVerwandlungzuSinoda,
    "Graumagierin (Schule der Vierfachen Verwandlung zu Sinoda)",
    351,
    vec![VorteilZauberer(25),
		SonderfertigkeitTradition(Gildenmagier)(155)],
    vec![SprachenSchriften12,
		BindungdesStabes,
		ScholarderSchulederVierfachenVerwandlungzuSinoda],
    vec![Diskusse8,
		Raufen10,
		Stangenwaffen10],
    vec![KoerperKlettern2,
		Koerperbeherrschung2,
		Schwimmen1,
		Selbstbeherrschung3,
		Sinnesschaerfe3,
		Verbergen3GesellschaftEtikette2,
		Menschenkenntnis4,
		Willenskraft4NaturFaehrtensuchen2,
		Fischen&Angeln2,
		Orientierung2,
		Pflanzenkunde4,
		Tierkunde6,
		Wildnisleben3WissenGeographie2,
		Geschichtswissen2,
		Goetter&Kulte4,
		Magiekunde4,
		Rechnen5,
		Rechtskunde3,
		Sagen&Legenden3,
		Sternkunde2HandwerkAlchimie2,
		HeilkundeGift4,
		Malen&Zeichnen2],
    vec![einZaubertrickausfolgenderListeInsektenbann,
		Lockruf,
		Regenbogenaugen,
		Schlangenhaende,
		Trocken,
        Adlerschwinge5,
		AxxeleratusElfen5,
		KlarumPurum4,
		Paralysis7,
		Pentagramma5,
		Psychostabilis5,
		Verwandlungsbann5],
    vec![],
    vec![Hitzeresistenz,
		HoheSeelenkraft,
		VerbesserteRegeneration(Astralenergie)],
    vec![Persoenlichkeitsschwaeche(Arroganz,
		Eitelkeit,
		VorurteilevorallemgegenNichtmaraskaner),
		Prinzipientreue(RurundGrorGlaube,
		FreiheitfuerMaraskan),
		SchlechteEigenschaft(Neugier),
		VerpflichtungenII(Gilde,
		Akademie,
		Lehrmeister)],
    vec![Kaelteresistenz],
    vec![Blutrausch,
		Persoenlichkeitsschwaeche(Neid)],
    vec![]
);

//Graumagierin (Schule des Seienden Scheins zu Zorgan)
profession!( 
    SchuledesSeiendenScheinszuZorgan,
    "Graumagierin (Schule des Seienden Scheins zu Zorgan)",
    261,
    vec![VorteilZauberer(25),
		SonderfertigkeitTradition(Gildenmagier)(155)],
    vec![SprachenSchriften12,
		BindungdesStabes,
		ScholarderSchuledesSeiendenScheinszuZorgan],
    vec![Stangenwaffen8],
    vec![KoerperGaukeleien3,
		Reiten2,
		Selbstbeherrschung2,
		Singen2,
		Sinnesschaerfe2,
		Tanzen2,
		Zechen1GesellschaftBetoeren2,
		Etikette4,
		Menschenkenntnis4,
		Überreden3,
		Willenskraft3NaturPflanzenkunde2,
		Tierkunde2WissenBrett&Gluecksspiel2,
		Geographie2,
		Geschichtswissen2,
		Goetter&Kulte2,
		Magiekunde4,
		Rechnen4,
		Rechtskunde3,
		Sagen&Legenden3,
		Sternkunde2HandwerkMalen&Zeichnen4],
    vec![einZaubertrickausfolgenderListeBauchreden,
		Duft,
		Hintergrundmusik,
		Luftstoß,
		Schminken,
        AurisIllusionis6,
		Duplicatus5,
		Illusionsbann6,
		ManusIllusionis4,
		OculusIllusionis7,
		Reflectimago6,
		VisibiliElfen5],
    vec![],
    vec![Gutaussehend,
		VerbesserteRegeneration(Astralenergie)],
    vec![Persoenlichkeitsschwaeche(Eitelkeit,
		Neid),
		SchlechteEigenschaft(Neugier),
		VerpflichtungenII(Gilde,
		Akademie,
		Lehrmeister)],
    vec![Unscheinbar],
    vec![Blutrausch,
		Blind,
		Farbenblind,
		Persoenlichkeitsschwaeche(Weltfremd),
		Taub],
    vec![]
);

//Graumagierin der Akademie zu Wagenhalt
profession!( 
    GraumagierinderAkademiezuWagenhalt,
    "Graumagierin der Akademie zu Wagenhalt",
    398VoraussetzungenVorteilZauberer(25),SonderfertigkeitTradition(Gildenmagier)(155)SonderfertigkeitenSprachenundSchriftenfür insgesamt8,BindungdesStabes, ScholarderAkademiezuWagenhaltKampftechnikenRaufen10,Stangenwaffen12 TalenteKörperKörperbeherrschung4,Selbstbeherrschung5, Sinnesschärfe4GesellschaftEinschüchtern3,Etikette5,Menschenkenntnis5,Willenskraft5NaturOrientierung4,Wildnisleben4WissenGeographie3,Geschichtswissen4,Götter& Kulte4,Kriegskunst5,Magiekunde5,Rechnen4, Rechtskunde5HandwerkHandel4,HeilkundeGift4,HeilkundeKrankheiten4,HeilkundeSeele2,HeilkundeWunden6Zauber/MagischeHandlungeneinZaubertrickausfolgenderListeSchnipsen, Signatur,Trocken,WechselscheinZauberAdamantium4, Armatrutz5,Balsam6,BlickindieGedanken4,Corpofesso6,Fulminictus5,Sensibar5EmpfohleneVorteileHoheSeelenkraft,Verbesserte Regeneration(Astralenergie)EmpfohleneNachteilePersönlichkeitsschwäche(Arroganz,Eitelkeit),VerpflichtungenII(Gilde,Spektabilität,Stoerrebrandt)UngeeigneteVorteile UngeeigneteNachteileBlutrausch,Niedrige Seelenkraft,
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

//Graumagierin der Pentagramm-Akademie zu Rashdul
profession!( 
    GraumagierinderPentagrammAkademiezuRashdul,
    "Graumagierin der Pentagramm-Akademie zu Rashdul",
    327VoraussetzungenVorteilZauberer(25),SonderfertigkeitTradition(Gildenmagier)(155)SonderfertigkeitenSprachenundSchriftenfürinsgesamt10,BindungdesStabes, ScholarderPentagramm-AkademiezuRashdulKampftechnikenDolche8,Stangenwaffen8TalenteKörperFliegen2,Selbstbeherrschung3,Sinnesschärfe2GesellschaftBekehren&Überzeugen2,Etikette4,Menschenkenntnis5,Überreden3,Willenskraft4Natur-WissenBrett-&Glücksspiel3,Geographie2,Geschichtswissen4,Götter&Kulte2,Magiekunde6, Rechnen5,Rechtskunde3,Sagen&Legenden4, Sphärenkunde7,Sternkunde6HandwerkAlchimie4,Malen&Zeichnen7Zauber/MagischeHandlungeneinZaubertrickausfolgenderListeAbkühlung,Bauchreden,Elementarling,Feuerfinger ZauberAeolito4,ElementarerDiener6,Flammenwand5, Hexagramma5,Manifesto10,Pentagramma6,Stein wandle5EmpfohleneVorteileAffinitätzuElementaren, HoheSeelenkraft,VerbesserteRegeneration (Astralenergie)EmpfohleneNachteilePersönlichkeitsschwäche(Arroganz),VerpflichtungenII(Akademie) UngeeigneteVorteile UngeeigneteNachteileBlutrausch,Niedrige Seelenkraft,
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

//Graumagierin der Schule der Beherrschung zu Neersand
profession!( 
    GraumagierinderSchulederBeherrschungzuNeersand,
    "Graumagierin der Schule der Beherrschung zu Neersand",
    325VoraussetzungenVorteilZauberer(25),SonderfertigkeitTradition(Gildenmagier)(155)SonderfertigkeitenSprachenundSchriftenfürinsgesamt12,BindungdesStabes,ScholarderSchulederBeherrschungzuNeersand KampftechnikenStangenwaffen8TalenteKörperKörperbeherrschung2,Selbstbeherrschung4,Sinnesschärfe4GesellschaftBekehren&Überzeugen3,Etikette2,Menschenkenntnis5,Überreden3,Willenskraft5NaturOrientierung3,Wildnisleben2WissenGeschichtswissen3,Götter&Kulte4,Magiekunde4,Rechnen6,Rechtskunde3,Sagen&Legenden3, Sphärenkunde3,Sternkunde2HandwerkAlchimie2,HeilkundeGift2,HeilkundeSeele 2,HeilkundeWunden2,Lederarbeiten2,Malen& Zeichen4Zauber/MagischeHandlungeneinZaubertrickausfolgenderListeBauchreden,Feenzauber,LockrufRegenbogenaugenZauberBannbaladin6,Einflussbann4,Horriphobus6,Imperavi5, Respondami4,Sensibar6,Somnigravis(Elfen)5EmpfohleneVorteileHoheSeelenkraft,Verbesserte Regeneration(Astralenergie)EmpfohleneNachteilePrinzipientreue(Pazifismus), SchlechteEigenschaft(Neugier),VerpflichtungenII (Akademie)UngeeigneteVorteileWaffenbegabungUngeeigneteNachteileBlutrausch,Niedrige Seelenkraft,
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

//Rashduler Dämonologe
profession!( 
    RashdulerDaemonologe,
    "Rashduler Dämonologe",
    330VoraussetzungenVorteilZauberer(25),SonderfertigkeitTradition(Gildenmagier)(155)SonderfertigkeitenSprachenundSchriftenfürinsgesamt10,BindungdesSchwerts, BindungdesStabes,RashdulerDämomologenKampftechnikenDolche8,Stangenwaffen8 TalenteKörperFliegen2,Selbstbeherrschung3,Sinnesschärfe2GesellschaftBekehren&Überzeugen2,Etikette4,Menschenkenntnis5,Überreden3,Willenskraft4Natur-WissenBrett-&Glücksspiel3,Geographie2,Geschichtswissen4,Götter&Kulte2,Magiekunde6, Rechnen5,Rechtskunde3,Sagen&Legenden4,Sphärenkunde7,Sternkunde6HandwerkAlchimie4,Malen&Zeichnen7 Zauber/MagischeHandlungeneinZaubertrickausfolgenderListeAbkühlung,Dämonling,Feuerfinger,SchnipsenZauberGeistessenz10,Geisterruf6,Heptagramma4,Invocatio Minor6,InvocatioMinima10,Pentagramma6, Skelettarius5EmpfohleneVorteileAffinitätzuDämonen, HoheSeelenkraft,VerbesserteRegeneration (Astralenergie)EmpfohleneNachteilePersönlichkeitsschwäche (Arroganz)UngeeigneteVorteile UngeeigneteNachteileBlutrausch,Niedrige Seelenkraft,
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
