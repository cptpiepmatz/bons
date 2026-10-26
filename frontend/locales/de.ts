import type en from "./en.ts";

export default {
  meta: {
    description:
      "Behalte eure gemeinsamen Ausgaben mit bons im Blick. Sieh eure monatlichen Ausgaben auf einen Blick und ordne alltägliche Einkäufe nach Kategorien.",
  },
  common: {
    tagline: "Eure Ausgaben. Alles im Blick.",
    cancel: "Abbrechen",
    close: "Schließen",
    delete: "Löschen",
    retry: "Erneut versuchen",
    amount: "Betrag",
    description: "Beschreibung",
    category: "Kategorie",
    date: "Datum",
    enteredBy: "Eingetragen von",
    user: "Benutzer {id}",
  },
  auth: {
    welcome: "Willkommen bei bons",
    instructions: "Füg deinen Zugangsschlüssel ein, dann geht’s los.",
    accessKey: "Zugangsschlüssel",
    placeholder: "Hier einfügen",
    start: "Los geht's",
    noKey:
      "Noch keinen Schlüssel? Frag bei der Person nach, die bons eingerichtet hat.",
    userMenu: "Benutzermenü für {name}",
    signOut: "Abmelden",
    sessionError:
      "Die Sitzung konnte nicht geprüft werden. Bitte prüf, ob das Backend läuft und aktuell ist, und versuch es erneut.",
    invalidKey: "Der Zugangsschlüssel ist ungültig.",
    loginError:
      "Die Anmeldung ist gerade nicht möglich. Bitte versuch es erneut.",
    logoutError:
      "Die Abmeldung ist gerade nicht möglich. Bitte versuch es erneut.",
  },
  spending: {
    title: "Unsere Ausgaben",
    new: "Neue Ausgabe",
    question: "Was habt ihr ausgegeben?",
    add: "Ausgabe hinzufügen",
    descriptionPlaceholder: "z.B. Wochenmarkt",
    enteredByYou: "Eingetragen von dir",
    saveError: "Die Ausgabe konnte nicht gespeichert werden.",
    previousMonth: "Vorheriger Monat",
    nextMonth: "Nächster Monat",
    total: "Insgesamt ausgegeben",
    loading: "Ausgaben werden geladen …",
    loadError: "Ausgaben konnten nicht geladen werden.",
    count: "{count} Ausgabe | {count} Ausgaben",
    empty: "Keine Ausgaben",
    byCategory: "Nach Kategorien",
    categoriesLoading: "Kategorien werden geladen …",
    categoriesUnavailable: "Keine Kategorieübersicht verfügbar.",
    monthEmpty: "Noch keine Ausgaben in diesem Monat.",
    all: "Alle Ausgaben",
    deleteLabel: "{description} löschen",
    deleteTitle: "Ausgabe löschen",
    deleteQuestion: "Möchtest du diese Ausgabe wirklich löschen?",
    deleteError: "Die Ausgabe konnte nicht gelöscht werden.",
  },
} satisfies typeof en;
