use crate::vorteil;
use crate::vorteile::Vorteil;

pub struct AngstvorIBisIII {
    stufe: u8,
}

vorteil! {
    @noauto
    AngstvorIBisIII,
    "Angst vor … I-III",
    "Keine",
    "Pro Stufe der Angst erleidet der Held eine Stufe Furcht, so lange er dem Auslöser ausgesetzt ist.",
    @call |perk: &AngstvorIBisIII| {perk.stufe as u16 * 8},
    true
}

pub struct ArmIBisIII {
    stufe: u8,
}

vorteil! {
    @noauto
    ArmIBisIII,
    "Arm I-III",
    "Kein Vorteil Reich",
    "Durch den Nachteil Arm erhält ein Held bei der Generierung 250 Silbertaler weniger pro Stufe. Wählt er also 3 Stufen Arm, beginnt er ohne Startkapital und besitzt nur ein Kleiderpaket der Kategorie arm.",
    @call |perk: &ArmIBisIII| {perk.stufe as u16},
    true
}

vorteil! {
    Artefaktgebunden,
    "Artefaktgebunden",
    "Vorteil Zauberer",
    "Ist der Held nicht in direktem Kontakt mit dem Artefakt (dies bedeutet: Kontakt mit der Haut), kann er nur erschwert Zaubern. Dieser Nachteil erschwert Zauber um 1, wenn die Bedingung des Körperkontaktes nicht erfüllt ist. Wird das Artefakt vernichtet, kann er so lange nur mit der Erschwernis Zauber wirken, bis er selbst ein neues Artefakt gleicher Machart hergestellt hat, in das die Artefaktgebundenheit automatisch einfließt.",
    10,
    true
}

vorteil! {
    Behaebig,
    "Behäbig",
    "Kein Vorteil Flink",
    "Durch Behäbig verliert ein Held 1 Punkt von seinem Geschwindigkeit-Grundwert.",
    4,
    true
}

vorteil! {
    Blind,
    "Blind",
    "Kein Vorteil Herausragender Sinn (Sicht), Keine Nachteil Einäugig, Keine Nachteil Farbenblind, Keine Nachteil Nachtblind, Keine Nachteil Eingeschränkter Sinn (Sicht)",
    "Durch den Nachteil ist ihm der Einsatz vieler Fertigkeiten erschwert oder erst gar nicht möglich. Dies gilt ebenfalls für Sonderfertigkeiten. Welche Handlungen um wie viel erschwert oder gar unmöglich sind, entscheidet der Meister. Die Blindheit ist nicht heilbar, außer durch ein Großes Wunder. Der Held erhält den Status Blind.",
    50,
    true
}

vorteil! {
    Blutrausch,
    "Blutrausch",
    "Schlechte Eigenschaft Jähzorn, Kein Nachteil Angst vor Blut",
    "Der Held erhält unter speziellen Umständen den Status Blutrausch. Der Blutrausch wird aktiviert, wenn der Held durch einen Angriff eine Stufe des Zustands Schmerz erleidet oder ihm seine Probe auf Willenskraft zur Aktivierung der Schlechten Eigenschaft Jähzorn misslingt.",
    10,
    true
}

vorteil! {
    BoeserNamensvetter,
    "Böser Namensvetter",
    "Ein Name mit Wiedererkennungswert, der aber nicht zu exotisch ist, Kein Vorteil Allerweltsname, Elfen können diesen Vorteil nicht wählen,  da ihre Namen einzigartig sind.",
    "Der Held gerät ständig in Schwierigkeiten, weil  sich jemand für ihn ausgibt oder schlimmer: tatsächlich so heißt wie er. Dies kann nach Meisterentscheid  Erschwernisse von 1 bis 3 auf Gesellschaftstalente  nach sich ziehen (mit Ausnahme von Menschenkenntnis und Willenskraft).  Die  Häufigkeit  und  die  Auswirkungen sollten von Spielerin und Spielleiterin zusammen festgelegt werden.",
    10, // NOTE: CAN be between 2 and 20 depending on the campaign, however this seems out of scope
    true
}

pub struct EingeschraenkterSinn {
    sinn: super::vorteile::Sinn,
}

vorteil! {
    @noauto
    EingeschraenkterSinn,
    "Eingeschränkter Sinn",
    "Kein Vorteil Herausragender Sinn für den gleichen Sinn, Kein Nachteil Blind (bei Sicht), Kein Nachteil Taub (bei Gehör)",
    "Wer über einen Eingeschränkten Sinn verfügt, dessen Proben auf Sinnesschärfe sind um 2 erschwert, wenn die Probe den entsprechenden Sinn betrifft. Eingeschränkte Sicht erschwert zudem Proben auf Fernkampf um 2. Folgende Sinne können gewählt werden: Sicht, Gehör, Geruch & Geschmack, Tastsinn. Ein Held kann bis zu zwei eingeschränkte Sinne besitzen.",
    @call |perk: &EingeschraenkterSinn| {
        match perk.sinn {
            super::vorteile::Sinn::Sicht => 15,
            super::vorteile::Sinn::Gehör => 10,
            super::vorteile::Sinn::Geruch |
            super::vorteile::Sinn::Geschmack => 6,
            super::vorteile::Sinn::Tastsinn => 2,
        }
    },
    true
}

vorteil! {
    EmpfindlichkeitunedleMetalle,
    "Empfindlichkeit (unedle Metalle) (*)",
    "Keine",
    "Verliert der Held Lebenspunkte durch Metallwaffen, steigen die Schadenspunkte um 4. Eine einfache Berührung mit entsprechenden Metallen richtet 1 Schadenspunkt pro 5 vollen Minuten an. Bei Unterbrechungen addieren sich die Teilzeiten auf, solange die Unterbrechungen die Dauer von 1 Stunde nicht überschreiten. Diese Empfindlichkeit gilt gegen unedle Metalle (z. B: Eisen, Stahl, Blei, Kupfer) nicht aber gegen magische Metalle oder Edelmetalle.",
    30,
    true
}

vorteil! {
    Farbenblind,
    "Farbenblind",
    "Kein Nachteil Blind",
    "Für den Abenteurer sind Farben nicht zu unterscheiden. Dies kann sich bei einigen Proben, etwa auf Alchimie („Rote Flüssigkeit in grüne Flüssigkeit schütten.\n“), beim Zuordnen von Wappen (Etikette) oder bei der Orientierung im Dschungel negativ auswirken. Ob eine Probe abgelegt werden kann und wie hoch die Erschwernis ist, entscheidet der Meister.",
    2,
    true
}

vorteil! {
    Fettleibig,
    "Fettleibig",
    "Nachteil Behäbig",
    "Der Nachteil hat zur Folge, dass Proben auf Klettern, Körperbeherrschung, Tanzen und Verbergen um 1 erschwert sind.",
    25,
    true
}

vorteil! {
    Friedlos,
    "Friedlos",
    "Der Held muss der Kultur Thorwaler angehören.",
    "Friedlose gelten bei den Thorwalern als Ausgestoßene. Alle Proben auf Bekehren & Überzeugen, Etikette, Handel und Überreden erhalten gegenüber Thorwalern eine Erschwernis von –1. Diese Erschwernis gilt nicht gegenüber Nicht- Thorwalern. Der soziale Stand eines Friedlosen ist zudem immer Stufe I (siehe Regelwerk Seite 338).",
    20,
    true
}

pub struct GiftanfaelligIBisII {
    stufe: u8,
}

vorteil! {
    @noauto
    GiftanfaelligIBisII,
    "Giftanfällig I-II",
    "Kein Vorteil Giftresistenz",
    "Eine Giftanfälligkeit verschlechtert Zähigkeit und Seelenkraft um 1 pro Stufe, wenn es darum geht, Giften zu widerstehen.",
    @call |perk: &GiftanfaelligIBisII| {perk.stufe as u16 * 5},
    true
}

vorteil! {
    Glaesern,
    "Gläsern",
    "Keine",
    "Die Wundschwelle des Helden sinkt durch den Nachteil um 1.",
    3,
    true
}

pub struct HaesslichIBisII {
    stufe: u8,
}

vorteil! {
    @noauto
    HaesslichIBisII,
    "Hässlich I-II",
    "Kein Vorteil Gutaussehend",
    "Durch jede Stufe dieses Nachteils erleidet der Held eine Erschwernis von 1 auf Betören (Anbändeln, Liebeskünste), Überreden (Manipulieren, Schmeicheln) und Handel (Feilschen).",
    @call |perk: &HaesslichIBisII| {perk.stufe as u16 * 10},
    true
}

vorteil! {
    Hitzeempfindlich,
    "Hitzeempfindlich",
    "Kein Vorteil Hitzeresistenz",
    "Hitzestufen werden für den Helden um 1 Stufe erhöht, sobald er mindestens unter Hitzestufe 2 leidet.",
    3,
    true
}

vorteil! {
    HitzeempfindlichII,
    "Hitzeempfindlich II (*)",
    "Kein Vorteil Hitzeresistenz, Kein Nachteil Hitzeempfindlich I",
    "Hitzestufen werden für den Helden um 2 Stufen erhöht, sobald er mindestens unter Hitzestufe 2 leidet.",
    6,
    true
}

vorteil! {
    InstabilerZauberer,
    "Instabiler Zauberer",
    "Vorteil Zauberer",
    "Würfelt der Held einen Patzer bei einem Zauber oder einer magischen Handlung, durchdringt ihn magische Energie und er erleidet 2W6 SP.",
    5,
    true
}

vorteil! {
    Jagdwildgeruch,
    "Jagdwildgeruch",
    "Keine",
    "Der Meister kann immer, wenn der Held nahegenug an hungrigen Raubtieren ist (50 Schritt Radius) und diese ihn wittern, mit dem W6 würfeln. Bei einer 1-2 halten sie den Abenteurer für Beute und greifen ihn an. Sollte es zu einem Kampf mit einer ganzen Heldengruppe kommen, so greifen die Raubtiere bevorzugt einen Helden mit dem Nachteil Jagdwildgeruch an. Als Raubtiere gelten beispielsweise Jaguare und Würgeschlangen, auf keinen Fall aber reine Pflanzenfresser. Welche Tiere von dem Geruch genau angelockt werden, entscheidet der Meister.",
    10,
    true
}

vorteil! {
    Kaelteempfindlich,
    "Kälteempfindlich",
    "Kein Vorteil Kälteresistenz",
    "Kältestufen werden für den Helden um 1 Stufe erhöht, sobald er mindestens unter Kältestufe 1 leidet.",
    3,
    true
}

vorteil! {
    Kaeltestarre,
    "Kältestarre (*)",
    "Spezies, Kultur oder Profession muss Vorteil als automatischen oder empfohlenen Vorteil aufweisen.",
    "Statt Kälteschaden zu erleiden, versetzt sich der Körper eines Achaz automatisch in eine Kältestarre. Er bekommt jede Minute bei Temperaturen, in denen er Kälteschaden erleiden würde, 1 Stufe Paralyse und bei der IV Stufe zusätzlich den Status Bewusstlos. In der Kältestarre kann er 1W20 Jahre überleben. Gelangt er wieder in eine wärmere Gegend, so bauen sich die Stufen Paralyse alle volle 30 Minuten um 1 Stufe ab. Ab Stufe III verliert er den Status Bewusstlos.",
    15,
    true
}

vorteil! {
    KeinVertrauter,
    "Kein Vertrauter (*)",
    "Vorteil Zauberer, Zauberertradition kann Sonderfertigkeit Vertrautenbindung wählen",
    "Der Held kann die Sonderfertigkeit Vertrautenbindung nicht erwerben. Dadurch wird die Sonderfertigkeit Tradition (Hexen) um 10 AP billiger.",
    25,
    true
}

vorteil! {
    KeineFlugsalbe,
    "Keine Flugsalbe (*)",
    "Vorteil Zauberer, Zauberertradition kann Sonderfertigkeit Flugsalbe wählen",
    "Der Held kann die Sonderfertigkeit Flugsalbe nicht erwerben. Dadurch wird die Sonderfertigkeit Tradition (Hexen) um 10 AP billiger.",
    25,
    true
}

pub struct KleineZauberauswahlIBisII {
    stufe: u8,
}

vorteil! {
    @noauto
    KleineZauberauswahlIBisII,
    "Kleine Zauberauswahl I-II",
    "Sonderfertigkeit Tradition Intuitive Zauberer, Animisten oder Runenschöpfer, Kein Vorteil Große Zauberauswahl",
    "Der intuitive Zauberer muss pro Stufe einen der drei Zauber streichen, über die er maximal verfügen kann.",
    @call |perk: &KleineZauberauswahlIBisII| {perk.stufe as u16 * 10},
    true
}

vorteil! {
    KoerpergebundeneKraft,
    "Körpergebundene Kraft",
    "Vorteil Zauberer, Spezies des Zauberers muss über Haare verfügen",
    "Sobald der Held, aus welchem Grunde auch immer, einen Teil seiner Haarpracht einbüßt (z. B. durch Abschneiden, Feuer etc.\n; unabhängig von der Länge), verliert er sofort 10 Astralpunkte (bis zu einem Minimum von 0). Diese können normal regeneriert werden. Der normale menschliche Haarverlust zählt nicht als Verlust der Haarpracht.",
    5,
    true
}

vorteil! {
    KoerperlicheAuffaelligkeit,
    "Körperliche Auffälligkeit",
    "Keine",
    "Ein Held kann maximal zwei Körperliche Auffälligkeiten wählen. Die Körperliche Auffälligkeit zieht gelegentlich Erschwernisse auf Gesellschaftstalente aufgrund von Aberglaube oder Misstrauen nach sich.",
    2,
    true
}

pub struct KrankheitsanfaelligIBisII {
    stufe: u8,
}

vorteil! {
    @noauto
    KrankheitsanfaelligIBisII,
    "Krankheitsanfällig I-II",
    "Kein Vorteil Krankheitsresistenz",
    "Eine Krankheitsanfälligkeit verschlechtert Zähigkeit und Seelenkraft um 1 pro Stufe, wenn es darum geht, Krankheiten zu widerstehen.",
    @call |perk: &KrankheitsanfaelligIBisII| {perk.stufe as u16 * 5},
    true
}

vorteil! {
    LaecherlicherName,
    "Lächerlicher Name",
    "Ein möglichst lächerlicher Name, Kein Vorteil Allerweltsname, Kein Vorteil Heldenhafter Name, Kein Nachteil Schurkenname",
    "Die Heldin hatte schon immer einen lächerlichen Namen, oder aber sie hat sich irgendwann einen  ehrenrührigen Beinamen verdient. Wenn die Heldin  ihren Namen nennt, muss ihr Gegenüber eine Probe  auf Willenskraft bestehen, um nicht unangemessen zu  reagieren. Hat sich jemand an den Namen gewöhnt,  entfällt diese Probe.",
    3,
    true
}

vorteil! {
    LaestigeBluetenfeen,
    "Lästige Blütenfeen",
    "Vorteil Zauberer",
    "Jedes Mal, wenn der Zauberer Astralenergie ausgibt und es eine Gegend ist, in der Blütenfeen leben, würfelt er mit dem W20. Ist das Ergebnis gleich oder kleiner der ausgegebenen Astralenergiemenge, dann tauchen eine oder mehrere Blütenfeen auf und belästigen den Zauberer. Alle folgenden Fertigkeitsproben sind in diesem Fall wegen der Ablenkung um 1 erschwert. Die Feenbleiben für 10 Minuten, ehe sie von selbst wieder verschwinden. Man kann nicht von mehr als einer Blütenfee gleichzeitig abgelenkt werden, die Erschwernis steigt also nicht durch weitere Feen, die durch den Nachteil erscheinen.",
    10,
    true
}

vorteil! {
    LaestigeMindergeister,
    "Lästige Mindergeister",
    "Vorteil Zauberer",
    "Jedes Mal wenn der Zauberer Astralenergie ausgibt, würfelt er mit dem W20. Ist das Ergebnis gleich oder kleiner der ausgegebenen Astralenergiemenge, tauchen Mindergeister auf und belästigen den Zauberer. Welche Art von Mindergeistern erscheinen, entscheidet der Meister je nach Situation. Beispiele sind Windbeutel, die das Haar des Zauberers selbst bei Windstille wallen lassen, Tränenlinge, die sich unter die Augen setzen, oder Feuerwusel, die Gegenstände des Zauberers erhitzen oder gar verkohlen. Außerdem sind alle Fertigkeitsproben wegen der Ablenkung um 1 erschwert. Die Mindergeister bleiben für 10 Minuten, ehe sie von selbst wieder verschwinden. Sind durch zu häufiges Zaubern mehrere Mindergeister erschienen, summieren sich die Erschwernis dennoch nicht auf.",
    20,
    true
}

vorteil! {
    Lernfaul,
    "Lernfaul",
    "Tradition (Schelme)",
    "Der Schelm kann keine Zauber in Schelmenstreiche umwandeln.",
    15,
    true
}

vorteil! {
    Lichtempfindlich,
    "Lichtempfindlich",
    "Keine",
    "Der Held erleidet eine Stufe Schmerz, sobald er Sonnenlicht ausgesetzt ist, das heller ist als Dämmerlicht. Diese Auswirkung lässt sich vermeiden, wenn der Betroffene seinen kompletten Körper inklusive der Augen verhüllt.",
    20,
    true
}

vorteil! {
    LichtempfindlichII,
    "Lichtempfindlich II (*)",
    "Kein Nachteil Lichtempfindlich I",
    "Der Held erleidet zwei Stufen Schmerz, sobald er Sonnenlicht ausgesetzt ist, das heller ist als Dämmerlicht. Diese Auswirkung lässt sich vermeiden, wenn der Betroffene seinen kompletten Körper inklusive der Augen verhüllt.",
    30,
    true
}

vorteil! {
    MagischeEinschraenkung,
    "Magische Einschränkung",
    "Vorteil Zauberer",
    "In der Umgebung, für die der Nachteil gilt, hat der Zauberer bei magischen Fertigkeitsproben keine Erschwernisse, ansonsten eine von 1.\nBeispiel für Magische Einschränkung (Umgebung)• Fluch der Auen• Fluch der Berge• Fluch des Eises• Fluch der Meere• Fluch der Nacht• Fluch des WaldesAnmerkung: Ein Fluch der Helligkeit ist möglich.",
    30,
    true
}

vorteil! {
    MisslungeneReifepruefung,
    "Misslungene Reifeprüfung",
    "Keine",
    "Unter Angehörigen ihrer Kultur hat der Held eine Erschwernisvon –1 auf Gesellschaftstalente außer Menschenkenntnis und Willenskraft, sofern das Gegenüber von dermisslungenen Reifeprüfung weiß. Sollten Angehörige die misslungene Reifeprüfung als nicht besonders schlimm empfinden, gilt die Erschwernis nicht.\n*",
    10,
    true
}

vorteil! {
    Nachtblind,
    "Nachtblind",
    "Kein Vorteil Dunkelsicht, Kein Nachteil Blind",
    "Sobald der Held mindestens eine Stufe Sichterschwernis durch Dunkelheit erleidet, wird diese Erschwernis für ihn um eine Stufe erhöht.",
    10,
    true
}

pub struct NiedrigeAstralkraftIBisVII {
    stufe: u8,
}

vorteil! {
    @noauto
    NiedrigeAstralkraftIBisVII,
    "Niedrige Astralkraft I-VII",
    "Vorteil Zauberer, Kein Vorteil Hohe Astralkraft",
    "Der AE-Grundwert sinkt durch diesen Nachteil um 1 Punkt pro Stufe des Nachteils.",
    @call |perk: &NiedrigeAstralkraftIBisVII| {perk.stufe as u16 * 2},
    true
}

pub struct NiedrigeKarmalkraftIBisVII {
    stufe: u8,
}

vorteil! {
    @noauto
    NiedrigeKarmalkraftIBisVII,
    "Niedrige Karmalkraft I-VII",
    "Vorteil Geweihter, Kein Vorteil Hohe Karmalkraft",
    "Der KE-Grundwert sinkt durch diesen Nachteil um 1 Punkt pro Stufe des Nachteils.",
    @call |perk: &NiedrigeKarmalkraftIBisVII| {perk.stufe as u16 * 2},
    true
}

pub struct NiedrigeLebenskraftIBisVII {
    stufe: u8,
}

vorteil! {
    @noauto
    NiedrigeLebenskraftIBisVII,
    "Niedrige Lebenskraft I-VII",
    "Kein Vorteil Hohe Lebenskraft",
    "Der LE-Grundwert sinkt durch diesen Nachteil um 1 Punkt pro Stufe des Nachteils.",
    @call |perk: &NiedrigeLebenskraftIBisVII| {perk.stufe as u16 * 4},
    true
}

vorteil! {
    NiedrigeSeelenkraft,
    "Niedrige Seelenkraft",
    "Kein Vorteil Hohe Seelenkraft",
    "Der SK-Grundwert des Helden ist um 1 schlechter.",
    25,
    true
}

vorteil! {
    NiedrigeZaehigkeit,
    "Niedrige Zähigkeit",
    "Kein Vorteil Hohe Zähigkeit",
    "Der ZK-Grundwert des Helden ist um 1 schlechter.",
    25,
    true
}

pub struct PechIBisIII {
    stufe: u8,
}

vorteil! {
    @noauto
    PechIBisIII,
    "Pech I-III",
    "Kein Vorteil Glück",
    "Der Held startet pro Stufe des Nachteils mit 1 Schicksalspunkt weniger als üblich. Sein Maximum an Schicksalspunkten sinkt hierdurch um einen Punkt pro Stufe. Die Anzahl der Schicksalspunkte darf hierbei nicht unter 0 fallen.",
    @call |perk: &PechIBisIII| {perk.stufe as u16 * 20},
    true
}

vorteil! {
    Pechmagnet,
    "Pechmagnet",
    "Keine",
    "Bei jeder zufälligen Bestimmung des Meisters (Welcher Held wird von dem Pfeil getroffen? Wer stand von der Gruppe in der Nähe der Falle, als sie ausgelöst wurde?) sind die Chancen bei einem Abenteurer mit diesem Nachteil mindestens doppelt so hoch wie die eines Helden ohne Pechmagnet. Der Nachteil sollte in der Heldengruppe nicht bei allen Helden auftreten, da sich der Nachteil ansonsten aufhebt. Dementsprechend erfordert die Wahl von Pechmagnet Rücksprache mit dem Meister.",
    5,
    true
}

pub enum Persoenlichkeitsschwaechen {
    Arroganz,
    Eitelkeit,
    Neid,
    Streitsucht,
    Unheimlich,
    Verwöhnt,
    Vorurteile,
    Weltfremd,
}

impl Persoenlichkeitsschwaechen {
    pub fn ap(&self) -> u16 {
        match *self {
            Persoenlichkeitsschwaechen::Arroganz => 10,
            Persoenlichkeitsschwaechen::Eitelkeit => 10,
            Persoenlichkeitsschwaechen::Neid => 5,
            Persoenlichkeitsschwaechen::Streitsucht => 10,
            Persoenlichkeitsschwaechen::Unheimlich => 8,
            Persoenlichkeitsschwaechen::Verwöhnt => 10,
            Persoenlichkeitsschwaechen::Vorurteile => 5,
            Persoenlichkeitsschwaechen::Weltfremd => 10,
        }
    }
}

pub struct Persoenlichkeitsschwaeche {
    schwaeche: Persoenlichkeitsschwaechen,
    schwaeche2: Option<Persoenlichkeitsschwaechen>,
}

vorteil! {
    @noauto
    Persoenlichkeitsschwaeche,
    "Persönlichkeitsschwäche",
    "Keine",
    "In passenden Situationen kann der Meister durchaus für die angegebenen Fertigkeiten eine Erschwernis von 1 aussprechen, wenn er dies für angemessen hält. Es dürfen bis zu zwei Persönlichkeitsschwächen pro Held gewählt werden.\n\nBeispiele:\n# Arroganz: Der Held hält sich für etwas Besseres und lässt dies jeden spüren.\nMögliche Erschwernisse: auf Handeln (Feilschen) und Gesellschaftstalente außer Einschüchtern und WillenskraftAP-Wert: –10 Abenteuerpunkte\n# Eitelkeit: Der Held hasst es, nicht standesgemäß gekleidet und frisiert oder sonst wie in seinem Auftreten beeinträchtigt zu sein.\nMögliche Erschwernisse: auf alle GesellschaftstalenteAP-Wert: –10 Abenteuerpunkte\n# Neid: Ein Held mit dieser Persönlichkeitsschwäche begehrt den Reichtum, die Frau oder die Waffe anderer.\nMögliche Erschwernisse: auf Handeln (Feilschen) und Gesellschaftstalente außer Einschüchtern und WillenskraftAP-Wert: –5 Abenteuerpunkte\n# Streitsucht: Jeder streitet sich mal, doch dieser Held macht sich unbeliebt, da er ständig und über den nichtigsten Grund mit anderen in Auseinandersetzungen gerät.\nMögliche Erschwernisse: auf Handeln (Feilschen) und Gesellschaftstalente außer Einschüchtern und WillenskraftAP-Wert: –10 Abenteuerpunkte\n# Unheimlich: Von dem Helden geht eine unangenehme Aura aus. Dies kann ein stechender Blick, seine mürrische Art oder ein unerklärbares Gefühl für seine Gefährlichkeit sein.\nMögliche Erschwernisse: alle Gesellschaftstalente außer Einschüchtern und WillenskraftAP-Wert: –8 Abenteuerpunkte\n# Verwöhnt: Verwöhnte Helden lassen sich gerne bedienen, sind faul und vermeiden jegliche Anstrengung. Ohne die nötigen Annehmlichkeiten wird der Held schnell unausstehlich, weinerlich oder aggressiv. Mögliche Erschwernisse: auf alle GesellschaftstalenteAP-Wert: –10 Abenteuerpunkte\n# Vorurteile: Manche Helden haben starke Vorteile gegen Nichtmenschen, Frauen, Männer, Zauberer, bestimmte Kulte oder andere Gruppen. Interaktion mit diesen Personen ist für diesen Helden eine echte Herausforderung. Vorurteile können mehrfach gewählt werden, allerdings erhält man nur einmal den AP-Wert.\nMögliche Erschwernisse: auf Handeln (Feilschen) und Gesellschaftstalente außer Einschüchtern und WillenskraftAP-Wert: –5 Abenteuerpunkte\n# Weltfremd: Der Held kennt sich mit vielen Aspekten des täglichen Lebens auf Dere kaum aus: Er hat keine Ahnung von Geld oder gesellschaftlichen Verhaltensweisen, die Regeln der Etikette sind ihm völlig unbekannt oder er hat kein Gefühl für die Errungenschaften der Zivilisation. Wie bei Vorurteile muss diese Schlechte Eigenschaft auf bestimmte Bereiche begrenzt werden, z.\nB. Adel, Besitz, Fortschritt, Geld, Götter, Hierarchien, Religion, Welt außerhalb dereigenen Kultur. Weltfremd kann mehrfach gewählt werden, allerdings erhält man nur für zwei Bereiche jeweils den AP-Wert (also insgesamt –20 AP).\nMögliche Erschwernisse: auf Handeln (Feilschen) und alle GesellschaftstalenteAP-Wert: –10 Abenteuerpunkte",
    @call |perk: &Persoenlichkeitsschwaeche| {
        let ap = perk.schwaeche.ap();
        let extra = if let Some(schwaeche) = &perk.schwaeche2 {
            schwaeche.ap()
        }else {0};
        ap + extra
    },
    true
}

pub struct PrinzipientreueIBisIII {
    stufe: u8,
}

vorteil! {
    @noauto
    PrinzipientreueIBisIII,
    "Prinzipientreue I-III",
    "Keine",
    "Prinzipientreue gibt es in drei Stufen. Während die erste Stufe den Helden nur leicht einschränkt, ist die zweite Stufe schon fordernd, die letzte Stufe benachteiligt den Helden schwer. Ein Held kann mehreren Prinzipien folgen, er bekommt jedoch nur einmal Abenteuerpunkte für den Nachteil (und zwar für die höchste Prinzipientreuestufe). Wer gegen seine Prinzipien verstößt, erleidet eine Erschwernis von 1 auf alle Fertigkeitsproben. Dieser Zustand dauert mindestens eine Stunde an, über die genaue Länge entscheidet der Meister unter Berücksichtigung der Schwere des Verstoßes.\n\n [Anmerkung: Im folgenden den jeweiligen Moralkodex für Geweihte bzw (weiter unten) den für Schamanen / (ungeweihte) Priestern als Beispiele des Nachteils.\n]\nMoralkodici der Geweihten:\nMoralkodex der Angroschkirche (Prinzipientreue II)# Wahrung des Feuers: In der Gegenwart des Angroschgeweihten sollen alle Feuer brennen. Haus- und Waldbrände dürfen aber eingedämmt werden, und wenn ein Gläubiger durch den Brand in direkte Gefahr gerät, darf auch ein Angroschgeweihter Feuer löschen.\n# Zusammenhalt: Der Angroschgeweihte trägt dafür Sorge, dass alle Zwerge zusammenhalten, schlichtet Streit unter ihnen und versucht, den Frieden unter den Angehörigen seines Volkes zu wahren.\n# Drachenwacht: Der Angroschgeweihte soll die Zwerge und die gesamte Welt vor dem Einfluss der Drachen beschützen.\n\nMoralkodex der Aveskirche (Prinzipientreue I)# Gefährten helfen: Reisegefährten in der Not soll ein Avesgeweihter stets helfen.\n# Reiseberichte anfertigen: Abenteuerreisen sollten immer auch schriftlich festgehalten oder mündlich weitergegeben werden.\n# Ungebundenheit: Ein Avesgeweihter soll keine Eide leisten, die ihn an einen Ort binden (außer einen Avestempel – und auch das nur für eine bestimmte Zeit).\n# Überschreitung des Horizonts: Der Avesgeweihte sollte immer danach streben, seinen Horizont in Form einer Reise zu erweitern.\n\nMoralkodex der Borongeweihten (Prinzipientreue II)# Bestattung: Kein Leichnam sollte unbestattet sein. Der Geweihte muss für die Totenruhe sorgen.\n# Schweigen: Schweigen ist eine Tugend. Kein Geweihter sollte ohne Grund reden.\n# Traum: Ergründe die Welt der Träume. In ihr kann der Geweihte den Willen Borons ergründen.\n\nMoralkodex der Chr’Ssir’Ssr-Priester (Prinzipientreue II) # Macht des Chr’Ssir’Ssr: Der Priester dient der Gottheit, die über allen anderen Göttern steht.\n# Sturmruf: Der Priester wacht über das Wetter und erforscht dessen Phänomene. Er versucht hinter das Geheimnis der Wetterkontrolle zu gelangen.\n\nMoralkodex der Efferdgeweihten (Prinzipientreue II)# Feuerbann und Ernährung: Efferdgeweihte versuchen Feuer als Lichtquelle zu vermeiden und entzünden in der Regel keine Flammen. Stattdessen nutzen sie beispielsweise das Licht des Gwen Petryl-Steins in ihren Tempeln und warme Quellen, um sich warmzuhalten. Efferdgeweihte meiden gegartes und geräuchertes Essen, worunter auch Gegrilltes und Gebratenes fällt. Streng verboten ist es ihnen zudem, vom Delphin, Wal oder Robbe zu essen. Es gibt Efferdgeweihte, die sich streng an das Nahrungsgebot halten. Die meisten jedoch essen auch Brot, Suppe und andere einfache Speisen, obwohl Feuer benötigt wird, um diese zuzubereiten. Allerdings lassen sie diese Speisen dann von anderen zubereiten, um nicht selbst in Kontakt mit Feuer zu gelangen.\n# Schicksalsergebenheit: Ein Efferdgeweihter fügt sich den Befehlen seines Vorgesetzten oder des Kapitäns, auf dessen  Schiff  er  sich  befindet,  solange  diese  nicht  seinem Moralkodex zuwiderlaufen.\n# Ausleben der Emotionen: Efferdgeweihte leben ihre Gefühle aus, egal ob sie Zorn, Trauer Schmerz oder Freude empfinden. Sie halten sich hierbei nicht zurück und lassen diesen Gefühlen freien Lauf.\n\nMoralkodex der Firunkirche (Prinzipientreue II)# Ernsthaftigkeit: Witze und ähnliche Verspieltheiten sind Zeitverschwendung und sollten gemieden werden.\n# Askese: Der Firungeweihte sollte nicht verschwenderisch mit Ressourcen umgehen. Er sollte nur das benutzen, was er auch wirklich braucht.\n# Belastbarkeit: Das Streben des Firungeweihten gilt den Herausforderungen, die ihm sein Gott schickt. Er sollte sich immer bis an die Grenzen seiner Belastbarkeit begeben und sich kontinuierlich steigern.\n\nMoralkodex des Graveshkults (Prinzipientreue II) # Unterstützung der Handwerker: Der Graveshpriester muss die Mitglieder seiner Sippe bei handwerklichen Arbeiten unterstützen, damit gute Ergebnisse erzielt werden.\n# Meisterprodukt: Der Graveshpriester strebt danach, ausgezeichnete Gegenstände, Waffen und Rüstungen herzustellen. Der Umgang mit den dazugehörigen Rohstoffen ist für ihn zweitrangig.\n\nMoralkodex der Hesindegeweihten (Prinzipientreue I)# Sammeln von Wissen: Artefakte, Bücher und anderes Wissen sind in den Augen der Göttin so wertvoll, dass es gesammelt werden muss.\n# Ewige Lehre: Der Geweihte sollte sich bemühen, sich stets fortzubilden.\n# Ästhetik: Die Welt ist schön und der Geweihte soll die Schönheit der Welt ehren und mehren.\n\nMoralkodex der H’Szint-Priester (Prinzipientreue I) # Schutz vor den H’Ranga: Der Priester sorgt dafür, dass die H’Ranga durch Opfergaben mild gestimmt werden, damit sie in ihrem Zorn nicht unter den Achaz wüten.\n# Suche nach der Wandlung: Der Priester sucht nach dem Geheimnis der Veränderung und erforscht die Magie.\n\nMoralkodex der Ifirnkirche (Prinzipientreue I)# Hilfsbereitschaft: Eine Ifirngeweihte hilft jedem Wesen, wenn es Hilfe braucht (mit Ausnahme von Wesen, die sich gegen die zwölfgöttliche Ordnung auflehnen oder deren erklärte Feinde sind).\n# Wärme: Die Ifirngeweihte versucht, Menschen mit Worten und Taten Wärme zu spenden.\n# Körperliche Leistungsfähigkeit: Der Körper der Ifirngeweihten sollte jederzeit den Belastungen, denen sie sich stellen muss, standhalten können.\n# Ewige Reise: Eine Ifirngeweihte begibt sich ständig auf Reise, immer auf der Suche nach Menschen, denen sie helfen kann.\n\nMoralkodex der Ingerimmgeweihten (Prinzipientreue II)# Wahrung des Feuers: In der Gegenwart des Ingerimmgeweihten sollen alle Feuer brennen. Haus- und Waldbrände dürfen aber eingedämmt werden, und wenn ein Gläubiger durch den Brand in direkte Gefahr gerät, darf auch ein Ingerimmgeweihter Feuer löschen.\n# Opfer: Schöne Edelsteine und handwerkliche Erzeugnisse opfert der Ingerimmgeweihte voller Freude seinem Gott (beispielsweise durch Verbrennen).\n# Demut und Geduld: Der Ingerimmgeweihte muss Geduld bewahren und soll nicht mit seinen Fähigkeiten prahlen.\n# Fleiß und Geschick: Der Ingerimmgeweihte strebt in seinen Künsten nach Perfektion und verwendet auf die Verbesserung seines Handwerks viel Zeit.\n# Zorn und Kraft: Sollte es nötig sein, dann muss ein Ingerimmgeweihter seinem Zorn freien Lauf lassen.\n\nMoralkodex der Korgeweihten (Prinzipientreue II)# Guter Kampf und Gutes Gold: Korgeweihte lehnen zwar nicht immer Heilmagie ab, aber sie bevorzugen es, darauf zu verzichten. Zudem lässt sich jeder Korgeweihte seine Dienste nach den Richtlinien des Khunchomer Kodex bezahlen. Haben Korgeweihte erst einmal einen Vertrag unterzeichnet, sind sie ihrem Auftraggeber gegenüber loyal und bestrafen z. B. auch Deserteure der Einheit, der sie dienen.\n# Gnadenlosigkeit: Ein Geweihter des Kor jammert nicht um Gnade. Ob er jemandem Gnade gewährt, muss er selbst entscheiden.\n# Söldnerehre: Der Korgeweihte achtet die Regeln des Krieges und hält sich an die Söldnerehre. Er geht gegen Schändungen (sowohl von Menschen als auch Dingen) vor, da es die Söldnerehre verbietet (Plünderungen sind jedoch erlaubt, da es hierfür Richtlinien im Kodex gibt). Auch ist er verantwortlich für Kors Gefolgsleute (Söldner und Gladiatoren) und tritt für deren Rechte ein.\n\nMoralkodex des Levthankults (Prinzipientreue I)# Eigennutz und Egoismus: An erster Stelle im Leben kommt der Geweihte, danach Levthan und dann andere Menschen. Der Priester muss nicht immer eigennützig handeln, aber egoistisches Verhalten ist durchaus erwünscht und bringt ihm in der Regel auch die größten Vorteile.\n# Vergnügen und Verführung: Der Levthangeweihte strebt danach, seinem eigenen Vergnügen nachzugehen, gleich ob es sich dabei um Sex, Essen oder Rauschmittel handelt. Dabei versucht er stets, auch andere Personen zu verführen und ihnen seine eigenen Freuden nahezubringen.\n# Geheimnisse des Gehörnten lüften: Levthan und sein Kult bergen viele Geheimnisse, und selbst seine Priester kennen sie nicht alle. Es ist gut, danach zu streben diese Geheimnisse aufzudecken, gleich ob sie dazu führen, dass Levthan in einem besseren oder schlechteren Licht dasteht.\n\nMoralkodex des Marbokults (Prinzipientreue II)# Bestattung: Kein Leichnam sollte unbestattet liegen bleiben. Die Geweihte muss für die Totenruhe sorgen.\n# Betreuung Sterbender: Eine Marbogeweihte wird sich immer mit Vorrang um die Betreuung Sterbenskranker kümmern – und sie notfalls von ihrem Leid erlösen.\n# Traum: Ergründe die Welt der Träume. In ihr kann die Geweihte den Willen Marbos erkennen.\n\nMoralkodex der Nanduskirche (Prinzipientreue I)# Streben nach Weisheit: Ein Nandusgeweihter sollte stets nach Wissen streben und somit auch nach Weisheit.\n# Erleuchtung für Andere: Es soll dem Nandusgeweihten eine wichtige Aufgabe sein, andere Menschen zu erleuchten und sie somit vor Dummheit und Fehlern zu bewahren.\n# Geheimwissen: Der Nandusgeweihte strebt danach, die höheren Mysterien der Kirche zu erlernen.\n# Einfluss: Es ist wichtig, die Kirche und ihre Lehren zu fördern und zu verbreiten. Der Nandusgeweihte sollte versuchen, Einfluss zu gewinnen, um dieses Ziel zu erreichen.\n\nMoralkodex der Numinorupriester (Prinzipientreue II)# Ausgeglichenheit: Diener Numinorus lassen sich nicht zu impulsiven Handlungen hinreißen und behalten stets das große Ganze im Blick. Sie sind ruhig und beherrscht, niemals Sklave ihrer Emotionen.\n# Wissbegier: Numinoru gewährt seinen Anhängern nicht die Gabe der Prophezeiung und die Herrschaft über das Meer, damit sie die Sicherheit der heimatlichen Gestade genießen, sondern damit sie zu neuen Horizonten aufbrechen und in die Tiefen hinabtauchen. Sie streben stets nach neuen Erkenntnissen.\n# Planung: Die Anhänger Numinorus gehen planvoll vor und denken weit in die prophezeite Zukunft, sie handeln nicht unvorbereitet.\n\nMoralkodex der Perainegeweihten (Prinzipientreue I)# Hilfe: Leiste jedem Hilfe, der Hilfe benötigt.\n# Aufopferung: Arbeite hart und halte dich vom Müßiggang fern.\n# Bescheidenheit: Verschwende nicht die Gaben der Göttin.\n# Heilkunst: Bilde dich in der Heilkunst fort.\n\nMoralkodex der Phexgeweihten  (Prinzipientreue I)# Gegenleistung: Für eine zu erfüllende Aufgabe muss der Geweihte stets eine Gegenleistung verlangen.\n# Heimlichkeit: Der Geweihte soll seine Pläne im Verborgenen ausführen.\n# Herausforderung: Je größer die Herausforderung, desto größer der Ruhm. Der Geweihte soll große # Herausforderungen suchen und sich ihnen stellen.\n\nMoralkodex der Praiosgeweihten (Prinzipientreue II)# Gehorsam: Der  Geweihte  ist  verpflichtet,  sich  an  die Befehle von Personen zu halten, die über ihm in der Kirchenhierarchie stehen.\n# Offensichtlichkeit: Der Geweihte versteckt sich nicht. Schutz von Gesetz und Staat: Der Geweihte verteidigt zwölfgöttliche Reiche und Strukturen und achtet auf die Einhaltung der Gesetze.\n# Magiebann: Magie, vor allem schwarze Magie, sollte gebannt werden. Weiße Magie kann unter Umständen toleriert werden. Das Gildenrecht der Magier ist zu achten.\n# Mission: Der Glaube an den Götterfürsten und seine Geschwister muss in alle Winkel Deres verbreitet werden.\n\nMoralkodex der Rahjakirche (Prinzipientreue I)# Freude & Harmonie: Die Rahjageweihte ist dazu bestimmt, Freude zu verbreiten und die Sorgen der Menschen zu vertreiben. Dabei soll die Geweihte das passende Maß finden und auf die Vorlieben der Gläubigen eingehen. Aufdringliches Verhalten seitens der Geweihten steht einem harmonischen Anspruch entgegen.\n# Ekstase & Leidenschaft: Den Rausch der Göttin soll nicht nur die Geweihte erleben, sondern ihn auch den Gläubigen schenken. Alles, was die Rahjageweihte verschenkt, soll sie mit Leidenschaft verschenken.\n# Hingabe & Gleichmut: Die Rahjageweihte erträgt auch Zeiten, in denen sie keine Freude empfinden kann, mit Gleichmut. Ihre Hingabe soll besonders der göttlichen Ekstase dienen, nicht vergänglichen Dingen.\n\nMoralkodex der Rondrageweihten (Prinzipientreue II)# Verteidigung des Glaubens: Die Verteidigung des Glaubens ist die Pflicht jedes Rondrageweihten.\n# Ritterlichkeit: Der Rondrageweihte setzt im Kampf keine Armbrüste ein oder verhält sich unehrenhaft.\n# Verantwortung: Der Schutz aller Gläubigen, der Heiligtümer und der Tempel der Zwölfgötter steht im Vordergrund der Aufgaben eines Geweihten.\n# Zweikampf: Der ehrenhafte Zweikampf ist von allen Kampfhandlungen die ehrenhafteste.\n# Schwertmeisterschaft: Sich in allen Waffengattungen auszukennen und sie zu meistern ist eine Selbstverständlichkeit für den Geweihten.\n\nMoralkodex der Swafnirkirche (Prinzipientreue II)# Ehrlichkeit & Ehre: Ein Swafnirgeweihter sagt immer, was er wirklich denkt, und behält es nicht für sich. Er lügt auch niemanden an. Zudem sollten seine Handlungen immer ehrenvoll sein, und er sollte nichts tun, das seine Ehre oder die seiner Gemeinschaft beschmutzt.\n# Walschutz: Wale und Delphine gilt es zu schützen, Walfänger müssen vertrieben oder gar getötet werden.\n# Mut: Der Swafnirgeweihte steht immer an vorderster Reihe, wenn es zum Kampf gegen Hranngargezücht oder andere Schrecken geht.\n# Gemeinschaft: Der Swafnirgeweihte sorgt dafür, dass es seiner Gemeinschaft gut geht, und kümmert sich um die walwütigen Swafnirskinder, damit sie niemanden verletzen.\n\nMoralkodex der Traviageweihten (Prinzipientreue II)# Gastfreundschaft: Die Traviageweihte gewährt jedem, der dies wünscht, Gastfreundschaft und achtet auf die Einhaltung der Gesetze der Gastfreundschaft (niemand soll einen anderen Gast beleidigen oder angreifen).\n# Mäßigung und Wahrung von Sitte und Anstand: Die Traviageweihte achtet auf das friedliche Zusammenleben der Menschen. Sie verurteilt Prunksucht, Unzuverlässigkeit und unzüchtiges Benehmen.\n# Treue: Wer sich untreu verhält, egal ob in der Ehe oder weil er Versprechen bricht, sollte seinen Fehler wieder gut machen, worüber die Traviageweihte wacht.\n\nMoralkodex der Tsageweihten (Prinzipientreue II)# Lebenserhaltung: Die Tsageweihte achtet und schützt das Leben.\n# Waffenbann: Die Tsageweihte soll keine Waffen benutzen und damit Lebewesen schaden oder gar töten.\n# Erneuerung: Stillstand und Routine sind der Tsageweihten ein Gräuel. Sie versucht stets, ihren Horizont zu erweitern und probiert gerne neue Sachen aus.\n\nMoralkodex der Zsahh-Priester (Prinzipientreue II) # Schutz vor den H’Ranga: Die Priesterin sorgt dafür, dass die H’Ranga durch Opfergaben mild gestimmt werden, damit sie in ihrem Zorn nicht unter den Achaz wüten.\n# Heilung: Verletzte Sippenmitglieder müssen versorgt und geheilt werden, zudem ist es wichtig, dass sich die Priesterin auch um schwangere Achaz und den Schutz der Brutnester kümmert.\n\n Moralkodici von Schamanen bzw. Priester:\nMoralkodex der Achazschamanen (Prinzipientreue I) # Schutz vor den H’Ranga: Der Schamane sorgt dafür, dass die H’Ranga durch Opfergaben milde gestimmt werden, damit sie in ihrem Zorn nicht unter den Achaz wüten.\n# Bewahrung der Sippe: Die Sippe ist der Mittelpunkt des Seins. Der Achazschamane tut alles, um den Frieden in der Sippe zu bewahren.\n\nMoralkodex der Ferkinaschamanen (Prinzipientreue I)# Bestattung der Toten: Der Schamane muss die Verstorbenen der Sippe würdevoll und mit den richtigen Zeremonien bestatten.\n# Feiern und Mannbarkeitsriten: Ferkinaschamanen sind sowohl für die Ausrichtung von Festen zu Ehren der Götter wie auch für die der Mannbarkeitsriten verantwortlich.\n# Kontakt zur Geisterwelt: Der Schamane ist für den Kontakt zur Geisterwelt verantwortlich. Er ist Mittler zwischen dieser Welt und der Welt der Geister.\n# Tabuzonen: Der Schamane errichtet und bewacht Tabuzonen, da das, was dort lauert, eine Gefahr für seine Sippe ist.\n\nMoralkodex der Fjarningerschamanen (Prinzipientreue I)# Bestattung der Toten: Der Schamane muss die Verstorbenen der Sippe würdevoll und mit den richtigen Zeremonien bestatten.\n# Heilung: Falls der Schamane Kranken begegnet, ist er verpflichtet, ihnen zu helfen, sofern es ihm möglich ist, denn das Leben jedes Einzelnen trägt zum Überleben des Stammes bei.\n# Kontakt zur Geisterwelt: Der Schamane ist für den Kontakt zur Geisterwelt verantwortlich. Er ist Mittler zwischen dieser Welt und der Welt der Geister.\n# Tabuzonen: Der Schamane errichtet und bewacht Tabuzonen, da das, was dort lauert, eine Gefahr für seine Sippe ist.\n\nMoralkodex der Gjalskerschamanen (Prinzipientreue I)# Bestattung der Toten: Der Schamane muss die Verstorbenen der Sippe würdevoll und mit den richtigen Zeremonien für ihre Reise an die Küste vorbereiten, wo sie im Meer bestattet werden.\n# Heilung: Falls der Schamane Kranken begegnet, ist er verpflichtet, ihnen zu helfen, sofern es ihm möglich ist.\n# Kontakt zur Geisterwelt: Der Schamane ist für den Kontakt zur Geisterwelt verantwortlich. Er ist Mittler zwischen dieser Welt und der Welt der Geister.\n# Tabuzonen: Der Schamane errichtet und bewacht Tabuzonen, da das, was dort lauert, eine Gefahr für seine Sippe ist.\n\nMoralkodex der Nivesenschamanen (Prinzipientreue I)# Bestattung der Toten: Der Schamane muss die Verstorbenen der Sippe würdevoll und mit den richtigen Zeremonien bestatten.\n# Heilung: Falls der Schamane Kranken begegnet, ist er verpflichtet, ihnen zu helfen, sofern es ihm möglich ist.\n# Kontakt zur Geisterwelt: Der Schamane ist für den Kontakt zur Geisterwelt verantwortlich. Er ist Mittler zwischen dieser Welt und der Welt der Geister.\n# Tabuzonen: Der Schamane errichtet und bewacht Tabuzonen, da das, was dort lauert, eine Gefahr für seine Sippe ist.\n\nMoralkodex der Rur-und-Gror-Kirche (Prinzipientreue II)# Mehrung der Schönheit: Rur-und-Gror-Priester sind dafür verantwortlich, die Schönheit der Welt zu mehren und zu verhindern, dass Dämonen und das Bruderlose die Schönheit mindern.\n# Finden der 64 Fragen des Seins: Die Priester sind dazu aufgerufen, den Mysterien von Rur und Gror auf den Grund zu gehen sowie weitere Menschen für den Glauben zu begeistern und zu inspirieren – nicht als Missionar, sondern als Vorbild.\n\nMoralkodex der Tahaya-Schamanen (Prinzipientreue I)# Bestattung der Toten: Der Schamane muss die Verstorbenen der Sippe würdevoll und mit den richtigen Zeremonien bestatten.\n# Heilung: Falls der Schamane Kranken begegnet, ist er verpflichtet, ihnen zu helfen, sofern es ihm möglich ist.\n# Kontakt zur Geisterwelt: Der Schamane ist für den Kontakt zur Geisterwelt verantwortlich. Er ist Mittler zwischen dieser Welt und der Welt der Geister.\n# Tabu-Zonen: Der Schamane errichtet und bewacht Tabu-Zonen, da das, was dort lauert, eine Gefahr für seine Sippe ist.\n\nMoralkodex des Tairachkults (Prinzipientreue I) # Bewahrung der Gesetze der Götter: Der Tairachschamane bewahrt die Überlieferung und verhindert jede Veränderung der bestehenden göttlichen Gesetze und Ordnung.\n# Mittler zwischen der Welt der Toten und Lebenden: Mächtige Geister werden von dem Schamanen gerufen, um dienlich zu sein, überwundene Gegner werden „entseelt“.\n# Heilung von Körper und Geist: Der Schamane kümmert sich um die Verletzungen der Mitglieder der Sippe und vertreibt ruhelose Geister.\n\nMoralkodex der Trollzackerschamanen (Prinzipientreue I)# Kontakt zur Geisterwelt: Der Schamane ist für den Kontakt zur Geisterwelt verantwortlich. Er ist Mittler zwischen dieser Welt und der Welt der Geister.\n# Schmerz erdulden: Schmerzen zu erdulden, ist eine Ehre. Je schlimmer der Schmerz im Diesseits, desto besser wird das Leben im Jenseits.\n# Tabuzonen: Der Schamane errichtet und bewacht Tabuzonen, da das, was dort lauert, eine Gefahr für seine Sippe ist. # Trollehre: Trolle und Riesen sind wie Götter oder wie Kinder der Götter zu behandeln.",
    @call |perk: &PrinzipientreueIBisIII| {perk.stufe as u16 * 10},
    true
}

vorteil! {
    Raubtiergeruch,
    "Raubtiergeruch",
    "Nachteil Unfähig (Reiten), Nachteil Unfähig (Tierkunde)",
    "Der Meister kann immer, wenn der Held nahe genug an domestizierten Tieren ist und diese ihn wittern, mit dem W6 würfeln. Bei einer 1 versuchen sie, vor der vermeintlichen Bedrohung zu fliehen. Hunde und Katzen meiden die Nähe eines Helden mit diesem Nachteil, Pferde scheuen und selbst Schafe und Ziegen fliehen blökend, sobald er sich ihnen nähert. Bei Proben auf Tierkunde und Reiten im Umgang mit domestizierten Tieren kann ein Held mit Raubtiergeruch maximal 1 QS übrig behalten.",
    3,
    true
}

vorteil! {
    Schlafwandler,
    "Schlafwandler",
    "Keine",
    "Kein Held sollte mehr als einmal pro Woche schlafwandeln. Am Tag nach dem Schlafwandeln erhält der Held 24 Stunden lang eine Stufe des Zustands Betäubung durch die geringe Erholung in der Nacht. Die Regeneration ist für den Held in der Nacht, in der er schlafwandelt um 1 gesenkt.",
    10,
    true
}

pub struct SchlechteAngewohnheit {
    angewohnheiten: u8, // TODO: limit to 3, since only 6 ap can be gained at max
}

vorteil! {
    @noauto
    SchlechteAngewohnheit,
    "Schlechte Angewohnheit",
    "Keine",
    "Ein Held kann so viele Schlechte Angewohnheiten wählen, wie er möchte, erhält hierdurch allerdings insgesamt maximal 6 AP. In seltenen Fällen kann der Meister auch Erschwernisse auf Gesellschaftstalente vergeben.\n\n \nBeispiele:\n# Barfüßer: Obwohl es in der Kultur des Charakters üblich ist, Schuhwerk zu tragen, läuft er andauernd barfuß herum – selbst vor Adligen oder bei schlechtem Wetter.\n\n# Belästigung: Der Abenteurer starrt hübschen Frauen/ Männern auf den Ausschnitt/den Hintern oder macht ständig anzügliche Kommentare.\n\n# Dritte Person: Der Held spricht immer in der dritten Person von sich („Alrik mag das. Alrik hätte gerne noch ein Ferdoker Bier.\n“).\n\n# Duzer: Obwohl es angebracht wäre, kann ein Held mit einer solchen Schlechten Angewohnheit niemanden mit „Ihr“ ansprechen, auch höchste Würdenträger nicht. Er spricht jeden mit „Du“ an.\n\n# Erster: Ob bei der Wahl des besten Zimmers in einem Gasthaus oder beim Einkauf – der Held muss immer der Erste sein.\n\n# Heulsuse: Ob aus Angst, Freude, Trauer oder Ärger, der Held kann leicht zum Weinen gebracht werden.\n\n# Hypochonder: Ständig glaubt der Held, dass er an einer schrecklichen Krankheit erkranken könnte, selbst wenn er sich bester Gesundheit erfreut oder eine Ansteckung absolut unwahrscheinlich ist. Sollte er tatsächlich einmal krank sein, nimmt er stets das Schlimmste an.\n\n# Junge: Wenn der Abenteurer mit jemanden ein Gespräch führt, endet der Satz oft auf „Junge“, selbst bei Frauen(„Nicht schlecht, Junge. Hast du gut gemacht, Junge.\n“).\n\n# Langschläfer: Der Abenteurer schläft gerne lange – wird er geweckt, hat er extrem schlechte Laune.\n\n# Links-Rechts-Schwäche: Der Charakter vertauscht ständig links und rechts.\n\n# Mein Kind: Der Held beginnt oder beendet fast jedes Gespräch mit den Worten „Mein Kind“.\n\n# Nägelkauer: Immer wenn er nichts zu tun hat, kaut der Charakter auf seinen Fingernägeln.\n\n# Nase-/Ohrenbohrer: Bei dem Helden ist die Zwangshandlung, seinen Finger in Nase oder Ohr zu stecken, sehr stark ausgeprägt.\n\n# Nervös: Ruhig abzuwarten ist für den Abenteurer ausgesprochen schwierig. Er läuft im Zimmer herum, nimmt Platz und steht wieder auf – und wiederholt diese Handlungen so oft, bis endlich etwas passiert.\n\n# Putzfimmel: Der Held leidet unter einem Putzzwang – er muss Gegenstände oder Räume stets sauber halten.\n\n# Raucher: Der Held ist leidenschaftlicher Genussraucher, aber nicht süchtig. Hat er einmal keinen Tabak zur Verfügung, wird er dies lautstark und anhaltend bedauern, erleidet aber ansonsten keine Nachteile.\n\n# Redet wie ein Wasserfall: Wenn der Held mit anderen spricht, redet er so schnell, dass man sich konzentrieren muss, ihm zu folgen.\n\n# Schlechte Tischmanieren: Der Abenteurer schlürft und rülpst, benutzt kein Besteck oder isst seinen Kameraden immer alles weg.\n\n# Selbstgespräche: Beim Nachdenken oder in unpassenden Situationen redet der Held gerne mit sich selbst.\n\n# Unordentlich: Überall, wo der Held nächtigt oder lebt, lässt er Kleidung und Sachen herumliegen.\n\n# Wir: Der Held redet, wenn er sich selbst meint, immer von wir („Wir verstehen, was du sagst.\n“).\n\n \nweitere Beispiele aus Aventurische Namen:\n# Falscher Name: Die Heldin spricht ihre Freunde sehr oft mit einem falschen Namen an. Manchmal vertauscht sie sogar das Geschlecht („Das war mal ein Abenteuer, Alrike.\n“ „Stimmt schon, aber mein Name ist immer noch Alrik.\n“)\n# He du! / He Ihr!: Der Held spricht so gut wie niemanden mit Namen an, sei es, weil er ihn sich nicht merken kann, oder weil er es einfach nicht für nötig hält. Je nach Stellung oder Gemüt des Angesprochenen, kann das natürlich leicht zu Problemen führen.\n\n# Namensnenner: Der Held spricht sein Gegenüber andauernd mit Namen an, selbst wenn er damit allen auf die Nerven fällt. („Also ich würde mir das wirklich gut überlegen, Alrik. Alrik, reich mir doch mal das Salz.\n“)\n# Namensverdreher: Die Heldin vertauscht immer Teile des Namens oder fügt Buchstaben in den Namen ihres Gegenübers ein. („Halda, gib mir mal deinen Dolch.\n“ „Ich heiße Hilda!“).",
    @call |perk: &SchlechteAngewohnheit| {perk.angewohnheiten as u16 * 2},
    true
}

pub enum SchlechteEigenschaften {
    Aberglaube,
    Autoritätsglaube,
    Geiz,
    Goldgier,
    Jaehzorn,
    Kleptomanie,
    Naiv,
    Neugier,
    Rachsucht,
    Spielsucht,
    Verschwendungssucht,
    SikaryanDurst,
}

impl SchlechteEigenschaften {
    pub fn ap(&self) -> u16 {
        match *self {
            SchlechteEigenschaften::Aberglaube => 5,
            SchlechteEigenschaften::Autoritätsglaube => 5,
            SchlechteEigenschaften::Geiz => 5,
            SchlechteEigenschaften::Goldgier => 5,
            SchlechteEigenschaften::Jaehzorn => 10,
            SchlechteEigenschaften::Kleptomanie => 10,
            SchlechteEigenschaften::Naiv => 10,
            SchlechteEigenschaften::Neugier => 5,
            SchlechteEigenschaften::Rachsucht => 5,
            SchlechteEigenschaften::Spielsucht => 5,
            SchlechteEigenschaften::Verschwendungssucht => 5,
            SchlechteEigenschaften::SikaryanDurst => 0,
        }
    }
}

pub struct SchlechteEigenschaft {
    eigenschaft: SchlechteEigenschaften,
    eigenschaft2: Option<SchlechteEigenschaften>,
}

vorteil! {
    @noauto
    SchlechteEigenschaft,
    "Schlechte Eigenschaft",
    "Keine",
    "In jeder Situation, in der der Held mit einem potenziellen Auslöser seiner Schlechten Eigenschaft konfrontiert wird, muss er eine Probe auf Willenskraft bestehen, um sich zu beherrschen. Gelingt ihm diese Probe, ist alles in Ordnung, ansonsten muss er entsprechend der Schlechten Eigenschaft agieren. Seine Schlechte Eigenschaft hat ihn so lange im Griff, wie er dem Auslöser ausgesetzt ist. Der Meister kann für die Probe auf Willenskraft entsprechend der Stärke des Auslösers Erschwernisse oder Erleichterungen aussprechen.\n\nEs dürfen bis zu zwei Schlechte Eigenschaften pro Held gewählt werden. Kombinationen, die sich ausschließen (z. B. Geiz und Verschwendungssucht), dürfen nicht gewählt werden. Das letzte Wort darüber hat der Spielleiter.\n\nBeispiele:\n\n# Aberglaube: Der Held weicht vor schwarzen Katzen, Unglückszahlen und anderen schlechten Omen zurück.\nAP-Wert: –5 Abenteuerpunkte\n# Autoritätsglaube: Der Held zweifelt nicht an den Worten höhergestellter Personen, selbst wenn sie unlogische Befehle erteilen oder unglaubwürdig sind.\nAP-Wert: –5 Abenteuerpunkte\n# Geiz: Wenn der Held erst einmal Gold oder andere Wertgegenstände besitzt, gibt er sie nur ungerne wieder her. Lieber schränkt er sich selbst und andere finanziell ein, und er geht sogar unnötige Risiken ein, um Geld zu sparen.\nAP-Wert: –5 Abenteuerpunkte\n# Goldgier: Goldgierige Helden lassen alle Vorsicht fallen, wenn es darum geht, eine Schatztruhe oder andere wertvolle Gegenstände zu untersuchen und an sich zu nehmen.\nAP-Wert: –5 Abenteuerpunkte\n# Jähzorn: Der Held ist sehr reizbar und neigt zu Wutausbrüchen, die meist nur kurz anhalten, aber zu gewalttätigem Verhalten führen können.\nAP-Wert: –10 Abenteuerpunkte\n# Kleptomanie: Manche Abenteurer stehlen um des Stehlens willen, egal ob das Diebesgut einen nennenswerten Wert hat oder nicht, ohne über die Konsequenzen nachzudenken.\nAP-Wert: –10 Abenteuerpunkte\n# Naiv: Der Held ist gutherzig und naiv, vertraut den falschen Leuten und kann sich kaum vorstellen, dass man manche Leute es nicht gut mit ihm meinen. Er wird auch durch Enttäuschungen nicht vorsichtiger.\nAP-Wert: –10 Abenteuerpunkte\n# Neugier: Jeder Abenteurer ist neugierig, aber einige Abenteurer werden so von ihrer Neugierde getrieben, dass sie unnötige Risiken eingehen, um sie zu befriedigen.\nAP-Wert: –5 Abenteuerpunkte\n# Rachsucht: Während jähzornige Helden sofort ihrer Wut freien Lauf lassen, sind rachsüchtige besonnener und planen langfristig, wie sie sich für Kränkungen, egal wie gering, rächen können.\nAP-Wert: –5 Abenteuerpunkte\n# Spielsucht: Spielsüchtige lassen sich leicht zu riskanten Glücksspielen und Wetten verleiten. Oft führt die Spielsucht zu hohen Spielschulden.\nAP-Wert: –5 Abenteuerpunkte\n# Verschwendungssucht: Geld bleibt nicht lange im Besitz des Helden. Er gibt es mit vollen Händen (für sinnlose Dinge) aus.\nAP-Wert: –5 Abenteuerpunkte\n# Sikaryan-Durst (nur für Sikaryan-Räuber; automatisch vergeben bei Zustand Sikaryan-Verlust): Diese Schlechte Eigenschaft sorgt dafür, dass ein Sikaryan-Räuber seinen Hunger stillen will. Je höher sein Sikaryan-Verlust ist, desto eindringlicher will er dem Verlangen nachgehen und ist bereit, seine Tarnung aufzugeben und sich auf den nächsten Kulturschaffenden zu stürzen.",
    @call |perk: &SchlechteEigenschaft| {
        let ap = perk.eigenschaft.ap();
        let extra = if let Some(eigenschaft) = &perk.eigenschaft2 {
            eigenschaft.ap()
        }else{0};
        ap + extra
    },
    true
}

pub struct SchlechteRegenerationAstralenergieIBisIII {
    stufe: u8,
}

vorteil! {
    @noauto
    SchlechteRegenerationAstralenergieIBisIII,
    "Schlechte Regeneration (Astralenergie) I-III",
    "Vorteil Zauberer, Kein Vorteil Verbesserte Regeneration (Astralenergie)",
    "Für jede Stufe des Nachteils muss der Held beim Regenerationswurf 1 Punkt abziehen (bis zu einem Minimum von 0).",
    @call |perk: &SchlechteRegenerationAstralenergieIBisIII| {perk.stufe as u16 * 10},
    true
}

pub struct SchlechteRegenerationKarmaenergieIBisIII {
    stufe: u8,
}

vorteil! {
    @noauto
    SchlechteRegenerationKarmaenergieIBisIII,
    "Schlechte Regeneration (Karmaenergie) I-III",
    "Vorteil Geweihter, Kein Vorteil Verbesserte Regeneration (Karmaenergie)",
    "Für jede Stufe des Nachteils muss der Held beim Regenerationswurf 1 Punkt abziehen (bis zu einem Minimum von 0).",
    @call |perk: &SchlechteRegenerationKarmaenergieIBisIII| {perk.stufe as u16 * 10},
    true
}

pub struct SchlechteRegenerationLebensenergieIBisIII {
    stufe: u8,
}

vorteil! {
    @noauto
    SchlechteRegenerationLebensenergieIBisIII,
    "Schlechte Regeneration (Lebensenergie) I-III",
    "Kein Vorteil Verbesserte Regeneration (Lebensenergie)",
    "Für jede Stufe des Nachteils muss der Held beim Regenerationswurf 1 Punkt abziehen (bis zu einem Minimum von 0).",
    @call |perk: &SchlechteRegenerationLebensenergieIBisIII| {perk.stufe as u16 * 10},
    true
}

vorteil! {
    SchlechtesNamensgedaechtnis,
    "Schlechtes Namensgedächtnis",
    "Keine",
    "Der Held kann sich kaum einen Namen richtig merken, auch nicht die seiner Reisegefährten, seines Lehnsherrn, des Kontaktmanns in den Schwarzen Landen, der eigenen Kinder oder seiner Angebeteten. Entweder bringt er alles durcheinander, nennt Leute  beim falschen Namen, oder aber er ändert sie aus Versehen ständig ab. Alle Teilproben auf Klugheit bei Proben auf Etikette (Heraldik & Stammbäume oder Klatsch & Tratsch) sind um 2 erschwert.",
    2,
    true
}

vorteil! {
    Schurkenname,
    "Schurkenname",
    "Ein möglichst schurkischer Name,, Kein Vorteil Allerweltsname,, Kein Vorteil Heldenhafter Name,, Kein Nachteil Lächerlicher Name,, Kein Nachteil Unpassender  Name., Elfen können diesen Vorteil nicht wählen, da  sie keine Schurkennamen aufweisen.",
    "Eine Heldin mit diesem Nachteil bekommt nach Meisterentscheid bei Proben auf die Talente Etikette und Überreden eine Erschwernis von 1 auf alle Teilproben, die auf Charisma abgelegt werden, wenn sich ihr Gegenüber an dem Schurkennamen stört und dieser dem Gegenüber bekannt ist. Dafür erhält die Heldin eine Erleichterung von 1 auf Teilproben bei Charisma in dem Talent Einschüchtern.",
    2,
    true
}

vorteil! {
    SchwacheZaubermelodien,
    "Schwache Zaubermelodien",
    "Sonderfertigkeit Tradition Zauberbarden, Kein Vorteil Machtvolle Zaubermelodien",
    "Die Heldin kann nur Personen bis zu SK 1 durch ihre Zaubermelodien verzaubern (statt bis zu SK 2).",
    8,
    true
}

vorteil! {
    SchwacheZaubertaenze,
    "Schwache Zaubertänze",
    "Vorteil Zauberer, Sonderfertigkeit Tradition Zaubertänzer, Kein Vorteil Machtvolle Zaubertänze",
    "Der Held kann nur Personen bis zu SK 1 durch seine magischen Handlungen verzaubern (statt bis zu SK 2).",
    8,
    true
}

vorteil! {
    SchwacherAstralkoerper,
    "Schwacher Astralkörper",
    "Vorteil Zauberer",
    "Jedes Mal, wenn der Held Astralpunkte ausgibt, verliert er einen Astralpunkt zusätzlich.",
    15,
    true
}

vorteil! {
    SchwacherKarmalkoerper,
    "Schwacher Karmalkörper",
    "Vorteil Geweihter",
    "Jedes Mal, wenn der Held Karmapunkte ausgibt, verliert er einen Karmapunkt zusätzlich.",
    15,
    true
}

vorteil! {
    SensiblerGeruchssinn,
    "Sensibler Geruchssinn (*)",
    "Spezies, Kultur oder Profession muss Sensibler Geruchssinn als automatischen oder empfohlenen Nachteil besitzen., Kein Eingeschränkter Sinn (Geruch)",
    "Solange der Held dem Geruch ausgesetzt ist, erleidet er eine Stufe Verwirrung.",
    10,
    true
}

vorteil! {
    Sprachfehler,
    "Sprachfehler",
    "Kein Nachteil Stumm",
    "Bei Gesellschaftstalenten (außer Menschenkenntnis und Willenskraft) und Handeln (Feilschen) kann er nach Meisterentscheid eine Erschwernis von 1 erhalten, wenn es darum geht, mit anderen Personen zu sprechen.",
    15,
    true
}

vorteil! {
    StechenderOrkgeruch,
    "Stechender Orkgeruch (*)",
    "Spezies, Kultur oder Profession muss Nachteil als automatischen oder empfohlenen Nachteil aufweisen.",
    "Wesen, die diesen Nachteil aufweisen, werden in Kampfsituationen gegen Tiere bevorzugt angegriffen. Zudem sind nach Meisterentscheid Proben auf Reiten und Tierkunde, um bestimmte Tiere abzurichten, zu beruhigen oder zu reiten, um –2 erschwert.",
    12,
    true
}

vorteil! {
    Stigma,
    "Stigma",
    "Keine",
    "Der Meister kann eine Erschwernis von 1 auf Gesellschaftstalente (ausgenommen Einschüchtern, Menschenkenntnis und Willenskraft) und Handel (Feilschen) aussprechen, wenn er dies für angemessen hält. Ein Held kann den Nachteil nur einmal wählen.",
    10,
    true
}

vorteil! {
    Stumm,
    "Stumm",
    "Kein Vorteil Geborener Redner, Kein Vorteil Wohlklang, Kein Nachteil Sprachfehler",
    "Der Held kann sich in den meisten Fällen mit anderen Personen nicht verständigen (außer über Zeichensprache oder schriftlich). Der Einsatz von Zaubern und anderen Handlungen, die Sprache erfordern, ist unmöglich oder stark erschwert. Der Held erhält dauerhaft den Status Stumm.",
    40,
    true
}

vorteil! {
    Taub,
    "Taub",
    "Kein Vorteil Herausragender Sinn (Gehör)",
    "Der Held kann keine Probe auf Sinnesschärfe ablegen, die mit dem Hören zu tun haben. Er erhält den Status Taub.",
    40,
    true
}

pub struct Unfaehig {}

vorteil! {
    @noauto
    Unfaehig,
    "Unfähig",
    "Kein Vorteil Begabung auf das  gleiche Talent, Maximal 2 Unfähigkeiten",
    "Der Held muss bei einer Probe auf das Talent das beste Würfelergebnis einer Teilprobe neu würfeln. Das zweite Ergebnis ist bindend. Ein Abenteurer kann maximal zwei Unfähigkeiten besitzen.",
    @call |perk: &Unfaehig| {todo!()}, // TODO: A-/B-/C-/D- Talent: –1/–2/–3/–4 Abenteuerpunkt(e),
    true
}

vorteil! {
    Unfrei,
    "Unfrei",
    "Keine",
    "Der Held kann eine Erschwernis erhalten, wenn er als Unfreier Höhergestellte mit Gesellschaftstalenten überzeugen will. In seiner Kultur hat er zudem mit Unterdrückung zu kämpfen, genießt fast keinen rechtlichen Schutz und gilt als Besitz eines Höhergestellten.",
    8,
    true
}

vorteil! {
    UnpassenderName,
    "Unpassender Name",
    "Ein möglichst unpassender Name,, Kein Vorteil Allerweltsname,, Kein Vorteil Heldenhafter Name,, Kein Nachteil Schurkenname., Kein Nachteil Lächerlicher Name,, Elfen können diesen Vorteil nicht wählen, da ihre Namen immer passend sind.",
    "Ein Held mit diesem Nachteil bekommt nach Meisterentscheid bei Proben auf die Talente Bekehren & Überzeugen und Überreden eine Erschwernis von 1  auf alle Teilproben, die auf Charisma abgelegt werden, wenn sich sein Gegenüber an dem unpassenden Namen stört und dieser dem Gegenüber bekannt ist.",
    4,
    true
}

vorteil! {
    UnvertraeglichkeitgegenueberAlkohol,
    "Unverträglichkeit gegenüber Alkohol",
    "Keine Sonderfertigkeit Kampftrinker",
    "Wer über diesen Nachteil verfügt, für den verdoppeln sich die Auswirkungen nach Giftproben beim Konsum von Alkohol. Würde der Held beispielsweise 1 Stufe Berauscht erleiden, erhält er stattdessen direkt 2 Stufen des Zustands.",
    2,
    true
}

pub struct VerminderteZaubermelodienIBisIII {
    stufe: u8,
}

vorteil! {
    @noauto
    VerminderteZaubermelodienIBisIII,
    "Verminderte Zaubermelodien I-III",
    "Sonderfertigkeit Tradition Zauberbarden, Kein Vorteil Weitreichende Zaubermelodien",
    "Die Zaubermelodien der Abenteurerin reichen pro Stufe des Nachteils 2 Schritt weniger weit als üblich.",
    @call |perk: &VerminderteZaubermelodienIBisIII| {perk.stufe as u16 * 2},
    true
}

pub struct VerminderteZaubertaenzeIBisIII {
    stufe: u8,
}

vorteil! {
    @noauto
    VerminderteZaubertaenzeIBisIII,
    "Verminderte Zaubertänze I-III",
    "Sonderfertigkeit Tradition Zaubertänzer, Kein Vorteil Weitreichende Zaubertänze",
    "Die Zaubertänze des Abenteurers reichen pro Stufe des Nachteils 2 Schritt weniger weit als üblich.",
    @call |perk: &VerminderteZaubertaenzeIBisIII| {perk.stufe as u16 * 2},
    true
}

pub struct VerpflichtungenIBisIII {
    stufe: u8,
}

vorteil! {
    @noauto
    VerpflichtungenIBisIII,
    "Verpflichtungen I-III",
    "Keine",
    "Der Held muss Anweisungen desjenigen befolgen, dem er verpflichtet ist, oder mit den Konsequenzen leben. Ein Held kann mehreren Institutionen oder Personen verpflichtet sein, er bekommt jedoch nur einmal Abenteuerpunkte für den Nachteil (und zwar für die höchste Verpflichtungsstufe).",
    @call |perk: &VerpflichtungenIBisIII| {perk.stufe as u16 * 10},
    true
}

pub enum Verstuemmelung {
    Einarmig,
    Einaeugig,
    Einbeinig,
    Einhaendig,
    Einohrig,
}

impl Verstuemmelung {
    pub fn ap(&self) -> u16 {
        match *self {
            Verstuemmelung::Einarmig => 30,
            Verstuemmelung::Einaeugig => 10,
            Verstuemmelung::Einbeinig => 30,
            Verstuemmelung::Einhaendig => 20,
            Verstuemmelung::Einohrig => 5,
        }
    }
}

pub struct Verstuemmelt {
    verstuemmelung: Verstuemmelung,
}

vorteil! {
    @noauto
    Verstuemmelt,
    "Verstümmelt",
    "Keine",
    "Viele Fertigkeitsproben können durch die Verstümmelung erschwert oder unmöglich sein. Die Entscheidung hierüber obliegt dem Meister je nach Situation.\n\n# Einarmig: Einer der beiden Arme des Charakters fehlt mindestens bis zum Ellbogen.\n# Einäugig: Eines der beiden Augen des Helden ist blind oder er hat es verloren. Fernkampfangriffe sind dadurch um 4 erschwert.\n# Einbeinig: Der Held hat nur ein Bein, bzw. es ist unterhalb des Knies amputiert. Der Geschwindigkeit- Grundwert ist halbiert. Krücken und einfache Prothesen wie ein Holzbein helfen gegen die Verminderung der Geschwindigkeit nicht.\n# Einhändig: Dem Abenteurer fehlt eine Hand.\n# Einohrig: Eines der Ohren ist abgeschnitten oder verkrüppelt. Proben auf Sinnesschärfe auf Hören sind für den Helden grundsätzlich um 1 erschwert.",
    @call |perk: &Verstuemmelt| {perk.verstuemmelung.ap()},
    true
}

vorteil! {
    Verweichlicht,
    "Verweichlicht",
    "Keine",
    "Bei der Probe auf Selbstbeherrschung (Handlungsfähigkeit bewahren oder Störungen ignorieren) bei Wundeffekten bekommt die Heldin eine Erschwernis von 2.",
    3,
    true
}

vorteil! {
    WahrerName,
    "Wahrer Name",
    "Keine",
    "Wenn ein Zauberer einen Zauberspruch gegen ein Ziel einsetzt, dessen Wahren Namen er kennt (und der über diesen Nachteil verfügt), verschlechtert sich die Seelenkraft und die Zähigkeit des Ziels um 4, womit es leichter zu verzaubern ist.",
    10,
    true
}

pub struct WenigePredigtenIBisII {
    stufe: u8,
}

vorteil! {
    @noauto
    WenigePredigtenIBisII,
    "Wenige Predigten I-II",
    "Vorteil Prediger",
    "Bei Stufe I kann der Zelot nur 2 Predigt-Sonderfertigkeiten beherrschen, bei Stufe II sogar nur 1.",
    @call |perk: &WenigePredigtenIBisII| {perk.stufe as u16 * 10},
    true
}

pub struct WenigeVisionenIBisII {
    stufe: u8,
}

vorteil! {
    @noauto
    WenigeVisionenIBisII,
    "Wenige Visionen I-II",
    "Vorteil Visionär",
    "Bei Stufe I kann der Zelot nur 2 Vision-Sonderfertigkeiten beherrschen, bei Stufe II sogar nur 1.",
    @call |perk: &WenigeVisionenIBisII| {perk.stufe as u16 * 10},
    true
}

vorteil! {
    WildeMagie,
    "Wilde Magie",
    "Vorteil Zauberer",
    "19er werden bei Fertigkeitsproben auf Zauber und Rituale in Hinsicht auf die Bestimmung eines Patzers wie eine 20 behandelt.",
    10,
    true
}

vorteil! {
    Yurach,
    "Yurach",
    "Der Held muss der Kultur Orkland angehören.",
    "Yurach gelten bei den Orks als Ausgestoßene. Alle Proben auf Bekehren & Überzeugen, Etikette, Handel und Überreden erhalten gegenüber Orks eine Erschwernis von –1. Diese Erschwernis gilt nicht gegenüber Nicht-Orks. Der soziale Stand eines Yurach ist zudem immer Stufe I (siehe Regelwerk).",
    20,
    true
}

pub struct ZauberanfaelligIBisII {
    stufe: u8,
}

vorteil! {
    @noauto
    ZauberanfaelligIBisII,
    "Zauberanfällig I-II",
    "Kein Vorteil Schwer zu verzaubern",
    "Gegen Magie sind Seelenkraft und Zähigkeit um 1 pro Stufe schlechter als üblich.",
    @call |perk: &ZauberanfaelligIBisII| {perk.stufe as u16 * 12},
    true
}

vorteil! {
    Zerbrechlich,
    "Zerbrechlich",
    "Kein Vorteil Zäher Hund",
    "Erschwernisse durch den Zustand Schmerz werden wie bei einer Stufe höher behandelt. Die Handlungsunfähigkeit tritt somit bereits bei Stufe III ein.",
    20,
    true
}
