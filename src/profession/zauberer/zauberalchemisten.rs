use crate::crate_prof;
use crate::profession;

crate_prof!();

//Zauberalchimistin
profession!(
    Zauberalchimistin,
    "Zauberalchimistin",
    176,
    vec![
        SonderfertigkeitTradition(Zauberalchimisten)(45),
        Zauberer(25)
    ],
    vec![
        SprachenSchriften8,
        FertigkeitsspezialisierungAlchimie,
        BindungderAlchimistenschale
    ],
    vec![Raufen10],
    vec![],
    vec![],
    vec![],
    vec![Begabung(Alchimie), HoheSeelenkraft],
    vec![LaestigeMindergeister, SchlechteEigenschaften(Neugier)],
    vec![SozialeAnpassungsfaehigkeit],
    vec![Unfrei],
    vec![]
);
