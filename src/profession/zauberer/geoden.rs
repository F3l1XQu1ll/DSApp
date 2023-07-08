use crate::crate_prof;
use crate::profession;

crate_prof!();

profession!( 
    DienerderErdmutter,
    "Diener der Erdmutter",
    352,
    vec![SpeziesZwerg(61),VorteilZauberer(25),SonderfertigkeitTradition(Geoden)(125)],
    vec![SprachenundSchriftenfürinsgesamt4,BindungdesRings,DienerderErdmutter],
    vec![Dolche8],
    vec![KörperSelbstbeherrschung4,Sinnesschärfe4GesellschaftMenschenkenntnis4,Willenskraft4NaturFährtensuchen3,Orientierung5,Pflanzenkunde 6,Tierkunde6,Wildnisleben6WissenGeschichtswissen3,Götter&Kulte5,Magiekunde5,Sagen&Legenden5,Sternkunde3 HandwerkHeilkundeGift3,HeilkundeKrankheiten4,HeilkundeSeele4,HeilkundeWunden4,Holzbearbeitung5,Steinbearbeitung6],
    vec![einZaubertrickausfolgenderListeHundekeks, Lockruf,Steinempathie,TrockenZauberEinsmitderNatur 4,ElementarerDiener5,Ignifaxius6,Flammenwand 5,GroßeVerwirrung4,Manifesto10,Tiergedanken (Elfen)6],
    vec![],
    vec![AffinitätzuElementaren,Altersresistenz,HoheSeelenkraft,VerhüllteAura],
    vec![Angstvor…(engenRäumen),KörpergebundeneKraft,LästigeMindergeister,Persönlichkeitsschwäche(Weltfremd),Unfähig (Gesellschaftstalente)],
    vec![EisenaffineAura,Soziale Anpassungsfähigkeit],
    vec![Angstvor…(ToteundUntote),NiedrigeSeelenkraft,Schwerzuverzaubern],
    vec![GefährtedesErzes(352)Archofaxius6stattIgnifaxius,Steinwand5stattFlammenwandGefährtedesHumus(352)Dornenwand5statt Flammenwand,Humofaxius6stattIgnifaxiusGefährtederLuft(352)Orcanofaxius6stattIgnifaxius,Sturmwand5stattFlammenwand GefährtedesWassers(354)Aquafaxius(6statt Ignifaxius,Wellenwand5stattFlammenwand)]
);

profession!( 
    HerrderErde,
    "Herr der Erde",
    349,
    vec![SpeziesZwerg(61),VorteilZauberer(25),SonderfertigkeitTradition(Geoden)(125)],
    vec![SprachenundSchriftenfürinsgesamt4,BindungdesRings],
    vec![Dolche8,Raufen8],
    vec![KörperSelbstbeherrschung4,Sinnesschärfe4GesellschaftBekehren&Überzeugen4,Menschenkenntnis2,Überreden3,Willenskraft4 NaturOrientierung4,Pflanzenkunde5,Tierkunde5,Wildnisleben5WissenGeschichtswissen4,Götter&Kulte4,Magiekunde5,Sagen&Legenden5,Sternkunde2HandwerkHeilkundeGift3,HeilkundeSeele6,HeilkundeWunden3,Holzbearbeitung4,Steinbearbeitung6],
    vec![einZaubertrickausfolgenderListeAbkühlung, Feuerfinger,Hundekeks,SteinempathieZauber BöserBlick 5,ElementarerDiener6,Hagelschlag4,Herrüberdas Tierreich5,Manifesto10,Orcanofaxius5,Sturmwand5],
    vec![],
    vec![AffinitätzuElementaren,Altersresistenz,HoheSeelenkraft,VerhüllteAura],
    vec![Angstvor(engenRäumen), KörpergebundeneKraft,LästigeMindergeister,Persönlichkeitsschwäche(Arroganz)],
    vec![EisenaffineAura,Soziale Anpassungsfähigkeit],
    vec![Angstvor…(ToteundUntote),NiedrigeSeelenkraft,Schwerzuverzaubern],
    vec![ Eisherr(349)Eiswand5stattSturmwand,Frigifaxius5stattOrcanofaxius,Lawinenfall4statt HagelschlagErzherr(344)Archofaxius5stattOrcanofaxius, Fesselfeld4stattHagelschlag,Steinwand5statt SturmwandFeuerherr(349)Flammenwand5stattSturmwand, Ignifaxius5stattOrcanofaxius,Ignisphaero4statt HagelschlagHumusherr(339)Dornenwand5stattSturmwand,Humofaxius6stattOrcanofaxius,Invinculo 4stattHagelschlagWasserherr(349)Aquafaxius5stattOrcanofaxius,Aquasphaero4stattHagelschlag,Wellenwand5stattSturmwand]
);
