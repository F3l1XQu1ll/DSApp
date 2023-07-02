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


## Implementierung

(Fast) 100% Rust. App wird als WASM binary compiliert.

Interface wird mit `egui`, `eframe` (und derzeit WebGL über `wgpu`) gerendert.

Alle internen Ressourcen, wie Standartwerte, Regeln, etc. sind in Code umgesetzt.

Die Daten wurde aus [DSAwiki](https://ulisses-regelwiki.de/start.html) gewonnen
