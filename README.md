# Grammaire B2 — application interactive

Application React pour réviser la grammaire française au niveau B2 :
26 chapitres complets (subjonctif, conditionnel, hypothèse, voix passive, pronoms,
relatifs, logique de la phrase, déterminants, adverbes, prépositions…),
chacun avec une leçon illustrée et 8 exercices interactifs (fill-in, QCM,
transformation de phrase).

## Structure des fichiers

```
.
├── GrammaireB2.jsx    Composant React (présentation + logique)
├── chapters.js        Données : leçons et exercices
└── README.md          Ce fichier
```

La séparation est volontaire : tu peux éditer, traduire, ajouter ou retirer
des chapitres en modifiant uniquement `chapters.js` sans toucher au code.

## Dépendances

- `react` (>= 17)
- `lucide-react` (icônes)
- La police Fraunces est chargée dynamiquement depuis Google Fonts au montage.

Les classes Tailwind ne sont pas utilisées : tout le style est en JS inline.
Aucun framework CSS requis.

## Déploiement rapide avec Vite

```bash
npm create vite@latest grammaire-b2 -- --template react
cd grammaire-b2
npm install lucide-react
```

Puis copie `GrammaireB2.jsx` et `chapters.js` dans `src/`, et remplace
`src/App.jsx` par :

```jsx
import GrammaireB2 from "./GrammaireB2.jsx";
export default function App() {
  return <GrammaireB2 />;
}
```

Lance :

```bash
npm run dev
```

Pour la production :

```bash
npm run build
```

Le dossier `dist/` contient une appli statique déployable sur n'importe quel
hébergeur (Netlify, Vercel, GitHub Pages, S3, etc.).

## Format des données

Chaque chapitre dans `chapters.js` suit la structure :

```js
{
  id: "subjonctif-present",
  number: 1,
  section: "Temps et modes",
  title: "Le subjonctif",       // partie en encre noire
  titleEm: "présent",            // partie en italique rouge
  lesson: {
    intro: "...",
    formation: ["..."],
    irregulars: [{ v: "...", forms: "..." }],
    triggers: [
      { title: "...", subcases: [{ label, example, highlight }], note? }
    ],
  },
  items: [
    // Trois types d'exercices :

    // 1. Fill-in (à trou)
    {
      before: "Bien que ce projet ",
      hint: "être",
      after: " ambitieux, nous le mènerons à terme.",
      answers: ["soit"],
      rule: "...",
      consolidation: { /* même structure */ }
    },

    // 2. QCM
    {
      type: "qcm",
      prompt: "Quel pronom convient ?",
      context: "Je m'___ souviens parfaitement.",
      options: ["le", "en", "y"],
      correctIndex: 1,
      rule: "...",
      consolidation: { /* idem, avec type: "qcm" */ }
    },

    // 3. Transformation
    {
      type: "transform",
      instruction: "Réécrivez à la voix passive.",
      source: "Un journaliste a révélé l'affaire.",
      answers: ["L'affaire a été révélée par un journaliste"],
      rule: "...",
      consolidation: { /* idem, avec type: "transform" */ }
    },
  ],
}
```

### Vérification des réponses

Pour les `fill` et `transform`, la comparaison est insensible à la casse, à la
ponctuation finale et aux apostrophes typographiques. Si plusieurs réponses
sont acceptables (ex. avec/sans complément), ajoute-les toutes dans le tableau
`answers`.

Pour les `qcm`, `correctIndex` est l'index de la bonne réponse dans `options`
(0 pour la première).

### Le « cas voisin »

Chaque `item` doit avoir une propriété `consolidation` avec la même structure.
Quand l'utilisateur se trompe, l'app affiche la règle puis lui propose ce
deuxième item pour consolider la même notion.

## Ajouter un chapitre

1. Ouvre `chapters.js`.
2. Copie un chapitre existant.
3. Modifie `id`, `number`, `section`, `title`, `titleEm`, et le contenu.
4. Place l'objet à la bonne position dans le tableau `CHAPTERS` — l'ordre
   d'affichage suit l'ordre du tableau, regroupé par `section`.

Pas besoin de redémarrer le serveur avec Vite : le hot-reload prend en charge
la modification immédiatement.

## Charger les données depuis une API (optionnel)

Si tu préfères héberger les chapitres sur un serveur (CMS, API, JSON statique
sur un CDN), remplace l'import par un `fetch` :

```jsx
import { useEffect, useState } from "react";

export default function GrammaireB2() {
  const [chapters, setChapters] = useState(null);
  useEffect(() => {
    fetch("/api/chapters.json").then(r => r.json()).then(setChapters);
  }, []);
  if (!chapters) return <p>Chargement…</p>;
  // … passer chapters au lieu de CHAPTERS
}
```

Tu devras alors convertir `chapters.js` en `chapters.json` (export → simple
tableau JSON, retirer les commentaires).

## Licence

Code et contenu pédagogique : libres d'usage pour tes besoins personnels.
