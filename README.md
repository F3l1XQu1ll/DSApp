# DSApp – DSA Character Sheet

Das Ziel dieses Projektes ist ein digitales Character Sheet für DSA5 zu erstellen.

DSA5 ist eine Pen and Paper von Ulisess Spiele und stellt das deutsche Gegenstück zu DnD da.
Es gibt ein Programm um digital DSA5 Charaktere zu erstellen, dieses hat aber Probleme mit der Aktualität und bietet bei Anwendung eigner Regeln eine schlechte Implentierung zum Einfügen eigner Inhalte.

DSA5 bietet Möglichkeiten das Projekt stark zu erweitern, aber als erstes wollen wir die Charaktererstellung digitalisieren.

Beispiel DSA5 Chartaktersheet:
- [1 / Main](/image/char_sheet_1.png)
- [2 / Talente](/image/char_sheet_2.png)
- [3 / Kampf](/image/char_sheet_3.png)
- [4 / Ausrüstung](/image/char_sheet_4.png)
- [5 / Zauberer](/image/char_sheet_5.png)
- [6 / Geweihter](/image/char_sheet_6.png)


## To Do's:
### Projekt:
| Status | ToDo | notes |
|---|---|---|
| in progress | Einarbeiten | Julian |
| in progress | Dokumentation | | 
| not | Issues | |
| in progress | time table | zur Abgabe fertig; Länge |


### Erstellen eines Charakters
| Status | ToDo | notes |
|---|---|---|
| in progress | Persönlichkeitsdaten | Familie|
| done | Erfahungsgrad / Abenteuerpunkte(AP) | check data|
| in progress | Spezies / Kultur | Boni/Mali|
| in progress | Professionen | Standarts einfügen |
| not | Sprachen / Schriften |
| done | Leiteigenschaften | check data |
| not | Vorteile / Nachteile |
| not | Sonderfertigkeiten | in Profession angefangen :(|
| in progress | Grundwerte (LE, AsP, KP, SK, ZK, As) | Wert ausrechenen, Zukauf ermöglichen|
| done | Talente | Untertalente; check data |
| not | Kampftechniken |
| not | Ausrüstung / Geld |
| not | Tragekraft |
| not | Tiere / Begleiter|
| not | Un-/Geeignete Vor-/Nachteile| Profession/Spezies/Kultur|
| not | Homebrew | done: professionen | 
### Dynamisches Nutzen des Charakters
| Status | ToDo |
|---|---|
| not | Statuseffekte |
| in progress | Speichern von Char |
| not | Speichern von Homebrew |
| not | würfeln |
### weiters
| Status | ToDo | notes |
|---|---|---|
| not | GUI | chapter|
| not | Programm | |
| not | Zonen Rüstung | |
| not | Verweis zu Ulisses-Wiki| |
| not | einheitliche Benennung| |

## Projekt UML (Ausschnitt)

```mermaid
classDiagram
direction RL

class DSApp {
    character: Character
}

class Character {
    +identity: Identity
    +spells: ZauberTabelle
    +talents: BTreeMap~usize#44;#32;u16~
    +erfahrungsgrade: Erfahrungsgrade
    +attributes: Attributes
}

class Attributes {
    +mu: u8
    +kl: u8,
    +in: u8,
    +ch: u8,
    +ff: u8,
    +ge: u8,
    +ko: u8,
    +kk: u8,
    +attr_mut(&mut self, attr: &AttrType): &mut u8
}

class Identity {
    +name: String
    +spezies: Spezies
    +profession: Profession
}


class Spezies {
    <<enum>>
    Achaz
    Elfen
    Halbelfen
    Halborks
    Holberker
    Menschen
    Orks
    Zwerge
}

class SpeziesBase {
    <<trait>>
    name() &'static&#32str
    le() u8
    sk() i8 
    zk() i8 
    gs() u8 
    eigenschaften_mod(&self, eigenschaften: Attributes)
    vorteile_mod(&self, character: Character)
    cost() u8
}



class Profession {
    name: String
    ap_cost: u32
    preconditions: Vec~AttrAPCost~
    specials: Vec~AttrAPCost~
    fighting: Vec~Kampftechnik~
    talents: Vec~CharacterTalent~
    zaubertrick: Talent
}

class AttrAPCost {
    +ap_cost: u32
    +name: String
}

class Kampftechnik {
    name: String
    leiteigenschaft: AttrType
    steigerungs_faktor: SteigerungsFaktor
    stufe: u16
}

class AttrType {
    <<enum>>
    Any
    MU
    KL
    IN
    CH
    FF
    GE
    KO
    KK
    +to_string_long(&self): String
}

class SteigerungsFaktor {
    <<enum>>
    A
    B
    C
    D
    +cost(&self, rank: u16, profession: bool): u32
}

class CharacterTalent {
    +base: CharacterTalentBases
    +stufe: u16
}

class CharacterTalentBases {
    <<enum>>
    …
}

class CharacterTalentBase {
    +name: &'static str
    +steigerungs_faktor: SteigerungsFaktor
    +probe: [AttrType; 3]
    +probe(&self): &[AttrType; 3]
}

class Talent {
    +name: String
    +description: String
}

class ZauberTabelle {
    +enabled_spells: BTreeMap~usize#44;&#32ZauberDescriptor~
    +search: String
    +show_selector: bool
}

class ZauberDescriptor {
    +level: u8
    +enabeld_extensions: BTreeSet~usize~
}

class Erfahrungsgrade {
    <<enum>>
    UNERFAHREN
    DURCHSCHNITTLICH
    ERFAHREN
    KOMPETENT
    MEISTERLICH
    BRILLIANT
    LEGENDAER
    +erfahrungsgrad(&self): &'static Erfahrungsgrad
}

class Erfahrungsgrad {
    +name: &'static str
    +ap_konto: u16
    +eigenschaft_max: u8
    +fertigkeit_max: u8
    +Kampftechnik_max: u8
    +eigenschaftspunkte_max: u8
    +zauber_max: u8
    +fremdzauber: u8
}

class BuildUi {
    <<trait>>
    ui(&mut self, ui: &mut egui::Ui)
}

Character --* DSApp 
Character ..|> BuildUi

Identity --* Character
Attributes --* Character
ZauberTabelle --* Character
Erfahrungsgrade --* Character
Erfahrungsgrade ..|> BuildUi


Erfahrungsgrad --* Erfahrungsgrade

ZauberDescriptor --* ZauberTabelle

Spezies --* Identity
Spezies ..|> BuildUi

Achaz ..|> SpeziesBase
Elfen ..|> SpeziesBase
Halbelfen ..|> SpeziesBase
Halborks ..|> SpeziesBase
Holberker ..|> SpeziesBase
Menschen ..|> SpeziesBase
Orks ..|> SpeziesBase
Zwerge ..|> SpeziesBase

Spezies*--Achaz
Spezies*--Elfen
Spezies*--Halbelfen
Spezies*--Halborks
Spezies*--Holberker
Spezies*--Menschen
Spezies*--Orks
Spezies*--Zwerge

Profession ..* Identity
Profession ..|> BuildUi

AttrAPCost ..* Profession
Kampftechnik ..* Profession
Talent ..* Profession
CharacterTalent ..* Profession

AttrType ..* Attributes
AttrType ..* Kampftechnik
SteigerungsFaktor ..* Kampftechnik


CharacterTalentBases ..* CharacterTalent

CharacterTalentBase --* CharacterTalentBases

SteigerungsFaktor ..* CharacterTalentBase
AttrType ..* CharacterTalentBase
```

## Implementierung

(Fast) 100% Rust. App wird als WASM binary compiliert.

Interface wird mit `egui`, `eframe` (und derzeit WebGL über `wgpu`) gerendert.

Alle internen Ressourcen, wie Standartwerte, Regeln, etc. sind in Code umgesetzt.

Die Daten wurde aus [DSAwiki](https://ulisses-regelwiki.de/start.html) gewonnen
