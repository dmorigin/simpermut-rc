# SimulationCraft Permut (Rust)

Beta: 0.1.4

## Vorwort

Dieses Programm ist entstanden um zwei Aufgaben zu erfüllen. Zum einen um die Sprache Rust
zu lernen. Daher nicht zu kritisch auf den Code schauen. Und zum anderen, hatte ich
eh vor mir ein kleines Tool zu bauen welches mir die beste Kombination aus allen
Gegenständen ermittelt. Es wurde daher beides zusammen gelegt.

## Installation

Zur Benutzung ist Rust nötig: https://www.rust-lang.org/en-US/
Als nächstes wird das Programm SimulationCraft benötigt: https://simulationcraft.org/

> Für SimulationCraft wird weder eine Installationsroutine benötigt noch
> administrative Rechte. Das gleiche gilt für Rust.

**Optional:** Das WoW Addon: SimulationCraft: https://www.curseforge.com/wow/addons/simulationcraft

Für die Installation dieses Programms folgende Schritte ausführen
```
git clone https://github.com/dmorigin/simpermut-rc.git target_directory
cd target_directory
cargo build
```

> Alle nötigen Abhängigkeiten werden automatisch durch das Tool cargo geladen.

## Konfiguration

Damit das Programm seinen Job verrichten kann, muss es erst konfiguriert werden. Eine
Beispielkonfiguration findet ihr in der Datei config.json. Wichtig sind hier die
Einstellungen bzgl. der Verzeichnisse. Hier nur die wichtigsten. Der Rest kann so bleiben
wie vorgegeben.

- output_dir: Gibt das Verzeichnis an in dem alle *generierten* Ausgaben abgelegt werden.
- simcraft.template: Der Name der Vorlage, welche zur Generierung der simc Datei für simc.exe genutzt werden soll.
- simcraft.executeable: Absolute Pfadangabe wo sich die simc.exe befindet.

Das Programm sucht standartmäßig nach der config.json. Solltet ihr eine andere Datei bevorzugen
könnt ihr dies durch den Parameter
```
--config myconfig.json
```
machen.

## Aufruf

Der erste Schritt ist, ihr braucht eine simc Datei. Diese könnt ihr entweder selbst schreiben
oder euch erstellen lassen. Die einfachste Methode ist ihr nutzt das Addon Simulationcraft. Logged
euch mit eurem Charakter ein und führt /simc aus. Dann kopiert ihr den kompletten Inhalt des sich
öffnenden Fenster und fügt diesen in eine Text Datei ein. Benennt diese z.B.: input.simc

Wenn ihr eure Konfigurationsdatei angefertigt habt und diese z.B. *config.json* heißt, könnt ihr
das Programm wie folgt aufrufen:

```
cargo run -- input.simc
```

Und schon geht es los.

**Hinweis:**
Das Programm sucht nach allen Gegenständen in input.simc. Dabei werden alle Kommentarzeichen "#" 
ignoriert. Wenn ihr Gegenstände nicht prüfen wollt, dann löscht sie aus der Datei.

**Wichtig:**
Es wird eine Permutation gestartet. D.h. es werden alle möglichen Kombinationen durchgerechnet.
Je mehr Gegenstände ihr in der Datei input.simc habt, desto länger dauert es. Es können dabei
mehrere tausend Iterationen ausgeführt. Wobei jede Iteration, je nach Hardware und Einstellung, 
15s bis 30s dauern kann.
Es sollte daher darauf geachtet werden das ihr auf keinen Fall simc.exe dazu veranlasst die 
Skalierungsfaktoren aus zu rechnen. Setzt also unbedingt: calculate_scale_factors=0

## Pre Compiled

Windows x64 Version 0.1.4
https://www.gamers-shell.de/wp-content/uploads/2018/07/sim_permut_0.1.4_winx64.zip