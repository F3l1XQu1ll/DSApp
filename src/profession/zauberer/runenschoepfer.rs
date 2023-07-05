use crate::profession;

//Runenschöpfer (Runahöfundur)
profession!(
    Runahoefundur,
    "Runenschöpfer (Runahöfundur)",
    352,
    vec![
        KulturThorwal,
        VorteilZauberer(25),
        SonderfertigkeitTradition(Runenschoepfer)(75)
    ],
    vec![SprachenSchriften10],
    vec![Dolche8, Hiebwaffen10, Raufen8],
    vec![],
    vec![],
    vec![],
    vec![BegabungineinemHandwerkstalent, Glueck],
    vec![],
    vec![],
    vec![],
    vec![]
);
