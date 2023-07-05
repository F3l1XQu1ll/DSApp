use crate::profession;

//Schelmin
profession!(
    Schelmin,
    "Schelmin",
    219,
    vec![SonderfertigkeitTradition(Schelm)(125), Zauberer(25)],
    vec![
        SprachenSchriften6,
        FertigkeitsspezialisierungGaukeleien,
        BindungderNarrenkappe
    ],
    vec![Raufen10],
    vec![],
    vec![],
    vec![],
    vec![Flink, GeborenerRedner, Schwerzuverzaubern],
    vec![LaestigeMindergeister, SchlechteEigenschaften(Neugier)],
    vec![SozialeAnpassungsfaehigkeit],
    vec![Blutrausch, Unfrei],
    vec![]
);
