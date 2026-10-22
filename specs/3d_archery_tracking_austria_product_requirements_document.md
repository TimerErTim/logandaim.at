# Product Requirements Document (PRD) & Konzeptbericht
## 3D-Bogensport Accuracy & Progress Tracking WebApp (Österreich)

---

## 1. Executive Summary & Vision
Ziel des Projekts ist die Konzeption und Entwicklung einer spezialisierten Web-Applikation (optimiert für Mobile/PWA) zur Erfassung, Analyse und Verfolgung des persönlichen Fortschritts im 3D-Bogensport in Österreich.

Bestehende Lösungen beschränken sich häufig auf reine Scorekarten oder Turniersoftware nach WA/IFAA-Regeln, ohne differenzierte Treffergenauigkeit über Zeit (z. B. auf Parcours-/Zielebene) oder stufenlose Distanzzuordnungen abzubilden. Die App schließt diese Lücke, indem sie Betreibern von Bogenparcours eine standardisierte Plattform bietet und Schützinnen und Schützen ein datengestütztes Trainingstool an die Hand gibt.

---

## 2. Stakeholder & Rollenmodell

| Rolle | Beschreibung & Ziel |
| :--- | :--- |
| **Gast / Casual Schütze** | Gelegentliches Schießen, einfache Scorekartenerfassung, Verzicht auf komplizierte Dateneingaben, Fokus auf Spaß und groben Überblick. |
| **Power-User / Sportschütze** | Ambitioniertes Training, Turniervorbereitung, detaillierte Treffer- & Fehleranalysen (Miss-Tendenzen, Distanz-Korrelation), historische Progression. |
| **Anlagenbetreiber (Parcours-Betreiber)** | Verwaltung von Parcours, Runden/Layouts, Zielkatalog und Pflockfarben; Einsicht in anonymisierte Nutzungsdaten, Feedback zu Zielen, Platzierung von Angeboten. |
| **Turnierleiter / Guide (Zukunft)** | Erstellung von Event-Runden, Gruppenwertungen und Live-Leaderboards für lokale Bewerbe. |
| **System-Administrator** | Stammdatenverwaltung, Moderation gemeldeter Anlagen, Rollenrechte, Systemgesundheit. |

---

## 3. Kernanforderungen (Funktionale Spezifikation)

### 3.1 Domain-Modell & Begrifflichkeiten (Domain-Driven Design)
* **Anlage / Parcours:** Physischer Standort eines Bogenvereins oder gewerblichen Anbieters.
* **Route / Parcours-Layout:** Vordefinierte Abfolge von Zielen (z. B. „Große Runde 32 Ziele“, „Winterrunde 18 Ziele“).
* **Ziel (3D-Target):** Einzelnes 3D-Tier mit hinterlegten Stammdaten (Tierart, Tiergruppe 1–4, Pflock-Konfiguration).
* **Pflock / Distanzmarkierung:** Farbige Pfosten im Gelände (Standard z. B. Weiß, Blau, Rot, Gelb), die Abschussdistanzen markieren.
* **Session (Runde):** Vom Schützen aktiv geloggter Durchlauf einer bestimmten Route an einem bestimmten Datum.
* **Schuss / Wertung:** Registrierter Treffer pro Ziel inkl. Pfeilnummer, Wertungszone und relativem Abschusspunkt.

---

### 3.2 Wertungssystem & Pfeillogik
Das System unterstützt standardmäßig die klassische 3-Pfeil-Wertung mit abnehmendem Punktwert:

| Pfeil | Spot (Inner Kill) | Kill | Body (Körper) | Miss (Vorbei) |
| :---: | :---: | :---: | :---: | :---: |
| **1. Pfeil** | 20 Punkte | 18 Punkte | 16 Punkte | 0 Punkte |
| **2. Pfeil** | 14 Punkte | 12 Punkte | 10 Punkte | 0 Punkte |
| **3. Pfeil** | 8 Punkte | 6 Punkte | 4 Punkte | 0 Punkte |

*Regelwerk-Constraint:* Trifft Pfeil 1 in eine Wertungszone (Spot, Kill, Body), wird das Ziel abgeschlossen und kein weiterer Pfeil für dieses Ziel gewertet. Bei „Miss“ darf Pfeil 2 gewertet werden; bei erneutem Miss Pfeil 3.

---

### 3.3 Distanzerfassung mittels stufenlosem Farbslider
* **Stammdaten-Definition:** Pro Ziel werden die vorhandenen Pflöcke in logischer Reihenfolge definiert (z. B. Rot = Weit, Blau = Mittel, Gelb = Nah).
* **Dynamischer UI-Slider:**
  * Der Slider spannt sich zwischen den konfigurierten Pflockfarben des jeweiligen Ziels auf.
  * Diskrete Rastpunkte markieren die exakten Pfosten.
  * Stufenlose Zwischenschritte ermöglichen exakte Eingaben wie *„Exakt zwischen Blau und Rot“* oder *„2 Schritte vor Gelb“*.
  * Optional: Direkte Eingabe metrischer Schätzwerte (z. B. 28 m), falls der Schütze mit Entfernungsmesser arbeitet.

---

### 3.4 Progressions- und Benchmark-Berechnungen
1. **Persönlicher Fortschritt (Intra-User):**
   * Vergleich desselben Nutzers auf derselben Route über Zeit (Score-Entwicklung, First-Arrow-Hit-Rate).
   * Visualisierung von Ziel-Schwächen (z. B. wiederholte Fehlschüsse bei Zielgruppe 4 auf weite Distanz).
2. **Globaler Benchmark (Inter-User):**
   * Berechnung der durchschnittlichen Punktzahl pro Route über alle registrierten Schützen.
   * Durchschnitts-Trefferquote pro individuellem 3D-Tier (Identifikation von „schweren“ oder unglücklich gestellten Tieren).

---

## 4. Erweiterungsvorschläge (Feature Roadmap nach Rollen)

### 4.1 Für Casual User
* **Offline-First Funktionalität:** Volle Erfassung ohne Internetempfang im Wald (PWA mit lokalem Cache / IndexedDB) und automatischer Background-Sync bei Netzempfang.
* **Schnelleingabe-Modus (One-Thumb-UI):** Große Touch-Targets für schnelle Erfassung bei Kälte oder mit Handschuhen/Tab.
* **Foto-Log:** Schnappschuss des Ziels oder der Gruppe direkt an die Session anhängen.
* **Bogenklassen-Auswahl:** Schnelles Umschalten zwischen Bogenarten (z. B. Blankbogen, Trad. Recurve, Longbow, Compound).

### 4.2 Für Power-User & Turnierschützen
* **Trefferlagen-Tendenz (Sight Mark & Arrow Drift):** Erfassung von Hoch-/Tief-/Rechts-/Links-Tendenzen für präzises Pfeiltuning.
* **Erweiterte Wertungsmodi:** Umschaltbar auf WA 2-Pfeil (11/10/8/5) oder IFAA 1-Pfeil Hunter/3D.
* **Equipment-Tracking:** Verknüpfung der Session mit spezifischem Setup (Bogen, Pfeilgewicht, Zuggewicht, Sehnenmaterial).
* **Wetter- & Geländedaten:** Automatisches Auslesen von Temperatur, Wind und Hangneigung via Smartphone-Sensoren.

### 4.3 Für Anlagenbetreiber (B2B & Vereinsnutzen)
* **Zustands- & Verschleiß-Monitoring:** Schützen können beschädigte Tiere (z. B. „Kill zerschossen“, „Pfeilstopp lose“) direkt per Klick melden.
* **Wegweiser- & Pfeilverlust-Meldungen:** „Fundgrube“-Funktion für gefundene/verlorene Pfeile mit Farb- und Schaftangabe.
* **Parcours-Traffic-Heatmap:** Anonymisierte Auswertung von Durchlaufzeiten zur Optimierung von Stauzonen im Wald.
* **Veranstaltungs- und Wartungsankündigungen:** Schließungen wegen Holzarbeiten oder Turnieren direkt an Follower der Anlage ausspielen.

---

## 5. Technische Architektur & Systemanforderungen (Übersicht)

```
[ Mobile Browser / PWA (Offline Storage / IndexedDB) ]
                         │
                   HTTPS / REST / tRPC
                         ▼
[ API Gateway & Backend Services (Node.js/Go/Python) ]
     │                   │                    │
     ▼                   ▼                    ▼
[ Auth & User DB ]   [ Parcours & Stamm ]  [ Session & Scoring Event Stream ]
```

* **Frontend:** PWA (Progressive Web App) für plattformunabhängige Installation auf iOS/Android ohne App-Store-Zwang.
* **Speicherstrategie:** Vollständiges Offline-Caching aller aktiven Parcours-Layouts und lokales Speichern der Session bis zum Upload.
* **Geodaten / GPS:** Optionale Standorterkennung zur automatischen Parcours- und Zielidentifikation entlang des Rundwegs.

---

## 6. Nächste Schritte (Action Items)
1. **Domain-Model festigen:** Finale Definition der Datenbankentitäten (`Facility`, `Course`, `Target`, `Stake`, `Session`, `Shot`).
2. **UI/UX-Drahtgittermodell (Wireframe):** Fokus auf die Ziel-Erfassungsmaske mit dem stufenlosen Distanz-Slider.
3. **Pilotparcours in Österreich:** Identifikation von 2–3 Referenzanlagen (z. B. in Oberösterreich/Steiermark) zur Einbindung von Echtdaten im MVP.
