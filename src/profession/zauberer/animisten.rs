use crate::profession;

//Anach-Nûr (Ferkina-Besessener)
profession!( 
    AnachNûr(FerkinaBesessener),
    "Anach-Nûr (Ferkina-Besessener)",
    355 VoraussetzungenKulturFerkinas,VorteilZauberer(25),SonderfertigkeitTradition(Animisten)(125)SonderfertigkeitenSprachenundSchriftenfürinsgesamt4,BindungderWaffeKampftechnikenDolche12,Hiebwaffen10,Raufen10TalenteKörperKlettern5,Körperbeherrschung6,Kraftakt3,Selbstbeherrschung3,Sinnesschärfe5,Verbergen5GesellschaftEinschüchtern6NaturFährtensuchen4,Orientierung4,Pflanzenkunde4,Tierkunde5,Wildnisleben4WissenGötter&Kulte2,Kriegskunst4,Magiekunde2,Sagen&Legenden4HandwerkHeilkundeWunden5,Holzbearbeitung3,Lederbearbeitung3,Metallbearbeitung2Zauber/MagischeHandlungeneinZaubertrickausfolgenderListeAbkühlung,Tätowierung,TrockenZauberArmatrutz4,Horriphobus5,Standfest6AnimistenkräfteBluttrinken6,Steinmacht5,Talentverbesserung5,Trümmerschlag4EmpfohleneVorteileWaffenbegabungEmpfohleneNachteileBlutrausch,Prinzipientreue(Vegetarier)UngeeigneteVorteileBegabung(Gesellschaftstalente),VertrauenerweckendUngeeigneteNachteileAngstvorBlut,Behäbig,
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

//Bakarogh (Ork-Blutkrieger)
profession!( 
    Bakarogh(OrkBlutkrieger),
    "Bakarogh (Ork-Blutkrieger)",
    323,
    vec![SpeziesHalborkoderOrk,
		KulturOrklandoderSvellttal,
		VorteilZauberer(25),
		SonderfertigkeitTradition(Animisten)(125)],
    vec![Sprachenfuerinsgesamt2,
		BindungderWaffe],
    vec![Hiebwaffen10,
		Raufen10,
		Schwerter11,
		Stangenwaffen12],
    vec![KoerperKlettern6,
		Koerperbeherrschung5,
		Kraftakt6,
		Selbstbeherrschung5,
		Sinnesschaerfe4,
		Verbergen3,
        Einschuechtern4,
        Orientierung4,
		Tierkunde3,
		Wildnisleben5,
        Magiekunde3,
		Sagen&Legenden4,
        HeilkundeKrankheiten3,
		HeilkundeWunden4,
		Holzbearbeitung3,
		Lederbearbeitung3,
		Steinbearbeitung5],
    vec![einZaubertrickausfolgenderListeAbkuehlung,
		Feuerfinger,
		Trocken;Adlerauge3,
		Axxeleratus4,
		Spurlos4;AnimistenkraefteHarteHaut3,
		Kampffaehigkeitenverbessern4,
		SchnellerAngriff5,
		Talentverbesserung4],
    vec![],
    vec![ZaeherHund],
    vec![Blutrausch,
		LaestigeMindergeister,
		MagischeEinschraenkung(FluchderBerge)],
    vec![SozialeAnpassungsfaehigkeit],
    vec![Zerbrechlich],
    vec![]
);

//Durro-Dûn (Gjalsker Tierkriegerin)
profession!( 
    DurroDûn(GjalskerTierkriegerin),
    "Durro-Dûn (Gjalsker Tierkriegerin)",
    381VoraussetzungenKulturGjalsker,VorteilZauberer(25 ),SonderfertigkeitTradition(Animisten)(125)SonderfertigkeitenSprachenundSchriftenfürinsgesamt4,BindungderWaffe KampftechnikenDolche8,Hiebwaffen10,Rauen12,Schwerter11,Zweihandhiebwaffen10TalenteKörperKlettern3,Körperbeherrschung6,Kraftakt4, Schwimmen3,Selbstbeherrschung5,Sinnesschärfe 6,Verbergen5GesellschaftEinschüchtern4NaturFährtensuchen5,Fischen&Angeln3,Orientierung4,Pflanzenkunde4,Tierkunde5,Wildnisleben6WissenGötter&Kulte2,Magiekunde2,Sagen&Legenden5HandwerkHeilkundeWunden5,Holzbearbeitung4, Lederbearbeitung4,Steinbearbeitung5Zauber/MagischeHandlungeneinZaubertrickaus folgenderListeHandwärmer,Lockruf;ZauberAttributo(Konstitution)5,Exposami6,LungedesLeviatan5;AnimistenkräfteGroßerSprung4,Immunität gegenKälte4,Patronruf6,Talentverbesserung5EmpfohleneVorteileBeidhändig,Flink,Schutzgeist,TierfreundEmpfohleneNachteileBlutrausch,Persönlichkeitsschwäche(Weltfremd),Raubtiergeruch,Unfähig (Gesellschaftstalente)UngeeigneteVorteileAngenehmerGeruch,Begabung(Gesellschaftstalente)UngeeigneteNachteileSchlechteRegeneration(Lebensenergie),Unfähig(Naturtalente),
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

//Hadan-Harigasta (Zauberschmiedin der Fjarninger)
profession!( 
    HadanHarigasta(ZauberschmiedinderFjarninger),
    "Hadan-Harigasta (Zauberschmiedin der Fjarninger)",
    330VoraussetzungenKulturFjarninger,VorteilZauberer(25),SonderfertigkeitTradition(Animisten)(125)SonderfertigkeitenSprachenundSchriftenfürinsgesamt2,BindungderWaffeKampftechnikenHiebwaffen10,Raufen10,Schwerter11,Wurfwaffen10,Zweihandhiebwaffen11TalenteKörperKlettern4,Kraftakt6,Selbstbeherrschung5,Sinnesschärfe3GesellschaftMenschenkenntnis4NaturOrientierung3,Wildnisleben4WissenGötter&Kulte2,Magiekunde4,Sagen&Legenden3HandwerkHandel3,HeilkundeWunden3,Holzbearbeitung5,Lederbearbeitung4,Metallbearbeitung8,Steinbearbeitung6Zauber/MagischeHandlungeneinZaubertrickausfolgenderListeFeuerfinger,HandwärmerZauber Attributo(Körperkraft)5,Eisenrost4,Physiostabilis5,;AnimistenkräfteHarteHaut4,ImmunitätgegenKälte5,Talentverbesserung5,Trümmerschlag5EmpfohleneVorteileBegabung(Metallbearbeitung),EisenaffineAura,WaffenbegabungEmpfohleneNachteileMagischeEinschränkung(FluchdesEises)UngeeigneteVorteileUngeeigneteNachteileHitzeempfindlich,NiedrigeZähigkeit,Unfähig(Handwerkstalente)UngeeigneteNachteileAngstvorBlut,Behäbig,
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

//Mudran’Nura (Trollzacker Elementmeisterin)
profession!( 
    MudranNura(TrollzackerElementmeisterin),
    "Mudran’Nura (Trollzacker Elementmeisterin)",
    355VoraussetzungenKulturTrollzacker,VorteilZauberer(25),SonderfertigkeitTradition(Animisten)(125)SonderfertigkeitenSprachenundSchriftenfürinsgesamt2,BindungderWaffeKampftechnikenHiebwaffen10,Raufen10,Schwerter12,Zweihandhiebwaffen11TalenteKörperKlettern6,Körperbeherrschung5,Kraftakt6,Selbstbeherrschung5,Sinnesschärfe4,Verbergen3GesellschaftBekehren&Überzeugen3,Einschüchtern4NaturOrientierung4,Tierkunde3,Wildnisleben5WissenMagiekunde3,Sagen&Legenden4HandwerkHeilkundeKrankheiten3,HeilkundeWunden4,Holzbearbeitung3,Lederbearbeitung3,Steinbearbeitung5Zauber/MagischeHandlungeneinZaubertrickausfolgenderListeAbkühlung,Feuerfinger,TrockenZauberBasaltleib6,Nebelwand4,Wellenwand5AnimistenkräfteDurchfesteMaterie4,HarteHaut5,Steinmacht6,Talentverbesserung5EmpfohleneVorteileZäherHundEmpfohleneNachteileBlutrausch,LästigeMindergeister,MagischeEinschränkung(FluchderBerge)UngeeigneteVorteileSozialeAnpassungsfähigkeitUngeeigneteNachteileZerbrechlich,
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

//Nipakau-kocan (Mohischer Geisterkrieger)
profession!( 
    Nipakaukocan(MohischerGeisterkrieger),
    "Nipakau-kocan (Mohischer Geisterkrieger)",
    380VoraussetzungenKulturMohas,VorteilZauberer(25),SonderfertigkeitTradition(Animisten)(125)SonderfertigkeitenSprachenundSchriftenfürinsgesamt4,BindungderWaffeKampftechnikenBögen10,Hiebwaffen12,Raufen8, Stangenwaffen10TalenteKörperKörperbeherrschung6,Kraftakt4,Schwimmen3,Selbstbeherrschung5,Sinnesschärfe4,Verbergen5GesellschaftEinschüchtern5NaturFährtensuchen4,Orientierung4,Pflanzenkunde5,Tierkunde5,Wildnisleben5WissenGötter&Kulte4,Magiekunde2,Sagen&Legenden5HandwerkHeilkundeGift3,HeilkundeKrankheiten3, HeilkundeWunden4,Holzbearbeitung5Zauber/MagischeHandlungeneinZaubertrickaus folgenderListeAbkühlung,Duft,SchlangenhändeZauberArmatrutz4,Attributo(Gewandtheit)6, Axxeleratus5AnimistenkräfteBluttrinken4,ImmunitätgegenHitze5,Kampffähigkeitenverbessern 6,Talentverbesserung5EmpfohleneVorteileSchutzgeistEmpfohleneNachteileBlutrausch,Verpflichtungen (Stamm),WildeMagieUngeeigneteVorteileBeidhändigUngeeigneteNachteileAngstvor(Toten&Untoten),Unfähig(Naturtalente),
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

//Nuan (Nivesisches Wolfskind)
profession!( 
    Nuan(NivesischesWolfskind),
    "Nuan (Nivesisches Wolfskind)",
    350VoraussetzungenKulturNivesen,VorteilZauberer(25),SonderfertigkeitTradition(Animisten)(125),AhnenblutWolfsblütig(15),Ahnenblut-VorteilWolfskind(20)SonderfertigkeitenSprachenundSchriftenfürinsgesamt6,BindungderWaffeKampftechnikenBögen8,Raufen10,Stangenwaffen12TalenteKörperKörperbeherrschung5,Selbstbeherrschung5,Sinnesschärfe6,Verbergen5GesellschaftEinschüchtern3NaturFährtensuchen6,Orientierung5,Pflanzenkunde3,Tierkunde5,Wildnisleben6WissenGötter&Kulte4,Sagen&Legenden5HandwerkHeilkundeKrankheiten3,HeilkundeWunden5,Holzbearbeitung4,Lederbearbeitung3Zauber/MagischeHandlungeneinZaubertrickausfolgenderListeHandwärmer,TrockenZauberAdlerauge6,Axxeleratus4,Firnlauf5AnimistenkräfteGroßerSprung4,ImmunitätgegenKälte6,Talentverbesserung5,Unerschöpflich5EmpfohleneVorteileDunkelsicht,Flink,HerausragenderSinn(GehöroderGeruch),SchutzgeistEmpfohleneNachteileSensiblerGeruchssinn,SchlechteEigenschaft(Neugier),Persönlichkeitsschwäche(Weltfremd),Verpflichtungen(Sippe),WildeMagieUngeeigneteVorteileMagischeEinstimmung(WesenderNacht)UngeeigneteNachteileBehäbig,Unfähig(Naturtalente),
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
