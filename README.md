# Grammaire B2 — interactive app

A React app for reviewing French grammar at the B2 level:
26 complete chapters (subjunctive, conditional, hypothesis, passive voice, pronouns,
relatives, sentence logic, determiners, adverbs, prepositions…),
each with an illustrated lesson and 8 interactive exercises (fill-in, multiple
choice, sentence transformation).

## File structure

```
.
├── GrammaireB2.jsx    React component (presentation + logic)
├── chapters.js        Data: lessons and exercises
└── README.md          This file
```

The separation is intentional: you can edit, translate, add or remove
chapters by modifying only `chapters.js` without touching the code.

## Dependencies

- `react` (>= 17)
- `lucide-react` (icons)
- The Fraunces font is loaded dynamically from Google Fonts on mount.

Tailwind classes are not used: all styling is inline JS.
No CSS framework required.

## Quick deployment with Vite

```bash
npm create vite@latest grammaire-b2 -- --template react
cd grammaire-b2
npm install lucide-react
```

Then copy `GrammaireB2.jsx` and `chapters.js` into `src/`, and replace
`src/App.jsx` with:

```jsx
import GrammaireB2 from "./GrammaireB2.jsx";
export default function App() {
  return <GrammaireB2 />;
}
```

Run:

```bash
npm run dev
```

For production:

```bash
npm run build
```

The `dist/` folder contains a static app that can be deployed to any
host (Netlify, Vercel, GitHub Pages, S3, etc.).

## Data format

Each chapter in `chapters.js` follows this structure:

```js
{
  id: "subjonctif-present",
  number: 1,
  section: "Tenses and moods",
  title: "The subjunctive",      // black-ink portion
  titleEm: "present",             // red italic portion
  lesson: {
    intro: "...",
    formation: ["..."],
    irregulars: [{ v: "...", forms: "..." }],
    triggers: [
      { title: "...", subcases: [{ label, example, highlight }], note? }
    ],
  },
  items: [
    // Three types of exercises:

    // 1. Fill-in (blank)
    {
      before: "Bien que ce projet ",
      hint: "être",
      after: " ambitieux, nous le mènerons à terme.",
      answers: ["soit"],
      rule: "...",
      consolidation: { /* same structure */ }
    },

    // 2. Multiple choice
    {
      type: "qcm",
      prompt: "Which pronoun fits?",
      context: "Je m'___ souviens parfaitement.",
      options: ["le", "en", "y"],
      correctIndex: 1,
      rule: "...",
      consolidation: { /* same, with type: "qcm" */ }
    },

    // 3. Transformation
    {
      type: "transform",
      instruction: "Rewrite in the passive voice.",
      source: "Un journaliste a révélé l'affaire.",
      answers: ["L'affaire a été révélée par un journaliste"],
      rule: "...",
      consolidation: { /* same, with type: "transform" */ }
    },
  ],
}
```

### Answer checking

For `fill` and `transform`, comparison is case-insensitive and ignores
trailing punctuation and typographic apostrophes. If several answers are
acceptable (e.g. with/without a complement), add them all to the `answers`
array.

For `qcm`, `correctIndex` is the index of the correct answer in `options`
(0 for the first).

### The "neighboring case"

Each `item` must have a `consolidation` property with the same structure.
When the user makes a mistake, the app displays the rule and then offers
this second item to consolidate the same concept.

## Adding a chapter

1. Open `chapters.js`.
2. Copy an existing chapter.
3. Modify `id`, `number`, `section`, `title`, `titleEm`, and the content.
4. Place the object at the correct position in the `CHAPTERS` array — the
   display order follows the array order, grouped by `section`.

No need to restart the server with Vite: hot-reload picks up the change
immediately.

## Loading data from an API (optional)

If you prefer to host the chapters on a server (CMS, API, static JSON on
a CDN), replace the import with a `fetch`:

```jsx
import { useEffect, useState } from "react";

export default function GrammaireB2() {
  const [chapters, setChapters] = useState(null);
  useEffect(() => {
    fetch("/api/chapters.json").then(r => r.json()).then(setChapters);
  }, []);
  if (!chapters) return <p>Loading…</p>;
  // … pass chapters instead of CHAPTERS
}
```

You'll then need to convert `chapters.js` into `chapters.json` (export →
plain JSON array, remove the comments).

## License

Code and educational content: free to use for personal needs.
