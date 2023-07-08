use crate::crate_prof;
use crate::profession;

crate_prof!();

//Gratenfelser Haindruide
profession!( 
    GratenfelserHaindruide,
    "Gratenfelser Haindruide",
    361,
    vec![VorteilZauberer(25),
		SonderfertigkeitTradition(Druiden)(125)],
    vec![SprachenSchriften2,
		BindungdesDolchesKampftechnikenDolche10],
    vec![],
    vec![],
    vec![],
    vec![],
    vec![HoheSeelenkraft,
		Krankheitsresistenz,
		Richtungssinn,
		VerbesserteRegeneration(Astralenergie),
		Zeitgefuehl],
    vec![LaestigeMindergeister,
		Persoenlichkeitsschwaeche(Weltfremd),
		SchlechteEigenschaft(Neugier)],
    vec![EisenaffineAura,
		SozialeAnpassungsfaehigkeit],
    vec![Blutrausch,
		NiedrigeSeelenkraft,
		SchlechteRegeneration(Astralenergie)],
    vec![]
);

//Haindruidin
profession!( 
    Haindruidin,
    "Haindruidin",
    361VoraussetzungenVorteilZauberer(25),SonderfertigkeitTradition(Druiden)(125)SonderfertigkeitenSprachenundSchriftenfürinsgesamt2,BindungdesDolches,BindungderSichelKampftechnikenDolche10TalenteKörperKlettern3,Schwimmen3,Selbstbeherrschung3,Sinnesschärfe4,Verbergen3GesellschaftWillenskraft3NaturFährtensuchen4,Orientierung5,Pflanzenkunde 6,Tierkunde6,Wildnisleben7WissenGeschichtswissen3,Götter&Kulte4,Magiekunde 5,Rechtskunde2,Sagen&Legenden4,Sternkunde5HandwerkHeilkundeGift4,HeilkundeKrankheiten4,HeilkundeWunden3,Holzbearbeitung6,Lebensmittelbearbeitung4,Steinbearbeitung6ZaubereinZaubertrickausfolgenderListeeinZaubertrickausfolgenderListeBartwuchs,Beeren& Nüsse,Lockruf,PflanzenempathieZauber Dornenwand5, Dunkelheit4,EinsmitderNatur5,ElementarerDiener4,HerrüberdasTierreich6,Manifesto10,Zorn derElemente5EmpfohleneVorteileHoheSeelenkraft,Krankheitsresistenz,Richtungssinn,VerbesserteRegeneration (Astralenergie),ZeitgefühlEmpfohleneNachteileLästigeMindergeister,Persönlichkeitsschwäche(Weltfremd),SchlechteEigenschaft(Neugier)UngeeigneteVorteileEisenaffineAura,Soziale AnpassungsfähigkeitUngeeigneteNachteileBlutrausch,NiedrigeSeelenkraft,SchlechteRegeneration(Astralenergie),
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

//Hüter der Kraft
profession!( 
    HueterderKraft,
    "Hüter der Kraft",
    376VoraussetzungenVorteilZauberer(25),SonderfertigkeitTradition(Druiden)(125)SonderfertigkeitenSprachenundSchriftenfür insgesamt4,BindungderSichel, BindungdesDolchesKampftechnikenDolche10,Stangenwaffen8TalenteKörperSelbstbeherrschung5,Sinnesschärfe4,Verbergen4GesellschaftBekehren&Überzeugen3,Willenskraft5 NaturFährtensuchen3,Orientierung3,Pflanzenkunde5,Tierkunde5,Wildnisleben6WissenGeschichtswissen5,Götter&Kulte5,Magiekunde6,Sagen&Legenden6,Sternkunde4HandwerkHeilkundeGift3,HeilkundeKrankheiten3, HeilkundeWunden3,Holzbearbeitung6,Steinbearbeitung6Zauber/MagischeHandlungeneinZaubertrickausfolgenderListeBartwuchs,Pflanzenempathie,Trocken;ZauberBlick durchfremdeAugen5,BöserBlick6,Dornenwand 4,Memorans5,ZornderElemente6,Zwingtanz4HerrschaftsritualFluchderPestilenz5EmpfohleneVorteileHoheSeelenkraft,Krankheitsresistenz,Richtungssinn,VerbesserteRegeneration (Astralenergie),ZeitgefühlEmpfohleneNachteileLästigeMindergeister,Persönlichkeitsschwäche(Weltfremd)UngeeigneteVorteileEisenaffineAura,Soziale AnpassungsfähigkeitUngeeigneteNachteileBlutrausch,NiedrigeSeelenkraft,SchlechteRegeneration(Astralenergie),
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

//Konzildruide
profession!( 
    Konzildruide,
    "Konzildruide",
    283 VoraussetzungenVorteilZauberer(25),SonderfertigkeitTradition(Druiden)(125)SonderfertigkeitenSprachenundSchriftenfürinsgesamt6,BindungdesDolchesKampftechnikenDolche8,Stangenwaffen8 TalenteKörperKlettern2,Selbstbeherrschung4,Sinnesschärfe 3GesellschaftMenschenkenntnis2,Willenskraft4 NaturFährtensuchen2,Pflanzenkunde3,Tierkunde3, Wildnisleben2WissenGeographie3,Geschichtswissen5,Götter&Kulte3,Magiekunde6,Rechnen4,Rechtskunde3,Sagen &Legenden3,Sternkunde2HandwerkAlchimie5,HeilkundeGift3,Holzbearbeitung3,Steinbearbeitung6Zauber/MagischeHandlungen einZaubertrickausfolgenderListeAbkühlung,Bartwuchs,Feuerfinger,HandwärmerZauber Adamantium6,Basaltleib5,EinsmitderNatur5,ElementarerDiener6,Manifesto10,Steinwand4,Zorn derElemente6EmpfohleneVorteileHoheSeelenkraft,Krankheitsresistenz,Richtungssinn,VerbesserteRegeneration (Astralenergie),ZeitgefühlEmpfohleneNachteilePersönlichkeitsschwäche (Weltfremd),SchlechteEigenschaft(Neugier),Unfähig(Gesellschaftstalente)UngeeigneteVorteileEisenaffineAuraUngeeigneteNachteileBlutrausch,NiedrigeSeelenkraft,SchlechteRegeneration(Astralenergie)VariantenFakultätdesEises(282)EmpfohlenerVorteilKälteresistenz,Steinbearbeitung0statt6, Caldofrigo5stattAdamantium,Dunkelheit5statt Basaltleib,Eiswand4stattSteinwandFakultätdesFeuers(291)EmpfohlenerVorteil Hitzeresistenz,Steinbearbeitung0statt6,Caldofrigo5stattAdamantium,Flammenwand4statt Basaltleib,Ignifaxius6stattSteinwandFakultätdesHumus(295)HeilkundeWunden4 statt0,Steinbearbeitung0statt6,Dornenwand4 stattAdamantium,SumusElixiere5statt Basaltleib,WeisheitderBäume4stattSteinwandFakultätderLuft(302)Fliegen4statt0,Steinbearbeitung0statt6,Hagelschlag5stattAdamantium,Sturmwand4stattBasaltleib,Wirbelform5stattSteinwandFakultätdesWassers(296)Schwimmen4statt 0,Steinbearbeitung0statt6,Nebelform5stattAdamantium,Wellenwand4stattBasaltleib,Wogenform5,stattSteinwand,
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

//Mehrer der Macht
profession!( 
    MehrerderMacht,
    "Mehrer der Macht",
    ,
    vec![VorteilZauberer(25),
		SonderfertigkeitTradition(Druide)(125)],
    vec![SprachenSchriften4,
		BindungdesDolches],
    vec![Dolche8,
		Raufen8],
    vec![],
    vec![],
    vec![],
    vec![HoheSeelenkraft,
		Krankheitsresistenz,
		Richtungssinn,
		VerbesserteRegeneration(Astralenergie),
		Zeitgefuehl],
    vec![SchlechteEigenschaften(Neugier)],
    vec![EisenaffineAura],
    vec![Blutrausch,
		NiedrigeSeelenkraft,
		SchlechteRegeneration(Astralenergie)],
    vec![]
);

//Sumudiener
profession!( 
    Sumudiener,
    "Sumudiener",
    330,
    vec![VorteilZauberer(25),
		SonderfertigkeitTradition(Druide)(125)],
    vec![SprachenSchriften4,
		BindungdesDolches],
    vec![Dolche8],
    vec![],
    vec![],
    vec![],
    vec![HoheSeelenkraft,
		Krankheitsresistenz,
		Richtungssinn,
		VerbesserteRegeneration(Astralenergie),
		Zeitgefuehl],
    vec![LaestigeMindergeister,
		SchlechteEigenschaften(Neugier)],
    vec![EisenaffineAura],
    vec![Blutrausch,
		NiedrigeSeelenkraft,
		SchlechteRegeneration(Astralenergie)],
    vec![]
);
