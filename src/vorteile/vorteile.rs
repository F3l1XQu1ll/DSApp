use crate::vorteil;
use crate::vorteile::Vorteil;

pub struct AdelIBisIII {
    stufe: u8,
}

vorteil! {
    @noauto
    AdelIBisIII,
    "Adel I-III",
    "Kultur muss über einen passenden Adel verfügen.",
    "Der Held ist angesehen, genießt die Privilegien des Adels und kann vom Meister Erleichterungen zugesprochen bekommen, wenn er gegenüber Rangniedrigeren agiert.",
    @call |adel: &AdelIBisIII| {adel.stufe as u16 * 5},
    false
}

vorteil! {
    AffinitaetzuDaemonen,
    "Affinität zu Dämonen",
    "Vorteil Zauberer, Kein Vorteil Affinität zu Elementaren",
    "Der Held kann Dämonen leichter zu einem Dienst überreden, wenn er sie selbst beschworen hat. Der Dämon erfüllt ihm einen zusätzlichen Dienst auch über das Maximum von 6 Diensten hinaus.",
    10,
    false
}

vorteil! {
    AffinitaetzuElementaren,
    "Affinität zu Elementaren",
    "Vorteil Zauberer, Kein Vorteil Affinität zu Dämonen",
    "Der Held kann Elementare leichter zu einem Dienst überreden, wenn er sie selbst beschworen hat. Das Elementar erfüllt ihm einen zusätzlichen Dienst auch über das Maximum von 6 Diensten hinaus.",
    10,
    false
}

vorteil! {
    Akoluth,
    "Akoluth",
    "Kein Vorteil Geweihter, Nachteil Prinzipientreue (Moralkodex einer Kirche), Verpflichtungen (Kirche)",
    "Ein Akoluth verfügt über die Fähigkeit, einen Geweihten einer einzigen ausgewählten Kirche bei Liturgien zu unterstützen. Es muss sich um die Kirche handeln, für die er vorher die Prinzipientreue und die Verpflichtungen erworben hat. Sollte ein Akoluth bei einer Liturgie genauso viele Aktionen aufwenden, wie für die Liturgiedauer notwendig sind, erhält der Geweihte eine Erleichterung von 1 für die Liturgie. Der Akoluth muss sich in einem Radius von 3 Schritt um den Geweihten aufhalten, damit die Unterstützung funktioniert. Dieser Effekt ist nicht kumulativ mit weiteren Akoluthen.",
    5,
    false
}

vorteil! {
    Allerweltsname,
    "Allerweltsname",
    "Ein gewöhnlicher Name ohne großen Wiedererkennungswert, Kein Vorteil Heldenhafter Name oder Nachteil Lächerlicher Name, Schurkenname oder Unpassender Name, Elfen können diesen  Vorteil nicht wählen, da sie über keine Allerweltsnamen verfügen.",
    "Ein Held mit diesem Vorteil ist schwer ausfindig  zu machen, weil sich niemand seinen Namen merken  kann oder ihn mit einem Träger eines ähnlichen Namens verwechselt. Die Teilprobe auf Klugheit bei Proben auf Gassenwissen (Informationsbeschaffung) ist um 1  erschwert, wenn es darum geht, den Helden ausfindig  zu machen oder Informationen über ihn zu sammeln.",
    2,
    false
}

vorteil! {
    Altersresistenz,
    "Altersresistenz (*)",
    "Spezies, Kultur oder Profession muss Altersresistenz als automatischen oder empfohlenen Vorteil aufweisen, Kein Nachteil Schnelle Alterung",
    "Der Held ist immun gegen natürliche wie magische Alterungserscheinungen. Negative Auswirkungen des Alters werden bei ihm nicht berücksichtigt.",
    5,
    false
}

vorteil! {
    AngenehmerGeruch,
    "Angenehmer Geruch",
    "Keine",
    "Fertigkeitsproben auf Betören (Anbändeln, Liebeskünste) sind um 1 erleichtert.",
    6,
    false
}

vorteil! {
    AnimalischeKraft,
    "Animalische Kraft",
    "Vorteil Feenblut (Großer Biestinger)",
    "Einmal am Tag kann der Held eine Probe auf Selbstbeherrschung ablegen. Er erhält bei Gelingen für 1 Minute QS/2 zu seiner Körperkraft. Dies kann auch den Schadensbonus erhöhen, ändert aber nichts an weiteren abgeleiteten Werten. Diese Fähigkeit zu aktivieren dauert 1 Aktion. Nach dem Ende der Wirkung erhält der Held 1 Stufe Betäubung.",
    12,
    false
}

vorteil! {
    Basiliskentoeter,
    "Basiliskentöter",
    "Der Held muss einen Basilisken getötet haben.",
    "Basiliskentöter werden respektvoll behandelt, erhalten regelmäßig Einladungen zu Festen der gehobenen Gesellschaft und oftmals kostenlos Speis und Trank in Tavernen. Der Held wird so behandelt, als hätte er einen um 2 höheren sozialen Rang als üblich (siehe Regelwerk Seite 338). Das Gegenüber muss aber wissen oder zumindest glauben, dass der Held ein Basiliskentöter ist, damit der Bonus zählt.",
    10,
    false
}

vorteil! {
    BegabterAufreisserin,
    "Begabte/r Aufreißer/in",
    "Keine",
    "Beim Abschleppen (siehe Seite 36) hat der Held mehr Erfolg als andere Personen. Bei der Probe auf Betören(Liebeskünste) zum Abschleppen bekommt er +1 FP (bis zu einem Maximum von 18 FP). Alternativ kann der Spieler auch beschließen, mehrere Personen gleichzeitig abzuschleppen. Die Probe ist dann jedoch pro Person, die er abschleppen will, um 2 erschwert. Dieser Vorteil kann mit der Sonderfertigkeit Abschleppspezialist/in (siehe Seite 41 Aventurisches Kompendium 2) kombiniert werden. Damit bekommt der Held +2 FP oder die Probe zum Abschleppen von mehreren Personen ist pro Person nur um 1 erschwert.",
    5,
    false
}

pub struct Begabung {}

vorteil! {
    @noauto
    Begabung,
    "Begabung",
    "Kein Nachteil Unfähig für diese Fertigkeit, nicht mehr als drei Begabungen pro Person",
    "Ein begabter Abenteurer kann bei jeder Probe auf eine bestimmte Fertigkeit einen W20 neu würfeln. Der Spieler kann zunächst die 3W20 würfeln und sich eines der drei Ergebnisse aussuchen, dass er neu würfeln kann. Es gilt das bessere Ergebnis beider Würfe. Eine Begabung muss für jede Fertigkeit einzeln gewählt werden, also zum Beispiel auf ein bestimmtes Talent, einen Zauber oder eine Liturgie. Man kann nicht mehrfach in einer Fertigkeit begabt sein und somit zwei oder gar drei W20-Würfe in einer Probe wiederholen. Fällt eine Doppel-20 oder Dreifach-20 kann man diesen Vorteil nicht nutzen. Eine Begabung kann nur für eine Fertigkeit, nicht aber für eine Kampftechnik gewählt werden (dafür gibt es jedoch den Vorteil Waffenbegabung). Ein Einsatz von Schicksalspunkten ist vor oder nach dem Einsatz einer Begabung erlaubt (siehe Seite 29). Dadurch ist es möglich, mehrfach neu zu würfeln oder eine andere Wirkung der Schicksalspunkte mit der Begabung zu kombinieren.",
    @call |begabung: &Begabung| {todo!()}, // TODO: 6/12/18/24 Abenteuerpunkte (A-/B-/C-/D-Fertigkeit),
    false
}

vorteil! {
    Beidhaendig,
    "Beidhändig",
    "Keine",
    "Der Held erleidet keine Erschwernisse bei Fertigkeitsproben, wenn er die falsche Hand benutzt. Im Kampf hebt der Vorteil alle Abzüge auf, die durch das Führen einer Waffe mit der falschen Hand anfallen.",
    15,
    false
}

pub struct BissIBisIII {}

vorteil! {
    @noauto
    BissIBisIII,
    "Biss I-III (*)",
    "Spezies, Kultur oder Profession muss Vorteil als automatischen oder empfohlenen Vorteil aufweisen.",
    "Der Held hat scharfe Zähne, die sich als Nahkampfwaffe eignen. Es gibt drei Stufen des Biss-Angriffs. Der Schaden ist von der Stufe und der Größenkategorie des Helden abhängig:# winzig, Stufe I/II/III: 1W3/1W3+1/1W3+2 TP# klein, Stufe I/II/III:1W6/1W6+1/1W6+2 TP# mittel, Stufe I/II/III: 1W6+1/1W6+2/1W6+3 TP# groß, Stufe I/II/III: 1W6+2/1W6+3/1W6+4 TP# riesig, Stufe I/II/III: 2W6/2W6+1/2W6+2 TPDer Angriff zählt als waffenloser Angriff und wird mit der Kampftechnik Raufen ausgeführt. Verfügt ein Held über den Biss-Angriff, kann er die Sonderfertigkeit Verbeißen für 10 AP erwerben.",
    @call |biss: &BissIBisIII|{todo!()}, // TODO: I/II/III: 5/10/15 Abenteuerpunkte + jeweils 0/1/2/3/4 AP für die Größenkategorie winzig/klein/mittel/groß/riesig (pro Stufe der SF),
    false
}

vorteil! {
    Blumenduft,
    "Blumenduft",
    "Vorteil Feenblut (Blütenfee)",
    "Die Heldin riecht immer nach einer bestimmten wohlriechenden Blumenart. Sie kann nie unter dem Status Übler Geruch leiden, und sie erhält bei Proben auf Überreden einen Bonus von +1 auf CH, wenn ihr Gegenüber den Blütengeruch mag (ansonsten kann die Probe auch um 1 erschwert sein).",
    8,
    false
}

vorteil! {
    Blutzauberer,
    "Blutzauberer",
    "Paktgeschenk Zauberei mit Lebenskraft",
    "Der Held wird zum Zauberer, obwohl er nicht mit der Gabe der Zauberei geboren wurde. Er verfügt nicht über einen eigenen AsP-Vorrat, sondern zaubert stattdessen mit seiner Lebensenergie. Dabei sind Lebenspunkte doppelt so viel Wert wie AsP, sodass ein Zauber, der 7 AsP kosten würde, den Helden nur 4 LeP kosten. Hat der Held einen eigenen AsP-Vorrat, kann er sich bei jedem Zauber entscheiden, ob er mit AsP oder LeP zaubern möchte.",
    0, // 0 Abenteuerpunkte (wird automatisch über das Paktgeschenk Zauberei mit Lebenskraft vergeben),
    false
}

vorteil! {
    DickesFell,
    "Dickes Fell",
    "Vorteil Feenblut (Großer Biestinger)",
    "Der Held weist einen natürlichen RS von 1 auf. Dieser kann mit dem RS von Rüstungen kombiniert werden, aber nur bis zu einem Wert von insgesamt RS 4.",
    15,
    false
}

vorteil! {
    Drachentoeter,
    "Drachentöter",
    "Der Held muss einen echten Drachen getötet haben.",
    "Drachentöter werden respektvoll behandelt, erhalten regelmäßig Einladungen zu Festen der gehobenen Gesellschaft und oftmals kostenlos Speis und Trank in Tavernen. Der Held wird so behandelt, als hätte er einen um 1 höheren sozialen Rang als üblich (siehe Regelwerk Seite 338). Das Gegenüber muss aber wissen oder zumindest glauben, dass der Held ein Drachentöter ist, damit der Bonus zählt. Der Held erhält zudem einen Bonus von +1 TP gegen alle Drachen, sowohl gegenüber echten als auch niederen Drachen.",
    10,
    false
}

pub struct DunkelsichtIBisIII {
    stufe: u8,
}

vorteil! {
    @noauto
    DunkelsichtIBisIII,
    "Dunkelsicht I-III (*)",
    "Spezies, Kultur oder Profession muss Dunkelsicht als automatischen oder empfohlenen Vorteil aufweisen, kein Nachteil Nachtblind",
    "Auf Stufe I werden Erschwernisse durch Dunkelheit um eine Stufe gesenkt, auf Stufe II können alle Erschwernisse durch Dunkelheit ignoriert werden. Bei vollständiger Finsternis ist aber Dunkelsicht (egal ob Stufe I oder II) wirkungslos und es gelten die Abzüge für vollständige Finsternis.",
    @call |dunkelsicht: &DunkelsichtIBisIII| {dunkelsicht.stufe as u16 * 10},
    false
}

vorteil! {
    EinblickeinsVerborgene,
    "Einblicke ins Verborgene",
    "Vorteil Feenblut (Blütenfee)",
    "Die Heldin bekommt im Talent Sinnesschärfe eine Erleichterung von 1, wenn sie Zauber mit dem Merkmal Illusion durchschauen will.",
    8,
    false
}

pub struct EinkommenIBisVI {
    stufe: u8,
}

vorteil! {
    @noauto
    EinkommenIBisVI,
    "Einkommen I-VI",
    "",
    "Für jede Stufe dieses Vorteils bekommt der Held die gleiche Stufe als dauerhaften Lebensstil. Das verdiente Geld des Vorteils fließt ausschließlich in den Erhalt des Lebensstils. Extrakosten wie Waffen, Rüstungen und andere Ausrüstung muss der Held von seinem Startkapital oder seinen Erwerbungen während des Spiels bezahlen. Alles was über den Lebensstil hinausgeht, muss allerdings extra gezahlt werden.",
    @call |einkommen: &EinkommenIBisVI| {match einkommen.stufe {
        1 => 0,
        2 => 2,
        3=> 6,
        4=> 20,
        5=>60,
        6=>200,
        _=>panic!("Stufe nicht möglich")
    }},
    false
}

vorteil! {
    EisenaffineAura,
    "Eisenaffine Aura",
    "Vorteil Zauberer",
    "Wer diesen Vorteil aufweist, erleidet durch den Bann des Eisens lediglich pro vollen 4 Stein Gewicht Erschwernisse, anstatt bereits bei 2 vollen Stein.",
    15,
    false
}

vorteil! {
    Eisern,
    "Eisern",
    "Keine",
    "Die Wundschwelle des Helden steigt durch den Vorteil um 1.",
    5,
    false
}

vorteil! {
    Entfernungssinn,
    "Entfernungssinn",
    "Kein Nachteil Blind, Eingeschränkter Sinn (Sicht), Farbenblind oder Verstümmelt (Einäugig)",
    "Proben auf Fernkampf mit Schusswaffen (und nur mit Schusswaffen) sind bei der Entfernungskategorie Weit nur um 1 anstatt um 2 erschwert.",
    10,
    false
}

vorteil! {
    FeenblutBluetenfee,
    "Feenblut (Blütenfee)",
    "Kein anderer Ahnenblut-Vorteil",
    "Die Heldin erhält bei Proben auf alle Gesellschaftstalente (außer Einschüchtern und Willenskraft) eine Erleichterung von 1 bei Teilproben auf IN. Sie kann zudem Angriffen besser ausweichen und bekommt einen Bonus von +1 AW, solange sie maximal Kleidung mit einem RS von bis zu 1 trägt. Bei Wissenstalenten erhält sie eine Erschwernis von 1 auf alle Teilproben, die auf KL gehen.",
    15,
    false
}

vorteil! {
    FeenblutDryade,
    "Feenblut (Dryade)",
    "Kein anderer Ahnenblut-Vorteil",
    "Die Heldin hat ein gutes Gespür für Bäume und wirkt auf alle humanoiden Spezies verführerisch. Sie erhält bei einer gelungenen Probe auf Pflanzenkunde +1 QS (bis zu einem Maximum von 6 QS), wenn es um Bäume geht, und bei Proben auf Betören einen Bonus von +1 auf beide Teilproben von CH. Dafür erleidet sie eine Erschwernis von 1 bei Proben auf Willenskraft (Betören widerstehen).",
    10,
    false
}

vorteil! {
    FeenblutGrosserEberbiestinger,
    "Feenblut (Großer Eberbiestinger)",
    "Kein anderer Ahnenblut-Vorteil",
    "Der Held erhält bei einer gelungenen Probe auf Tierkunde +1 FP (bis zu einem Maximum von 18 FP). Außerdem friert er kaum, da ihm selbst im Winter aufgrund eines dichten Fells am Körper warm ist. Bei der Bestimmung von Kälteschaden schützt der Vorteil wie Dicke Wollsachen (Kältestufe –1, Regelwerk Seite 346). Bei Proben auf Willenskraft, die zum Widerstehen von Einschüchtern (Provokation) dienen, erleidet der Held eine zusätzliche Erschwernis von 1.",
    10,
    false
}

vorteil! {
    FeenblutNixer,
    "Feenblut (Nixer)",
    "Kein Ahnenblut-Vorteil",
    "Die Haut des Helden fühlt sich feucht an, als würde er gerade aus dem Wasser kommen. Haltegriffe gegen ihn sind daher um 2 erschwert, und Proben auf Fesseln (Fesselungen) gegen ihn sind zusätzlich zu allen weiteren Modifikatoren um 1 Punkt erschwert. Der Held erhält bei einer gelungenen Probe auf Schwimmen\n+2 FP (bis zu einem Maximum von 18 FP). Bei Schaden durch Feuer werden die TP gegen den Helden um 2 erhöht.",
    10,
    false
}

vorteil! {
    Feenfreund,
    "Feenfreund",
    "",
    "Proben auf Überreden (Aufschwatzen, Herausreden, Manipulieren, Schmeicheln), um Feen freundlich zu stimmen, gelten bei Gelingen der Probe als um 1 QS besser. Allerdings kann sich so die QS nie über 6 hinaus erhöhen.",
    3,
    false
}

vorteil! {
    Flink,
    "Flink",
    "Kein Nachteil Behäbig, Fettleibig oder Verstümmelt (Einbeinig)",
    "Durch Flink bekommt ein Held 1 Punkt auf seinen Geschwindigkeit-Grundwert hinzu.",
    8,
    false
}

vorteil! {
    Fuchssinn,
    "Fuchssinn",
    "Keine",
    "Der Held kann Fallen erspüren, die mit den herkömmlichen Sinnen nicht entdeckt werden können. Durch den Vorteil wird das Anwendungsgebiet Fuchssinn im Talent Sinnesschärfe erworben.",
    15,
    false
}

vorteil! {
    GeborenerRedner,
    "Geborener Redner",
    "Kein Nachteil Stumm",
    "Proben auf das Talent Bekehren & Überzeugen (öffentliche Rede) sind um 1 erleichtert.",
    4,
    false
}

vorteil! {
    Geweihter,
    "Geweihter",
    "Keine",
    "Der Geweihte erhält als Karmaenergie-Grundwert 20 KaP. Der Vorteil beinhaltet nicht die Sonderfertigkeit Tradition. Diese muss einzeln erworben werden. Jeder Geweihte muss mit einer Tradition starten (eine Tradition ist eine spezielle Sonderfertigkeit). Ein Held kann auch im späteren Verlauf seines Lebens diesen Vorteil erwerben und Geweihter werden. Allerdings kann man nur eine geweihte Tradition besitzen. Ein Erwerb einer weiteren geweihten Tradition ist nicht möglich.",
    25,
    false
}

pub struct GiftresistenzIBisII {
    stufe: u8,
}

vorteil! {
    @noauto
    GiftresistenzIBisII,
    "Giftresistenz I-II",
    "Kein Nachteil Giftanfällig",
    "Eine Giftresistenz verbessert Zähigkeit und Seelenkraft um 1 pro Stufe, wenn es darum geht, Giften zu widerstehen.",
    @call |resi: &GiftresistenzIBisII| {resi.stufe as u16 *10},
    false
}

vorteil! {
    GlatteHaut,
    "Glatte Haut",
    "Vorteil Feenblut (Nixer)",
    "Die Haut des Helden ist elastisch und glatt. Dadurch gleiten alle Nahkampfwaffen im Kampf ab, wenn bei der TP-Bestimmung das minimale Ergebnis erwürfelt wurde. Der Held erleidet dann keinen Schaden. Diese Regel findet jedoch nur Anwendung, wenn der RS des Helden maximal 2 beträgt.",
    5,
    false
}

pub struct GlueckIBisIII {
    stufe: u8,
}

vorteil! {
    @noauto
    GlueckIBisIII,
    "Glück I-III",
    "Kein Nachteil Pech",
    "Das für den Helden erreichbare Maximum an Schicksalspunkten steigt hierdurch um 1 je Stufe und der Held er erhält diesen Schip gleich zu Spielbeginn zu seinem Vorrat.",
    @call |glueck: &GlueckIBisIII| {glueck.stufe as u16 * 30},
    false
}

vorteil! {
    Gon,
    "Gon",
    "Die Heldin muss ein Gon’da-Gon-Palenkel gewonnen haben.",
    "Die Heldin wird von Gjalskern so behandelt, als hätte sie einen um 1 höheren sozialen Rang als üblich (siehe Regelwerk Seite 338). Das Gegenüber muss aber wissen oder zumindest glauben, dass die Heldin ein Gon ist, damit der Bonus zählt. Die Heldin erhält zudem einen Bonus von +2/+1 bei Proben auf Einschüchtern gegenüber anderen Gjalskern/ Nicht-Gjalskern.",
    10,
    false
}

vorteil! {
    Greifschwanz,
    "Greifschwanz (*)",
    "Spezies, Kultur oder Profession muss Vorteil als automatischen oder empfohlenen Vorteil aufweisen.",
    "Der lange Schwanz des Helden kann Gegenstände so mühelos tragen wie ein Arm, kann aber keine Waffen oder Schilde führen. Er kann Gegenstände mit dem Schwanz feinfühlig genug manipulieren, um sie effektiv als Werkzeuge einzusetzen.",
    8,
    false
}

pub struct GrosseZauberauswahlIBisII {
    stufe: u8,
}

vorteil! {
    @noauto
    GrosseZauberauswahlIBisII,
    "GrosseZauberauswahl I-II",
    "Sonderfertigkeit Tradition Intuitive Zauberer. Animisten oder Runenschöpfer, kein Nachteil Kleine Zauberauswahl",
    "Die Heldin kann pro Stufe des Vorteils einen zusätzlichen Zauber aktivieren und damit die Begrenzung von maximal 3 Zaubern übersteigen. Ob sie diese Zauber zu Spielbeginn oder später aktiviert, ist ihr überlassen.",
    @call |auswahl: &GrosseZauberauswahlIBisII| {auswahl.stufe as u16 * 8},
    false
}

pub struct GutaussehendIBisII {
    stufe: u8,
}

vorteil! {
    @noauto
    GutaussehendIBisII,
    "Gutaussehend I-II",
    "Kein Nachteil Hässlich",
    "Jede Stufe des Vorteils gibt dem Helden eine Erleichterung von 1 auf Betören (Anbändeln, Liebeskünste), Überreden (Aufschwatzen, Herausreden, Manipulieren, Schmeicheln) und Handel (Feilschen).",
    @call |aussehen: &GutaussehendIBisII| {aussehen.stufe as u16 * 20},
    false
}

vorteil! {
    GutesNamensgedaechtnis,
    "Gutes Namensgedächtnis",
    "Kein Nachteil Schlechtes Namensgedächtnis",
    "Die Abenteurerin kann sich an so gut wie jeden Namen (samt Titel, Ränge und Ämter) erinnern. Der Meister sollte voraussetzen, dass die Heldin Personen immer den korrekten Namen zuordnen kann, auch wenn der Spieler der Heldin sich nicht immer erinnert. Zudem kann der Meister in passenden Situationen, z. B. auf Festen oder wenn der korrekte Name eine wichtige Rolle spielt, dem Spieler eine Erleichterung von 1 bei der Teilprobe auf Klugheit bei Etikette (Heraldik & Stammbäume oder Klatsch & Tratsch) zugestehen.",
    2,
    false
}

vorteil! {
    HarmoniedesWaldes,
    "Harmonie des Waldes",
    "Vorteil Feenblut (Dryade)",
    "Befindet die Heldin sich in einem Wald, so kann sie einmal am Tag eine Probe auf Willenskraft ablegen, um QS/2 Zustandsstufen von Betäubung, Furcht, Paralyse oder Schmerz abzubauen. Wenn mehrere Stufen auf diese Weise aufgehoben werden können, können diese zwischen den Zuständen aufgeteilt werden (z. B. 1 Stufe Furcht und 2 Stufen Betäubung). Die Probe abzulegen dauert 1 Minute.",
    10,
    false
}

vorteil! {
    HartimNehmen,
    "Hart im Nehmen",
    "Keine",
    "Muss eine Probe auf Selbstbeherrschung (Handlungsfähigkeit bewahren oder Störungen ignorieren) bei Wundeffekten abgelegt werden, bekommt die Spielerin eine Erleichterung von 1.",
    3,
    false
}

pub struct Hass {
    kosten: u16,
}

fn hass_ap(hass: &Hass) -> u16 {
    hass.kosten
}

vorteil! {
    @noauto
    Hass,
    "Hass",
    "Keine",
    "Der Held richtet gegen eine Art von Wesen, das beim Erwerb dieses Vorteils festgelegt werden muss, +1 TP im Nahkampf an. Der Typus des Wesens muss dabei näher spezifiziert werden. Je häufiger der Typus des Wesens in Aventurien vorkommt, desto teurer der AP-Wert für diesen Vorteil. Ausgeschlossen als wählbare Typen sind die Oberkategorien Lebewesen und Nicht Lebende. Neben dem im Regelwerk aufgeführten Typen und Unterkategorien kann man auch Spezies und Kulturen wählen. Die eigene Spezies/Kultur ist prinzipiell wählbar, jedoch sind Helden mit Hass auf die eigene Spezies/Kultur oft nur schwer spielbar. Der Meister hat das letzte Wort bezüglich der Wählbarkeit eines Hassobjektes. Je kleinteiliger ein Typus gewählt wird und je seltener eine potenzielle Begegnung mit ihm ist, desto niedriger die AP-Kosten für den Vorteil. Mehr zu den Typen und ihren Unterkategorien siehe Regelwerk Seite 355.\n\nBeispiele für die Klassifizierung des Hasses# Häufig : Menschen, Orks, Goblins AP-Wert: 15 Abenteuerpunkte# Gelegentlich : Baumdrachen, Elfen, Karene, Untote, Zwerge AP-Wert: 10 Abenteuerpunkte# Selten : Basilisken, Feen, Lehmgolems, Greife, Riesenlindwurm AP-Wert: 5 Abenteuerpunkte",
    @call hass_ap,
    false
}

vorteil! {
    Hauttasche,
    "Hauttasche (*)",
    "Spezies, Kultur oder Profession muss Vorteil als automatischen oder empfohlenen Vorteil aufweisen.",
    "Der Held hat zwei kleine Hauttaschen, die je einen tassengroßen Gegenstand oder vier handflächengroße Gegenstände aufnehmen können. Verfügt er über den Vorteil Greifschwanz, kann er die Gegenstände mit 1 freien Aktion aus der Hauttasche holen. Um die Gegenstände zu bemerken, ist eine Probe auf Sinnesschärfe (Suchen oder Wahrnehmen) –2 notwendig.",
    10,
    false
}

vorteil! {
    HeldenhafterName,
    "Heldenhafter Name",
    "Ein heldenhafter Name, kein Vorteil Allerweltsname, kein Nachteil Lächerlicher Name, Schurkenname oder Unpassender Name",
    "Abenteurer mit einem heldenhaften Namen  werden respektvoller behandelt als andere Menschen  und man glaubt ihnen eher. Teilproben auf Charisma bei Bekehren & Überzeugen und Überreden sind um 1 erleichtert. Voraussetzung dafür ist, dass das Gegenüber des Helden den Namen als den Namen eines  Helden kennt.",
    8,
    false
}

pub struct HerausragendeFertigkeit {
    pub skalierung: crate::data::SteigerungsFaktor,
}

fn herausragende_fertigkeit_ap(fertigkeit: &HerausragendeFertigkeit) -> u16 {
    use crate::data::SteigerungsFaktor as StF;
    match fertigkeit.skalierung {
        StF::A => 2,
        StF::B => 4,
        StF::C => 6,
        StF::D => 8,
    }
}

vorteil! {
    @noauto
    HerausragendeFertigkeit,
    "Herausragende Fertigkeit",
    "Kein Nachteil Unfähig in der Fertigkeit",
    "Durch den Vorteil Herausragende Fertigkeit verschiebt sich das Maximum in einer Fertigkeit um 1 nach oben, sowohl beim Maximum der Heldenerschaffung als auch beim endgültigen Maximum (abhängig von der höchsten beteiligten Eigenschaft). Zwar erhält ein Held nicht automatisch einen Fertigkeitspunkt dazu, aber er kann zu Spielbeginn zum Beispiel bei Erfahrungsgrad Erfahren mit einem Wert von 11 starten (anstatt 10) und auch sein absolutes Maximum in der Fertigkeit ist um 1 erhöht. Der Vorteil kann bis zu viermal gewählt werden, wobei keine Fertigkeit mehr als zweimal gewählt werden kann. Bei zweifacher Zuteilung auf eine Fertigkeit erhöht der Vorteil das Maximum um 2.",
    @call herausragende_fertigkeit_ap,
    false
}

pub struct HerausragendeKampftechnik {
    stufe: crate::data::SteigerungsFaktor,
}

fn kampftechnik_ap(kampftechnik: &HerausragendeKampftechnik) -> u16 {
    match kampftechnik.stufe {
        crate::data::SteigerungsFaktor::A => 0, // FIXME: This should not be allowed in the UI
        crate::data::SteigerungsFaktor::B => 8,
        crate::data::SteigerungsFaktor::C => 12,
        crate::data::SteigerungsFaktor::D => 16,
    }
}

vorteil! {
    @noauto
    HerausragendeKampftechnik,
    "Herausragende Kampftechnik",
    "",
    "Durch den Vorteil Herausragende Kampftechnik verschiebt sich das Maximum in einer Kampftechnik um 1 nach oben, sowohl beim Maximum der Heldenerschaffung als auch beim endgültigen Maximum (abhängig von der Leiteigenschaft). Zwar erhält der Held nicht automatisch einen Kampftechnikpunkt dazu, aber er kann zu Spielbeginn zum Beispiel bei Erfahrungsgrad Erfahren mit einem Wert von 13 starten (anstatt 12) und auch das absolute Maximum in der Kampftechnik ist um 1 erhöht. Der Vorteil kann bis zu zweimal gewählt werden, wobei jede Kampftechnik nur einmal gewählt werden kann.",
    @call kampftechnik_ap,
    false
}

pub enum Sinn {
    Sicht,
    Gehör,
    Geruch,
    Geschmack,
    Tastsinn,
}

pub struct HerausragenderSinn {
    sinn: Sinn,
}

vorteil! {
    @noauto
    HerausragenderSinn,
    "Herausragender Sinn",
    "Kein Nachteil Eingeschränkter Sinn für den gleichen Sinn, kein Nachteil Blind (für Sicht), kein Nachteil Taub (für Gehör)",
    "Wer über einen Herausragenden Sinn verfügt, dessen Proben auf Sinnesschärfe sind um 1 erleichtert, wenn die Probe den entsprechenden Sinn betrifft. Folgende Sinne können gewählt werden: Sicht, Gehör, Geruch & Geschmack, Tastsinn. Herausragender Sinn ist mehrfach für die verschiedenen Sinne wählbar, nicht aber für einen einzelnen.",
    @call |sinn: &HerausragenderSinn| {
        match sinn.sinn {
            Sinn::Sicht | Sinn::Gehör => 12,
            Sinn::Geruch | Sinn::Geschmack => 6,
            Sinn::Tastsinn => 2,
        }
    },
    false
}

vorteil! {
    Hitzeresistenz,
    "Hitzeresistenz",
    "Kein Nachteil Hitzeempfindlich",
    "Hitzestufen werden für den Helden um 1 Stufe reduziert, bis zu einem Minimum von Hitzestufe 1.",
    5,
    false
}

pub struct HoheAstralkraftIBisVII {
    stufe: u8,
}

vorteil! {
    @noauto
    HoheAstralkraftIBisVII,
    "HoheAstralkraft I-VII",
    "Vorteil Zauberer, kein Nachteil Niedrige Astralkraft",
    "Der AE-Grundwert steigt durch diesen Vorteil um 1 Punkt pro Stufe des Vorteils.",
    @call |ast: &HoheAstralkraftIBisVII| {ast.stufe as u16 * 6},
    false
}

pub struct HoheKarmalkraftIBisVII {
    stufe: u16,
}

vorteil! {
    @noauto
    HoheKarmalkraftIBisVII,
    "Hohe Karmalkraft I-VII",
    "Vorteil Geweihter, kein Nachteil Niedrige Karmalkraft",
    "Der KE-Grundwert steigt durch diesen Vorteil um 1 Punkt pro Stufe des Vorteils.",
    @call |kam: &HoheKarmalkraftIBisVII| {kam.stufe as u16 * 6},
    false
}

pub struct HoheLebenskraftIBisVII {
    stufe: u16,
}

vorteil! {
    @noauto
    HoheLebenskraftIBisVII,
    "Hohe Lebenskraft I-VII",
    "Kein Nachteil Niedrige Lebenskraft",
    "Der LE-Grundwert steigt durch diesen Vorteil um 1 Punkt pro Stufe des Vorteils.",
    @call |lek: &HoheLebenskraftIBisVII| {lek.stufe as u16 * 6},
    false
}

vorteil! {
    HoheSeelenkraft,
    "Hohe Seelenkraft",
    "Kein Nachteil Niedrige Seelenkraft",
    "Der SK-Grundwert des Helden ist um 1 besser.",
    25,
    false
}

vorteil! {
    HoheZaehigkeit,
    "Hohe Zähigkeit",
    "Kein Nachteil Niedrige Zähigkeit",
    "Der ZK-Grundwert des Helden ist um 1 besser.",
    25,
    false
}

pub struct ImmunitaetgegenGift {}

vorteil! {
    @noauto
    ImmunitaetgegenGift,
    "Immunität gegen (Gift)",
    "Keine",
    "Wer immun gegen ein bestimmtes Gift ist, auf den hat dieses Gift keinerlei Wirkung. Eine Immunität muss für jedes Gift einzeln erworben werden.",
    @call |immun: &ImmunitaetgegenGift| {todo!()}, // TODO: halbe Giftstufe in Abenteuerpunkten
    false
}

pub struct ImmunitaetgegenKrankheit {}

vorteil! {
    @noauto
    ImmunitaetgegenKrankheit,
    "Immunität gegen (Krankheit)",
    "Keine",
    "Wer immun gegen eine bestimmte Krankheit ist, kann nicht an ihr erkranken. Eine Immunität muss für jede Krankheit einzeln erworben werden.",
    @call |immun: &ImmunitaetgegenKrankheit| {todo!()}, // TODO: halbe Krankheitsstufe in Abenteuerpunkten
    false
}

vorteil! {
    Kaelteresistenz,
    "Kälteresistenz",
    "Kein Nachteil Kälteempfindlich",
    "Kältestufen werden für den Helden um 1 Stufe reduziert, bis zu einem Minimum von 0.",
    5,
    false
}

vorteil! {
    KeinBanndesEisens,
    "Kein Bann des Eisens",
    "Sonderfertigkeit Tradition Meistertalentierte, kein Vorteil Eisenaffine Aura",
    "Der Held kann die Erschwernisse durch Bann des Eisens (siehe Regelwerk Seite 256) ignorieren. Er ist weder durch das Tragen von Eisen in Form von Waffen, Rüstungen oder Ausrüstung betroffen, noch erleidet er irgendwelche Nachteile dadurch, dass er mit dem Material arbeitet",
    25,
    false
}

vorteil! {
    Koboldfreund,
    "Koboldfreund",
    "Kein Vorteil Hass auf Kobolde (und deren Untergruppen), keine Schlechte Eigenschaft Vorurteile gegen Kobolde",
    "Proben auf Überreden (Aufschwatzen, Herausreden, Manipulieren, Schmeicheln), um einen Kobold/Klabauterfreundlich zu stimmen, gelten bei Gelingen der Probe als um 1 QS besser. Allerdings kann sich die QS so nie über 6 hinaus erhöhen.",
    3,
    false
}

vorteil! {
    KontaktBettler,
    "Kontakt (Bettler)",
    "",
    "Von den Menschen kaum beachtet, ist ein Bettler eine Informationsquelle für allerlei Gerüchte, die sich auf den Gassen einer Stadt abspielen. Er kann Auge und Ohr offen halten und für ein paar Silbertaler einem Helden Gerüchte über Stadtwachen, Diebe und Patrizier verraten.\n\nTalente: Gassenwissen 11 (12/14/12), Handel 7 (12/14/12), Überreden 11 (10/14/12), Willenskraft 4 (10/14/12)\nEinfluss: 1\nZuverlässigkeit: 2",
    5,
    false
}

vorteil! {
    KontaktBotenreiter,
    "Kontakt (Botenreiter)",
    "",
    "Ein Botenreiter kommt viel herum und ist deshalb eine gute Informationsquelle für Geschehnisse, die sich an deren Orten zugetragen haben, oder auch, was sich auf dem Weg dorthin ereignet hat. Zudem kann er den Helden neue Pferde oder Schreiber vermitteln.\n\nTalente: Gassenwissen 9 (12/13/12), Handel 9 (12/13/12), Reiten 13 (12/13/13), Willenskraft 5 (13/13/12)\nEinfluss: 2\nZuverlässigkeit: 3",
    13,
    false
}

vorteil! {
    KontaktFreundausKindheitstagen,
    "Kontakt (Freund aus Kindheitstagen)",
    "",
    "Der Freund aus Kindheitstagen eines Helden ist ein treuer Kamerad, der vielleicht nicht über viel Einfluss verfügt, aber stets bereit ist, dem Helden einen kleinen Gefallen zu tun.\n\nTalente: Gassenwissen 5 (11/13/12), Handel 5 (11/13/12), Willenskraft 6 (13/13/12)\nEinfluss: 2\nZuverlässigkeit: 5",
    29,
    false
}

vorteil! {
    KontaktGeweihte,
    "Kontakt (Geweihte)",
    "",
    "Ein einfacher Geweihter kann dem Helden bei Fragen zu kirchlichen Themen beistehen oder für ihn auch eine passende Liturgie wirken, um ihm bei der Lösung eines Problems zu helfen, sofern dies nicht im Widerspruch zur Gottheit oder Kirche des Geweihten steht.\n\nTalente: Gassenwissen 4 (13/14/14), Götter & Kulte 10 (14/14/14), Handel 5 (13/14/14), Willenskraft 7 (13/14/14)\nEinfluss: 3\nZuverlässigkeit: 3",
    18,
    false
}

vorteil! {
    KontaktHandwerksmeister,
    "Kontakt (Handwerksmeister)",
    "",
    "Der Handwerksmeister, beispielsweise ein Schmied, kann ein Kontakt des Helden sein, der ihm ermöglicht, deutlich bessere Ausrüstung zu oder die Hilfe einer Zunft oder Gilde zu erhalten.\n\nTalente: ein Handwerkstalent 12 (13/13/13), Gassenwissen 6 (11/12/11), Handel 7 (11/12/11), Willenskraft 5 (12/12/11)\nEinfluss: 2\nZuverlässigkeit: 2",
    8,
    false
}

vorteil! {
    KontaktHehler,
    "Kontakt (Hehler)",
    "",
    "Hehler sind Experten darin, Waren ein- und zu verkaufen, allerdings meist gestohlene Ware oder Schmuggelgut. Für einen Helden kann ein Hehler der wichtigste Kontakt in der Unterwelt einer Stadt sein, verschafft er dem Abenteurer doch Ausrüstung (und Informationen), an die er sonst nicht herankommt.\n\nTalente: Gassenwissen 13 (13/14/13), Handel 13 (13/14/13), Willenskraft 8 (13/14/13)\nEinfluss: 3\nZuverlässigkeit: 1",
    10,
    false
}

vorteil! {
    KontaktJunkerin,
    "Kontakt (Junkerin)",
    "",
    "Die Junkerin ist eine Kleinadlige, die diverse Kontakte zu Adelskreisen oder einflussreichen Persönlichkeiten hat.\n\nTalente: Etikette 10 (12/12/12), Gassenwissen 6 (12/12/12), Handel 6 (12/12/12), Willenskraft 5 (12/12/12)\nEinfluss: 3\nZuverlässigkeit: 2",
    13,
    false
}

vorteil! {
    KontaktMagier,
    "Kontakt (Magier)",
    "",
    "Ein Magier kann einem Helden als zuverlässiger Garant für Informationen über magische Ereignisse dienen oder ihm auch mit seiner Magie zur Seite stehen, wenn der Abenteurer von ihm einen Gefallen verlangt.\n\nTalente: Etikette 4 (14/14/14), Gassenwissen 4 (14/14/14), Handel 5 (14/14/14), Magiekunde 12 (14/14/14), Willenskraft 7 (14/14/14)\nEinfluss: 3\nZuverlässigkeit: 3",
    18,
    false
}

vorteil! {
    KontaktSchmuggler,
    "Kontakt (Schmuggler)",
    "",
    "Schmuggler sind ausgesprochen nützliche Kontakte, wenn man ungesehen in Städte hinein oder hinaus will. Zudem sind sie ortskundig, kennen eine Menge anderer Leute und können einem möglicherweise den einen oder anderen Ausrüstungsgegenstand beschaffen, den man ansonsten nicht bekommt.\n\nTalente: Gassenwissen 11 (12/14/12), Handel 10 (12/14/12), Willenskraft 6 (14/14/12)\nEinfluss: 3\nZuverlässigkeit: 4",
    25,
    false
}

vorteil! {
    KontaktSklavin,
    "Kontakt (Sklavin)",
    "",
    "Auf eine Sklavin achtet kaum jemand. Dennoch kann gerade sie, die von anderen kaum bemerkt wird, eine ausgezeichneteQuelle für Informationen über ihre Herrin oder deren Gäste sein.\n\nTalente: Etikette 7 (11/12/12), Gassenwissen 7 (11/12/12), Handel 5 (11/12/12), Willenskraft 3 (9/12/12)\nEinfluss: 1\nZuverlässigkeit: 1",
    2,
    false
}

vorteil! {
    KontaktSoldat,
    "Kontakt (Soldat)",
    "",
    "Ein Soldat hat schon auf einigen Schlachtfeldern Aventuriens gekämpft. Für einen Helden kann er ein Ansprechpartner sein, wenn sie Informationen über einen bestimmten Feind benötigen, jemand, der sich beim Kampf unterstützt, oder jemand, der sie mit Informationen über militärische Kreise versorgt.\n\nTalente: Gassenwissen 6 (10/12/11), Handel 5 (10/12/11), Kriegskunst 8 (14/10/12), Willenskraft 5 (14/10/11)\nEinfluss: 1\nZuverlässigkeit: 3",
    10,
    false
}

vorteil! {
    KontaktSoeldnerhauptmann,
    "Kontakt (Söldnerhauptmann)",
    "",
    "Der Söldnerhauptmann hebt sich deutlich von seinen Untergebenen ab und hat zahlreiche Kontakte zu einfachen Söldnern, wie auch zur Oberschicht. Somit kann er eine sehr vielseitige Verbindung sein.\n\nTalente: Gassenwissen 7 (12/12/12), Handel 7 (12/12/12), Kriegskunst 11 (14/12/12), Willenskraft 8 (14/12/11)\nEinfluss: 3\nZuverlässigkeit: 2",
    13,
    false
}

vorteil! {
    KontaktStadtmagistrat,
    "Kontakt (Stadtmagistrat)",
    "",
    "Städte besitzen in der Regel mehrere Beamte, die den Herrscher unterstützen. Der Stadtmagistrat ist ein Teil der Stadtregierung und kann einem Helden helfen, wenn er Ärger in der Stadt hat.\n\nTalente: Etikette 9 (13/13/13), Gassenwissen 6(13/13/13), Handel 8(13/13/13), Willenskraft 7 (12/13/13)\nEinfluss: 3\nZuverlässigkeit: 2",
    13,
    false
}

vorteil! {
    KontaktStadtwache,
    "Kontakt (Stadtwache)",
    "",
    "Ein Stadtgardist kann den Helden einen Gefallen tun, wenn sie mit dem Gesetz in Konflikt sind und noch mal ein Auge zudrücken. Zudem ist er eine gute Quelle für Informationen, da er tagtäglich mit vielen Leuten auf der Straße zu tun hat.\n\nTalente: Gassenwissen 10 (12/13/12), Handel 7 (12/13/12), Willenskraft 6 (14/13/12)\nEinfluss: 2\nZuverlässigkeit: 3",
    13,
    false
}

vorteil! {
    KontaktTempelvorsteherin,
    "Kontakt (Tempelvorsteherin)",
    "",
    "Eine Tempelvorsteherin ist eine hochangesehene Persönlichkeit, die einem Helden nicht nur helfen kann, indem sie ihm eine Waffe oder ihn selbst segnet, sondern auch andere Kontakte vermittelt.\n\nTalente: Etikette 10 (14/14/14), Gassenwissen 6 (14/14/14), Handel 6 (14/14/14), Willenskraft (14/14/14)\nEinfluss: 4\nZuverlässigkeit: 3",
    25,
    false
}

vorteil! {
    KontaktUnterweltkoenig,
    "Kontakt (Unterweltkönig)",
    "",
    "In Städten gibt es Unterweltbanden, die einen mehr oder weniger fähigen Anführer haben. Einer dieser Unterweltkönigekann für einen Helden ein wertvoller Kontakt sein, da er Schläger zum Schutz des Helden bereitstellen, einen Gegenstand von seinen Leuten stehlen lassen oder den Helden mit Gerüchten versorgen kann.\n\nTalente: Gassenwissen 14 (13/14/14), Handel 9 (13/14/14), Willenskraft 10 (14/14/14)\nEinfluss: 2\nZuverlässigkeit: 4",
    20,
    false
}

vorteil! {
    KontaktWirt,
    "Kontakt (Wirt)",
    "",
    "An kaum einem anderen Ort treffen sich Helden so häufig wie in einer Taverne oder einem Wirtshaus. Der Wirt ist oftmals ein zuverlässiger Freund des Abenteurers, bei dem er einen sicheren Unterschlupf finden, der ihm Aufträge vermitteln oder auch die eine oder andere Informationen beschaffen kann.\n\nTalente: Gassenwissen 9 (11/14/12), Handel 9 (11/14/12), Willenskraft 6 (12/14/12)\nEinfluss: 2\nZuverlässigkeit: 4",
    20,
    false
}

vorteil! {
    KraftderBaeume,
    "Kraft der Bäume",
    "Vorteil Feenblut (Dryade)",
    "Schläft die Heldin unter einem Baum, kann sie einmal am Tag einen Teil von dessen Lebenskraft in sich aufnehmen. Sie regeneriert dadurch 1W3 LeP. Hierfür muss sie jedoch mindestens eine halbe Stunde unter dem Baum schlafen.",
    10,
    false
}

pub struct KrankheitsresistenzIBisII {
    stufe: u8,
}

vorteil! {
    @noauto
    KrankheitsresistenzIBisII,
    "Krankheitsresistenz I-II",
    "Kein Nachteil Krankheitsanfällig",
    "Eine Krankheitsresistenz verbessert Zähigkeit und Seelenkraft um 1 pro Stufe, wenn es darum geht, Krankheiten zu widerstehen.",
    @call |resi: &KrankheitsresistenzIBisII| {resi.stufe as u16 * 10},
    false
}

vorteil! {
    LeichterGang,
    "Leichter Gang",
    "Vorteil Feenblut (Blütenfee)",
    "Die Heldin kann sich schneller bewegen als normale Menschen. Ihre GS erhält einen Bonus von 1. Zudem erhält sie + 1FP bei gelungenen Proben auf Tanzen (bis zu einem Maximum von 18 FP).",
    10,
    false
}

vorteil! {
    MachtvolleZaubermelodien,
    "Machtvolle Zaubermelodien",
    "Sonderfertigkeit Tradition Zauberbarden, kein Nachteil Schwache Zaubermelodien",
    "Der Abenteurer kann Personen bis zu SK 3 durch seine Zaubermelodien verzaubern (statt nur bis zu SK 2).",
    15,
    false
}

vorteil! {
    MachtvolleZaubertaenze,
    "Machtvolle Zaubertänze",
    "Sonderfertigkeit Tradition Zaubertänzer, kein Nachteil Schwache Zaubertänze",
    "Die Heldin kann Personen bis zu SK 3 durch ihre Zaubertänze verzaubern (statt nur bis zu SK 2).",
    15,
    false
}

vorteil! {
    MagischeEinstimmung,
    "Magische Einstimmung",
    "Vorteil Zauberer, kein Nachteil Magische Einschränkung auf die gleiche Umgebung",
    "Man muss sich für eine Variante des Vorteils entscheiden (Beispiele sind dem nebenstehenden Kasten zu entnehmen). In der betreffenden Umgebung oder Situation, für die der Vorteil gewählt wurde, hat der Zauberer bei magischen Fertigkeitsproben eine Erleichterung von 1.\nBeispiel für Magische Einstimmung (Umgebung)• Wesen der Auen• Wesen der Berge• Wesen des Eises• Wesen der Meere• Wesen der Nacht• Wesen des WaldesAnmerkung: Wesen des Tages gibt es nicht",
    40,
    false
}

vorteil! {
    Mystiker,
    "Mystiker",
    "Vorteil Geweihter, kein Vorteil Pragmatiker",
    "Bei einem Mystiker baut sich Entrückung erst nach zwei Stunden statt einer Stunde pro Stufe ab.",
    20,
    false
}

vorteil! {
    NatuerlicheWaffeBiss,
    "Natürliche Waffe (Biss) (*)",
    "Spezies, Kultur oder Profession muss Vorteil als automatischen oder empfohlenen Vorteil aufweisen.",
    "Das Wesen kann den Vorteil Biss erhalten und diesen mit einer Raufen-AT einsetzen. Dieser Angriff ersetzt den sonst üblichen Waffenlosen Raufen-Angriff.",
    5,
    false
}

vorteil! {
    NatuerlicheWaffeSchwanz,
    "Natürliche Waffe (Schwanz) (*)",
    "Spezies, Kultur oder Profession muss Vorteil als automatischen oder empfohlenen Vorteil aufweisen.",
    "Das Wesen kann den Vorteil Schwanzschlag erhalten und diesen mit einer Raufen-AT einsetzen. Dieser Angriff ersetzt den sonst üblichen waffenlosen Raufen-Angriff.",
    5,
    false
}

pub struct NatuerlicherRuestungsschutzIBisII {
    stufe: u16,
}

vorteil! {
    @noauto
    NatuerlicherRuestungsschutzIBisII,
    "NatuerlicherRuestungsschutz I-II (*)",
    "Spezies, Kultur oder Profession muss Natürlicher Rüstungsschutz als automatischen oder empfohlenen Vorteil aufweisen.",
    "Das Wesen verfügt pro Stufe des Vorteils über einen natürlichen Rüstungsschutz, der mit anderen Rüstungen kombiniert werden kann.",
    @call |ruestung: &NatuerlicherRuestungsschutzIBisII| {ruestung.stufe as u16 * 20},
    false
}

vorteil! {
    Nichtschlaefer,
    "Nichtschläfer (*)",
    "Spezies, Kultur oder Profession muss Nichtschläfer als automatischen oder empfohlenen Vorteil besitzen",
    "Ein Wesen mit diesem Vorteil muss nicht regelmäßig schlafen, sondern kann bis zu einer Woche wach bleiben, ohne zu ermüden. Der Schlaf muss aber anschließend nachgeholt werden, was bei einigen Spezies durch echten Schlaf, bei anderen durch Meditation oder vergleichbare Techniken erreicht wird. Während der Zeit, in der der Nichtschläfer nicht schläft, kann er auch nicht durch Ausruhen und Schlafen regenerieren (Ausnahme stellt die Verwendung von Heilkräutern und der Einsatz des Talents Heilkunde Wunden dar).",
    8,
    false
}

vorteil! {
    Pragmatiker,
    "Pragmatiker",
    "Vorteil Geweihter, kein Vorteil Mystiker",
    "Bei einem Pragmatiker baut sich Entrückung bereits nach einer halben Stunde statt einer Stunde pro Stufe ab.",
    10,
    false
}

vorteil! {
    Prediger,
    "Prediger",
    "Nachteil Prinzipientreue I (ausgewählte Gottheit), kein Vorteil Geweihter, kein Vorteil Visionär",
    "Der Held bekommt eine neue Einsatzmöglichkeit für Bekehren & Überzeugen (öffentliche Rede). Er kann bis zu drei Predigt-Sonderfertigkeiten erlernen, auch wenn er nicht über den Vorteil Geweihter verfügt. Der Zelot muss eine Gottheit auswählen, wenn er sich für diesen Vorteil entscheidet. Lässt sich ein Prediger im späteren Verlauf seines Lebens der ausgewählten Gottheit weihen, bekommt er die 10 AP dieses Vorteils wieder zurück bzw. kann sie mit Geweihter verrechnen.",
    10,
    false
}

pub struct ReichIBisX {
    stufe: u8,
}

vorteil! {
    @noauto
    ReichIBisX,
    "Reich I-X",
    "Kein Nachteil Arm",
    "Durch den Vorteil Reich erhält ein Held bei der Generierung zusätzlich 250 Silbertaler pro Stufe.",
    @call |reich: &ReichIBisX| {reich.stufe as u16},
    false
}

vorteil! {
    Richtungssinn,
    "Richtungssinn",
    "Kein Nachtteil Unfähig in Orientierung",
    "Proben auf Orientierung sind immer um 1 erleichtert.",
    10,
    false
}

vorteil! {
    Sagaheld,
    "Sagaheld",
    "Die Heldin muss eine immense Ruhmestat vollbracht haben.",
    "Die Heldin wird von Thorwalern so behandelt, als hätte sie einen um 1 höheren sozialen Rang als üblich (siehe Regelwerk Seite 338). Das Gegenüber muss aber wissen oder zumindest glauben, dass die Heldin eine Ruhmestat vollbracht hat, die einer Saga würdig ist, damit der Bonus zählt. Die Heldin erhält zudem einen Bonus von +2/+1 bei Proben auf Überreden gegenüber anderen Thorwalern/Nicht-Thorwalern.",
    15,
    false
}

pub struct SaumagenIBisII {
    stufe: u8,
}

vorteil! {
    @noauto
    SaumagenIBisII,
    "Saumagen I-II",
    "Keine",
    "Mit Saumagen kann der Held zähes Fleisch problemlos essen, so als ob es normales Fleisch wäre. Bei Stufe II kann er sogar ungenießbares Fleisch verzehren. Es wird ihm nicht sonderlich schmecken, aber er kann es verdauen, ohne dass ihm übel wird und er sich erbrechen muss. Gifte und Krankheitskeime können jedoch nicht durch den Vorteil umgangen werden. Ähnliches gilt auch für pflanzliche Nahrungsmittel. Schwerverdauliche Kost ist essbar, ohne dass es weitere Auswirkungen hat.",
    @call |magen: &SaumagenIBisII| {magen.stufe as u16* 2 },
    false
}

vorteil! {
    Schlangenmensch,
    "Schlangenmensch",
    "Keine",
    "Proben auf Körperbeherrschung (Akrobatik, Entwinden) sind um 1 erleichtert.",
    6,
    false
}

vorteil! {
    SchnellwiederaufdenBeinen,
    "Schnell wieder auf den Beinen",
    "Keine",
    "Mit diesem Vorteil benötigt man nur noch die Hälfte der Zeit, bis eine Stufe des Zustands Betäubung oder Berauscht, der durch Alkohol verursacht wurde, abgebaut ist.",
    2,
    false
}

vorteil! {
    Schutzgeist,
    "Schutzgeist",
    "Tradition Animisten, Intuitive Zauberer oder Meistertalentierte",
    "Wer über einen Schutzgeist verfügt, der kann die folgenden Optionen einsetzen. Sie kosten weder eine Aktion noch eine freie Aktion, sondern können jederzeit eingesetzt werden.\n\n#Bei einem Patzer kann der Held einen Schip einsetzen. Das Ergebnis gilt einfach als misslungen, ist aber kein Patzer.\n\n#Wenn der Held nur noch 5 oder weniger LeP besitzt, kann er 1 Schip einsetzen, um 1W6 LeP zurückzubekommen. Der Einsatz dieser Option ist nur einmal am Tag gestattet.",
    10,
    false
}

vorteil! {
    Schwerzuverzaubern,
    "Schwer zu verzaubern (*)",
    "Spezies, Kultur oder Profession muss Schwer zu verzaubern als automatischen oder empfohlenen Vorteil besitzen.",
    "Viele Zauber, die auf den Helden gesprochen werden können, die auf ihn wirken, sind um 1 erschwert. Dies gilt für alle Zauber und magischen Effekte mit den Merkmalen Einfluss, Heilung, Hellsicht und Verwandlung sowie für solche, die durch Seelenkraft oder Zähigkeit modifiziert werden. Diese Erschwernis gilt auch dann, wenn der Held sich verzaubern lassen möchte.",
    15,
    false
}

vorteil! {
    Seesprache,
    "Seesprache",
    "Vorteil Feenblut (Nixer)",
    "Der Held kann sich mit allen Süß- und Salzwassertieren unterhalten. Je nach KL des Tiers kann es unterschiedlich gut Auskunft geben, das Gespräch ist dabei jedoch immer durch die tierische Intelligenz begrenzt. Tiefgreifende Diskussionen sind daher nicht zu erwarten.",
    10,
    false
}

vorteil! {
    SozialeAnpassungsfaehigkeit,
    "Soziale Anpassungsfähigkeit",
    "Kein Nachteil Unfähig auf ein Gesellschaftstalent, kein Nachteil Unfrei",
    "Personen mit Sozialer Anpassungsfähigkeit können Erschwernisse aufGesellschaftstalente ignorieren, die durch Standesunterschiede entstehen. Die Soziale Anpassungsfähigkeit täuscht aber keine Personen des Hochadels.",
    10,
    false
}

vorteil! {
    Tierfreund,
    "Tierfreund",
    "Kein Vorteil Hass auf die Tierart",
    "Proben auf Tierkunde (domestizierte Tiere, Wildtiere) gelten bei Gelingen der Probe als um 1 QS besser, allerdings kann sich die QS dabei nie über 6 erhöhen.",
    10,
    false
}

vorteil! {
    TiergeistauraPatron,
    "Tiergeistaura (Patron)",
    "Tradition (Animisten), kein Nachteil Raubtiergeruch oder Unfähig (Tierkunde)",
    "Tiere verspüren ein Misstrauen gegenüber der Heldin. Proben auf Reiten und Tierkunde, wenn es um das Beruhigen oder Abrichten des Tieres geht, sind um –1 erschwert. Allerdings erhält die Heldin auch +1 QS (bis zu einem Maximum von 6 QS) bei Proben auf Tierkunde, wenn es um das Beruhigen oder Abrichten von Tieren geht, die ihrem primären Tierpatron entsprechen.",
    2,
    false
}

vorteil! {
    TierischeKampfgefaehrten,
    "Tierische Kampfgefährten",
    "Vorteil Feenblut (Großer Biestinger)",
    "Gelingt dem Helden eine Probe auf Tierkunde (Wildtiere), so stürzen sich Säugetiere und/oder Vögel der Größenkategorie winzig auf bis zu 3 vom Helden benannte Feinde, die nicht weiter als 16 Schritt von ihm entfernt sein dürfen. Die Gegner erleiden durch die kleinen Tiere für QS/2 KR eine Erschwernis von 1 auf alle Proben inklusive AT, PA, AW und FK. Nach diesem Zeitraum ziehen sich die Tiere wieder zurück. Diese Fähigkeit kann nur einmal am Tag genutzt werden und setzt zudem die Anwesenheit der Tiere in einer Umgebung von bis zu 64 Schritt Radius um den Helden voraus. Der Meister kann bei einer zu kleinen Zahl an Tieren auch die Anzahl der abgelenkten Feinde begrenzen (auch auf bis zu 0). Der Einsatz kostet 1 freie Aktion.",
    12,
    false
}

vorteil! {
    UnbeschwertesZaubern,
    "Unbeschwertes Zaubern",
    "Tradition (Schelme)",
    "Der Schelm kann eine SK und ZK seiner Ziele von bis zu 2 bei seinen Schelmenstreichen ignorieren. Dieser Vorteil ist mit der Sonderfertigkeit Lockeres Zaubern (siehe Seite 72) kombinierbar, sodass SK und ZK von bis zu 3 ignoriert werden können.",
    15,
    false
}

vorteil! {
    Unscheinbar,
    "Unscheinbar",
    "Keine",
    "Helden mit diesem Vorteil erhalten eine Erleichterung von 1 auf Gassenwissen (Beschatten).",
    4,
    false
}

pub struct VerbesserteRegenerationAstralenergieIBisIII {
    stufe: u8,
}

vorteil! {
    @noauto
    VerbesserteRegenerationAstralenergieIBisIII,
    "Verbesserte Regeneration (Astralenergie) I-III",
    "Vorteil Zauberer, kein Nachteil Schlechte Regeneration (Astralenergie)",
    "Für jede Stufe des Vorteils bekommt der Held 1 zusätzlichen Punkt dazu, wenn er regeneriert.",
    @call |regen: &VerbesserteRegenerationAstralenergieIBisIII| {regen.stufe as u16 * 10},
    false
}

pub struct VerbesserteRegenerationKarmaenergieIBisIII {
    stufe: u8,
}

vorteil! {
    @noauto
    VerbesserteRegenerationKarmaenergieIBisIII,
    "Verbesserte Regeneration (Karmaenergie) I-III",
    "Vorteil Geweihter, kein Nachteil Schlechte Regeneration (Karmaenergie)",
    "Für jede Stufe des Vorteils bekommt der Held 1 zusätzlichen Punkt dazu, wenn er regeneriert.",
    @call |regen: &VerbesserteRegenerationKarmaenergieIBisIII| {regen.stufe as u16 * 10},
    false
}

pub struct VerbesserteRegenerationLebensenergieIBisIII {
    stufe: u8,
}

vorteil! {
    @noauto
    VerbesserteRegenerationLebensenergieIBisIII,
    "Verbesserte Regeneration (Lebensenergie) I-III",
    "Kein Nachteil Schlechte Regeneration (Lebensenergie)",
    "Für jede Stufe des Vorteils bekommt der Held 1 zusätzlichen Punkt dazu, wenn er regeneriert,",
    @call |regen: &VerbesserteRegenerationLebensenergieIBisIII| {regen.stufe as u16 * 10},
    false
}

vorteil! {
    VerfuehrerischeGestalt,
    "Verführerische Gestalt",
    "Vorteil Feenblut (Dryade)",
    "Dieser Vorteil eröffnet eine Einsatzmöglichkeit des Talents Betören (Anbändeln). Wenn die Heldin sich verführerisch und charmant gibt, kann sie mehrere Personen in ihren Bann ziehen, sodass diese als leicht abgelenkt gelten und sie z. B. bei Proben auf Sinnesschärfe eine Erschwernis von 1 erhalten. Die Spielerin würfelt hierzu eine Probe auf Betören (Anbändeln). Die Reichweite der Wirkung liegt bei QS x 3 Schritt, maximal kann die Abenteurerin QS Personen ihrer Wahl durch die Wirkung beeinflussen. Die Wirkung hält 5 Minuten an und kann nicht in einem Kampf eingesetzt werden.",
    10,
    false
}

vorteil! {
    VerhuellteAura,
    "Verhüllte Aura",
    "Vorteil Zauberer",
    "Bei ungerichteter magischer Wahrnehmung erscheint der Zauberkundige immer als nicht magisch. Wer gezielt versucht, den Zauberkundigen zu entdecken oder zu analysieren, beispielsweise mit dem Zauber ODEM oder der Liturgie Magiesicht, muss eine Erschwernis von 3 hinnehmen.",
    20,
    false
}

vorteil! {
    Vertrauenerweckend,
    "Vertrauenerweckend",
    "Kein Nachteil Unfähig in einem Gesellschaftstalent",
    "Proben auf Bekehren & Überzeugen (öffentliche Rede, Einzelgespräch, Diskussionsführung), Überreden (Betteln, Manipulieren, Schmeicheln) und Handel (Feilschen) sind um 1 erleichtert.",
    25,
    false
}

vorteil! {
    Visionaer,
    "Visionär",
    "Nachteil Prinzipientreue I (ausgewählte Gottheit), kein Vorteil Geweihter, kein Vorteil Prediger",
    "Die Heldin bekommt eine neue Einsatzmöglichkeit für Götter & Kulte (eigene Gottheit). Sie kann bis zu drei Vision-Sonderfertigkeiten erlernen, auch wenn sie nicht über den Vorteil Geweihter verfügt. Die Zelotin muss eine Gottheit auswählen, wenn sie sich für diesen Vorteil entscheidet. Lässt sich eine Visionärin im späteren Verlauf ihres Lebens der ausgewählten Gottheit weihen, bekommt sie die 10 AP dieses Vorteils wieder zurück bzw. kann sie mit Geweihter verrechnen.",
    10,
    false
}

pub struct Waffenbegabung {}

vorteil! {
    @noauto
    Waffenbegabung,
    "Waffenbegabung",
    "Keine",
    "Eine Waffenbegabung erlaubt bei Kritischen Erfolgen und Patzern bei einer Probe auf eine Kampftechnik eine Wiederholung des Bestätigungswurfs.",
    @call |begabung: &Waffenbegabung| {todo!()}, // TODO: 5/10/15 Abenteuerpunkte ((B-/C-/D-Kampftechnik)),
    false
}

vorteil! {
    WalwutSwafskari,
    "Walwut (Swafskari)",
    "Kultur Thorwal, Nachteil Blutrausch",
    "Personen mit der Kultur Thorwal erleiden eine Erschwernis von –1 auf Vergleichsproben bei Gesellschaftstalenten gegenüber der Heldin (sofern sie diese als Swafnirkind erkennen). Eine Heldin mit diesem Vorteil verliert jedoch auch leichter die Beherrschung und gerät häufigerin Blutrausch. Proben auf Selbstbeherrschung und Willenskraft (um den Blutrausch zu vermeiden), sind um –1 erschwert. Außerdem kann die Heldin keinen höheren Sozialen Stand als Stufe 2 aufweisen.",
    5,
    false
}

vorteil! {
    WasserdesLebens,
    "Wasser des Lebens",
    "Vorteil Feenblut (Nixer)",
    "Gelangt der Held in Kontakt mit einer größeren Menge an Wasser (z. B.\n: aus Meer, Bach, Fluss oder See), kann er eine Regenerationsphase des angebrochenen Tages sofort nutzen. Die nächste reguläre Regenerationsphase danach entfällt für diesen Helden dann jedoch. Die Regeneration dauert 5 Minuten.",
    10,
    false
}

vorteil! {
    Wasserlebewesen,
    "Wasserlebewesen (*)",
    "Spezies, Kultur oder Profession muss Wasserlebewesen als automatischen oder empfohlenen Vorteil aufweisen.",
    "Wesen mit diesem Vorteil erhalten automatisch die Sonderfertigkeit Unterwasserkampf (die Kosten der SF sind in diesem Vorteil einberechnet). Wasserlebewesen können unter Wasser atmen. Außerdem können sie sich darin mit Leichtigkeit fortbewegen, da es sich um ihr natürliches Element handelt. Sie müssen keine Proben ablegen, um nicht zu ertrinken.",
    15,
    false
}

pub struct WeitreichendeZaubermelodienIBisIII {
    stufe: u8,
}

vorteil! {
    @noauto
    WeitreichendeZaubermelodienIBisIII,
    "Weitreichende Zaubermelodien I-III",
    "Sonderfertigkeit Tradition Zauberbarden, kein Nachteil Verminderte Zaubermelodien",
    "Die Zaubermelodien des Abenteurers reichen pro Stufe des Vorteils 2 Schritt weiter als üblich.",
    @call |melodien: &WeitreichendeZaubermelodienIBisIII| {melodien.stufe as u16 * 5},
    false
}

pub struct WeitreichendeZaubertaenzeIBisIII {
    stufe: u8,
}

vorteil! {
    @noauto
    WeitreichendeZaubertaenzeIBisIII,
    "Weitreichende Zaubertänze I-III",
    "Sonderfertigkeit Tradition Zaubertänzer, kein Nachteil Verminderte Zaubertänze",
    "Die Zaubertänze der Abenteurerin reichen pro Stufe des Vorteils 2 Schritt weiter als üblich.",
    @call |taenze: &WeitreichendeZaubertaenzeIBisIII| {taenze.stufe as u16 * 5},
    false
}

vorteil! {
    Wohlklang,
    "Wohlklang",
    "Kein Nachteil Stumm",
    "Fertigkeitsproben auf Singen sind um 1 erleichtert.",
    5,
    false
}

vorteil! {
    Wolfsblut,
    "Wolfsblut",
    "Spezies Mensch (Nivese), Kultur Nivesen",
    "Der Held verfügt über feine Sinne und hat ein Talent darin, Spuren zu folgen. Er erhält bei einer gelungenen Probe auf Fährtensuchen und Sinnesschärfe +1 QS (bis zu einem Maximum von 6 QS). Allerdings lenkt ihn der Vollmond so stark ab, dass alle Proben in der Vollmondnacht und an dem Tag davor und danach um 1 erschwert sind.",
    15,
    false
}

vorteil! {
    Wolfsgebiss,
    "Wolfsgebiss",
    "Vorteil Wolfsblut",
    "Der Held kann sich innerhalb 1 Aktion ein Wolfsgebiss wachsen lassen und damit im Nahkampf mehr Schaden verursachen. Er benutzt dazu eine Raufen-AT und richtet +1 TP im Waffenlosen Kampf an. Zudem kann er die Sonderfertigkeit Verbeißen einsetzen. Die Rückverwandlung zu einem normalen Gebiss dauert ebenfalls 1 Aktion. Außerdem erzielt er, während die Verwandlung andauert, bei Proben auf Einschüchtern +1 FP (bis zu einem Maximum von 18 FP).",
    15,
    false
}

vorteil! {
    Wolfskind,
    "Wolfskind",
    "Vorteil Wolfsblut",
    "Die Heldin kann sich in einen Grimwolf verwandeln (siehe Aventurischer Almanach Seite 168). Dazu benötigt sie 4 Aktionen. Während der Wirkungsdauer verwendet die Heldin die körperlichen Werte des Tieres, einschließlich seiner Eigenschaften, Talente und Kampfwerte. Eine niedrige LE wird bei der Verwandlung anteilsmäßig angerechnet, gleiches geschieht bei der Rückverwandlung. Die geistigen Werte der Heldin bleiben ebenso wie ihr Bewusstsein erhalten. Die Verwandlung betrifft nur die Heldin selbst, nicht ihre Kleidung oder Ausrüstung. Die Wirkungsdauer beträgt 1 Stunde. Die Heldin kann sich nicht willentlich zurückverwandeln. Am Ende der Wirkungsdauer erleidet die Heldin 1 Stufe Betäubung.",
    20,
    false
}

vorteil! {
    ZaeherHund,
    "Zäher Hund",
    "Kein Nachteil Zerbrechlich",
    "Dieser Vorteil sorgt dafür, dass der Held die Auswirkungen der höchsten Stufe des Zustands Schmerz ignorieren darf. Er erleidet lediglich die Auswirkungen der nächstniedrigeren Stufe. Wenn ein Held also beispielsweise drei Stufen Schmerz erlitten hat, gelten für ihn nur die Auswirkungen von Stufe II. Bei Schmerz Stufe IV wird der Held dennoch handlungsunfähig. Schmerz Stufe I wird so behandelt, als hätte der Held keine Stufe Schmerz.",
    20,
    false
}

pub struct ZahlreichePredigtenIBisII {
    stufe: u8,
}

vorteil! {
    @noauto
    ZahlreichePredigtenIBisII,
    "Zahlreiche Predigten I-II",
    "Vorteil Prediger",
    "Bei Stufe I kann der Zelot 4 Predigt-Sonderfertigkeiten beherrschen, bei Stufe II sogar 5.",
    @call |predigten: &ZahlreichePredigtenIBisII| {predigten.stufe as u16 * 8},
    false
}

pub struct ZahlreicheVisionenIBisII {
    stufe: u8,
}

vorteil! {
    @noauto
    ZahlreicheVisionenIBisII,
    "Zahlreiche Visionen I-II",
    "Vorteil Visionär",
    "Bei Stufe I kann der Zelot 4 Vision-Sonderfertigkeiten beherrschen, bei Stufe II sogar 5.",
    @call |visionen: &ZahlreicheVisionenIBisII| {visionen.stufe as u16 * 8},
    false
}

vorteil! {
    Zauberer,
    "Zauberer",
    "Keine",
    "Der Zauberer erhält als Astralenergie- Grundwert 20 AsP. Der Vorteil beinhaltet nicht die Sonderfertigkeit Tradition (siehe Regelwerk Seite 275). Diese muss einzeln erworben werden. Jeder Zauberer muss mit einer Tradition starten (eine Tradition ist eine spezielle Sonderfertigkeit). Dieser Vorteil lässt sich im späteren Spielverlauf nicht mehr erwerben.",
    25,
    false
}

vorteil! {
    Zeitgefuehl,
    "Zeitgefühl",
    "Keine",
    "Der Charakter hat ein ausgezeichnetes Zeitgefühl und kann, ohne den Stand der Sonne zu sehen oder auf andere Hilfsmittel zurückzugreifen, die Tageszeit genau bestimmen.",
    2,
    false
}

vorteil! {
    ZusaetzlicheGliedmassenSchwanz,
    "Zusätzliche Gliedmaßen (Schwanz) (*)",
    "Spezies, Kultur oder Profession muss Vorteil als automatischen oder empfohlenen Vorteil aufweisen.",
    "Das Wesen verfügt über einen Schwanz und kann die Vorteile Natürliche Waffe (Schwanz) und Greifschwanz erhalten. Bei Proben auf Körperbeherrschung (Balance) sind Teilproben auf GE um +1 erleichtert.",
    5,
    false
}

vorteil! {
    ZweistimmigerGesang,
    "Zweistimmiger Gesang (*)",
    "Spezies, Kultur oder Profession muss Zweistimmiger Gesang als automatischen oder empfohlenen Vorteil besitzen.",
    "Ein Held kann den Zweistimmigen Gesang singen und hat somit die Möglichkeit, Elfenlieder zu erlernen. Durch den Vorteil erwirbt der Held das Anwendungsgebiet Zweistimmiger Gesang im Talent Singen.",
    5,
    false
}

vorteil! {
    Zwergennase,
    "Zwergennase",
    "Keine",
    "Der Abenteurer erhält bei Proben auf Sinnesschärfe zur Entdeckung von Geheimtüren, Hohlräumen, versteckten Schubladen und Ähnlichem eine Erleichterung von 1.",
    8,
    false
}
