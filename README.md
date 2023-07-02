# DSApp – DSA Character Sheet

Ein selbstrechnendes Character Sheet für DSA – Work in Progress.

Das Ziel dieses Projektes ist ein digitales Charakter sheet für DSA5 zu erstellen.

DSA5 bietet Möglichkeiten das Projekt stark zu erweitern, aber als erstes wollen wir die Charaktererstellung digitalisieren.

Beispiel DSA5 Chartaktersheet:


## To Do's:
### Erstellen eines Charakters
| Status | ToDo |
|---|---|
|| Persönlichkeitsdaten |
|| Erfahungsgrad / Abenteuerpunkte(AP) |
|| Spezies / Kultur |
|| Professionen |
|| Sprachen / Schriften |
|| Leiteigenschaften |
|| Vorteile / Nachteile |
|| Sonderfertigkeiten |
|| Grundwerte (LE, AsP, KP, SK, ZK, As) |
|| Talente |
|| Kampftechniken |
|| Ausrüstung / Geld |
|| Tragekraft |
|| Tiere / Begleiter|
|| Homebrew |
### Dynamisches Nutzen des Charakters
| Status | ToDo |
|---|---|
|| Statuseffekte |

## Implementierung

(Fast) 100% Rust. App wird als WASM binary compiliert.

Interface wird mit `egui`, `eframe` (und derzeit WebGL über `wgpu`) gerendert.

Alle internen Ressourcen, wie Standartwerte, Regeln, etc. sind in Code umgesetzt.

Die Daten wurde mit `x` aus [DSAwiki](https://ulisses-regelwiki.de/start.html) gewonnen
