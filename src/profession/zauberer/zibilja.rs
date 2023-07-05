use crate::profession;

//Zibilja
profession!(
    Zibilja,
    "Zibilja",
    334,
    vec![
        KulturNorbarden,
        VorteilZauberer(25),
        SonderfertigkeitTradition(Zibilja)(100)
    ],
    vec![SprachenSchriften8, BindungderChronik],
    vec![Dolche8],
    vec![],
    vec![],
    vec![],
    vec![Tierfreund, Vertrauenerweckend, Wohlklang],
    vec![Artefaktgebunden],
    vec![],
    vec![Blutrausch, SchlechteEigenschaft(Jaehzorn, Rachsucht), Stumm],
    vec![]
);
