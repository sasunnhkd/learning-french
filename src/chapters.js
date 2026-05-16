// All B2 grammar chapter content (lessons + exercises).
// Original content authored for this application.
// Edit, translate, or extend freely — the component reads from this array.

export const CHAPTERS = [
  /* ---------- 1. SUBJONCTIF PRÉSENT ---------- */
  {
    id: "subjonctif-present",
    number: 1,
    section: "Temps et modes",
    title: "Le subjonctif",
    titleEm: "présent",
    lesson: {
      intro:
        "Le subjonctif est le mode du subjectif, du non-réalisé, du jugement. Il n'apparaît presque jamais seul : il est déclenché par un mot ou une tournure de la proposition principale. Au B2, l'enjeu n'est plus de le former mais de savoir le repérer.",
      formation: [
        "Pour la plupart des verbes, on part de la 3ᵉ personne du pluriel du présent de l'indicatif, on retire la terminaison -ent et on ajoute -e, -es, -e, -ions, -iez, -ent.",
        "Exemple : ils finissent → que je finisse, que tu finisses, qu'il finisse, que nous finissions, que vous finissiez, qu'ils finissent.",
        "À noter : aux 1ʳᵉ et 2ᵉ personnes du pluriel, les terminaisons (-ions, -iez) sont identiques à celles de l'imparfait. C'est une source de confusion classique.",
      ],
      irregulars: [
        { v: "être", forms: "sois · soit · soyons · soyez · soient" },
        { v: "avoir", forms: "aie · ait · ayons · ayez · aient" },
        { v: "aller", forms: "aille · ailles · allions · alliez · aillent" },
        { v: "faire", forms: "fasse · fasses · fassions · fassiez · fassent" },
        { v: "savoir", forms: "sache · saches · sachions · sachiez · sachent" },
        { v: "pouvoir", forms: "puisse · puisses · puissions · puissiez · puissent" },
        { v: "vouloir", forms: "veuille · veuilles · voulions · vouliez · veuillent" },
        { v: "valoir", forms: "vaille · vailles · valions · valiez · vaillent" },
        { v: "falloir", forms: "qu'il faille" },
        { v: "pleuvoir", forms: "qu'il pleuve" },
      ],
      triggers: [
        {
          title: "Conjonctions",
          subcases: [
            {
              label: "Concession (bien que, quoique)",
              example: "Bien que la route soit longue, nous arriverons à temps.",
              highlight: "soit",
            },
            {
              label: "But (pour que, afin que)",
              example: "Téléphone-moi pour que je sache où tu en es.",
              highlight: "sache",
            },
            {
              label: "Antériorité ou restriction, souvent avec « ne » explétif (avant que, à moins que, jusqu'à ce que)",
              example: "Partons maintenant, à moins que tu ne veuilles rester.",
              highlight: "veuilles",
            },
            {
              label: "Condition (pourvu que, à condition que, sans que)",
              example: "Tu réussiras pourvu que tu aies confiance en toi.",
              highlight: "aies",
            },
          ],
        },
        {
          title: "Verbes et tournures",
          subcases: [
            {
              label: "Volonté (vouloir, exiger, proposer, demander que…)",
              example: "Le directeur exige que vous soyez ponctuels.",
              highlight: "soyez",
            },
            {
              label: "Sentiment (être ravi/triste/étonné que, regretter, craindre…)",
              example: "Je regrette qu'elle ne puisse pas venir avec nous.",
              highlight: "puisse",
            },
            {
              label: "Doute (douter que, ne pas croire/penser que, il est peu probable…)",
              example: "Je doute qu'il dise toute la vérité.",
              highlight: "dise",
            },
            {
              label: "Tournures impersonnelles (il faut que, il est essentiel/nécessaire/temps que…)",
              example: "Il faut que tout soit prêt pour demain matin.",
              highlight: "soit",
            },
          ],
        },
        {
          title: "Cas particuliers",
          subcases: [
            {
              label: "Après un superlatif relatif (le meilleur, le seul, l'unique… que)",
              example: "C'est le meilleur conseil qu'on puisse te donner.",
              highlight: "puisse",
            },
            {
              label: "Avec un antécédent indéfini ou recherché",
              example: "Je cherche un appartement qui ait une vue sur le parc.",
              highlight: "ait",
            },
            {
              label: "Après une négation absolue (il n'y a personne, rien… qui)",
              example: "Il n'y a personne ici qui sache réparer cela.",
              highlight: "sache",
            },
            {
              label: "En tête de phrase pour un ordre ou un souhait",
              example: "Qu'elle vienne ou non, partons à l'heure.",
              highlight: "vienne",
            },
          ],
        },
        {
          title: "Pièges fréquents",
          body:
            "Espérer que prend l'indicatif, pas le subjonctif. Croire et penser à la forme affirmative prennent l'indicatif, mais basculent au subjonctif à la forme interrogative ou négative. Après il est probable que : indicatif. Après il est peu probable que : subjonctif.",
        },
      ],
    },
    items: [
      {
        before: "Bien que ce projet ",
        hint: "être",
        after: " ambitieux, nous le mènerons à terme.",
        answers: ["soit"],
        rule:
          "« Bien que » exige toujours le subjonctif, même quand le fait est avéré. C'est une conjonction de concession.",
        consolidation: {
          before: "Quoique nous ",
          hint: "avoir",
          after: " peu de temps, finissons proprement.",
          answers: ["ayons"],
        },
      },
      {
        before: "Je doute fort qu'il ",
        hint: "savoir",
        after: " la vérité sur cette affaire.",
        answers: ["sache"],
        rule:
          "Les verbes de doute (douter que, ne pas croire que, ne pas penser que…) appellent le subjonctif. « Savoir » a un radical irrégulier : sach-.",
        consolidation: {
          before: "Elle ne pense pas que tu ",
          hint: "savoir",
          after: " ce qui s'est vraiment passé.",
          answers: ["saches"],
        },
      },
      {
        before: "Il est essentiel que vous ",
        hint: "faire",
        after: " preuve de patience dans cette négociation.",
        answers: ["fassiez"],
        rule:
          "Les tournures impersonnelles d'obligation ou de nécessité (il faut que, il est essentiel que, il importe que…) imposent le subjonctif. « Faire » → radical fass-.",
        consolidation: {
          before: "Il est indispensable que nous ",
          hint: "faire",
          after: " appel à un expert externe.",
          answers: ["fassions"],
        },
      },
      {
        before: "Nous partirons avant qu'il ne ",
        hint: "pleuvoir",
        after: " à verse.",
        answers: ["pleuve"],
        rule:
          "« Avant que » déclenche le subjonctif et autorise un « ne » explétif (sans valeur négative). Pleuvoir → pleuve.",
        consolidation: {
          before: "Préviens-moi avant que le train ne ",
          hint: "partir",
          after: " sans toi.",
          answers: ["parte"],
        },
      },
      {
        before: "C'est le meilleur exposé que je ",
        hint: "connaître",
        after: " sur le sujet.",
        answers: ["connaisse"],
        rule:
          "Après un superlatif relatif (le meilleur, le seul, le premier, l'unique…), la subordonnée relative se met au subjonctif.",
        consolidation: {
          before: "C'est la seule personne qui ",
          hint: "comprendre",
          after: " vraiment mon point de vue.",
          answers: ["comprenne"],
        },
      },
      {
        before: "Je cherche un assistant qui ",
        hint: "parler",
        after: " couramment trois langues.",
        answers: ["parle"],
        rule:
          "Quand l'antécédent d'une proposition relative est indéfini, recherché ou hypothétique (un, quelqu'un…), la relative passe au subjonctif.",
        consolidation: {
          before: "Connais-tu quelqu'un qui ",
          hint: "pouvoir",
          after: " traduire ce document rapidement ?",
          answers: ["puisse"],
        },
      },
      {
        before: "Il faut absolument que tu ",
        hint: "aller",
        after: " consulter un spécialiste.",
        answers: ["ailles"],
        rule:
          "« Il faut que » exige le subjonctif. Aller a un radical irrégulier : aill- (sauf nous/vous, qui prennent allions/alliez).",
        consolidation: {
          before: "Il faut qu'elle ",
          hint: "aller",
          after: " chercher les enfants à dix-sept heures.",
          answers: ["aille"],
        },
      },
      {
        before: "Je suis ravi que vous ",
        hint: "prendre",
        after: " part à ce projet.",
        answers: ["preniez"],
        rule:
          "Les verbes et adjectifs de sentiment (être ravi, regretter, craindre, s'étonner, avoir peur…) sont suivis du subjonctif.",
        consolidation: {
          before: "Nous regrettons que tu ",
          hint: "prendre",
          after: " cette décision si vite.",
          answers: ["prennes"],
        },
      },
    ],
  },

  /* ---------- 2. SUBJONCTIF PASSÉ ---------- */
  {
    id: "subjonctif-passe",
    number: 2,
    section: "Temps et modes",
    title: "Le subjonctif",
    titleEm: "passé",
    lesson: {
      intro:
        "Le subjonctif passé exprime une action antérieure à celle de la principale, ou simplement achevée. Les déclencheurs sont les mêmes que pour le subjonctif présent : seul change le rapport temporel.",
      formation: [
        "Forme composée : avoir ou être au subjonctif présent + participe passé.",
        "Avec avoir : que j'aie parlé, que tu aies parlé, qu'il ait parlé, que nous ayons parlé, que vous ayez parlé, qu'ils aient parlé.",
        "Avec être : que je sois parti(e), que tu sois parti(e), qu'il soit parti, qu'elle soit partie, que nous soyons parti(e)s, que vous soyez parti(e)(s), qu'ils soient partis, qu'elles soient parties.",
        "Les règles d'accord du participe passé restent les mêmes qu'au passé composé.",
      ],
      irregulars: [],
      triggers: [
        {
          title: "Choisir entre présent et passé du subjonctif",
          subcases: [
            {
              label: "Action simultanée ou postérieure à la principale → subjonctif présent",
              example: "Je suis content qu'elle vienne ce soir.",
              highlight: "vienne",
            },
            {
              label: "Action antérieure ou achevée par rapport à la principale → subjonctif passé",
              example: "Je suis content qu'elle soit venue hier.",
              highlight: "soit venue",
            },
            {
              label: "Achèvement avant une échéance (souvent avec une date ou « avant »)",
              example: "Il faut que tu aies fini avant midi.",
              highlight: "aies fini",
            },
          ],
        },
        {
          title: "Les mêmes déclencheurs qu'au subjonctif présent",
          subcases: [
            {
              label: "Concession portant sur le passé",
              example: "Bien qu'il ait beaucoup travaillé, il a échoué.",
              highlight: "ait beaucoup travaillé",
            },
            {
              label: "Sentiment portant sur le passé",
              example: "Je regrette qu'elle soit partie sans dire au revoir.",
              highlight: "soit partie",
            },
            {
              label: "Doute portant sur le passé",
              example: "Je ne crois pas qu'il ait compris la consigne.",
              highlight: "ait compris",
            },
            {
              label: "Superlatif ou expérience passée",
              example: "C'est le plus beau film que j'aie vu cette année.",
              highlight: "aie vu",
            },
          ],
        },
        {
          title: "Pièges fréquents",
          body:
            "Avec « avant que » et « à moins que », on garde généralement le subjonctif présent même pour évoquer un événement futur. L'accord du participe passé suit les règles habituelles : avec être, accord avec le sujet ; avec avoir, accord avec le COD seulement s'il précède le verbe.",
        },
      ],
    },
    items: [
      {
        before: "Bien qu'il ",
        hint: "travailler",
        after: " toute la nuit, le rapport n'est pas encore terminé.",
        answers: ["ait travaillé"],
        rule:
          "Concession portant sur une action passée et achevée → subjonctif passé. Forme : avoir au subj. présent + participe passé.",
        consolidation: {
          before: "Quoiqu'elle ",
          hint: "faire",
          after: " de son mieux, le résultat reste décevant.",
          answers: ["ait fait"],
        },
      },
      {
        before: "Je regrette qu'il ",
        hint: "partir",
        after: " si tôt hier soir.",
        answers: ["soit parti"],
        rule:
          "Sentiment (regretter) + action passée et terminée → subjonctif passé. Verbe « partir » → auxiliaire être, donc accord avec le sujet.",
        consolidation: {
          before: "Nous sommes désolés qu'elle ",
          hint: "partir",
          after: " sans nous prévenir.",
          answers: ["soit partie"],
        },
      },
      {
        before: "Je doute fort qu'elle ",
        hint: "lire",
        after: " l'ensemble du contrat avant de signer.",
        answers: ["ait lu"],
        rule:
          "Doute + action antérieure → subjonctif passé. Le doute porte sur ce qui s'est (ou ne s'est pas) déjà passé.",
        consolidation: {
          before: "Je ne pense pas qu'ils ",
          hint: "comprendre",
          after: " la portée de la décision.",
          answers: ["aient compris"],
        },
      },
      {
        before: "C'est le voyage le plus mémorable que nous ",
        hint: "faire",
        after: " ensemble.",
        answers: ["ayons fait"],
        rule:
          "Superlatif relatif + bilan rétrospectif d'une expérience passée → subjonctif passé.",
        consolidation: {
          before: "C'est la chose la plus surprenante qu'il ",
          hint: "dire",
          after: " durant toute la soirée.",
          answers: ["ait dite", "ait dit"],
        },
      },
      {
        before: "Il faut absolument que tu ",
        hint: "finir",
        after: " ton dossier avant midi.",
        answers: ["aies fini"],
        rule:
          "Tournure impersonnelle + échéance → subjonctif passé pour marquer l'achèvement attendu à une date donnée.",
        consolidation: {
          before: "Il est essentiel que vous ",
          hint: "relire",
          after: " votre copie avant de la rendre.",
          answers: ["ayez relu"],
        },
      },
      {
        before: "Je cherche un témoin qui ",
        hint: "voir",
        after: " l'accident de ses propres yeux.",
        answers: ["ait vu"],
        rule:
          "Antécédent recherché ou indéfini + action passée → subjonctif passé.",
        consolidation: {
          before: "Connaissez-vous quelqu'un qui ",
          hint: "déjà essayer",
          after: " ce restaurant ?",
          answers: ["ait déjà essayé"],
        },
      },
      {
        before: "Il n'y a personne ici qui ",
        hint: "entendre",
        after: " un bruit pareil auparavant.",
        answers: ["ait entendu"],
        rule:
          "Négation absolue (il n'y a personne, rien… qui) + action située dans le passé → subjonctif passé.",
        consolidation: {
          before: "Il n'y a rien qu'elle ",
          hint: "ne pas tenter",
          after: " pour le convaincre.",
          answers: ["n'ait pas tenté"],
        },
      },
      {
        before: "Pourvu qu'il ",
        hint: "recevoir",
        after: " notre message à temps !",
        answers: ["ait reçu"],
        rule:
          "« Pourvu que » introduit ici un souhait portant sur le passé : on espère qu'une chose s'est déjà produite → subjonctif passé.",
        consolidation: {
          before: "Pourvu qu'elle ",
          hint: "ne pas oublier",
          after: " son passeport !",
          answers: ["n'ait pas oublié"],
        },
      },
    ],
  },

  /* ---------- 3. CONCORDANCE DES TEMPS ---------- */
  {
    id: "concordance-temps",
    number: 3,
    section: "Temps et modes",
    title: "La concordance des",
    titleEm: "temps",
    lesson: {
      intro:
        "La concordance des temps règle le choix du temps dans la subordonnée selon deux paramètres : le temps de la principale et la relation temporelle (antériorité, simultanéité, postériorité). Au B2, c'est ce qui distingue un récit cohérent d'un texte qui sonne faux.",
      formation: [
        "Quand la principale est au présent ou au futur : antériorité → passé composé ou plus-que-parfait ; simultanéité → présent ; postériorité → futur.",
        "Quand la principale est au passé : antériorité → plus-que-parfait ; simultanéité → imparfait ; postériorité → conditionnel présent (= futur dans le passé).",
        "Au subjonctif (en français contemporain courant) : action simultanée ou postérieure → subjonctif présent ; action antérieure → subjonctif passé.",
        "Avec « si » + hypothèse : si + imparfait, conditionnel présent ; si + plus-que-parfait, conditionnel passé. Jamais de conditionnel ni de futur après si hypothétique.",
      ],
      irregulars: [],
      triggers: [
        {
          title: "Principale au passé",
          subcases: [
            {
              label: "Action antérieure → plus-que-parfait",
              example: "Elle a expliqué qu'elle avait perdu son badge la veille.",
              highlight: "avait perdu",
            },
            {
              label: "Action simultanée → imparfait",
              example: "Il m'a dit qu'il était très fatigué.",
              highlight: "était",
            },
            {
              label: "Action postérieure → conditionnel présent",
              example: "Le ministre a annoncé qu'il présenterait son plan la semaine suivante.",
              highlight: "présenterait",
            },
          ],
        },
        {
          title: "Hypothèses avec « si »",
          subcases: [
            {
              label: "Hypothèse irréelle dans le présent → si + imparfait, conditionnel présent",
              example: "Si j'avais plus de temps, je lirais davantage.",
              highlight: "lirais",
            },
            {
              label: "Hypothèse irréelle dans le passé → si + plus-que-parfait, conditionnel passé",
              example: "Si tu m'avais prévenu, j'aurais annulé mon rendez-vous.",
              highlight: "aurais annulé",
            },
          ],
        },
        {
          title: "Pièges fréquents",
          body:
            "Jamais de futur ni de conditionnel dans la subordonnée en « si » hypothétique : on dit « si j'avais », jamais « si j'aurais ». Au discours rapporté à un temps passé, le présent devient imparfait, le passé composé devient plus-que-parfait, le futur devient conditionnel présent.",
        },
      ],
    },
    items: [
      {
        before: "Hier, il m'a expliqué qu'il ne ",
        hint: "pouvoir",
        after: " pas venir aujourd'hui.",
        answers: ["pouvait"],
        rule:
          "Principale au passé (a expliqué) + action simultanée à ce moment passé → imparfait dans la subordonnée.",
        consolidation: {
          before: "Elle a affirmé qu'elle ",
          hint: "être",
          after: " parfaitement d'accord avec nous.",
          answers: ["était"],
        },
      },
      {
        before: "Elle a juré qu'elle ",
        hint: "déjà envoyer",
        after: " le document la veille.",
        answers: ["avait déjà envoyé"],
        rule:
          "Principale au passé + action antérieure (la veille) → plus-que-parfait.",
        consolidation: {
          before: "Il a reconnu qu'il ",
          hint: "se tromper",
          after: " la fois précédente.",
          answers: ["s'était trompé"],
        },
      },
      {
        before: "Le ministre a promis qu'il ",
        hint: "présenter",
        after: " son plan la semaine suivante.",
        answers: ["présenterait"],
        rule:
          "Principale au passé + action postérieure (futur dans le passé) → conditionnel présent.",
        consolidation: {
          before: "Nous lui avons dit que nous ",
          hint: "revenir",
          after: " le lendemain matin.",
          answers: ["reviendrions"],
        },
      },
      {
        before: "Je suis ravi que tu ",
        hint: "réussir",
        after: " ton examen la semaine dernière.",
        answers: ["aies réussi"],
        rule:
          "Sentiment au présent + action antérieure → subjonctif passé.",
        consolidation: {
          before: "Nous sommes étonnés qu'elle ",
          hint: "accepter",
          after: " si vite leur proposition.",
          answers: ["ait accepté"],
        },
      },
      {
        before: "Si nous avions plus de temps, nous ",
        hint: "visiter",
        after: " volontiers le musée.",
        answers: ["visiterions"],
        rule:
          "Hypothèse irréelle dans le présent : si + imparfait dans la subordonnée, conditionnel présent dans la principale.",
        consolidation: {
          before: "Si tu m'écoutais davantage, tu ",
          hint: "éviter",
          after: " bien des erreurs.",
          answers: ["éviterais"],
        },
      },
      {
        before: "Si vous m'aviez prévenu plus tôt, j'",
        hint: "annuler",
        after: " mon rendez-vous sans hésiter.",
        answers: ["aurais annulé"],
        rule:
          "Hypothèse irréelle dans le passé : si + plus-que-parfait, conditionnel passé.",
        consolidation: {
          before: "Si nous étions partis avant l'orage, nous ",
          hint: "arriver",
          after: " à l'heure.",
          answers: ["serions arrivés"],
        },
      },
      {
        type: "transform",
        instruction:
          "Mettez cette phrase au discours rapporté en commençant par « Le journaliste a rapporté que… ».",
        source: "« Les négociations ont échoué hier soir. »",
        answers: [
          "Le journaliste a rapporté que les négociations avaient échoué la veille au soir",
          "Le journaliste a rapporté que les négociations avaient échoué la veille",
        ],
        rule:
          "Au discours rapporté avec un verbe introducteur au passé, on applique la concordance des temps : le passé composé (ont échoué) devient plus-que-parfait (avaient échoué). Les indicateurs temporels changent aussi : « hier » → « la veille ».",
        consolidation: {
          type: "transform",
          instruction:
            "Mettez cette phrase au discours rapporté en commençant par « Elle a expliqué que… ».",
          source: "« Je viendrai demain matin avec les documents. »",
          answers: [
            "Elle a expliqué qu'elle viendrait le lendemain matin avec les documents",
          ],
          rule:
            "Le futur (viendrai) devient conditionnel présent (viendrait). « Demain » devient « le lendemain ». La 1ʳᵉ personne (je) devient 3ᵉ (elle).",
        },
      },
      {
        before: "Il faut absolument que tout ",
        hint: "être",
        after: " prêt avant son arrivée.",
        answers: ["soit"],
        rule:
          "Principale au présent + action simultanée ou à venir → subjonctif présent. La concordance reste vivante au subjonctif aussi.",
        consolidation: {
          before: "Je voudrais que vous ",
          hint: "me prévenir",
          after: " dès que possible.",
          answers: ["me préveniez"],
        },
      },
    ],
  },

  /* ---------- 4. CONDITIONNEL ---------- */
  {
    id: "conditionnel",
    number: 4,
    section: "Temps et modes",
    title: "Le conditionnel",
    titleEm: "présent et passé",
    lesson: {
      intro:
        "Le conditionnel n'est pas que le mode du « si ». C'est aussi celui de la politesse, du conseil, de l'information non confirmée et du regret. Le maîtriser, c'est gagner en nuance et en élégance.",
      formation: [
        "Conditionnel présent : radical du futur + terminaisons de l'imparfait (-ais, -ais, -ait, -ions, -iez, -aient). Exemple : je parlerais, tu finirais, il prendrait, nous viendrions, vous seriez, ils auraient.",
        "Conditionnel passé : avoir ou être au conditionnel présent + participe passé. Exemple : j'aurais parlé, je serais venu(e), nous nous serions vus.",
        "Attention : le radical est celui du futur, pas de l'infinitif. Pour les verbes irréguliers, c'est le même radical qu'au futur (j'irais, je ferais, je verrais, je saurais, je pourrais, je voudrais, je devrais).",
      ],
      irregulars: [
        { v: "aller", forms: "j'irais · nous irions" },
        { v: "avoir", forms: "j'aurais · nous aurions" },
        { v: "être", forms: "je serais · nous serions" },
        { v: "faire", forms: "je ferais · nous ferions" },
        { v: "pouvoir", forms: "je pourrais · nous pourrions" },
        { v: "savoir", forms: "je saurais · nous saurions" },
        { v: "voir", forms: "je verrais · nous verrions" },
        { v: "vouloir", forms: "je voudrais · nous voudrions" },
        { v: "devoir", forms: "je devrais · nous devrions" },
        { v: "venir", forms: "je viendrais · nous viendrions" },
      ],
      triggers: [
        {
          title: "Emplois du conditionnel présent",
          subcases: [
            {
              label: "Politesse, demande atténuée",
              example: "Je voudrais un café, s'il vous plaît.",
              highlight: "voudrais",
            },
            {
              label: "Hypothèse irréelle dans le présent",
              example: "Si j'étais à ta place, j'accepterais cette offre.",
              highlight: "accepterais",
            },
            {
              label: "Information non confirmée (style journalistique)",
              example: "D'après les médias, le suspect serait toujours en fuite.",
              highlight: "serait",
            },
            {
              label: "Futur dans le passé (concordance des temps)",
              example: "Elle a promis qu'elle reviendrait avant la nuit.",
              highlight: "reviendrait",
            },
          ],
        },
        {
          title: "Emplois du conditionnel passé",
          subcases: [
            {
              label: "Regret ou reproche",
              example: "Tu aurais dû me prévenir avant de partir.",
              highlight: "aurais dû",
            },
            {
              label: "Hypothèse irréelle dans le passé",
              example: "Sans son aide, nous n'aurions jamais terminé à temps.",
              highlight: "n'aurions jamais terminé",
            },
            {
              label: "Information non confirmée portant sur le passé",
              example: "Selon la presse, l'incident aurait eu lieu vers minuit.",
              highlight: "aurait eu",
            },
          ],
        },
        {
          title: "Pièges fréquents",
          body:
            "Ne jamais employer le conditionnel après « si » hypothétique : on dit « si j'avais », jamais « si j'aurais ». À l'oral, la distinction entre futur (je parlerai) et conditionnel (je parlerais) repose uniquement sur la voyelle finale — soyez attentif à l'écrit.",
        },
      ],
    },
    items: [
      {
        before: "",
        hint: "vouloir",
        after: "-vous bien patienter quelques instants ?",
        answers: ["Voudriez", "voudriez"],
        rule:
          "Demande polie → conditionnel présent. « Vouloir » → je voudrais, vous voudriez.",
        consolidation: {
          before: "",
          hint: "pouvoir",
          after: "-tu me passer le sel, s'il te plaît ?",
          answers: ["Pourrais", "pourrais"],
        },
      },
      {
        before: "Si j'étais à ta place, j'",
        hint: "accepter",
        after: " sans hésiter cette proposition.",
        answers: ["accepterais"],
        rule:
          "Hypothèse irréelle dans le présent : si + imparfait dans la subordonnée, conditionnel présent dans la principale.",
        consolidation: {
          before: "Si nous avions le choix, nous ",
          hint: "rester",
          after: " volontiers une nuit de plus.",
          answers: ["resterions"],
        },
      },
      {
        before: "D'après les médias, le président ",
        hint: "envisager",
        after: " une démission prochaine.",
        answers: ["envisagerait"],
        rule:
          "Information non confirmée, source journalistique → conditionnel présent. C'est une manière de rapporter sans engager sa propre responsabilité.",
        consolidation: {
          before: "Selon les premiers témoignages, l'incendie ",
          hint: "se déclarer",
          after: " dans la cuisine.",
          answers: ["se serait déclaré"],
        },
      },
      {
        before: "Tu ",
        hint: "devoir",
        after: " m'avertir avant de prendre une telle décision.",
        answers: ["aurais dû"],
        rule:
          "Reproche portant sur le passé → conditionnel passé. « Tu aurais dû » est la forme typique du reproche en français.",
        consolidation: {
          before: "Vous ",
          hint: "pouvoir",
          after: " nous prévenir, nous serions venus avec joie.",
          answers: ["auriez pu"],
        },
      },
      {
        before: "Elle nous a assuré qu'elle ",
        hint: "revenir",
        after: " avant la tombée de la nuit.",
        answers: ["reviendrait"],
        rule:
          "Concordance des temps : principale au passé + futur dans le passé → conditionnel présent.",
        consolidation: {
          before: "Le directeur a annoncé qu'il ",
          hint: "prendre",
          after: " sa décision dans la semaine.",
          answers: ["prendrait"],
        },
      },
      {
        before: "Sans son intervention, nous ",
        hint: "ne jamais finir",
        after: " à temps.",
        answers: ["n'aurions jamais fini"],
        rule:
          "Hypothèse irréelle dans le passé (sans son intervention = s'il n'était pas intervenu) → conditionnel passé.",
        consolidation: {
          before: "Sans cette panne de réveil, j'",
          hint: "arriver",
          after: " à l'heure.",
          answers: ["serais arrivé", "serais arrivée"],
        },
      },
      {
        before: "J'",
        hint: "aimer",
        after: " tellement vous accompagner, mais je suis pris ce soir.",
        answers: ["aimerais"],
        rule:
          "Souhait atténué, regret au présent → conditionnel présent. « J'aimerais » est plus poli que « je veux ».",
        consolidation: {
          before: "Nous ",
          hint: "souhaiter",
          after: " avoir votre avis sur ce point.",
          answers: ["souhaiterions"],
        },
      },
      {
        before: "Si tu m'avais écouté, tu ",
        hint: "éviter",
        after: " bien des ennuis.",
        answers: ["aurais évité"],
        rule:
          "Hypothèse irréelle dans le passé : si + plus-que-parfait, conditionnel passé. Construction très fréquente pour exprimer un regret ou un reproche.",
        consolidation: {
          before: "Si nous étions partis plus tôt, nous ",
          hint: "ne pas manquer",
          after: " le début du film.",
          answers: ["n'aurions pas manqué"],
        },
      },
    ],
  },

  /* ---------- 5. HYPOTHÈSE / SI ---------- */
  {
    id: "hypothese",
    number: 5,
    section: "Logique de la phrase",
    title: "L'hypothèse",
    titleEm: "avec « si »",
    lesson: {
      intro:
        "Le français possède trois constructions principales avec « si » pour exprimer l'hypothèse. Chacune correspond à un degré différent de réalité : possible, irréel présent, irréel passé. Bien les distinguer, c'est éviter les erreurs les plus stigmatisantes au B2.",
      formation: [
        "Type 1 — hypothèse réelle ou très probable : si + présent, [présent / futur / impératif]. Ex. : Si tu travailles régulièrement, tu réussiras.",
        "Type 2 — hypothèse irréelle dans le présent ou improbable dans l'avenir : si + imparfait, conditionnel présent. Ex. : Si j'avais le temps, je voyagerais davantage.",
        "Type 3 — hypothèse irréelle dans le passé (regret, reproche) : si + plus-que-parfait, conditionnel passé. Ex. : Si tu étais venu, tu aurais adoré.",
        "On peut aussi mélanger les types pour exprimer des nuances : si + plus-que-parfait, conditionnel présent (= une cause passée avec une conséquence actuelle). Ex. : Si j'avais étudié hier, je saurais répondre aujourd'hui.",
      ],
      irregulars: [],
      triggers: [
        {
          title: "Les trois types d'hypothèse",
          subcases: [
            {
              label: "Type 1 — hypothèse possible (présent ou futur)",
              example: "Si la météo le permet, nous partirons demain matin.",
              highlight: "partirons",
            },
            {
              label: "Type 2 — hypothèse irréelle dans le présent",
              example: "Si j'étais plus jeune, je reprendrais mes études.",
              highlight: "reprendrais",
            },
            {
              label: "Type 3 — hypothèse irréelle dans le passé",
              example: "Si nous avions su, nous aurions agi autrement.",
              highlight: "aurions agi",
            },
          ],
        },
        {
          title: "Constructions mixtes",
          subcases: [
            {
              label: "Cause passée, conséquence présente",
              example: "Si j'avais mieux dormi, je serais plus efficace ce matin.",
              highlight: "serais",
            },
            {
              label: "Cause présente, conséquence passée hypothétique",
              example: "S'il était plus organisé, il aurait déjà terminé.",
              highlight: "aurait déjà terminé",
            },
          ],
        },
        {
          title: "Autres expressions de l'hypothèse",
          subcases: [
            {
              label: "« À condition que » + subjonctif",
              example: "Je viendrai à condition que tu me préviennes la veille.",
              highlight: "préviennes",
            },
            {
              label: "« Au cas où » + conditionnel",
              example: "Prends un parapluie au cas où il pleuvrait.",
              highlight: "pleuvrait",
            },
            {
              label: "Gérondif à valeur d'hypothèse",
              example: "En travaillant régulièrement, tu progresseras vite.",
              highlight: "travaillant",
            },
          ],
        },
        {
          title: "Pièges fréquents",
          body:
            "Jamais de conditionnel après « si » hypothétique. « Si j'aurais » est une faute majeure, considérée comme une marque d'inculture par les francophones. Toujours : si j'avais. Attention aussi : après « si » non hypothétique (= chaque fois que, comme dans « si tu sors, prends ton parapluie »), c'est le présent qui s'emploie.",
        },
      ],
    },
    items: [
      {
        before: "Si la météo le permet, nous ",
        hint: "partir",
        after: " demain à l'aube.",
        answers: ["partirons"],
        rule:
          "Type 1 — hypothèse réelle : si + présent dans la subordonnée, futur dans la principale.",
        consolidation: {
          before: "Si tu finis tôt, ",
          hint: "passer",
          after: " me chercher à la sortie du bureau.",
          answers: ["passe"],
        },
      },
      {
        before: "Si elle ",
        hint: "être",
        after: " plus attentive, elle comprendrait mieux les consignes.",
        answers: ["était"],
        rule:
          "Type 2 — hypothèse irréelle dans le présent : si + imparfait, conditionnel présent. Jamais de conditionnel après « si ».",
        consolidation: {
          before: "Si nous ",
          hint: "avoir",
          after: " le temps, nous prendrions volontiers un café.",
          answers: ["avions"],
        },
      },
      {
        before: "Si vous m'aviez écouté, vous ",
        hint: "éviter",
        after: " cette mésaventure.",
        answers: ["auriez évité"],
        rule:
          "Type 3 — hypothèse irréelle dans le passé : si + plus-que-parfait, conditionnel passé. C'est la construction du regret et du reproche.",
        consolidation: {
          before: "Si j'avais su, je ",
          hint: "ne pas venir",
          after: ".",
          answers: ["ne serais pas venu", "ne serais pas venue"],
        },
      },
      {
        before: "Si tu ",
        hint: "venir",
        after: " hier soir, tu aurais rencontré mon frère.",
        answers: ["étais venu", "étais venue"],
        rule:
          "Type 3 vu du côté de la subordonnée : si + plus-que-parfait. « Venir » se conjugue avec être au plus-que-parfait.",
        consolidation: {
          before: "Si elle ",
          hint: "se reposer",
          after: " la veille, elle n'aurait pas été aussi fatiguée.",
          answers: ["s'était reposée"],
        },
      },
      {
        before: "Si j'avais bien dormi la nuit dernière, je ",
        hint: "être",
        after: " plus efficace ce matin.",
        answers: ["serais"],
        rule:
          "Construction mixte : cause passée, conséquence présente. Si + plus-que-parfait dans la subordonnée, conditionnel présent dans la principale.",
        consolidation: {
          before: "Si tu avais étudié hier, tu ",
          hint: "savoir",
          after: " répondre à cette question aujourd'hui.",
          answers: ["saurais"],
        },
      },
      {
        type: "qcm",
        prompt:
          "Quelle construction est correcte après « au cas où » ?",
        context:
          "Prends ton manteau au cas où il ___ froid ce soir.",
        options: ["fasse", "ferait", "fera"],
        correctIndex: 1,
        rule:
          "« Au cas où » se construit obligatoirement avec le conditionnel (présent ou passé), jamais avec le subjonctif ni avec le futur. Confusion fréquente avec « pourvu que » ou « à condition que » qui prennent, eux, le subjonctif.",
        consolidation: {
          type: "qcm",
          prompt:
            "Quelle construction est correcte après cette conjonction ?",
          context:
            "Je viendrai à condition que tu me ___ au moins la veille.",
          options: ["préviennes", "préviendras", "préviendrais"],
          correctIndex: 0,
          rule:
            "« À condition que » introduit une condition stricte et appelle le subjonctif présent. Ne pas confondre avec « au cas où » (conditionnel) ni avec « si » hypothétique (jamais de conditionnel ni de futur).",
        },
      },
      {
        before: "Je viendrai à condition que tu me ",
        hint: "prévenir",
        after: " au moins la veille.",
        answers: ["préviennes"],
        rule:
          "« À condition que » introduit une condition stricte et appelle le subjonctif présent.",
        consolidation: {
          before: "Nous acceptons, à condition que vous ",
          hint: "respecter",
          after: " scrupuleusement les délais.",
          answers: ["respectiez"],
        },
      },
      {
        before: "En ",
        hint: "travailler",
        after: " régulièrement, tu progresseras vite.",
        answers: ["travaillant"],
        rule:
          "Le gérondif (en + participe présent) peut exprimer une hypothèse ou une condition équivalente à « si tu travailles ».",
        consolidation: {
          before: "En ",
          hint: "partir",
          after: " plus tôt, tu éviterais les embouteillages.",
          answers: ["partant"],
        },
      },
    ],
  },

  /* ---------- 6. VOIX PASSIVE ---------- */
  {
    id: "voix-passive",
    number: 6,
    section: "Constructions verbales",
    title: "La voix",
    titleEm: "passive",
    lesson: {
      intro:
        "La voix passive met en relief l'objet ou la victime de l'action plutôt que celui qui la fait. C'est une ressource précieuse au B2 : elle structure les textes journalistiques, administratifs et scientifiques, et offre des nuances de point de vue qu'une simple phrase active ne permet pas.",
      formation: [
        "Structure : être + participe passé du verbe principal, accordé en genre et en nombre avec le sujet.",
        "Le temps de l'auxiliaire être reproduit celui du verbe actif d'origine. Présent : est invité. Passé composé : a été invité. Imparfait : était invité. Futur : sera invité. Conditionnel : serait invité.",
        "L'agent (celui qui fait l'action) est introduit le plus souvent par « par ». Avec certains verbes de sentiment ou de description statique, on préfère « de » : être aimé de tous, être connu de chacun, être suivi d'une foule.",
        "Seuls les verbes transitifs directs (ceux qui ont un COD) peuvent passer au passif. On ne dit pas « il a été téléphoné » : téléphoner est transitif indirect.",
      ],
      irregulars: [],
      triggers: [
        {
          title: "Quand employer la voix passive ?",
          subcases: [
            {
              label: "Mettre en valeur l'objet ou la victime",
              example: "Ce roman a été traduit dans quinze langues.",
              highlight: "a été traduit",
            },
            {
              label: "Quand l'agent est inconnu ou volontairement effacé",
              example: "Le tableau a été dérobé pendant la nuit.",
              highlight: "a été dérobé",
            },
            {
              label: "Style journalistique ou administratif",
              example: "Le projet de loi sera examiné en commission demain.",
              highlight: "sera examiné",
            },
          ],
        },
        {
          title: "Par ou de ?",
          subcases: [
            {
              label: "Action concrète, agent réel → par",
              example: "Le manuscrit a été corrigé par l'auteur lui-même.",
              highlight: "par",
            },
            {
              label: "Sentiment, description statique, accompagnement → de",
              example: "Elle est respectée de tous ses collègues.",
              highlight: "de",
            },
          ],
        },
        {
          title: "Alternatives au passif",
          subcases: [
            {
              label: "« On » comme sujet indéfini (très fréquent à l'oral)",
              example: "On a annulé la réunion au dernier moment.",
              highlight: "On",
            },
            {
              label: "Forme pronominale à sens passif",
              example: "Ce livre se lit en une soirée.",
              highlight: "se lit",
            },
            {
              label: "Se faire / se voir + infinitif",
              example: "Il s'est fait voler son portefeuille dans le métro.",
              highlight: "s'est fait voler",
            },
          ],
        },
        {
          title: "Pièges fréquents",
          body:
            "Les verbes transitifs indirects (parler à, téléphoner à, obéir à, nuire à…) ne peuvent pas se mettre au passif. L'accord du participe se fait toujours avec le sujet, jamais avec l'agent. Le passif est plus rare en français qu'en anglais : « on » et la forme pronominale sont souvent préférés à l'oral.",
        },
      ],
    },
    items: [
      {
        before: "Le rapport ",
        hint: "remettre",
        after: " au directeur la semaine dernière.",
        answers: ["a été remis"],
        rule:
          "Voix passive au passé composé : auxiliaire être au passé composé (a été) + participe passé accordé avec le sujet « le rapport » (masculin singulier).",
        consolidation: {
          before: "Cette décision ",
          hint: "prendre",
          after: " hier soir lors de la réunion.",
          answers: ["a été prise"],
        },
      },
      {
        before: "Cette cathédrale ",
        hint: "construire",
        after: " au treizième siècle par des artisans flamands.",
        answers: ["a été construite"],
        rule:
          "Le participe passé s'accorde avec le sujet (cathédrale, féminin singulier) → construite. L'agent est introduit par « par » car il s'agit d'une action concrète.",
        consolidation: {
          before: "Ces ruines ",
          hint: "découvrir",
          after: " par une équipe d'archéologues en 1972.",
          answers: ["ont été découvertes"],
        },
      },
      {
        type: "qcm",
        prompt: "Quelle préposition convient pour introduire l'agent ?",
        context:
          "Cette directrice est respectée ___ tous ses collaborateurs depuis des années.",
        options: ["par", "de", "avec"],
        correctIndex: 1,
        rule:
          "Avec un verbe de sentiment ou de jugement stable (respecter, admirer, aimer, craindre, connaître, suivre…), on introduit traditionnellement l'agent par « de » plutôt que par « par ». « Par » s'emploie pour une action concrète et ponctuelle.",
        consolidation: {
          type: "qcm",
          prompt: "Quelle préposition convient ici ?",
          context:
            "Le manuscrit a été corrigé ___ l'auteur lui-même avant la publication.",
          options: ["par", "de", "à"],
          correctIndex: 0,
          rule:
            "Action concrète et ponctuelle (corriger un manuscrit) → « par ». La distinction avec « de » se joue sur la nature du verbe : action vs sentiment/description.",
        },
      },
      {
        before: "Les résultats ",
        hint: "annoncer",
        after: " demain matin lors d'une conférence de presse.",
        answers: ["seront annoncés"],
        rule:
          "Voix passive au futur : auxiliaire être au futur (seront) + participe passé accordé avec le sujet « les résultats » (masculin pluriel).",
        consolidation: {
          before: "Les nouvelles mesures ",
          hint: "appliquer",
          after: " dès le mois prochain.",
          answers: ["seront appliquées"],
        },
      },
      {
        type: "transform",
        instruction:
          "Réécrivez cette phrase active à la voix passive (en conservant le temps verbal).",
        source: "Un journaliste indépendant a révélé l'affaire la semaine dernière.",
        answers: [
          "L'affaire a été révélée par un journaliste indépendant la semaine dernière",
          "L'affaire a été révélée la semaine dernière par un journaliste indépendant",
        ],
        rule:
          "Passage actif → passif : le COD (l'affaire) devient sujet, le verbe se met à la forme passive au même temps (a révélé → a été révélée, accord avec « affaire »), et l'ancien sujet devient l'agent introduit par « par ».",
        consolidation: {
          type: "transform",
          instruction:
            "Réécrivez cette phrase active à la voix passive.",
          source: "Le comité examinera les candidatures dès la semaine prochaine.",
          answers: [
            "Les candidatures seront examinées par le comité dès la semaine prochaine",
            "Les candidatures seront examinées dès la semaine prochaine par le comité",
          ],
          rule:
            "Futur actif → futur passif : examinera → seront examinées (auxiliaire être au futur + participe accordé avec « candidatures »).",
        },
      },
      {
        before: "Ce dictionnaire ",
        hint: "vendre",
        after: " très bien dans toutes les librairies.",
        answers: ["se vend"],
        rule:
          "Forme pronominale à sens passif, courante avec les biens et les denrées. Équivalent : « est vendu », mais la tournure pronominale est plus naturelle.",
        consolidation: {
          before: "Ce vin blanc ",
          hint: "boire",
          after: " bien frais, à environ huit degrés.",
          answers: ["se boit"],
        },
      },
      {
        before: "Il ",
        hint: "se faire voler",
        after: " son téléphone hier soir dans le métro.",
        answers: ["s'est fait voler"],
        rule:
          "« Se faire + infinitif » exprime un passif où le sujet subit l'action. Le participe « fait » suivi d'un infinitif reste invariable.",
        consolidation: {
          before: "Elle ",
          hint: "se faire renverser",
          after: " par un cycliste en traversant la rue.",
          answers: ["s'est fait renverser"],
        },
      },
      {
        before: "Ces conclusions ",
        hint: "ne pas partager",
        after: " par l'ensemble des experts.",
        answers: ["ne sont pas partagées"],
        rule:
          "Voix passive au présent à la forme négative : ne + sont + pas + participe accordé. L'agent (par l'ensemble des experts) est introduit par « par » car il s'agit d'un acte intellectuel concret.",
        consolidation: {
          before: "Cette hypothèse ",
          hint: "ne plus défendre",
          after: " aujourd'hui par les chercheurs.",
          answers: ["n'est plus défendue"],
        },
      },
    ],
  },

  /* ---------- 7. VERBES PRONOMINAUX ---------- */
  {
    id: "verbes-pronominaux",
    number: 7,
    section: "Constructions verbales",
    title: "Les verbes",
    titleEm: "pronominaux",
    lesson: {
      intro:
        "Un verbe pronominal s'emploie avec un pronom réfléchi (me, te, se, nous, vous, se) qui renvoie au sujet. Au B2, l'enjeu est triple : reconnaître la valeur sémantique du pronominal, conjuguer correctement aux temps composés (toujours avec être) et appliquer la règle d'accord du participe, qui n'est pas la même pour toutes les catégories.",
      formation: [
        "Aux temps simples : pronom réfléchi placé devant le verbe. Je me lève, tu te lèves, il se lève, nous nous levons, vous vous levez, ils se lèvent.",
        "Aux temps composés : toujours avec l'auxiliaire être. Je me suis levé(e), tu t'es levé(e), il s'est levé, elle s'est levée, nous nous sommes levé(e)s.",
        "À l'impératif affirmatif, le pronom suit le verbe : lève-toi, levons-nous, levez-vous. À l'impératif négatif, il le précède : ne te lève pas.",
        "Avec un infinitif, le pronom s'adapte à la personne : je dois me lever, tu dois te lever, nous devons nous lever.",
      ],
      irregulars: [],
      triggers: [
        {
          title: "Les quatre types de pronominaux",
          subcases: [
            {
              label: "Réfléchi : le sujet exerce l'action sur lui-même",
              example: "Elle se regarde dans le miroir.",
              highlight: "se regarde",
            },
            {
              label: "Réciproque : action mutuelle entre plusieurs sujets",
              example: "Ils se sont rencontrés à l'université.",
              highlight: "se sont rencontrés",
            },
            {
              label: "Sens passif : équivalent à un passif",
              example: "Ce plat se prépare en dix minutes.",
              highlight: "se prépare",
            },
            {
              label: "Essentiellement pronominaux : n'existent qu'à la forme pronominale",
              example: "Je me souviens parfaitement de ce jour.",
              highlight: "me souviens",
            },
          ],
        },
        {
          title: "Accord du participe passé",
          subcases: [
            {
              label: "Essentiellement pronominaux et sens passif → accord avec le sujet",
              example: "Elle s'est enfuie sans dire un mot.",
              highlight: "s'est enfuie",
            },
            {
              label: "Réfléchis ou réciproques → accord avec le COD si placé avant",
              example: "Les lettres qu'elle s'est écrites sont émouvantes.",
              highlight: "écrites",
            },
            {
              label: "Pronom = COI → pas d'accord",
              example: "Elles se sont parlé pendant des heures.",
              highlight: "parlé",
            },
          ],
        },
        {
          title: "Verbes essentiellement pronominaux fréquents au B2",
          body:
            "s'absenter · s'abstenir · se méfier · se moquer · se repentir · se souvenir · s'enfuir · s'évanouir · se suicider · s'envoler · s'écrouler · s'évader.",
        },
        {
          title: "Pièges fréquents",
          body:
            "Avec se parler, se téléphoner, se sourire, s'écrire (quand « écrire à quelqu'un »), le pronom est COI : pas d'accord. Comparer : « Elles se sont vues » (se = COD, accord) et « Elles se sont parlé » (se = COI, pas d'accord). Le verbe « se rendre compte » est invariable au participe (rendu compte), car « compte » est le vrai COD.",
        },
      ],
    },
    items: [
      {
        before: "Hier soir, elle ",
        hint: "s'endormir",
        after: " devant la télévision sans même s'en rendre compte.",
        answers: ["s'est endormie"],
        rule:
          "« S'endormir » est essentiellement pronominal : accord du participe avec le sujet. Elle (féminin singulier) → endormie.",
        consolidation: {
          before: "Les enfants ",
          hint: "s'endormir",
          after: " très tard cette nuit.",
          answers: ["se sont endormis"],
        },
      },
      {
        before: "Ils ",
        hint: "se rencontrer",
        after: " lors d'un colloque à Bruxelles il y a deux ans.",
        answers: ["se sont rencontrés"],
        rule:
          "Réciproque (l'un l'autre) : « se » est COD direct du verbe rencontrer. Accord avec le sujet masculin pluriel → rencontrés.",
        consolidation: {
          before: "Anne et Marie ",
          hint: "se retrouver",
          after: " devant le cinéma à dix-neuf heures.",
          answers: ["se sont retrouvées"],
        },
      },
      {
        before: "Elles ",
        hint: "se parler",
        after: " longuement au téléphone hier soir.",
        answers: ["se sont parlé"],
        rule:
          "« Parler à quelqu'un » : le pronom « se » est COI. Pas d'accord du participe passé, même avec un sujet féminin pluriel.",
        consolidation: {
          before: "Les deux ministres ",
          hint: "se téléphoner",
          after: " plusieurs fois durant la crise.",
          answers: ["se sont téléphoné"],
        },
      },
      {
        before: "Cette tarte ",
        hint: "se manger",
        after: " tiède, accompagnée d'une boule de glace.",
        answers: ["se mange"],
        rule:
          "Pronominal à sens passif (= est mangée). Très fréquent avec les plats, les boissons et tout ce qui se consomme ou se vend.",
        consolidation: {
          before: "Ce mot ",
          hint: "s'écrire",
          after: " avec deux « m », pas avec un seul.",
          answers: ["s'écrit"],
        },
      },
      {
        before: "Voici les chaussures qu'elle ",
        hint: "s'acheter",
        after: " la semaine dernière.",
        answers: ["s'est achetées"],
        rule:
          "Pronominal réfléchi avec COD placé avant (« qu' » reprend chaussures). Accord du participe avec ce COD féminin pluriel → achetées.",
        consolidation: {
          before: "Voilà les livres qu'il ",
          hint: "se procurer",
          after: " pour ses recherches.",
          answers: ["s'est procurés"],
        },
      },
      {
        before: "Je ",
        hint: "se souvenir",
        after: " encore très bien de notre première rencontre.",
        answers: ["me souviens"],
        rule:
          "« Se souvenir » est essentiellement pronominal. Au présent, à la première personne : je me souviens.",
        consolidation: {
          before: "Nous ",
          hint: "se méfier",
          after: " des promesses trop belles pour être vraies.",
          answers: ["nous méfions"],
        },
      },
      {
        before: "",
        hint: "se dépêcher",
        after: " un peu, le train part dans cinq minutes !",
        answers: ["Dépêche-toi", "Dépêchez-vous", "Dépêchons-nous"],
        rule:
          "Impératif affirmatif d'un verbe pronominal : le pronom suit le verbe avec un trait d'union. Forme tu : dépêche-toi. Forme vous : dépêchez-vous. Forme nous : dépêchons-nous.",
        consolidation: {
          before: "",
          hint: "ne pas s'inquiéter",
          after: ", tout va bien se passer.",
          answers: ["Ne t'inquiète pas", "Ne vous inquiétez pas"],
        },
      },
      {
        before: "Après l'annonce du verdict, elle ",
        hint: "s'évanouir",
        after: " brusquement dans la salle d'audience.",
        answers: ["s'est évanouie"],
        rule:
          "« S'évanouir » est essentiellement pronominal : accord avec le sujet féminin singulier → évanouie.",
        consolidation: {
          before: "Devant la foule, les deux candidates ",
          hint: "s'évanouir",
          after: " presque en même temps.",
          answers: ["se sont évanouies"],
        },
      },
    ],
  },

  /* ---------- 8. INFINITIF ---------- */
  {
    id: "infinitif",
    number: 8,
    section: "Constructions verbales",
    title: "L'infinitif",
    titleEm: "présent et passé",
    lesson: {
      intro:
        "L'infinitif est la forme nominale du verbe : il peut occuper toutes les fonctions d'un nom (sujet, COD, complément). Au B2, deux points sont décisifs : choisir entre infinitif présent et infinitif passé selon le rapport temporel, et savoir quelles prépositions ou quels verbes l'introduisent.",
      formation: [
        "Infinitif présent : forme de base du verbe. Parler, finir, prendre, partir, vouloir.",
        "Infinitif passé : avoir ou être à l'infinitif + participe passé. Avoir parlé, être venu(e), s'être levé(e). L'auxiliaire suit les mêmes règles qu'aux temps composés.",
        "Pour les verbes pronominaux à l'infinitif, le pronom s'accorde avec le sujet implicite : je suis content de m'être levé tôt ; elle est contente de s'être levée tôt.",
        "L'infinitif négatif place les deux éléments devant le verbe : ne pas oublier, ne plus fumer, ne rien dire.",
      ],
      irregulars: [],
      triggers: [
        {
          title: "Choisir entre présent et passé",
          subcases: [
            {
              label: "Action simultanée ou postérieure à la principale → infinitif présent",
              example: "Je suis content de te voir aujourd'hui.",
              highlight: "voir",
            },
            {
              label: "Action antérieure à la principale → infinitif passé",
              example: "Je suis content de t'avoir vu hier.",
              highlight: "avoir vu",
            },
            {
              label: "Après « après », l'infinitif passé est obligatoire",
              example: "Après avoir terminé son discours, il a quitté la scène.",
              highlight: "avoir terminé",
            },
          ],
        },
        {
          title: "Principales constructions avec infinitif",
          subcases: [
            {
              label: "Après une préposition (sans, pour, avant de, afin de, au lieu de…)",
              example: "Il est parti sans avoir prévenu personne.",
              highlight: "avoir prévenu",
            },
            {
              label: "Après un verbe modal (devoir, pouvoir, vouloir, savoir, falloir)",
              example: "Tu devrais lui parler avant qu'il ne s'en aille.",
              highlight: "lui parler",
            },
            {
              label: "Après un verbe de perception (voir, entendre, sentir, regarder, écouter)",
              example: "Je l'ai entendu chanter dans la pièce voisine.",
              highlight: "chanter",
            },
            {
              label: "Comme sujet, souvent en début de phrase",
              example: "Avoir des amis fidèles est une véritable richesse.",
              highlight: "Avoir",
            },
          ],
        },
        {
          title: "Pièges fréquents",
          body:
            "Confusion entre « après » + infinitif passé et « avant de » + infinitif présent : ce sont les seules options correctes. On ne dit pas « après faire » ni « avant d'avoir fait ». La négation se place en bloc devant l'infinitif : ne pas y aller, et non « ne y pas aller ». Quand le sujet de l'infinitif diffère de celui du verbe principal, on emploie une proposition complète au subjonctif ou à l'indicatif, jamais un infinitif.",
        },
      ],
    },
    items: [
      {
        before: "Après ",
        hint: "terminer",
        after: " sa présentation, elle a répondu aux questions du public.",
        answers: ["avoir terminé"],
        rule:
          "Après « après », l'infinitif passé est obligatoire pour marquer l'antériorité. Verbe avec auxiliaire avoir → avoir terminé.",
        consolidation: {
          before: "Après ",
          hint: "rentrer",
          after: " chez lui, il s'est immédiatement couché.",
          answers: ["être rentré"],
        },
      },
      {
        before: "Avant de ",
        hint: "partir",
        after: ", n'oublie pas de fermer toutes les fenêtres.",
        answers: ["partir"],
        rule:
          "« Avant de » est suivi obligatoirement de l'infinitif présent, jamais de l'infinitif passé. Symétrique exact de « après » + infinitif passé.",
        consolidation: {
          before: "Avant de ",
          hint: "prendre",
          after: " une telle décision, réfléchis bien.",
          answers: ["prendre"],
        },
      },
      {
        before: "Il regrette amèrement de ",
        hint: "ne pas étudier",
        after: " davantage durant ses jeunes années.",
        answers: ["ne pas avoir étudié", "n'avoir pas étudié"],
        rule:
          "Regret portant sur le passé → infinitif passé. À la forme négative, « ne pas » se place en bloc devant l'infinitif (ou « ne » + auxiliaire + « pas »).",
        consolidation: {
          before: "Elle s'en veut de ",
          hint: "ne pas écouter",
          after: " les conseils de son père.",
          answers: ["ne pas avoir écouté", "n'avoir pas écouté"],
        },
      },
      {
        before: "Je suis très heureuse de ",
        hint: "vous rencontrer",
        after: " enfin en personne.",
        answers: ["vous rencontrer"],
        rule:
          "Sentiment portant sur le moment présent → infinitif présent. Le pronom complément se place devant l'infinitif.",
        consolidation: {
          before: "Nous sommes ravis de ",
          hint: "vous accueillir",
          after: " dans notre établissement.",
          answers: ["vous accueillir"],
        },
      },
      {
        before: "",
        hint: "fumer",
        after: " est interdit dans tous les lieux publics.",
        answers: ["Fumer"],
        rule:
          "L'infinitif peut occuper la fonction de sujet en tête de phrase. Il fonctionne alors comme un nom abstrait. Majuscule initiale.",
        consolidation: {
          before: "",
          hint: "voyager",
          after: " seule lui a appris beaucoup sur elle-même.",
          answers: ["Voyager"],
        },
      },
      {
        before: "Nous l'avons vu ",
        hint: "sortir",
        after: " précipitamment du bâtiment vers vingt heures.",
        answers: ["sortir"],
        rule:
          "Après un verbe de perception (voir, entendre, sentir, regarder, écouter), on emploie l'infinitif présent — jamais un participe ou une proposition.",
        consolidation: {
          before: "Je l'ai entendue ",
          hint: "pleurer",
          after: " toute la nuit dans la chambre voisine.",
          answers: ["pleurer"],
        },
      },
      {
        before: "Il est parti sans ",
        hint: "prévenir",
        after: " qui que ce soit de sa décision.",
        answers: ["prévenir"],
        rule:
          "« Sans » + infinitif présent quand l'action est simultanée à la principale. Si elle est antérieure, on utiliserait l'infinitif passé : « sans avoir prévenu ».",
        consolidation: {
          before: "Elle a tout accepté sans ",
          hint: "discuter",
          after: " un seul instant.",
          answers: ["discuter"],
        },
      },
      {
        before: "Elle est contente de ",
        hint: "se lever",
        after: " plus tôt ces derniers temps.",
        answers: ["se lever"],
        rule:
          "Verbe pronominal à l'infinitif : le pronom s'accorde avec le sujet implicite (elle → se). À l'infinitif présent ici, car l'habitude est simultanée au présent.",
        consolidation: {
          before: "Je suis fier de ",
          hint: "s'engager",
          after: " dans cette cause depuis plus de dix ans.",
          answers: ["m'engager", "m'être engagé", "m'être engagée"],
        },
      },
    ],
  },

  /* ---------- 9. PARTICIPE PRÉSENT ET GÉRONDIF ---------- */
  {
    id: "participe-gerondif",
    number: 9,
    section: "Constructions verbales",
    title: "Participe présent et",
    titleEm: "gérondif",
    lesson: {
      intro:
        "Trois formes en -ant qui se ressemblent mais ne jouent pas le même rôle : le participe présent (verbal, invariable), le gérondif (en + -ant, complément circonstanciel) et l'adjectif verbal (variable, qualifie un nom). Les distinguer, c'est une marque sûre du B2.",
      formation: [
        "Participe présent : on part du radical de la 1ʳᵉ personne du pluriel du présent indicatif et on ajoute -ant. Nous finissons → finissant. Nous prenons → prenant. Nous voyons → voyant.",
        "Trois exceptions à mémoriser : être → étant, avoir → ayant, savoir → sachant.",
        "Gérondif : « en » + participe présent. Toujours invariable. En marchant, en finissant, en prenant.",
        "Adjectif verbal : même forme orthographique que le participe présent dans la plupart des cas, mais variable (s'accorde avec le nom). Une histoire intéressante (adj.), des enfants obéissants (adj.).",
        "Quelques adjectifs verbaux ont une orthographe légèrement différente du participe : fatigant / fatiguant ; convaincant / convainquant ; différent / différant ; influent / influant ; précédent / précédant.",
      ],
      irregulars: [
        { v: "être", forms: "étant" },
        { v: "avoir", forms: "ayant" },
        { v: "savoir", forms: "sachant" },
      ],
      triggers: [
        {
          title: "Emplois du gérondif (en + -ant)",
          subcases: [
            {
              label: "Simultanéité (deux actions en même temps)",
              example: "Elle écoute la radio en cuisinant.",
              highlight: "en cuisinant",
            },
            {
              label: "Manière (comment on fait quelque chose)",
              example: "Il est entré dans la pièce en chantonnant.",
              highlight: "en chantonnant",
            },
            {
              label: "Cause ou moyen",
              example: "Tu progresseras en lisant tous les jours.",
              highlight: "en lisant",
            },
            {
              label: "Hypothèse / condition",
              example: "En partant maintenant, vous arriverez à l'heure.",
              highlight: "En partant",
            },
            {
              label: "Opposition (tout en + gérondif)",
              example: "Tout en reconnaissant ses torts, il refuse de s'excuser.",
              highlight: "Tout en reconnaissant",
            },
          ],
        },
        {
          title: "Emplois du participe présent (sans « en »)",
          subcases: [
            {
              label: "Cause, raison (équivalent à « parce que »)",
              example: "Ne sachant que répondre, elle est restée silencieuse.",
              highlight: "Ne sachant",
            },
            {
              label: "Caractérisation d'un nom (équivalent à une relative)",
              example: "Les passagers ayant des bagages doivent passer par ce comptoir.",
              highlight: "ayant",
            },
          ],
        },
        {
          title: "Pièges fréquents",
          body:
            "Le sujet implicite du gérondif est obligatoirement le même que celui du verbe principal. « En arrivant à la gare, le train partait » est incorrect (ce n'est pas le train qui arrive). Il faut dire : « Quand je suis arrivé à la gare, le train partait ». Attention aussi à l'orthographe : on écrit « il est convaincant » (adj.) mais « me convainquant de venir » (participe).",
        },
      ],
    },
    items: [
      {
        before: "Elle a appris l'italien ",
        hint: "regarder",
        after: " des films en version originale.",
        answers: ["en regardant"],
        rule:
          "Gérondif exprimant le moyen ou la manière → « en » + participe présent. « Regarder » : nous regardons → regardant.",
        consolidation: {
          before: "Il a réussi son examen ",
          hint: "travailler",
          after: " sans relâche pendant trois mois.",
          answers: ["en travaillant"],
        },
      },
      {
        before: "",
        hint: "ne pas savoir",
        after: " quoi répondre, elle est restée muette face à la question.",
        answers: ["Ne sachant pas"],
        rule:
          "Participe présent à valeur causale, équivalent à « parce qu'elle ne savait pas ». Savoir est l'un des trois verbes à participe présent irrégulier : sachant.",
        consolidation: {
          before: "",
          hint: "ne pas avoir",
          after: " la clé, j'ai dû attendre dehors.",
          answers: ["N'ayant pas"],
        },
      },
      {
        type: "qcm",
        prompt: "Dans la phrase suivante, quelle est la nature du mot en majuscules ?",
        context: "Les enfants OBÉISSANTS reçoivent une récompense en fin de semaine.",
        options: ["Participe présent", "Gérondif", "Adjectif verbal"],
        correctIndex: 2,
        rule:
          "« Obéissants » s'accorde avec « enfants » (masculin pluriel). Or seuls les adjectifs verbaux varient en genre et en nombre. Le participe présent et le gérondif sont invariables.",
        consolidation: {
          type: "qcm",
          prompt:
            "Dans la phrase suivante, quelle est la nature du mot en majuscules ?",
          context: "EN PARLANT à voix basse, elle évitait de réveiller le bébé.",
          options: ["Participe présent", "Gérondif", "Adjectif verbal"],
          correctIndex: 1,
          rule:
            "La forme est précédée de « en » : c'est un gérondif. Il indique ici la manière dont l'action principale est accomplie.",
        },
      },
      {
        type: "transform",
        instruction:
          "Réécrivez la subordonnée relative à l'aide d'un participe présent.",
        source:
          "Les passagers qui ont des bagages volumineux doivent passer par le comptoir B.",
        answers: [
          "Les passagers ayant des bagages volumineux doivent passer par le comptoir B",
        ],
        rule:
          "« qui ont » se transforme en participe présent invariable « ayant ». Cette tournure, plus dense, est typique des textes administratifs, juridiques ou journalistiques.",
        consolidation: {
          type: "transform",
          instruction:
            "Réécrivez la subordonnée relative à l'aide d'un participe présent.",
          source:
            "Les étudiants qui souhaitent participer doivent s'inscrire avant vendredi.",
          answers: [
            "Les étudiants souhaitant participer doivent s'inscrire avant vendredi",
          ],
          rule:
            "« qui souhaitent » devient « souhaitant ». Le participe reste invariable même si le sujet est pluriel.",
        },
      },
      {
        before: "",
        hint: "partir",
        after: " maintenant, vous serez chez vous avant la nuit.",
        answers: ["En partant"],
        rule:
          "Gérondif à valeur hypothétique, équivalent à « si vous partez maintenant ». Très courant pour conseiller ou prévoir.",
        consolidation: {
          before: "",
          hint: "se dépêcher",
          after: " un peu, nous pourrions attraper le dernier métro.",
          answers: ["En nous dépêchant"],
        },
      },
      {
        type: "qcm",
        prompt: "Quelle forme convient dans cette phrase ?",
        context: "Ce sont des arguments particulièrement ___ pour la défense.",
        options: ["convainquant", "convaincants", "convaincant"],
        correctIndex: 1,
        rule:
          "Ici, le mot qualifie « arguments » (masculin pluriel) : c'est donc un adjectif verbal qui s'accorde → convaincants. Le participe présent s'écrirait « convainquant » (avec « qu ») et serait invariable. C'est l'un des pièges orthographiques les plus classiques au B2.",
        consolidation: {
          type: "qcm",
          prompt: "Quelle forme convient dans cette phrase ?",
          context: "Cette longue marche en montagne fut vraiment ___.",
          options: ["fatigant", "fatiguant", "fatigante"],
          correctIndex: 2,
          rule:
            "Le mot qualifie « marche » (féminin singulier) : adjectif verbal → fatigante. Le participe présent s'écrirait « fatiguant » (avec « u »).",
        },
      },
      {
        before: "",
        hint: "être",
        after: " malade depuis plusieurs jours, il a fini par consulter un médecin.",
        answers: ["Étant"],
        rule:
          "Participe présent du verbe être : étant (forme irrégulière). Ici à valeur causale.",
        consolidation: {
          before: "",
          hint: "être",
          after: " encore très jeune, elle n'a pas le droit de voter.",
          answers: ["Étant"],
        },
      },
      {
        before: "Il a glissé ",
        hint: "descendre",
        after: " l'escalier et s'est cassé la cheville.",
        answers: ["en descendant"],
        rule:
          "Gérondif de simultanéité : les deux actions (glisser, descendre) ont lieu en même temps et concernent le même sujet.",
        consolidation: {
          before: "Elle s'est blessée ",
          hint: "couper",
          after: " du pain.",
          answers: ["en coupant"],
        },
      },
    ],
  },

  /* ---------- 10. FAIRE / LAISSER + INFINITIF ---------- */
  {
    id: "causatif",
    number: 10,
    section: "Constructions verbales",
    title: "Faire et laisser",
    titleEm: "+ infinitif",
    lesson: {
      intro:
        "Le « faire causatif » signifie qu'on provoque une action sans la faire soi-même : on la fait faire à quelqu'un. « Laisser » exprime au contraire la permission ou la non-intervention. Ces constructions sont indispensables pour parler avec naturel, mais leur syntaxe — choix du COD/COI, place des pronoms, accord du participe — déroute beaucoup d'apprenants.",
      formation: [
        "Faire/laisser + infinitif forme un bloc verbal : on ne peut rien insérer entre les deux. « Je fais réparer ma voiture », jamais « je fais ma voiture réparer ».",
        "Si l'infinitif n'a pas de COD, la personne qui exécute l'action est COD direct : « Je fais venir le médecin ». « Le médecin » est COD direct.",
        "Si l'infinitif a déjà un COD, la personne qui exécute l'action devient COI, introduite par « à » : « Je fais lire ce livre à mon fils ». « Ce livre » est COD ; « à mon fils » est COI.",
        "Les pronoms se placent devant « faire » ou « laisser », pas devant l'infinitif : « Je le fais réparer », « Je lui fais lire ce livre », « Je le lui fais lire ».",
        "Accord du participe passé : avec « faire » causatif, le participe « fait » est toujours invariable. Avec « laisser », il est désormais également invariable selon les rectifications orthographiques (mais l'ancienne règle d'accord existait).",
      ],
      irregulars: [],
      triggers: [
        {
          title: "Faire causatif",
          subcases: [
            {
              label: "Provoquer, faire faire (à quelqu'un d'autre)",
              example: "Le professeur fait travailler les élèves en silence.",
              highlight: "fait travailler",
            },
            {
              label: "Provoquer un état, un sentiment",
              example: "Cette nouvelle m'a fait pleurer pendant des heures.",
              highlight: "a fait pleurer",
            },
            {
              label: "Se faire + infinitif → sens passif (on subit l'action)",
              example: "Elle s'est fait couper les cheveux ce matin.",
              highlight: "s'est fait couper",
            },
          ],
        },
        {
          title: "Laisser + infinitif",
          subcases: [
            {
              label: "Permettre, autoriser",
              example: "Mes parents me laissent sortir le samedi soir.",
              highlight: "laissent sortir",
            },
            {
              label: "Ne pas intervenir, ne pas empêcher",
              example: "Laisse-le finir son histoire avant de répondre.",
              highlight: "Laisse-le finir",
            },
            {
              label: "Se laisser + infinitif → subir, ne pas résister",
              example: "Il s'est laissé convaincre par leurs arguments.",
              highlight: "s'est laissé convaincre",
            },
          ],
        },
        {
          title: "Place des pronoms",
          subcases: [
            {
              label: "Un seul pronom → devant faire/laisser",
              example: "Je le fais entrer immédiatement.",
              highlight: "le fais",
            },
            {
              label: "Deux pronoms → devant faire/laisser dans l'ordre habituel",
              example: "Cette histoire ? Je la lui ai fait raconter trois fois.",
              highlight: "la lui ai fait raconter",
            },
            {
              label: "À l'impératif affirmatif → après le verbe",
              example: "Fais-le entrer, il attend depuis dix minutes !",
              highlight: "Fais-le",
            },
          ],
        },
        {
          title: "Pièges fréquents",
          body:
            "« Fait » suivi d'un infinitif est toujours invariable, même quand le COD précède : « Les vêtements qu'elle s'est fait livrer ». La construction « se faire + infinitif » fonctionne souvent comme un passif : « il s'est fait gronder » = on l'a grondé. Avec « laisser », la règle moderne (rectifications de 1990) recommande aussi l'invariabilité, mais l'accord se rencontre encore en français soutenu.",
        },
      ],
    },
    items: [
      {
        before: "Chaque matin, elle ",
        hint: "faire",
        after: " préparer son café par son assistante.",
        answers: ["fait"],
        rule:
          "Faire causatif au présent : le sujet fait accomplir l'action par quelqu'un d'autre. Construction faire + infinitif, sans rien entre les deux.",
        consolidation: {
          before: "Le directeur ",
          hint: "faire",
          after: " réviser tous les contrats par le service juridique.",
          answers: ["fait"],
        },
      },
      {
        before: "Cette comédie nous ",
        hint: "faire rire",
        after: " du début à la fin.",
        answers: ["a fait rire", "fait rire"],
        rule:
          "Faire + infinitif au sens de « provoquer un sentiment, un état ». Le participe « fait » reste invariable même placé après un pronom COD.",
        consolidation: {
          before: "Ses paroles m'",
          hint: "faire réfléchir",
          after: " toute la nuit.",
          answers: ["ont fait réfléchir"],
        },
      },
      {
        before: "Elle ",
        hint: "se faire couper",
        after: " les cheveux très court avant l'été.",
        answers: ["s'est fait couper"],
        rule:
          "« Se faire + infinitif » à valeur passive (= on lui a coupé les cheveux). « Fait » est invariable devant infinitif.",
        consolidation: {
          before: "Il ",
          hint: "se faire opérer",
          after: " du genou la semaine dernière.",
          answers: ["s'est fait opérer"],
        },
      },
      {
        before: "Ses parents ",
        hint: "laisser",
        after: " choisir son orientation en toute liberté.",
        answers: ["l'ont laissé", "l'ont laissée"],
        rule:
          "Laisser + infinitif au passé composé. Le pronom COD se place devant l'auxiliaire. Selon les rectifications de 1990, « laissé » peut rester invariable ; l'accord avec un COD féminin antéposé reste acceptable.",
        consolidation: {
          before: "Le professeur ",
          hint: "laisser",
          after: " terminer leur exposé sans les interrompre.",
          answers: ["les a laissés", "les a laissé"],
        },
      },
      {
        before: "Trop fatiguée pour discuter, elle ",
        hint: "se laisser convaincre",
        after: " sans véritablement résister.",
        answers: ["s'est laissé convaincre", "s'est laissée convaincre"],
        rule:
          "Se laisser + infinitif exprime que le sujet subit l'action ou n'oppose pas de résistance. L'invariabilité du participe est désormais recommandée.",
        consolidation: {
          before: "Les enfants ",
          hint: "se laisser tenter",
          after: " par les bonbons à la caisse.",
          answers: ["se sont laissé tenter", "se sont laissés tenter"],
        },
      },
      {
        type: "qcm",
        prompt:
          "Quelle est la forme correcte à l'impératif affirmatif ?",
        context:
          "Ce témoignage est essentiel : ___ tout de suite au commissariat.",
        options: ["le fais enregistrer", "fais-le enregistrer", "fais le enregistrer"],
        correctIndex: 1,
        rule:
          "À l'impératif affirmatif, le pronom complément suit le verbe et s'y rattache par un trait d'union. Avec « faire + infinitif », le pronom se place donc après « fais » : « fais-le enregistrer ». À l'impératif négatif, en revanche, il précéderait : « ne le fais pas enregistrer ».",
        consolidation: {
          type: "qcm",
          prompt:
            "Quelle forme est correcte ?",
          context:
            "Ce dossier est urgent : ___ avant midi, s'il te plaît.",
          options: ["le fais signer", "fais-le signer", "signe-le faire"],
          correctIndex: 1,
          rule:
            "Même règle : impératif affirmatif + faire causatif + pronom COD → fais-le signer. La construction « faire + infinitif » forme un bloc, et le pronom se place avant ce bloc, qui devient « fais-le » à l'impératif.",
        },
      },
      {
        before: "Ce film m'",
        hint: "faire pleurer",
        after: " comme rarement au cinéma.",
        answers: ["a fait pleurer"],
        rule:
          "Faire causatif d'émotion au passé composé. Auxiliaire avoir + « fait » invariable + infinitif.",
        consolidation: {
          before: "Cette mauvaise nouvelle nous ",
          hint: "faire perdre",
          after: " tout enthousiasme.",
          answers: ["a fait perdre"],
        },
      },
      {
        before: "Le médecin ",
        hint: "faire passer",
        after: " plusieurs examens au patient avant d'établir son diagnostic.",
        answers: ["a fait passer"],
        rule:
          "Faire causatif au passé composé avec deux compléments : examens = COD de « passer » ; au patient = COI (à + personne). Le participe « fait » reste invariable.",
        consolidation: {
          before: "Le professeur ",
          hint: "faire apprendre",
          after: " un poème de Verlaine à chaque élève.",
          answers: ["a fait apprendre"],
        },
      },
    ],
  },

  /* ---------- 11. PRONOMS COD, COI, EN, Y ---------- */
  {
    id: "pronoms-personnels",
    number: 11,
    section: "Pronoms",
    title: "COD, COI,",
    titleEm: "en et y",
    lesson: {
      intro:
        "Les pronoms personnels compléments remplacent un groupe nominal pour éviter la répétition. Au B2, l'enjeu n'est plus de les reconnaître isolément, mais de choisir entre quatre formes très proches (le/la, lui, en, y), de les combiner correctement et de les placer au bon endroit, notamment à l'impératif et aux temps composés.",
      formation: [
        "COD (qui ? quoi ?) : me, te, le/la, nous, vous, les. Exemple : Je vois Marie → Je la vois.",
        "COI (à qui ?) : me, te, lui, nous, vous, leur. Exemple : Je parle à Marie → Je lui parle.",
        "« En » remplace un complément introduit par « de » (de + chose, expression de quantité, partitif). Exemple : Je viens de Paris → J'en viens. Je bois du café → J'en bois.",
        "« Y » remplace un complément introduit par « à » (à + chose, lieu). Exemple : Je pense à mon avenir → J'y pense. Je vais à Paris → J'y vais.",
        "Place habituelle : le pronom se place devant le verbe conjugué (Je le vois) ou devant l'infinitif (Je veux le voir). À l'impératif affirmatif, il suit le verbe avec trait d'union (Regarde-le, parle-lui, vas-y, prends-en).",
        "Ordre des doubles pronoms : me/te/se/nous/vous → le/la/les → lui/leur → y → en. Exemples : Je le lui donne. Il m'en parle. Donnes-en-lui.",
      ],
      irregulars: [],
      triggers: [
        {
          title: "Le/la (COD) ou lui (COI) ?",
          subcases: [
            {
              label: "Verbes transitifs directs (sans préposition) → COD (le/la/les)",
              example: "Tu connais Marc ? — Oui, je le connais bien.",
              highlight: "le",
            },
            {
              label: "Verbes construits avec « à quelqu'un » → COI (lui/leur)",
              example: "Tu as téléphoné à ta sœur ? — Oui, je lui ai parlé hier.",
              highlight: "lui",
            },
            {
              label: "Pièges : « écouter, regarder, attendre, chercher » sont transitifs directs en français → COD",
              example: "J'attends Pierre depuis une heure. — Je l'attends aussi.",
              highlight: "l'",
            },
          ],
        },
        {
          title: "Quand utiliser « en » ?",
          subcases: [
            {
              label: "Remplace un complément introduit par « de »",
              example: "Tu te souviens de ce voyage ? — Oui, je m'en souviens parfaitement.",
              highlight: "en",
            },
            {
              label: "Remplace un partitif (du, de la, des)",
              example: "Tu veux du café ? — Oui, j'en veux bien, merci.",
              highlight: "en",
            },
            {
              label: "Remplace un complément de quantité (le nombre, lui, reste exprimé)",
              example: "Combien de livres as-tu lus ? — J'en ai lu trois cette semaine.",
              highlight: "en",
            },
          ],
        },
        {
          title: "Quand utiliser « y » ?",
          subcases: [
            {
              label: "Remplace un complément de lieu (à, dans, chez, sur…)",
              example: "Tu vas souvent au cinéma ? — J'y vais une fois par mois.",
              highlight: "y",
            },
            {
              label: "Remplace un complément introduit par « à » + chose",
              example: "As-tu pensé à ton rendez-vous ? — Oui, j'y pense.",
              highlight: "y",
            },
            {
              label: "Attention : avec « à + personne », on emploie lui/leur ou un pronom tonique, pas « y »",
              example: "Je pense à mes parents → Je pense à eux (et non « j'y pense »).",
              highlight: "à eux",
            },
          ],
        },
        {
          title: "Pièges fréquents",
          body:
            "Ne pas confondre « se souvenir de » (→ en) et « se rappeler » (→ COD, le/la/les) : ils ont le même sens mais des constructions différentes. À l'impératif affirmatif, on écrit « vas-y », « manges-en » : on ajoute un « s » au verbe pour faire la liaison. À l'écrit soutenu, le pronom COD précédant un infinitif passif décline normalement : « il faut les faire venir », jamais « il faut faire les venir ».",
        },
      ],
    },
    items: [
      {
        type: "transform",
        instruction:
          "Réécrivez la phrase en remplaçant les compléments par les pronoms qui conviennent.",
        source: "Marie a offert un bouquet de fleurs à sa grand-mère hier soir.",
        answers: [
          "Marie lui en a offert un bouquet hier soir",
          "Marie le lui a offert hier soir",
          "Marie lui en a offert hier soir",
        ],
        rule:
          "Deux compléments à remplacer : « un bouquet de fleurs » (COD) et « à sa grand-mère » (COI). Le COI au pluriel singulier 3ᵉ pers. → lui. Avec un nom non spécifié, « en » reprend la partie après « de ». Ordre des pronoms : lui en (COI avant en).",
        consolidation: {
          type: "transform",
          instruction:
            "Réécrivez la phrase en remplaçant les compléments par les pronoms.",
          source: "Le professeur a expliqué la consigne aux étudiants.",
          answers: [
            "Le professeur la leur a expliquée",
            "Le professeur la leur a expliqué",
          ],
          rule:
            "« La consigne » (COD féminin singulier) → la. « Aux étudiants » (COI pluriel) → leur. Ordre : COD avant COI 3ᵉ pers. → la leur. Accord du participe avec le COD antéposé (la) → expliquée.",
        },
      },
      {
        type: "qcm",
        prompt: "Quel pronom convient dans cette phrase ?",
        context: "Tu te souviens de ce voyage en Italie ? — Oui, je m'___ souviens encore très bien.",
        options: ["le", "y", "en"],
        correctIndex: 2,
        rule:
          "« Se souvenir de » se construit avec la préposition « de » → le pronom à utiliser est « en ». Ne pas confondre avec « se rappeler quelque chose » (sans préposition) qui prend « le/la/les ».",
        consolidation: {
          type: "qcm",
          prompt: "Quel pronom convient ?",
          context: "Tu as parlé à tes parents de ta décision ? — Oui, je ___ ai parlé hier soir.",
          options: ["leur en", "leur le", "les en"],
          correctIndex: 0,
          rule:
            "Deux pronoms : « à tes parents » (COI pluriel) → leur ; « de ta décision » (introduit par de) → en. Ordre : leur en.",
        },
      },
      {
        before: "Tu penses souvent à tes vacances en Grèce ? — Oui, j'",
        hint: "pronom",
        after: " pense presque tous les jours.",
        answers: ["y"],
        rule:
          "« Penser à + chose » → « y ». Attention : « penser à + personne » prendrait « à elle/lui/eux ». Ici « vacances » est une chose donc on emploie y.",
        consolidation: {
          before: "Tu joues encore au tennis le week-end ? — J'",
          hint: "pronom",
          after: " joue chaque dimanche matin.",
          answers: ["y"],
        },
      },
      {
        type: "transform",
        instruction:
          "Réécrivez à l'impératif affirmatif en remplaçant le complément par un pronom.",
        source: "Va à la boulangerie chercher du pain pour ce soir.",
        answers: [
          "Vas-y chercher du pain pour ce soir",
          "Vas-y",
        ],
        rule:
          "Impératif affirmatif : le pronom suit le verbe avec un trait d'union. « À la boulangerie » (lieu) → y. À l'impératif, « va » prend un « s » euphonique devant « y » : vas-y.",
        consolidation: {
          type: "transform",
          instruction:
            "Réécrivez à l'impératif affirmatif en remplaçant le complément.",
          source: "Prends des fruits pour la route.",
          answers: ["Prends-en pour la route", "Prends-en"],
          rule:
            "« Des fruits » (partitif) → en. À l'impératif, « prends » prend également un « s » euphonique devant « en » : prends-en.",
        },
      },
      {
        before: "Tu connais bien cette histoire ? — Bien sûr, je ",
        hint: "pronom",
        after: " connais par cœur depuis l'enfance.",
        answers: ["la"],
        rule:
          "« Connaître » est un verbe transitif direct (connaître quelque chose). « Cette histoire » est COD féminin singulier → la.",
        consolidation: {
          before: "Tu as vu le dernier film de Dupontel ? — Oui, je ",
          hint: "pronom",
          after: " ai vu la semaine dernière.",
          answers: ["l'"],
        },
      },
      {
        type: "qcm",
        prompt: "Quelle combinaison de pronoms convient ?",
        context: "Tu m'as bien envoyé les documents ce matin ? — Oui, je ___ ai envoyés à neuf heures.",
        options: ["te les", "les te", "te en"],
        correctIndex: 0,
        rule:
          "« À moi » (COI 1ʳᵉ pers.) → te (au passage de la 1ʳᵉ à la 2ᵉ personne dans la réponse). « Les documents » (COD masculin pluriel) → les. Ordre : me/te/se avant le/la/les. Le participe « envoyés » s'accorde avec le COD antéposé.",
        consolidation: {
          type: "qcm",
          prompt: "Quelle combinaison convient ?",
          context: "Cette nouvelle, qui te l'a apprise ? — C'est Sophie qui ___ a apprise.",
          options: ["me la", "la me", "me l'"],
          correctIndex: 2,
          rule:
            "« Me » + « la » s'élide en « me l' » devant la voyelle de l'auxiliaire « a ». Ordre : me avant la. Participe accordé avec COD antéposé féminin singulier → apprise.",
        },
      },
      {
        before: "Combien de candidats avez-vous reçus ? — Nous ",
        hint: "deux pronoms",
        after: " avons reçu une vingtaine cette semaine.",
        answers: ["en"],
        rule:
          "Quand un nom est repris avec une expression de quantité, on emploie « en » et la quantité reste exprimée : « j'en ai trois », « il en reste beaucoup », « nous en avons reçu une vingtaine ». Le participe ne s'accorde pas avec « en ».",
        consolidation: {
          before: "Vous avez beaucoup d'amis à Paris ? — Oui, j'",
          hint: "pronom",
          after: " ai plusieurs.",
          answers: ["en"],
        },
      },
      {
        type: "transform",
        instruction:
          "Réécrivez la phrase à la forme négative en conservant les pronoms.",
        source: "Donne-le-lui tout de suite avant qu'il ne s'impatiente.",
        answers: [
          "Ne le lui donne pas tout de suite avant qu'il ne s'impatiente",
          "Ne le lui donne pas",
        ],
        rule:
          "À l'impératif négatif, les pronoms reprennent leur place habituelle devant le verbe (pas de trait d'union) et l'ordre normal s'applique : le lui. La forme orale « donne-le-lui » devient « ne le lui donne pas ».",
        consolidation: {
          type: "transform",
          instruction:
            "Réécrivez à la forme négative.",
          source: "Parlez-leur-en sans hésiter.",
          answers: [
            "Ne leur en parlez pas sans hésiter",
            "Ne leur en parlez pas",
          ],
          rule:
            "Impératif négatif : pronoms devant le verbe, ordre normal (leur en). « Parlez-leur-en » → « Ne leur en parlez pas ».",
        },
      },
    ],
  },

  /* ---------- 12. PRONOMS RELATIFS SIMPLES ---------- */
  {
    id: "relatifs-simples",
    number: 12,
    section: "Pronoms",
    title: "Les pronoms relatifs",
    titleEm: "simples",
    lesson: {
      intro:
        "Les pronoms relatifs simples (qui, que, dont, où) servent à fusionner deux phrases en une seule, en évitant la répétition. Au B2, l'enjeu est de choisir le bon pronom en fonction de la fonction qu'occupe l'antécédent dans la subordonnée — un calcul automatique chez les natifs, mais qui demande un peu d'analyse.",
      formation: [
        "« Qui » remplace un sujet : la subordonnée n'a pas son propre sujet, c'est l'antécédent qui le fournit. Ex. : L'homme qui parle est mon voisin (qui = sujet de parle).",
        "« Que » (ou « qu' » devant voyelle) remplace un COD : la subordonnée a déjà un sujet, et l'antécédent en est l'objet direct. Ex. : Le livre que j'ai acheté est passionnant (que = COD de j'ai acheté).",
        "« Dont » remplace un complément introduit par « de » : un COI avec « de », un complément du nom, un complément d'adjectif ou de verbe. Ex. : C'est le livre dont je t'ai parlé (parler de). C'est l'homme dont la fille est médecin (la fille de…).",
        "« Où » remplace un complément de lieu ou de temps. Ex. : La ville où je vis. Le jour où nous nous sommes rencontrés.",
        "Accord du participe passé après « que » : avec un verbe à un temps composé, le participe s'accorde avec l'antécédent (qui est le COD antéposé). Ex. : Les fleurs que j'ai cueillies.",
      ],
      irregulars: [],
      triggers: [
        {
          title: "Quelle fonction dans la subordonnée ?",
          subcases: [
            {
              label: "Sujet → qui",
              example: "Voici la lettre qui vient d'arriver par la poste.",
              highlight: "qui",
            },
            {
              label: "COD → que/qu'",
              example: "Le roman que je lis en ce moment est extraordinaire.",
              highlight: "que",
            },
            {
              label: "Complément avec « de » → dont",
              example: "C'est un sujet dont nous reparlerons longuement.",
              highlight: "dont",
            },
            {
              label: "Lieu ou temps → où",
              example: "Je me souviens du jour où nous nous sommes rencontrés.",
              highlight: "où",
            },
          ],
        },
        {
          title: "Les emplois de « dont »",
          subcases: [
            {
              label: "Complément d'un verbe construit avec « de »",
              example: "Voici le livre dont je t'avais parlé hier.",
              highlight: "dont",
            },
            {
              label: "Complément du nom (= de qui, de quoi)",
              example: "C'est un écrivain dont les œuvres sont traduites partout.",
              highlight: "dont",
            },
            {
              label: "Complément d'un adjectif construit avec « de »",
              example: "Voilà un succès dont je suis particulièrement fier.",
              highlight: "dont",
            },
            {
              label: "Expression d'une quantité ou d'une partie d'un ensemble",
              example: "J'ai cinquante étudiants, dont vingt sont étrangers.",
              highlight: "dont",
            },
          ],
        },
        {
          title: "Pièges fréquents",
          body:
            "On ne dit jamais « dont je parle de » : « dont » contient déjà « de ». Idem : « dont la voiture de mon père » est incorrect (= dont la voiture). Après « que » + temps composé, accord obligatoire du participe avec l'antécédent : « les chansons que j'ai écoutées ». « Qui » ne s'élide jamais (qui il est, et non « qu'il est » au sens relatif). Distinguer « où » relatif (lieu/temps) et « ou » conjonction (alternative).",
        },
      ],
    },
    items: [
      {
        type: "transform",
        instruction:
          "Reliez les deux phrases en une seule en utilisant un pronom relatif.",
        source:
          "J'ai rencontré une femme. Cette femme parle six langues couramment.",
        answers: [
          "J'ai rencontré une femme qui parle six langues couramment",
        ],
        rule:
          "Dans la seconde phrase, « cette femme » est sujet du verbe « parle ». Le pronom relatif qui remplace un sujet est « qui ».",
        consolidation: {
          type: "transform",
          instruction:
            "Reliez les deux phrases avec un pronom relatif.",
          source:
            "Le train est en retard. Le train arrive de Lyon.",
          answers: [
            "Le train qui arrive de Lyon est en retard",
          ],
          rule:
            "« Le train » est sujet de « arrive » → qui. L'incise relative s'insère entre le sujet et le verbe principal de la phrase d'accueil.",
        },
      },
      {
        before: "Le livre ",
        hint: "pronom relatif",
        after: " tu m'as offert hier soir est absolument passionnant.",
        answers: ["que", "qu'"],
        rule:
          "« Tu m'as offert le livre » : « le livre » est COD de « as offert » → pronom relatif « que ». Devant une voyelle, élision en « qu' » (qu'on, qu'il…).",
        consolidation: {
          before: "L'histoire ",
          hint: "pronom relatif",
          after: " elle a racontée nous a tous bouleversés.",
          answers: ["qu'"],
        },
      },
      {
        type: "qcm",
        prompt: "Quel pronom relatif convient ?",
        context:
          "Voilà l'auteur ___ je t'avais parlé la semaine dernière.",
        options: ["que", "dont", "qui"],
        correctIndex: 1,
        rule:
          "« Parler de quelqu'un » : la préposition « de » fait que le pronom relatif est obligatoirement « dont ». Ne pas dire « que je t'avais parlé » (faute courante à l'oral).",
        consolidation: {
          type: "qcm",
          prompt: "Quel pronom relatif convient ?",
          context:
            "C'est une expérience ___ je me souviendrai longtemps.",
          options: ["que", "dont", "qui"],
          correctIndex: 1,
          rule:
            "« Se souvenir de » se construit avec « de » → dont. Comme pour « parler de », c'est la préposition qui décide.",
        },
      },
      {
        before: "Je n'oublierai jamais le jour ",
        hint: "pronom relatif",
        after: " nous nous sommes rencontrés pour la première fois.",
        answers: ["où"],
        rule:
          "« Où » exprime un complément de temps quand l'antécédent est un nom de temps (jour, moment, instant, année, époque…). Ne pas confondre avec la conjonction « ou » (alternative), qui s'écrit sans accent.",
        consolidation: {
          before: "C'est l'année ",
          hint: "pronom relatif",
          after: " j'ai commencé à apprendre le français.",
          answers: ["où"],
        },
      },
      {
        type: "transform",
        instruction:
          "Reliez les deux phrases en utilisant un pronom relatif. Attention à l'accord.",
        source:
          "Les photos sont magnifiques. J'ai vu ces photos chez toi hier.",
        answers: [
          "Les photos que j'ai vues chez toi hier sont magnifiques",
        ],
        rule:
          "« Ces photos » est COD de « j'ai vu » → que. Avec « que » + auxiliaire avoir au temps composé, le participe s'accorde avec l'antécédent antéposé (les photos, fém. pluriel) → vues.",
        consolidation: {
          type: "transform",
          instruction:
            "Reliez les phrases avec un pronom relatif (attention à l'accord).",
          source:
            "Les lettres ont été écrites avec soin. Elle a envoyé ces lettres ce matin.",
          answers: [
            "Les lettres qu'elle a envoyées ce matin ont été écrites avec soin",
          ],
          rule:
            "« Ces lettres » est COD → qu'. Participe accordé avec l'antécédent féminin pluriel → envoyées.",
        },
      },
      {
        before: "C'est une écrivaine ",
        hint: "pronom relatif",
        after: " les livres sont traduits dans le monde entier.",
        answers: ["dont"],
        rule:
          "« Les livres de cette écrivaine » : complément du nom introduit par « de » → dont. Attention : on dit « dont les livres » et non « dont ses livres » (le possessif disparaît avec dont).",
        consolidation: {
          before: "Je connais un peintre ",
          hint: "pronom relatif",
          after: " les œuvres se vendent à prix d'or.",
          answers: ["dont"],
        },
      },
      {
        type: "qcm",
        prompt:
          "Laquelle de ces phrases est correcte ?",
        context: "",
        options: [
          "C'est le sujet dont je t'ai parlé.",
          "C'est le sujet dont je t'ai parlé de.",
          "C'est le sujet que je t'ai parlé.",
        ],
        correctIndex: 0,
        rule:
          "« Dont » contient déjà « de ». Répéter « de » est une faute grammaticale (option B). Et « parler » se construit avec « de » : ce n'est donc pas « que » mais « dont » qui convient (option C est fausse).",
        consolidation: {
          type: "qcm",
          prompt: "Laquelle de ces phrases est correcte ?",
          context: "",
          options: [
            "Voici l'ami dont sa sœur est avocate.",
            "Voici l'ami dont la sœur est avocate.",
            "Voici l'ami que la sœur est avocate.",
          ],
          correctIndex: 1,
          rule:
            "« Dont » remplace le possessif : on ne dit jamais « dont sa/son/ses ». La forme correcte est « dont la sœur » (= la sœur de l'ami).",
        },
      },
      {
        before: "Le restaurant ",
        hint: "pronom relatif",
        after: " nous avons dîné hier soir était excellent.",
        answers: ["où"],
        rule:
          "« Où » remplace un complément de lieu : « nous avons dîné dans ce restaurant ». À l'oral on entend parfois « que nous avons dîné », mais c'est une faute à l'écrit.",
        consolidation: {
          before: "Voici la maison ",
          hint: "pronom relatif",
          after: " j'ai passé toute mon enfance.",
          answers: ["où"],
        },
      },
    ],
  },

  /* ---------- 13. PRONOMS RELATIFS COMPOSÉS ---------- */
  {
    id: "relatifs-composes",
    number: 13,
    section: "Pronoms",
    title: "Les pronoms relatifs",
    titleEm: "composés",
    lesson: {
      intro:
        "Les pronoms relatifs composés (lequel, laquelle, lesquels, lesquelles et leurs formes contractées) s'emploient après une préposition autre que « de ». Ils permettent d'écrire un français plus précis et plus soutenu — indispensable au B2 pour les écrits formels et nuancés.",
      formation: [
        "Forme de base : lequel (masc. sg.), laquelle (fém. sg.), lesquels (masc. pl.), lesquelles (fém. pl.). Ils s'accordent en genre et en nombre avec l'antécédent.",
        "Contractions avec « à » : auquel, à laquelle, auxquels, auxquelles. Ex. : Le projet auquel je travaille. Les questions auxquelles je dois répondre.",
        "Contractions avec « de » (rare, après une locution prépositive) : duquel, de laquelle, desquels, desquelles. Ex. : La maison près de laquelle nous habitons. L'arbre au pied duquel ils se sont assis.",
        "Pour les personnes, on préfère généralement « qui » après une préposition simple (à, avec, pour, sans, chez…) : la personne avec qui je travaille, l'ami à qui je pense. Les formes en « lequel » restent possibles mais plus formelles.",
      ],
      irregulars: [],
      triggers: [
        {
          title: "Après une préposition simple",
          subcases: [
            {
              label: "Avec une chose ou un concept → lequel/laquelle/lesquels/lesquelles",
              example: "Voici la raison pour laquelle je suis venu.",
              highlight: "pour laquelle",
            },
            {
              label: "Avec une personne → préposition + qui (de préférence)",
              example: "Le collègue avec qui je partage mon bureau est très sympathique.",
              highlight: "avec qui",
            },
            {
              label: "Préposition « à » + chose → forme contractée (auquel…)",
              example: "Le projet auquel nous travaillons depuis six mois aboutira bientôt.",
              highlight: "auquel",
            },
          ],
        },
        {
          title: "Après une locution prépositive (avec « de »)",
          subcases: [
            {
              label: "« à côté de », « près de », « au milieu de »…",
              example: "L'immeuble à côté duquel nous habitons va être démoli.",
              highlight: "à côté duquel",
            },
            {
              label: "« au cours de », « à la suite de », « grâce à »",
              example: "La réunion à la suite de laquelle il a démissionné a été houleuse.",
              highlight: "à la suite de laquelle",
            },
          ],
        },
        {
          title: "« Dont » ou « duquel » ?",
          subcases: [
            {
              label: "Verbe ou adjectif construit avec « de » → dont",
              example: "Voici le projet dont je suis fier (être fier de).",
              highlight: "dont",
            },
            {
              label: "Locution prépositive contenant « de » → duquel/de laquelle…",
              example: "Le banc à côté duquel passent les enfants chaque matin.",
              highlight: "à côté duquel",
            },
          ],
        },
        {
          title: "Pièges fréquents",
          body:
            "Ne pas employer « auquel » quand un simple « à qui » suffirait pour une personne : on dit plus naturellement « la personne à qui je pense » que « la personne à laquelle je pense ». À l'écrit administratif, on utilise systématiquement les formes composées. Confusion fréquente : « la raison pour laquelle » (correcte) et « la raison pourquoi » (incorrecte en français standard).",
        },
      ],
    },
    items: [
      {
        before: "Voici la véritable raison pour ",
        hint: "pronom relatif composé",
        after: " j'ai dû quitter mon ancien emploi.",
        answers: ["laquelle"],
        rule:
          "« La raison » est féminin singulier → laquelle. Après une préposition autre que « de », on emploie un relatif composé accordé avec l'antécédent. Forme idiomatique : « la raison pour laquelle ».",
        consolidation: {
          before: "Les motifs pour ",
          hint: "pronom relatif composé",
          after: " il a démissionné restent encore obscurs.",
          answers: ["lesquels"],
        },
      },
      {
        type: "qcm",
        prompt: "Quel pronom relatif convient ?",
        context:
          "Le projet ___ nous travaillons depuis un an touche à sa fin.",
        options: ["dont", "auquel", "lequel"],
        correctIndex: 1,
        rule:
          "« Travailler à » : préposition « à ». Avec un antécédent masculin singulier (le projet) → contraction à + lequel = auquel. « Dont » ne convient pas car « travailler » se construit avec « à », pas avec « de ».",
        consolidation: {
          type: "qcm",
          prompt: "Quel pronom convient ?",
          context: "Les questions ___ je dois répondre sont nombreuses.",
          options: ["dont", "lesquelles", "auxquelles"],
          correctIndex: 2,
          rule:
            "« Répondre à » : préposition « à ». Antécédent féminin pluriel → à + lesquelles = auxquelles.",
        },
      },
      {
        type: "transform",
        instruction:
          "Reliez les deux phrases en utilisant un pronom relatif composé.",
        source:
          "Voici la maison. Nous habitons à côté de cette maison depuis dix ans.",
        answers: [
          "Voici la maison à côté de laquelle nous habitons depuis dix ans",
        ],
        rule:
          "« À côté de » est une locution prépositive contenant « de ». Avec un antécédent féminin singulier → de laquelle. On ne peut pas utiliser « dont » car « à côté de » est une locution complète, pas une simple préposition « de ».",
        consolidation: {
          type: "transform",
          instruction:
            "Reliez les phrases avec un pronom relatif composé.",
          source:
            "Voici l'arbre. Les enfants se sont assis au pied de cet arbre.",
          answers: [
            "Voici l'arbre au pied duquel les enfants se sont assis",
          ],
          rule:
            "« Au pied de » → locution avec « de ». Antécédent masculin singulier → de + lequel = duquel.",
        },
      },
      {
        before: "Le collègue avec ",
        hint: "pronom relatif",
        after: " je partage mon bureau est d'une rare gentillesse.",
        answers: ["qui", "lequel"],
        rule:
          "Préposition « avec » + personne → « qui » est la forme préférée en français contemporain (« avec lequel » reste possible mais sonne plus formel). Les deux sont correctes.",
        consolidation: {
          before: "Les amis chez ",
          hint: "pronom relatif",
          after: " nous avons passé le week-end vivent en Bretagne.",
          answers: ["qui", "lesquels"],
        },
      },
      {
        type: "qcm",
        prompt:
          "Laquelle de ces phrases est la plus naturelle en français contemporain ?",
        context: "",
        options: [
          "L'avocate à laquelle j'ai téléphoné est très compétente.",
          "L'avocate à qui j'ai téléphoné est très compétente.",
          "L'avocate dont j'ai téléphoné est très compétente.",
        ],
        correctIndex: 1,
        rule:
          "Avec une personne et une préposition simple comme « à », on préfère largement « à qui » à « à laquelle » en français contemporain. « Dont » est exclu : on dit « téléphoner à quelqu'un », pas « téléphoner de quelqu'un ».",
        consolidation: {
          type: "qcm",
          prompt: "Quelle formulation est la plus naturelle ?",
          context: "",
          options: [
            "Le voisin auquel je m'adresse refuse de me répondre.",
            "Le voisin à qui je m'adresse refuse de me répondre.",
            "Le voisin dont je m'adresse refuse de me répondre.",
          ],
          correctIndex: 1,
          rule:
            "Personne + préposition « à » → « à qui » est plus naturel. La forme « auquel » s'utilise plutôt pour les choses.",
        },
      },
      {
        before: "L'événement à la suite ",
        hint: "pronom relatif composé",
        after: " la direction a démissionné reste mystérieux.",
        answers: ["duquel"],
        rule:
          "« À la suite de » : locution prépositive avec « de ». Antécédent masculin singulier (l'événement) → de + lequel = duquel. Cas typique où « dont » serait incorrect.",
        consolidation: {
          before: "La réunion à l'issue ",
          hint: "pronom relatif composé",
          after: " ils ont signé l'accord a duré six heures.",
          answers: ["de laquelle"],
        },
      },
      {
        type: "transform",
        instruction:
          "Reliez les deux phrases avec le pronom relatif composé qui convient.",
        source:
          "Voici les associations. Notre fondation apporte un soutien financier à ces associations.",
        answers: [
          "Voici les associations auxquelles notre fondation apporte un soutien financier",
        ],
        rule:
          "« Apporter à » : préposition « à ». Antécédent féminin pluriel (les associations) → à + lesquelles = auxquelles.",
        consolidation: {
          type: "transform",
          instruction:
            "Reliez les phrases avec le pronom relatif qui convient.",
          source:
            "Voici l'instrument. Je joue de cet instrument depuis vingt ans.",
          answers: [
            "Voici l'instrument dont je joue depuis vingt ans",
          ],
          rule:
            "« Jouer de » : préposition « de » sans locution → « dont » suffit. On n'a pas besoin de « duquel » ici, car « de » n'appartient pas à une locution comme « à côté de ».",
        },
      },
      {
        before: "La table sur ",
        hint: "pronom relatif composé",
        after: " tu as posé tes clés appartenait à ma grand-mère.",
        answers: ["laquelle"],
        rule:
          "Préposition « sur » + chose féminine singulière → sur laquelle. Avec une préposition autre que « à » ou « de », on emploie directement la forme de base (lequel, laquelle…), sans contraction.",
        consolidation: {
          before: "Les étagères sur ",
          hint: "pronom relatif composé",
          after: " sont rangés mes livres ont besoin d'être nettoyées.",
          answers: ["lesquelles"],
        },
      },
    ],
  },

  /* ---------- 14. PRONOMS DÉMONSTRATIFS ---------- */
  {
    id: "demonstratifs",
    number: 14,
    section: "Pronoms",
    title: "Les pronoms",
    titleEm: "démonstratifs",
    lesson: {
      intro:
        "Les pronoms démonstratifs (celui, celle, ceux, celles, ce, ceci, cela, ça) permettent de désigner sans répéter le nom. Au B2, deux usages dominent : la reprise avec un complément (celui de mon père, ceux qui veulent) et le démonstratif neutre (ce, ceci, cela), qui renvoie à une idée entière.",
      formation: [
        "Formes variables : celui (masc. sg.), celle (fém. sg.), ceux (masc. pl.), celles (fém. pl.). Ils s'accordent avec le nom remplacé.",
        "Formes renforcées avec -ci ou -là : celui-ci désigne ce qui est proche ; celui-là, ce qui est plus éloigné. À l'écrit moderne, la distinction proche/lointain s'efface souvent : -là sert de forme neutre.",
        "Formes neutres : ce/c' (devant être : c'est, ce sont), ceci (rare, plus formel), cela (soutenu), ça (oral et courant).",
        "Les formes variables (celui, celle…) ne peuvent jamais être seules : elles sont obligatoirement suivies d'un complément (avec « de »), d'une proposition relative, ou prennent la forme renforcée (-ci, -là).",
      ],
      irregulars: [],
      triggers: [
        {
          title: "Celui/celle/ceux/celles : reprise d'un nom",
          subcases: [
            {
              label: "Suivis de « de » (possession, origine)",
              example: "Mon vélo est ancien ; celui de mon frère est tout neuf.",
              highlight: "celui de mon frère",
            },
            {
              label: "Suivis d'une proposition relative",
              example: "Parmi ces livres, ceux qui te plaisent peuvent partir avec toi.",
              highlight: "ceux qui te plaisent",
            },
            {
              label: "Avec -ci / -là pour distinguer ou désigner",
              example: "J'hésite entre ces deux modèles : celui-ci est plus léger, celui-là est plus solide.",
              highlight: "celui-ci",
            },
          ],
        },
        {
          title: "Démonstratifs neutres : ce, ceci, cela, ça",
          subcases: [
            {
              label: "« Ce/c' » devant être (présentation, identification)",
              example: "Ce sont les meilleurs amis que j'aie jamais eus.",
              highlight: "Ce",
            },
            {
              label: "« Cela » (formel) ou « ça » (oral) reprend une idée entière",
              example: "Tu as raison, cela mérite réflexion.",
              highlight: "cela",
            },
            {
              label: "« Ceci » annonce ce qui suit, « cela » reprend ce qui précède (à l'écrit soigné)",
              example: "Retenez ceci : la patience finit toujours par payer.",
              highlight: "ceci",
            },
          ],
        },
        {
          title: "Pièges fréquents",
          body:
            "On ne dit jamais « celui qui est rouge est joli » sans antécédent dans le contexte : il faut un nom déjà mentionné ou évident. « Ceci » est très peu employé à l'oral ; on dit naturellement « cela » ou « ça ». Devant un verbe autre que « être », on emploie « cela » ou « ça », pas « ce » : « cela me plaît », non « ce me plaît ». Distinction « ce »/« se » : « ce » est démonstratif (c'est lui), « se » est réfléchi (il se voit).",
        },
      ],
    },
    items: [
      {
        before: "Mon ordinateur portable est lent ; ",
        hint: "pronom démonstratif",
        after: " de mon frère est beaucoup plus rapide.",
        answers: ["celui"],
        rule:
          "Reprise d'un nom masculin singulier (ordinateur) avec un complément introduit par « de » → celui. La forme variable du démonstratif s'accorde avec le nom remplacé.",
        consolidation: {
          before: "Ma valise est ouverte ; ",
          hint: "pronom démonstratif",
          after: " de ma sœur reste fermée.",
          answers: ["celle"],
        },
      },
      {
        type: "qcm",
        prompt: "Quel pronom démonstratif convient ?",
        context:
          "Parmi tous les candidats, ___ qui maîtrisent l'anglais seront retenus.",
        options: ["ce", "ceux", "celui"],
        correctIndex: 1,
        rule:
          "Reprise d'un nom masculin pluriel (candidats) + proposition relative (qui maîtrisent…) → ceux. « Ce » est neutre invariable. « Celui » est masculin singulier.",
        consolidation: {
          type: "qcm",
          prompt: "Quel pronom convient ?",
          context: "Parmi ces idées, ___ qui me semblent les plus prometteuses méritent un examen approfondi.",
          options: ["celle", "celles", "ceux"],
          correctIndex: 1,
          rule:
            "« Idées » est féminin pluriel → celles. Le démonstratif s'accorde toujours avec le nom remplacé.",
        },
      },
      {
        before: "Tu hésites entre ces deux livres ? ",
        hint: "pronom démonstratif renforcé",
        after: " est plus court mais plus dense que l'autre.",
        answers: ["Celui-ci", "celui-ci"],
        rule:
          "Avec deux objets présents, on emploie -ci et -là pour les distinguer. -ci désigne ce qui est plus proche ou mentionné en dernier ; -là, ce qui est plus éloigné.",
        consolidation: {
          before: "Voilà deux propositions très différentes : ",
          hint: "pronom démonstratif renforcé",
          after: " me semble plus réaliste, l'autre plus ambitieuse.",
          answers: ["celle-ci", "Celle-ci"],
        },
      },
      {
        type: "transform",
        instruction:
          "Réécrivez la phrase en remplaçant le second « avis » par un pronom démonstratif suivi de « de ».",
        source:
          "Son avis ne compte pas pour moi, mais l'avis de mes parents compte énormément.",
        answers: [
          "Son avis ne compte pas pour moi, mais celui de mes parents compte énormément",
        ],
        rule:
          "« L'avis de mes parents » devient « celui de mes parents » : le démonstratif masculin singulier remplace « avis » et conserve le complément en « de ».",
        consolidation: {
          type: "transform",
          instruction:
            "Réécrivez en remplaçant le second « voiture » par un pronom démonstratif.",
          source:
            "Ma voiture est en panne, mais la voiture de mon voisin marche très bien.",
          answers: [
            "Ma voiture est en panne, mais celle de mon voisin marche très bien",
          ],
          rule:
            "« Voiture » est féminin singulier → celle. La structure reste « celle de + complément ».",
        },
      },
      {
        before: "Tu as raison, ",
        hint: "démonstratif neutre soutenu",
        after: " mérite vraiment qu'on y réfléchisse à deux fois.",
        answers: ["cela"],
        rule:
          "Reprise d'une idée entière (le fait que tu aies raison), pas d'un nom précis → démonstratif neutre. À l'écrit soigné : cela. À l'oral et à l'écrit familier : ça.",
        consolidation: {
          before: "Vous avez signalé une erreur dans le rapport : ",
          hint: "démonstratif neutre",
          after: " nous aidera à améliorer la prochaine version.",
          answers: ["cela"],
        },
      },
      {
        type: "qcm",
        prompt:
          "Laquelle de ces phrases est correcte ?",
        context: "",
        options: [
          "Ce me plaît beaucoup.",
          "Cela me plaît beaucoup.",
          "Celui me plaît beaucoup.",
        ],
        correctIndex: 1,
        rule:
          "Devant un verbe autre que « être », on emploie « cela » (ou « ça »), pas « ce ». « Ce » est réservé à la position devant le verbe être : c'est, ce sont, ce sera. « Celui » exigerait un nom antécédent.",
        consolidation: {
          type: "qcm",
          prompt: "Laquelle est correcte ?",
          context: "",
          options: [
            "Ce ne me dérange pas du tout.",
            "Cela ne me dérange pas du tout.",
            "Celui ne me dérange pas.",
          ],
          correctIndex: 1,
          rule:
            "Même règle : devant un verbe autre que « être », on utilise « cela » (ou « ça »), jamais « ce ».",
        },
      },
      {
        before: "Retenez bien ",
        hint: "démonstratif neutre qui annonce",
        after: " : la patience est la clé de toute réussite durable.",
        answers: ["ceci"],
        rule:
          "« Ceci » (rare et soutenu) annonce ce qui va suivre. « Cela » renverrait à ce qui précède. À l'écrit soigné, la distinction est maintenue ; à l'oral, on dit presque toujours « ça ».",
        consolidation: {
          before: "Sache ",
          hint: "démonstratif neutre qui annonce",
          after: " : sans préparation, aucun projet ne tient longtemps.",
          answers: ["ceci"],
        },
      },
      {
        type: "transform",
        instruction:
          "Réécrivez en remplaçant la deuxième occurrence de « livres » par un pronom démonstratif.",
        source:
          "Les livres anciens m'attirent davantage que les livres qui viennent de paraître.",
        answers: [
          "Les livres anciens m'attirent davantage que ceux qui viennent de paraître",
        ],
        rule:
          "« Les livres qui viennent de paraître » → « ceux qui viennent de paraître ». Démonstratif masculin pluriel + proposition relative. Construction très fréquente pour comparer ou distinguer.",
        consolidation: {
          type: "transform",
          instruction:
            "Réécrivez en remplaçant la deuxième occurrence par un pronom démonstratif.",
          source:
            "Les histoires drôles m'amusent moins que les histoires qui font réfléchir.",
          answers: [
            "Les histoires drôles m'amusent moins que celles qui font réfléchir",
          ],
          rule:
            "« Histoires » féminin pluriel → celles. Structure : celles + relative.",
        },
      },
    ],
  },

  /* ---------- 15. PRONOMS POSSESSIFS ---------- */
  {
    id: "possessifs",
    number: 15,
    section: "Pronoms",
    title: "Les pronoms",
    titleEm: "possessifs",
    lesson: {
      intro:
        "Les pronoms possessifs (le mien, le tien, le sien, le nôtre, le vôtre, le leur, et leurs formes féminines et pluriels) évitent la répétition d'un nom déjà mentionné, en exprimant le possesseur. Souvent négligés par les apprenants au profit de tournures plus lourdes (« la voiture qui est à moi »), ils donnent à la phrase concision et naturel.",
      formation: [
        "Formes pour un seul possesseur : le mien, le tien, le sien (masc. sg.) ; la mienne, la tienne, la sienne (fém. sg.) ; les miens, les tiens, les siens (masc. pl.) ; les miennes, les tiennes, les siennes (fém. pl.).",
        "Formes pour plusieurs possesseurs : le nôtre, le vôtre, le leur (masc. sg.) ; la nôtre, la vôtre, la leur (fém. sg.) ; les nôtres, les vôtres, les leurs (toujours pluriel).",
        "L'article (le, la, les) se contracte avec « à » ou « de » : au mien, du tien, aux siennes, des nôtres.",
        "Attention à l'accent circonflexe : nôtre, vôtre (pronoms possessifs) prennent un accent ; notre, votre (adjectifs possessifs) n'en prennent pas. Comparez : « notre maison » et « la nôtre ».",
      ],
      irregulars: [],
      triggers: [
        {
          title: "Quand employer un pronom possessif ?",
          subcases: [
            {
              label: "Pour éviter la répétition d'un nom déjà cité",
              example: "Ton appartement est immense ; le mien est beaucoup plus modeste.",
              highlight: "le mien",
            },
            {
              label: "Dans une comparaison",
              example: "Sa stratégie a échoué, mais la nôtre a parfaitement fonctionné.",
              highlight: "la nôtre",
            },
            {
              label: "Pour insister sur la possession",
              example: "Cette idée n'est pas la mienne, je l'ai empruntée à un collègue.",
              highlight: "la mienne",
            },
          ],
        },
        {
          title: "Accord du pronom possessif",
          subcases: [
            {
              label: "Accord avec l'objet possédé, pas avec le possesseur",
              example: "Voici ma clé ; où est la tienne ?",
              highlight: "la tienne",
            },
            {
              label: "Pluriel quand l'objet possédé est au pluriel",
              example: "Mes parents sont en voyage, les tiens sont restés à la maison.",
              highlight: "les tiens",
            },
            {
              label: "Plusieurs possesseurs, un seul objet → forme en nôtre/vôtre/leur",
              example: "Notre proposition diffère légèrement de la leur sur deux points.",
              highlight: "la leur",
            },
          ],
        },
        {
          title: "Expressions figées",
          subcases: [
            {
              label: "« Y mettre du sien » : faire un effort, contribuer",
              example: "Pour que ce projet réussisse, chacun doit y mettre du sien.",
              highlight: "du sien",
            },
            {
              label: "« Être des nôtres / des vôtres » : faire partie de notre/votre groupe",
              example: "Viendrez-vous dîner ce soir ? Nous serions ravis que vous soyez des nôtres.",
              highlight: "des nôtres",
            },
            {
              label: "« Faire des siennes » : faire des bêtises ou des caprices",
              example: "Le moteur a encore fait des siennes ce matin.",
              highlight: "des siennes",
            },
          ],
        },
        {
          title: "Pièges fréquents",
          body:
            "Ne pas oublier l'accent sur nôtre/vôtre quand ce sont des pronoms : « la nôtre », « les vôtres ». La confusion avec les adjectifs possessifs (notre, votre) est très répandue à l'écrit. Le pronom possessif s'accorde toujours avec ce qui est possédé : si tu parles de « sa voiture » à un homme, tu diras « la sienne » (féminin pour voiture), pas « le sien ». Et au pluriel : « ses voitures » → « les siennes ».",
        },
      ],
    },
    items: [
      {
        before: "Ma maison est petite, mais ",
        hint: "pronom possessif",
        after: " est vraiment immense.",
        answers: ["la tienne"],
        rule:
          "Possesseur : « tu » → tien. Objet possédé : maison (féminin singulier) → la tienne. Le pronom possessif s'accorde toujours avec l'objet possédé, jamais avec le possesseur.",
        consolidation: {
          before: "Mon bureau est encombré, mais ",
          hint: "pronom possessif",
          after: " est parfaitement rangé.",
          answers: ["le tien"],
        },
      },
      {
        type: "qcm",
        prompt: "Quel pronom possessif convient ?",
        context:
          "Voici mes documents. Pouvez-vous me montrer ___, s'il vous plaît ?",
        options: ["le vôtre", "la vôtre", "les vôtres"],
        correctIndex: 2,
        rule:
          "« Documents » est masculin pluriel → les vôtres. Plusieurs possesseurs (« vous ») et objet pluriel → « les vôtres » (toujours pluriel pour les formes en -res).",
        consolidation: {
          type: "qcm",
          prompt: "Quel pronom convient ?",
          context: "Nos enfants sont à l'école ; où sont ___ ?",
          options: ["le leur", "les leurs", "le vôtre"],
          correctIndex: 1,
          rule:
            "« Enfants » est masculin pluriel ; les possesseurs sont eux/elles → les leurs. Le contexte (à qui s'adresse-t-on ?) impose ici « leurs », pas « vôtres ».",
        },
      },
      {
        type: "transform",
        instruction:
          "Réécrivez en remplaçant le second groupe nominal par un pronom possessif.",
        source:
          "Ses arguments sont solides, mais nos arguments le sont davantage encore.",
        answers: [
          "Ses arguments sont solides, mais les nôtres le sont davantage encore",
        ],
        rule:
          "« Nos arguments » devient « les nôtres ». Plusieurs possesseurs (« nous ») + objet masculin pluriel → les nôtres (avec accent circonflexe).",
        consolidation: {
          type: "transform",
          instruction:
            "Réécrivez en remplaçant le second groupe nominal par un pronom possessif.",
          source:
            "Leur proposition était intéressante, mais notre proposition l'était davantage.",
          answers: [
            "Leur proposition était intéressante, mais la nôtre l'était davantage",
          ],
          rule:
            "« Notre proposition » → la nôtre. Féminin singulier, plusieurs possesseurs (« nous »), accent circonflexe.",
        },
      },
      {
        before: "J'ai déjà parlé de mes idées à l'équipe ; et toi, tu as présenté ",
        hint: "pronom possessif",
        after: " ?",
        answers: ["les tiennes"],
        rule:
          "« Idées » est féminin pluriel ; possesseur « tu » → les tiennes. Accord toujours avec l'objet possédé.",
        consolidation: {
          before: "Elle a obtenu de bons résultats ; et moi, j'attends ",
          hint: "pronom possessif",
          after: " avec une certaine inquiétude.",
          answers: ["les miens"],
        },
      },
      {
        type: "qcm",
        prompt:
          "Laquelle de ces phrases est orthographiée correctement ?",
        context: "",
        options: [
          "Votre voiture est belle ; la votre est ancienne.",
          "Votre voiture est belle ; la vôtre est ancienne.",
          "Vôtre voiture est belle ; la nôtre est ancienne.",
        ],
        correctIndex: 1,
        rule:
          "Adjectif possessif (votre voiture) : pas d'accent. Pronom possessif (la vôtre) : accent circonflexe sur le ô. La confusion adjectif/pronom est une faute d'orthographe très courante au B2.",
        consolidation: {
          type: "qcm",
          prompt: "Quelle est l'orthographe correcte ?",
          context: "",
          options: [
            "Notre maison est en bord de mer ; la nôtre aussi.",
            "Nôtre maison est en bord de mer ; la notre aussi.",
            "Notre maison est en bord de mer ; la notre aussi.",
          ],
          correctIndex: 0,
          rule:
            "Adjectif (notre maison) : sans accent. Pronom (la nôtre) : avec accent circonflexe.",
        },
      },
      {
        before: "Pour que le projet réussisse, il faudra que chacun y mette ",
        hint: "expression figée avec pronom possessif",
        after: ".",
        answers: ["du sien"],
        rule:
          "Expression figée « y mettre du sien » = faire un effort, contribuer. « Du » est la contraction de « de + le ». La forme reste invariable dans cette expression (même si on parle de plusieurs personnes : chacun met du sien).",
        consolidation: {
          before: "Cette vieille machine fait encore ",
          hint: "expression figée",
          after: " ce matin : elle refuse de démarrer.",
          answers: ["des siennes"],
        },
      },
      {
        before: "Ce livre n'est pas à moi : il est à ma sœur, c'est ",
        hint: "pronom possessif",
        after: ".",
        answers: ["le sien"],
        rule:
          "Possesseur : « elle » (ma sœur) → sien. Objet possédé : livre (masculin singulier) → le sien. Attention : le genre du pronom suit l'objet, pas le possesseur.",
        consolidation: {
          before: "Cette voiture n'est pas à mon mari, c'est ",
          hint: "pronom possessif",
          after: ".",
          answers: ["la mienne"],
        },
      },
      {
        type: "transform",
        instruction:
          "Réécrivez en remplaçant le groupe souligné par un pronom possessif. (Le groupe souligné est : « à vos collègues ».)",
        source:
          "Nous comptons présenter nos résultats à nos collègues ; vous pourriez présenter les vôtres à vos collègues.",
        answers: [
          "Nous comptons présenter nos résultats à nos collègues ; vous pourriez présenter les vôtres aux vôtres",
        ],
        rule:
          "« À vos collègues » → « aux vôtres ». Contraction obligatoire de « à + les » → aux, suivi du pronom possessif accordé (collègues : masculin pluriel).",
        consolidation: {
          type: "transform",
          instruction:
            "Réécrivez en remplaçant le groupe « de tes voisins » par un pronom possessif.",
          source:
            "Je m'occupe de mes plantes ; toi, tu pourrais t'occuper de tes voisins.",
          answers: [
            "Je m'occupe de mes plantes ; toi, tu pourrais t'occuper des tiens",
            "Je m'occupe de mes plantes ; toi, tu pourrais t'occuper des tiennes",
          ],
          rule:
            "« De tes voisins » → « des tiens » (avec contraction de + les). Le pronom s'accorde avec « voisins » (masculin pluriel par défaut).",
        },
      },
    ],
  },

  /* ---------- 16. CAUSE ET CONSÉQUENCE ---------- */
  {
    id: "cause-consequence",
    number: 16,
    section: "Logique de la phrase",
    title: "Cause et",
    titleEm: "conséquence",
    lesson: {
      intro:
        "Exprimer la cause, c'est dire pourquoi une chose se produit ; exprimer la conséquence, c'est dire ce qui en résulte. Au B2, on attend de l'apprenant qu'il sorte du duo « parce que » / « donc » et qu'il varie son arsenal selon le registre, le degré de subjectivité et la place de l'information.",
      formation: [
        "Cause + indicatif : parce que, puisque, comme, étant donné que, vu que, sous prétexte que, du fait que.",
        "Cause + nom : à cause de (cause neutre ou négative), grâce à (cause positive), en raison de (formel et neutre), faute de (manque de), à force de (intensité, répétition).",
        "Conséquence + indicatif : si bien que, de sorte que, de telle sorte que, à tel point que, tellement… que, si… que.",
        "Conséquence + subjonctif : trop / assez… pour que (le sujet de la subordonnée est différent de celui de la principale).",
        "Connecteurs entre deux propositions indépendantes : donc, alors, ainsi, par conséquent, c'est pourquoi, du coup (familier).",
      ],
      irregulars: [],
      triggers: [
        {
          title: "Nuances de la cause",
          subcases: [
            {
              label: "Parce que → cause neutre, en réponse à « pourquoi ? »",
              example: "Je suis en retard parce que mon train a été annulé.",
              highlight: "parce que",
            },
            {
              label: "Puisque → cause connue, partagée, qui sert d'argument",
              example: "Puisque tu es là, autant en profiter pour discuter.",
              highlight: "Puisque",
            },
            {
              label: "Comme → cause placée en tête de phrase",
              example: "Comme il pleuvait, nous avons annulé la promenade.",
              highlight: "Comme",
            },
            {
              label: "Étant donné que / Vu que → cause objective, ton soutenu / familier",
              example: "Étant donné que la situation a évolué, nous devons revoir notre stratégie.",
              highlight: "Étant donné que",
            },
          ],
        },
        {
          title: "À cause de / Grâce à / En raison de",
          subcases: [
            {
              label: "À cause de → cause négative ou neutre + nom",
              example: "Le match a été reporté à cause de l'orage.",
              highlight: "à cause de",
            },
            {
              label: "Grâce à → cause positive + nom",
              example: "Grâce à son aide, nous avons terminé à temps.",
              highlight: "Grâce à",
            },
            {
              label: "En raison de → cause neutre, registre formel",
              example: "Les bureaux seront fermés en raison des fêtes de fin d'année.",
              highlight: "en raison de",
            },
          ],
        },
        {
          title: "Conséquence",
          subcases: [
            {
              label: "Si bien que / De sorte que → conséquence factuelle + indicatif",
              example: "Il avait beaucoup travaillé, si bien qu'il a réussi haut la main.",
              highlight: "si bien qu'",
            },
            {
              label: "Tellement / Si… que → intensité + conséquence",
              example: "Elle parle si vite que je n'arrive pas à la suivre.",
              highlight: "si vite que",
            },
            {
              label: "Assez / Trop… pour que → conséquence + subjonctif",
              example: "Il fait trop froid pour que nous puissions sortir dîner en terrasse.",
              highlight: "puissions",
            },
          ],
        },
        {
          title: "Pièges fréquents",
          body:
            "« Car » est synonyme de « parce que » mais ne peut pas commencer une phrase et n'est jamais précédé d'une virgule de la même manière. « En effet » introduit une explication, pas une cause au sens strict. « Du coup » est très familier : à éviter à l'écrit. « Tellement de » + nom (= beaucoup de) : tellement de monde, tellement de bruit. Ne pas confondre « à cause de » (cause) et « parce que » (cause + verbe) : on dit « à cause de la pluie » mais « parce qu'il pleut ».",
        },
      ],
    },
    items: [
      {
        type: "qcm",
        prompt: "Quel connecteur convient le mieux ?",
        context:
          "___ tu connais déjà la réponse, pourquoi me poses-tu la question ?",
        options: ["Parce que", "Puisque", "Comme"],
        correctIndex: 1,
        rule:
          "« Puisque » introduit une cause considérée comme connue ou évidente, et sert souvent d'argument dans un raisonnement. « Parce que » répond à une question ; « comme » introduit une cause placée en tête, mais ici l'idée de « tu le sais déjà » correspond à puisque.",
        consolidation: {
          type: "qcm",
          prompt: "Quel connecteur convient ?",
          context: "___ il pleuvait des cordes, nous avons décidé de reporter la randonnée.",
          options: ["Parce que", "Puisque", "Comme"],
          correctIndex: 2,
          rule:
            "« Comme » s'emploie typiquement en tête de phrase pour introduire la cause avant la conséquence. C'est une construction très naturelle pour ouvrir un récit ou une explication.",
        },
      },
      {
        before: "Le concert a été annulé ",
        hint: "préposition de cause + nom",
        after: " la maladie soudaine de la chanteuse principale.",
        answers: ["à cause de", "en raison de"],
        rule:
          "Devant un nom, on emploie « à cause de » (cause neutre ou négative) ou « en raison de » (registre formel). On ne peut pas employer « parce que » + nom : « parce que » introduit obligatoirement une proposition avec un verbe conjugué.",
        consolidation: {
          before: "L'autoroute est fermée ",
          hint: "préposition de cause",
          after: " un grave accident survenu ce matin.",
          answers: ["à cause d'", "en raison d'"],
        },
      },
      {
        type: "transform",
        instruction:
          "Réécrivez la phrase en exprimant la cause avec « grâce à » + nom.",
        source:
          "Nous avons obtenu ce contrat parce que notre équipe a fait preuve d'une grande persévérance.",
        answers: [
          "Nous avons obtenu ce contrat grâce à la persévérance de notre équipe",
          "Grâce à la persévérance de notre équipe, nous avons obtenu ce contrat",
        ],
        rule:
          "« Parce que » + proposition verbale devient « grâce à » + nom. « Grâce à » marque une cause positive : le résultat est favorable. La transformation oblige à nominaliser le verbe (a fait preuve de persévérance → la persévérance).",
        consolidation: {
          type: "transform",
          instruction:
            "Réécrivez en exprimant la cause avec « grâce à ».",
          source:
            "Elle a réussi son examen parce que son professeur l'a beaucoup aidée.",
          answers: [
            "Elle a réussi son examen grâce à l'aide de son professeur",
            "Grâce à l'aide de son professeur, elle a réussi son examen",
          ],
          rule:
            "« L'a beaucoup aidée » se nominalise en « l'aide de son professeur ». Cause positive → grâce à.",
        },
      },
      {
        before: "Il avait tellement insisté ",
        hint: "conjonction de conséquence",
        after: " j'ai fini par accepter sa proposition malgré mes réticences.",
        answers: ["que", "qu'"],
        rule:
          "« Tellement… que » exprime une conséquence liée à l'intensité d'une action ou d'une qualité. La structure est : verbe + tellement + (que + conséquence).",
        consolidation: {
          before: "Le bruit était si assourdissant ",
          hint: "conjonction de conséquence",
          after: " personne ne s'entendait plus parler.",
          answers: ["que", "qu'"],
        },
      },
      {
        type: "qcm",
        prompt: "Quelle construction convient pour exprimer la conséquence ?",
        context:
          "Le dossier est trop complexe ___ nous puissions le boucler avant vendredi.",
        options: ["pour qu'", "pour que", "si bien que"],
        correctIndex: 1,
        rule:
          "« Trop… pour que » + subjonctif exprime une conséquence empêchée par excès. Le verbe « puissions » est au subjonctif présent, déclenché par « pour que ». « Si bien que » prendrait l'indicatif et un sens factuel.",
        consolidation: {
          type: "qcm",
          prompt: "Quelle construction convient ?",
          context:
            "Il a assez d'expérience ___ on lui confie cette mission.",
          options: ["pour que", "si bien que", "parce que"],
          correctIndex: 0,
          rule:
            "« Assez… pour que » + subjonctif exprime une conséquence rendue possible par une qualité suffisante. Construction symétrique à « trop… pour que ».",
        },
      },
      {
        before: "",
        hint: "connecteur de cause en tête de phrase",
        after: " vous insistez tant, je vais finir par céder.",
        answers: ["Puisque", "Comme"],
        rule:
          "En tête de phrase, devant une cause connue ou évidente, « puisque » ou « comme » conviennent. Les deux marquent une cause prise comme acquise. « Parce que » est rare en tête de phrase (sauf en réponse directe à « pourquoi ? »).",
        consolidation: {
          before: "",
          hint: "connecteur de cause en tête",
          after: " je n'avais pas la clé, j'ai dû attendre près d'une heure dans le froid.",
          answers: ["Comme"],
        },
      },
      {
        type: "transform",
        instruction:
          "Réécrivez la phrase en exprimant la conséquence avec « si bien que ».",
        source:
          "Elle a beaucoup travaillé. Elle a obtenu une promotion en moins d'un an.",
        answers: [
          "Elle a beaucoup travaillé, si bien qu'elle a obtenu une promotion en moins d'un an",
        ],
        rule:
          "« Si bien que » + indicatif relie deux faits dans une relation de conséquence factuelle. La virgule devant « si bien que » est habituelle.",
        consolidation: {
          type: "transform",
          instruction:
            "Reliez les deux phrases avec « de sorte que ».",
          source:
            "Le brouillard était dense. Tous les vols ont été retardés.",
          answers: [
            "Le brouillard était dense, de sorte que tous les vols ont été retardés",
          ],
          rule:
            "« De sorte que » + indicatif = conséquence factuelle. Quasi-synonyme de « si bien que », légèrement plus soutenu.",
        },
      },
      {
        before: "Faute ",
        hint: "préposition + nom",
        after: " préparation suffisante, nous avons échoué dès la première épreuve.",
        answers: ["de"],
        rule:
          "« Faute de » + nom exprime une cause négative liée à un manque. C'est une construction soutenue, fréquente à l'écrit. Synonyme moins formel : « par manque de ».",
        consolidation: {
          before: "À force ",
          hint: "préposition + nom ou infinitif",
          after: " répéter les mêmes erreurs, il finira par tout perdre.",
          answers: ["de"],
        },
      },
    ],
  },

  /* ---------- 17. BUT ---------- */
  {
    id: "but",
    number: 17,
    section: "Logique de la phrase",
    title: "L'expression du",
    titleEm: "but",
    lesson: {
      intro:
        "Le but, c'est l'objectif visé par une action. Trois variables déterminent le choix de la tournure : le sujet du but (le même que celui de la principale, ou un autre ?), le ton (positif ou négatif), et le registre (courant ou soutenu). Maîtriser ces variables, c'est gagner en précision et en élégance.",
      formation: [
        "Même sujet → préposition + infinitif : pour, afin de (soutenu), de manière à, de façon à, en vue de, dans le but de. Ex. : Je pars tôt pour éviter les embouteillages.",
        "Sujets différents → conjonction + subjonctif : pour que, afin que, de manière que, de façon que, de sorte que. Ex. : Je t'explique tout afin que tu comprennes bien.",
        "But négatif (pour éviter que) → de peur que, de crainte que, de peur de, de crainte de + ne explétif possible. Ex. : Parle bas de peur qu'on ne nous entende.",
        "Avec un nom → pour, en vue de, dans le but de + nom. Ex. : Il économise pour ses prochaines vacances.",
      ],
      irregulars: [],
      triggers: [
        {
          title: "Même sujet : préposition + infinitif",
          subcases: [
            {
              label: "Pour + infinitif → courant, polyvalent",
              example: "Je me lève tôt pour profiter pleinement de la matinée.",
              highlight: "pour profiter",
            },
            {
              label: "Afin de + infinitif → registre soutenu",
              example: "Le ministre a pris la parole afin de clarifier sa position.",
              highlight: "afin de clarifier",
            },
            {
              label: "De manière à / De façon à → idée de moyen vers un but",
              example: "Range tes affaires de manière à les retrouver facilement.",
              highlight: "de manière à les retrouver",
            },
          ],
        },
        {
          title: "Sujets différents : conjonction + subjonctif",
          subcases: [
            {
              label: "Pour que + subjonctif → courant",
              example: "Le professeur répète plusieurs fois pour que chacun comprenne.",
              highlight: "comprenne",
            },
            {
              label: "Afin que + subjonctif → soutenu, formel",
              example: "Nous publions ce communiqué afin que la situation soit clarifiée.",
              highlight: "soit",
            },
            {
              label: "De sorte que / De manière que + subjonctif → but visé (mais indicatif si conséquence)",
              example: "Parlez plus fort, de sorte que tout le monde vous entende.",
              highlight: "entende",
            },
          ],
        },
        {
          title: "But négatif : éviter, prévenir",
          subcases: [
            {
              label: "De peur de / De crainte de + infinitif → même sujet",
              example: "Il a quitté la salle discrètement de peur de déranger.",
              highlight: "de peur de déranger",
            },
            {
              label: "De peur que / De crainte que + subjonctif → sujets différents",
              example: "J'ai parlé bas de peur qu'on ne m'entende de la pièce voisine.",
              highlight: "n'entende",
            },
            {
              label: "Pour éviter que + subjonctif → tournure plus naturelle à l'oral",
              example: "Mets une alarme pour éviter que tu n'oublies le rendez-vous.",
              highlight: "n'oublies",
            },
          ],
        },
        {
          title: "Pièges fréquents",
          body:
            "Erreur classique : « pour que + même sujet ». Si le sujet est le même, il faut un infinitif : « je travaille pour réussir », jamais « je travaille pour que je réussisse ». Avec « de sorte que », attention : + subjonctif → but visé ; + indicatif → conséquence constatée. « De peur que » et « de crainte que » admettent un « ne » explétif sans valeur négative (de peur qu'il ne vienne = j'ai peur qu'il vienne).",
        },
      ],
    },
    items: [
      {
        before: "Je prends toujours mes notes au crayon, afin de ",
        hint: "verbe à l'infinitif",
        after: " les modifier facilement si besoin.",
        answers: ["pouvoir"],
        rule:
          "Même sujet (je) → infinitif après « afin de ». Le sens « pour avoir la possibilité de modifier » → pouvoir + infinitif.",
        consolidation: {
          before: "Elle est partie en avance afin de ",
          hint: "verbe à l'infinitif",
          after: " la circulation aux heures de pointe.",
          answers: ["éviter"],
        },
      },
      {
        type: "qcm",
        prompt:
          "Laquelle de ces phrases est correctement construite ?",
        context: "",
        options: [
          "Je travaille tard le soir pour que je termine ce projet à temps.",
          "Je travaille tard le soir pour terminer ce projet à temps.",
          "Je travaille tard le soir afin que je termine ce projet à temps.",
        ],
        correctIndex: 1,
        rule:
          "Erreur très fréquente : avec un même sujet, on emploie obligatoirement « pour / afin de » + infinitif, jamais « pour que / afin que » + verbe conjugué. La forme correcte est donc « pour terminer ».",
        consolidation: {
          type: "qcm",
          prompt: "Laquelle est correcte ?",
          context: "",
          options: [
            "Il étudie chaque soir afin qu'il réussisse son concours.",
            "Il étudie chaque soir afin de réussir son concours.",
            "Il étudie chaque soir pour qu'il réussisse son concours.",
          ],
          correctIndex: 1,
          rule:
            "Même règle : sujet identique → « afin de » + infinitif, pas « afin qu'il » (qui suppose un sujet différent).",
        },
      },
      {
        before: "Le directeur a convoqué toute l'équipe pour que chacun ",
        hint: "comprendre",
        after: " bien les nouveaux objectifs de l'année.",
        answers: ["comprenne"],
        rule:
          "Sujets différents (le directeur / chacun) → « pour que » + subjonctif. « Comprendre » au subjonctif présent : que je comprenne, qu'il comprenne.",
        consolidation: {
          before: "J'ai tout expliqué en détail afin que vous ",
          hint: "pouvoir",
          after: " prendre une décision éclairée.",
          answers: ["puissiez"],
        },
      },
      {
        type: "transform",
        instruction:
          "Réécrivez la phrase en utilisant « pour que » au lieu de « pour » + infinitif. (Le sujet de la subordonnée sera « les enfants ».)",
        source:
          "Les parents lisent une histoire chaque soir pour aider les enfants à s'endormir.",
        answers: [
          "Les parents lisent une histoire chaque soir pour que les enfants s'endorment",
        ],
        rule:
          "Avec deux sujets différents (les parents / les enfants), on passe à « pour que » + subjonctif. « Aider X à s'endormir » se reformule en « pour que X s'endorme » (transformation très idiomatique).",
        consolidation: {
          type: "transform",
          instruction:
            "Réécrivez en utilisant « pour que » et « tu » comme sujet de la subordonnée.",
          source:
            "Je t'envoie ces documents pour faciliter ta préparation.",
          answers: [
            "Je t'envoie ces documents pour que tu prépares facilement",
            "Je t'envoie ces documents pour que tu te prépares facilement",
          ],
          rule:
            "Sujets différents → pour que + subjonctif. « Faciliter ta préparation » se reformule avec un verbe conjugué.",
        },
      },
      {
        before: "J'ai parlé à voix basse de peur que les voisins ne nous ",
        hint: "entendre",
        after: " à travers les cloisons.",
        answers: ["entendent"],
        rule:
          "But négatif (j'évite qu'ils entendent) → « de peur que » + subjonctif. Le « ne » qui suit est explétif (sans valeur négative). Au subjonctif : qu'ils entendent.",
        consolidation: {
          before: "Marche doucement de crainte que tu ne ",
          hint: "réveiller",
          after: " le bébé qui vient de s'endormir.",
          answers: ["réveilles"],
        },
      },
      {
        type: "qcm",
        prompt: "Quelle construction convient ici ?",
        context:
          "Il a tout vérifié trois fois ___ ne rien laisser au hasard.",
        options: ["de peur de", "de peur qu'il", "pour qu'il"],
        correctIndex: 0,
        rule:
          "Même sujet (il) + but négatif → « de peur de » + infinitif. La conjonction « de peur que » exigerait un sujet différent dans la subordonnée.",
        consolidation: {
          type: "qcm",
          prompt: "Quelle construction convient ?",
          context: "Elle a téléphoné dès son arrivée ___ inquiéter sa famille.",
          options: ["pour ne pas", "pour qu'elle ne", "afin qu'elle ne"],
          correctIndex: 0,
          rule:
            "Même sujet (elle) → infinitif. La négation se place en bloc devant l'infinitif : « pour ne pas inquiéter ».",
        },
      },
      {
        before: "Le gouvernement a engagé ces réformes en vue ",
        hint: "préposition + nom",
        after: " la croissance économique du pays.",
        answers: ["de"],
        rule:
          "« En vue de » + nom (ou infinitif) exprime un but souvent à long terme, dans un registre formel. La préposition est invariablement « de ».",
        consolidation: {
          before: "Ces mesures ont été prises dans le but ",
          hint: "préposition + infinitif",
          after: " protéger les espèces menacées.",
          answers: ["de"],
        },
      },
      {
        type: "transform",
        instruction:
          "Réécrivez la phrase en remplaçant « pour » par « de manière à » (même sens, registre légèrement plus soutenu).",
        source:
          "Range le dossier au bon endroit pour le retrouver facilement la prochaine fois.",
        answers: [
          "Range le dossier au bon endroit de manière à le retrouver facilement la prochaine fois",
          "Range le dossier au bon endroit de façon à le retrouver facilement la prochaine fois",
        ],
        rule:
          "« De manière à » et « de façon à » + infinitif sont des synonymes de « pour » avec une nuance supplémentaire de moyen : la manière dont on agit conduit au but visé.",
        consolidation: {
          type: "transform",
          instruction:
            "Réécrivez en utilisant « de façon à » au lieu de « pour ».",
          source:
            "Parle plus lentement pour que tous les élèves te comprennent.",
          answers: [
            "Parle plus lentement de façon à ce que tous les élèves te comprennent",
            "Parle plus lentement de façon que tous les élèves te comprennent",
          ],
          rule:
            "Avec deux sujets différents, « de façon que » + subjonctif (ou la variante courante « de façon à ce que »). Forme symétrique à « pour que ».",
        },
      },
    ],
  },

  /* ---------- 18. OPPOSITION ET CONCESSION ---------- */
  {
    id: "opposition-concession",
    number: 18,
    section: "Logique de la phrase",
    title: "Opposition et",
    titleEm: "concession",
    lesson: {
      intro:
        "Deux notions à distinguer. L'opposition met côte à côte deux faits différents, sans rapport logique attendu : « lui aime le sucré, elle préfère le salé ». La concession admet un fait qui aurait dû empêcher un autre fait : « bien qu'il fasse froid, je sors » (le froid aurait dû me retenir). Le B2 demande de varier les connecteurs et de choisir le mode (indicatif ou subjonctif) selon la conjonction.",
      formation: [
        "Concession + subjonctif : bien que, quoique, encore que, où que, qui que, quel que (suivi du verbe être).",
        "Concession + indicatif : même si (jamais de subjonctif après « même si » !).",
        "Concession + nom : malgré, en dépit de.",
        "Opposition + indicatif : alors que, tandis que, pendant que (quand il oppose).",
        "Connecteurs entre indépendantes : cependant, néanmoins, pourtant, toutefois, en revanche, par contre (familier), or, mais.",
        "Concession avec « avoir beau » : avoir beau + infinitif = même si on… (avoir beau essayer = même si on essaie).",
      ],
      irregulars: [],
      triggers: [
        {
          title: "Concession (admettre un fait qui aurait dû empêcher l'autre)",
          subcases: [
            {
              label: "Bien que / Quoique + subjonctif",
              example: "Bien qu'elle soit très occupée, elle trouve toujours du temps pour ses amis.",
              highlight: "soit",
            },
            {
              label: "Malgré / En dépit de + nom",
              example: "Malgré la pluie, ils ont décidé de partir en randonnée.",
              highlight: "Malgré",
            },
            {
              label: "Même si + indicatif (jamais de subjonctif !)",
              example: "Même si tu insistes, je ne changerai pas d'avis sur ce point.",
              highlight: "insistes",
            },
            {
              label: "Avoir beau + infinitif",
              example: "J'ai beau lui répéter les consignes, il continue à les ignorer.",
              highlight: "ai beau lui répéter",
            },
          ],
        },
        {
          title: "Opposition (mettre côte à côte sans rapport de cause)",
          subcases: [
            {
              label: "Alors que / Tandis que + indicatif",
              example: "Il adore la mer, alors que sa sœur préfère la montagne.",
              highlight: "alors que",
            },
            {
              label: "En revanche, par contre → connecteurs entre phrases",
              example: "Le matin, je suis très productif ; en revanche, l'après-midi, c'est plus difficile.",
              highlight: "en revanche",
            },
            {
              label: "Au lieu de + infinitif",
              example: "Au lieu de te plaindre, propose une solution concrète.",
              highlight: "Au lieu de te plaindre",
            },
          ],
        },
        {
          title: "Tournures avec « quel que », « où que », « qui que »",
          subcases: [
            {
              label: "Quel que soit / quelle que soit + sujet (= peu importe)",
              example: "Quelle que soit ta décision, je la respecterai.",
              highlight: "Quelle que soit",
            },
            {
              label: "Où que + subjonctif (= en quelque lieu que)",
              example: "Où que tu ailles, je te suivrai.",
              highlight: "Où que tu ailles",
            },
            {
              label: "Qui que ce soit / quoi que ce soit (= n'importe qui / n'importe quoi)",
              example: "Quoi que tu fasses, je serai fier de toi.",
              highlight: "Quoi que tu fasses",
            },
          ],
        },
        {
          title: "Pièges fréquents",
          body:
            "Erreur fréquente : « même si » + subjonctif. C'est faux : « même si » prend toujours l'indicatif. Ne pas confondre « quoique » (= bien que, subjonctif) et « quoi que » en deux mots (= quelle que soit la chose, subjonctif aussi mais sens différent). « Par contre » est très courant à l'oral mais souvent jugé incorrect par les puristes : préférer « en revanche » à l'écrit soigné. Avec « avoir beau », le verbe « avoir » se conjugue normalement : j'avais beau, nous aurons beau, etc.",
        },
      ],
    },
    items: [
      {
        before: "Bien qu'elle ",
        hint: "savoir",
        after: " la vérité depuis longtemps, elle n'a rien dit à personne.",
        answers: ["sache"],
        rule:
          "« Bien que » + subjonctif (toujours). « Savoir » au subjonctif présent : que je sache, qu'elle sache.",
        consolidation: {
          before: "Quoique le projet ",
          hint: "être",
          after: " ambitieux, l'équipe garde confiance.",
          answers: ["soit"],
        },
      },
      {
        type: "qcm",
        prompt:
          "Laquelle de ces phrases est correctement construite ?",
        context: "",
        options: [
          "Même si tu m'aies prévenu, je n'aurais rien pu faire.",
          "Même si tu m'avais prévenu, je n'aurais rien pu faire.",
          "Même si tu m'aurais prévenu, je n'aurais rien pu faire.",
        ],
        correctIndex: 1,
        rule:
          "« Même si » prend toujours l'indicatif, jamais le subjonctif. Et jamais de conditionnel non plus dans la subordonnée. Ici : si + plus-que-parfait (avais prévenu), conditionnel passé dans la principale (aurais pu).",
        consolidation: {
          type: "qcm",
          prompt: "Laquelle est correcte ?",
          context: "",
          options: [
            "Même s'il pleut demain, nous irons faire la randonnée.",
            "Même s'il pleuve demain, nous irons faire la randonnée.",
            "Même s'il plût demain, nous irons faire la randonnée.",
          ],
          correctIndex: 0,
          rule:
            "« Même si » + présent de l'indicatif. Pas de subjonctif après « même si », jamais.",
        },
      },
      {
        type: "transform",
        instruction:
          "Réécrivez la phrase en utilisant « malgré » + nom à la place de « bien que ».",
        source:
          "Bien qu'il soit très fatigué, il a accepté de nous accompagner.",
        answers: [
          "Malgré sa fatigue, il a accepté de nous accompagner",
          "Malgré sa grande fatigue, il a accepté de nous accompagner",
        ],
        rule:
          "« Bien que » + verbe se transforme en « malgré » + nom. L'adjectif « fatigué » devient le nom « fatigue ». Construction très utile pour alléger une phrase à l'écrit.",
        consolidation: {
          type: "transform",
          instruction:
            "Réécrivez avec « malgré » + nom.",
          source:
            "Bien que la situation soit difficile, l'équipe garde le moral.",
          answers: [
            "Malgré la difficulté de la situation, l'équipe garde le moral",
            "Malgré une situation difficile, l'équipe garde le moral",
          ],
          rule:
            "« La situation soit difficile » devient « la difficulté de la situation » (nominalisation de l'adjectif).",
        },
      },
      {
        before: "Lui adore la grasse matinée, ",
        hint: "conjonction d'opposition",
        after: " sa femme se lève systématiquement à six heures.",
        answers: ["alors que", "tandis que"],
        rule:
          "Opposition (deux comportements différents) → alors que / tandis que + indicatif. Ces deux conjonctions sont quasi-synonymes ; « tandis que » est légèrement plus soutenu.",
        consolidation: {
          before: "Ma sœur est très extravertie, ",
          hint: "conjonction d'opposition",
          after: " moi je préfère la tranquillité.",
          answers: ["alors que", "tandis que"],
        },
      },
      {
        before: "J'ai beau ",
        hint: "verbe à l'infinitif",
        after: " tous les jours, je n'arrive toujours pas à parler couramment.",
        answers: ["étudier", "travailler"],
        rule:
          "« Avoir beau » + infinitif = même si on essaie / fait, le résultat n'est pas là. Construction typiquement française, très naturelle à l'écrit comme à l'oral. Le verbe « avoir » se conjugue normalement.",
        consolidation: {
          before: "Nous avons eu beau ",
          hint: "verbe à l'infinitif",
          after: " plusieurs fois, personne ne nous a répondu.",
          answers: ["insister", "appeler"],
        },
      },
      {
        type: "qcm",
        prompt: "Quel connecteur convient le mieux ?",
        context:
          "L'équipe a perdu le match. ___, elle reste en tête du classement.",
        options: ["Cependant", "Parce que", "C'est pourquoi"],
        correctIndex: 0,
        rule:
          "« Cependant » exprime une opposition ou concession entre deux phrases : on s'attendrait à ce que perdre fasse descendre au classement, mais ce n'est pas le cas. Synonymes : néanmoins, toutefois, pourtant.",
        consolidation: {
          type: "qcm",
          prompt: "Quel connecteur convient ?",
          context:
            "Il n'a presque pas révisé. ___, il a réussi brillamment son examen.",
          options: ["Donc", "Pourtant", "Ainsi"],
          correctIndex: 1,
          rule:
            "« Pourtant » exprime un fait contraire à l'attente : ne pas réviser aurait dû mener à l'échec.",
        },
      },
      {
        before: "",
        hint: "tournure de concession généralisante",
        after: " soient vos motivations, vous devez respecter le règlement.",
        answers: ["Quelles que"],
        rule:
          "« Quel que / quelle que / quels que / quelles que » + être au subjonctif + sujet = peu importe… L'accord se fait avec le sujet qui suit (motivations, féminin pluriel → quelles que).",
        consolidation: {
          before: "",
          hint: "tournure de concession généralisante",
          after: " soit le résultat, nous aurons beaucoup appris.",
          answers: ["Quel que"],
        },
      },
      {
        type: "transform",
        instruction:
          "Réécrivez la phrase en utilisant « avoir beau » + infinitif.",
        source:
          "Même s'il essaie sans cesse, il n'arrive pas à convaincre son interlocuteur.",
        answers: [
          "Il a beau essayer sans cesse, il n'arrive pas à convaincre son interlocuteur",
        ],
        rule:
          "« Même si » + verbe conjugué se transforme souvent en « avoir beau » + infinitif. La construction est plus dense et plus expressive en français contemporain.",
        consolidation: {
          type: "transform",
          instruction:
            "Réécrivez avec « avoir beau » + infinitif.",
          source:
            "Même si elle me prévient des dizaines de fois, j'oublie toujours nos rendez-vous.",
          answers: [
            "Elle a beau me prévenir des dizaines de fois, j'oublie toujours nos rendez-vous",
          ],
          rule:
            "Sujets différents (elle / je) : « avoir beau » se conjugue à la personne du sujet de la subordonnée concessive.",
        },
      },
    ],
  },

  /* ---------- 19. TEMPS (CONJONCTIONS) ---------- */
  {
    id: "temps-conjonctions",
    number: 19,
    section: "Logique de la phrase",
    title: "L'expression du",
    titleEm: "temps",
    lesson: {
      intro:
        "Les conjonctions de temps relient deux actions dans une chronologie : avant, après, pendant, depuis, jusqu'à ce que. Le choix du mode (indicatif ou subjonctif) dépend de la conjonction, et certaines formes très fréquentes au B2 — comme « après que » suivi de l'indicatif ou la valeur précise de « depuis que » — sont des points-clés à maîtriser.",
      formation: [
        "Antériorité (la subordonnée arrive avant la principale) + subjonctif : avant que, en attendant que, jusqu'à ce que.",
        "Postériorité (la subordonnée arrive après la principale) + indicatif : après que, une fois que, dès que, aussitôt que.",
        "Simultanéité + indicatif : pendant que, tandis que, alors que, au moment où, comme.",
        "Durée + indicatif : depuis que, tant que, aussi longtemps que.",
        "Avec un nom : avant, après, pendant, depuis, dès, jusqu'à, lors de, au cours de.",
        "Avec un infinitif (même sujet) : avant de, après (+ infinitif passé obligatoirement), au moment de, en attendant de.",
      ],
      irregulars: [],
      triggers: [
        {
          title: "Antériorité (subjonctif)",
          subcases: [
            {
              label: "Avant que + subjonctif (souvent avec ne explétif)",
              example: "Préviens-moi avant que tu ne partes en voyage.",
              highlight: "partes",
            },
            {
              label: "En attendant que + subjonctif",
              example: "Patientez en attendant que le médecin soit disponible.",
              highlight: "soit",
            },
            {
              label: "Jusqu'à ce que + subjonctif",
              example: "Continuez les recherches jusqu'à ce que vous trouviez une réponse.",
              highlight: "trouviez",
            },
          ],
        },
        {
          title: "Postériorité (indicatif)",
          subcases: [
            {
              label: "Après que + indicatif (et non subjonctif, malgré l'usage répandu)",
              example: "Après que la pluie a cessé, les enfants sont sortis jouer.",
              highlight: "a cessé",
            },
            {
              label: "Dès que / Aussitôt que + indicatif",
              example: "Préviens-moi dès que tu arrives à l'aéroport.",
              highlight: "arrives",
            },
            {
              label: "Une fois que + indicatif",
              example: "Une fois que tu auras terminé, nous pourrons relire ensemble.",
              highlight: "auras terminé",
            },
          ],
        },
        {
          title: "Simultanéité et durée",
          subcases: [
            {
              label: "Pendant que → action qui se déroule en même temps",
              example: "J'ai relu mes notes pendant que tu téléphonais.",
              highlight: "pendant que",
            },
            {
              label: "Depuis que → durée à partir d'un point passé",
              example: "Depuis qu'il a déménagé, on ne le voit presque plus.",
              highlight: "Depuis qu'",
            },
            {
              label: "Tant que / Aussi longtemps que → durée d'une condition continue",
              example: "Tant que tu travailleras sérieusement, tu progresseras.",
              highlight: "Tant que",
            },
            {
              label: "Au moment où → instant précis",
              example: "Le téléphone a sonné au moment où je sortais de la maison.",
              highlight: "au moment où",
            },
          ],
        },
        {
          title: "Pièges fréquents",
          body:
            "L'erreur la plus fréquente au B2 : « après que » + subjonctif. La règle officielle veut l'indicatif, par symétrie avec « avant que » (subjonctif). On dit donc « après qu'il est arrivé », non « après qu'il soit arrivé », même si cette dernière forme est extrêmement répandue. « Avant que » et « en attendant que » admettent un « ne » explétif (sans valeur négative). Avec « après » + infinitif, on emploie obligatoirement l'infinitif passé : après avoir terminé, après être sorti.",
        },
      ],
    },
    items: [
      {
        before: "Préviens-moi avant que tu ne ",
        hint: "partir",
        after: " pour ton long voyage.",
        answers: ["partes"],
        rule:
          "« Avant que » + subjonctif (toujours). Le « ne » explétif est facultatif mais courant. Partir au subjonctif présent : que je parte, que tu partes.",
        consolidation: {
          before: "Finissons ce travail avant qu'il ne ",
          hint: "être",
          after: " trop tard pour rendre le dossier.",
          answers: ["soit"],
        },
      },
      {
        type: "qcm",
        prompt:
          "Laquelle de ces phrases est officiellement correcte ?",
        context: "",
        options: [
          "Après qu'il soit arrivé, nous avons commencé la réunion.",
          "Après qu'il est arrivé, nous avons commencé la réunion.",
          "Après qu'il fût arrivé, nous avons commencé la réunion.",
        ],
        correctIndex: 1,
        rule:
          "Règle officielle : « après que » + indicatif, par symétrie avec « avant que » (subjonctif). Le passé composé « il est arrivé » convient. La forme avec subjonctif (« soit arrivé ») est extrêmement répandue, mais reste considérée comme incorrecte par l'Académie.",
        consolidation: {
          type: "qcm",
          prompt: "Quelle phrase est officiellement correcte ?",
          context: "",
          options: [
            "Une fois qu'elle aura terminé, nous relirons ensemble.",
            "Une fois qu'elle ait terminé, nous relirons ensemble.",
            "Une fois qu'elle terminera, nous relirons ensemble.",
          ],
          correctIndex: 0,
          rule:
            "« Une fois que » + indicatif, comme « après que ». Avec une action future et achevée → futur antérieur (aura terminé).",
        },
      },
      {
        before: "Patientez en attendant que le médecin ",
        hint: "être",
        after: " disponible pour vous recevoir.",
        answers: ["soit"],
        rule:
          "« En attendant que » + subjonctif. Être au subjonctif : que je sois, qu'il soit.",
        consolidation: {
          before: "Continue à arroser le jardin jusqu'à ce qu'il ",
          hint: "pleuvoir",
          after: " enfin de manière significative.",
          answers: ["pleuve"],
        },
      },
      {
        type: "transform",
        instruction:
          "Réécrivez la phrase en utilisant « après » + infinitif passé.",
        source:
          "Quand il a terminé son discours, il a quitté immédiatement la scène.",
        answers: [
          "Après avoir terminé son discours, il a quitté immédiatement la scène",
        ],
        rule:
          "Même sujet (il) → « après » + infinitif passé. Forme obligatoire : avoir/être à l'infinitif + participe passé. « Terminer » → avoir terminé.",
        consolidation: {
          type: "transform",
          instruction:
            "Réécrivez la phrase en utilisant « après » + infinitif passé.",
          source:
            "Dès qu'elle est rentrée à la maison, elle s'est jetée sur son lit.",
          answers: [
            "Après être rentrée à la maison, elle s'est jetée sur son lit",
          ],
          rule:
            "Verbe « rentrer » conjugué avec être → être rentrée à l'infinitif passé. Accord du participe avec le sujet (elle).",
        },
      },
      {
        before: "Depuis qu'il ",
        hint: "déménager",
        after: " à Lyon, on ne le voit presque plus.",
        answers: ["a déménagé"],
        rule:
          "« Depuis que » + indicatif. Le déménagement est un événement ponctuel passé → passé composé (a déménagé). La conséquence (on ne le voit plus) est au présent.",
        consolidation: {
          before: "Depuis que tu ",
          hint: "commencer",
          after: " ce nouveau travail, tu sembles épanoui.",
          answers: ["as commencé"],
        },
      },
      {
        type: "qcm",
        prompt: "Quelle conjonction convient le mieux ici ?",
        context:
          "___ tu continueras à pratiquer chaque jour, tu progresseras rapidement.",
        options: ["Dès que", "Tant que", "Avant que"],
        correctIndex: 1,
        rule:
          "« Tant que » exprime une durée conditionnée à une continuité (= aussi longtemps que). « Dès que » marquerait un déclenchement immédiat ; « avant que » l'antériorité avec subjonctif.",
        consolidation: {
          type: "qcm",
          prompt: "Quelle conjonction convient ?",
          context:
            "___ tu auras pris ta décision, fais-le-moi savoir.",
          options: ["Pendant que", "Avant que", "Dès que"],
          correctIndex: 2,
          rule:
            "« Dès que » + futur antérieur exprime l'immédiateté à partir d'un moment futur. Construction : dès que + auras pris (futur antérieur), fais-moi savoir (impératif).",
        },
      },
      {
        before: "Le téléphone a sonné au moment où je ",
        hint: "sortir",
        after: " de la salle de réunion.",
        answers: ["sortais"],
        rule:
          "« Au moment où » + indicatif. Action en cours dans le passé → imparfait (je sortais). L'action ponctuelle (a sonné) interrompt l'action en cours.",
        consolidation: {
          before: "Elle est arrivée juste au moment où nous ",
          hint: "partir",
          after: ".",
          answers: ["partions"],
        },
      },
      {
        type: "transform",
        instruction:
          "Réécrivez la phrase en remplaçant « avant que » par « avant de » + infinitif. (Le sujet sera le même dans les deux parties.)",
        source:
          "Réfléchis bien avant que tu ne signes ce contrat important.",
        answers: [
          "Réfléchis bien avant de signer ce contrat important",
        ],
        rule:
          "Quand le sujet est le même dans les deux propositions, on emploie obligatoirement « avant de » + infinitif (et non « avant que » + subjonctif). C'est la règle classique d'évitement du sujet redondant.",
        consolidation: {
          type: "transform",
          instruction:
            "Réécrivez avec « avant de » + infinitif.",
          source:
            "Lave-toi soigneusement les mains avant que tu ne passes à table.",
          answers: [
            "Lave-toi soigneusement les mains avant de passer à table",
          ],
          rule:
            "Même sujet (tu) → avant de + infinitif. Tournure plus naturelle et plus économique.",
        },
      },
    ],
  },

  /* ---------- 20. COMPARAISON ET SUPERLATIF ---------- */
  {
    id: "comparaison",
    number: 20,
    section: "Logique de la phrase",
    title: "Comparaison et",
    titleEm: "superlatif",
    lesson: {
      intro:
        "Exprimer une comparaison ou un classement est essentiel pour décrire, argumenter, évaluer. Au B2, l'enjeu est de manier précisément le comparatif (supériorité, infériorité, égalité), de gérer les formes irrégulières (meilleur, pire, mieux), et de construire le superlatif avec l'article et la préposition « de » sans hésitation.",
      formation: [
        "Comparatif d'un adjectif ou d'un adverbe : plus / moins / aussi + adj/adv + que. Ex. : plus grand que, moins vite que, aussi intelligent que.",
        "Comparatif d'un nom : plus de / moins de / autant de + nom + que. Ex. : plus de patience que toi, autant d'amis qu'avant.",
        "Comparatif d'un verbe : verbe + plus / moins / autant + que. Ex. : Je travaille plus que toi, elle dort autant que lui.",
        "Superlatif d'un adjectif : le/la/les + plus/moins + adjectif + (de + groupe). Ex. : le plus rapide de la classe.",
        "Formes irrégulières : bon → meilleur (comparatif), le meilleur (superlatif). Bien → mieux (comparatif), le mieux (superlatif). Mauvais → pire ou plus mauvais. Mal → pire ou plus mal.",
      ],
      irregulars: [
        { v: "bon", forms: "meilleur(e)(s) · le meilleur, la meilleure, les meilleur(e)s" },
        { v: "bien", forms: "mieux · le mieux (toujours invariable)" },
        { v: "mauvais", forms: "pire ou plus mauvais · le pire, la pire, les pires" },
        { v: "mal", forms: "pire ou plus mal · le pire (rare comme adverbe)" },
        { v: "petit", forms: "plus petit (taille) · moindre (importance) · le moindre" },
      ],
      triggers: [
        {
          title: "Comparatif de supériorité, infériorité, égalité",
          subcases: [
            {
              label: "Supériorité : plus… que",
              example: "Ce roman est plus captivant que le précédent.",
              highlight: "plus captivant que",
            },
            {
              label: "Infériorité : moins… que",
              example: "Il est moins patient que sa sœur.",
              highlight: "moins patient que",
            },
            {
              label: "Égalité : aussi… que (adj/adv) ou autant que (verbe/nom)",
              example: "Elle parle aussi bien français qu'anglais.",
              highlight: "aussi bien",
            },
          ],
        },
        {
          title: "Formes irrégulières à connaître",
          subcases: [
            {
              label: "Bon → meilleur (adjectif, s'accorde)",
              example: "Cette boulangerie fait le meilleur pain du quartier.",
              highlight: "le meilleur",
            },
            {
              label: "Bien → mieux (adverbe, invariable)",
              example: "Il chante mieux que moi, c'est indéniable.",
              highlight: "mieux",
            },
            {
              label: "Mauvais → pire (insiste sur la gravité, plus expressif que « plus mauvais »)",
              example: "Cette décision est encore pire que la précédente.",
              highlight: "pire",
            },
          ],
        },
        {
          title: "Constructions parallèles",
          subcases: [
            {
              label: "Plus… plus / Moins… moins → progression parallèle",
              example: "Plus je le connais, plus je l'apprécie.",
              highlight: "Plus… plus",
            },
            {
              label: "Plus… moins / Moins… plus → variation inverse",
              example: "Plus tu insistes, moins j'ai envie d'accepter.",
              highlight: "Plus… moins",
            },
            {
              label: "D'autant plus / moins… que → renforce une cause",
              example: "Sa réussite est d'autant plus impressionnante qu'il a peu de moyens.",
              highlight: "d'autant plus… que",
            },
          ],
        },
        {
          title: "Pièges fréquents",
          body:
            "Ne pas confondre « meilleur » (adjectif, qualifie un nom : un meilleur résultat) et « mieux » (adverbe, modifie un verbe : il travaille mieux). Le superlatif demande l'article : « le plus grand », jamais « plus grand » seul. La préposition « de » introduit le groupe de comparaison : « le plus grand de la classe », non « le plus grand dans la classe ». Avec un adjectif placé après le nom, l'article se répète : « le livre le plus intéressant », non « le livre plus intéressant ».",
        },
      ],
    },
    items: [
      {
        before: "Cette tarte est ",
        hint: "comparatif de « bon »",
        after: " que celle que tu as faite la semaine dernière.",
        answers: ["meilleure"],
        rule:
          "« Bon » → comparatif irrégulier « meilleur ». L'adjectif s'accorde avec le nom (tarte, féminin singulier) → meilleure.",
        consolidation: {
          before: "Ces résultats sont nettement ",
          hint: "comparatif de « bon »",
          after: " que ceux du trimestre précédent.",
          answers: ["meilleurs"],
        },
      },
      {
        type: "qcm",
        prompt:
          "Laquelle de ces phrases est correctement construite ?",
        context: "",
        options: [
          "Elle danse plus bien que moi.",
          "Elle danse meilleur que moi.",
          "Elle danse mieux que moi.",
        ],
        correctIndex: 2,
        rule:
          "« Bien » est un adverbe ; son comparatif est « mieux » (forme irrégulière). « Meilleur » est le comparatif de l'adjectif « bon » et ne peut pas modifier un verbe. On ne dit jamais « plus bien ».",
        consolidation: {
          type: "qcm",
          prompt: "Laquelle est correcte ?",
          context: "",
          options: [
            "Ce restaurant est mieux que l'autre.",
            "Ce restaurant est meilleur que l'autre.",
            "Ce restaurant est plus bon que l'autre.",
          ],
          correctIndex: 1,
          rule:
            "Ici, on qualifie le nom « restaurant » avec un adjectif → « meilleur » (comparatif de bon). « Mieux » serait un adverbe et ne peut pas qualifier un nom.",
        },
      },
      {
        type: "transform",
        instruction:
          "Réécrivez la phrase au superlatif (en ajoutant « de toutes les pièces de Molière »).",
        source:
          "Cette pièce est très célèbre.",
        answers: [
          "Cette pièce est la plus célèbre de toutes les pièces de Molière",
          "C'est la plus célèbre de toutes les pièces de Molière",
        ],
        rule:
          "Superlatif d'un adjectif : article défini (la) + plus + adjectif accordé + de + groupe. La préposition « de » introduit l'ensemble dans lequel s'opère le classement.",
        consolidation: {
          type: "transform",
          instruction:
            "Réécrivez au superlatif (avec « de la classe »).",
          source:
            "Cet élève est très studieux.",
          answers: [
            "Cet élève est le plus studieux de la classe",
            "C'est l'élève le plus studieux de la classe",
          ],
          rule:
            "Superlatif : le plus studieux + de la classe. La préposition est toujours « de », jamais « dans ».",
        },
      },
      {
        before: "J'ai ",
        hint: "comparatif d'égalité + nom",
        after: " patience que toi, mais je ne le montre pas.",
        answers: ["autant de"],
        rule:
          "Comparatif d'égalité avec un nom → « autant de » + nom + que. À distinguer de « aussi… que » qui s'emploie avec un adjectif ou un adverbe.",
        consolidation: {
          before: "Il y a ",
          hint: "comparatif d'égalité + nom",
          after: " participants cette année que l'année dernière.",
          answers: ["autant de"],
        },
      },
      {
        type: "qcm",
        prompt: "Laquelle de ces phrases est correctement construite ?",
        context: "",
        options: [
          "C'est le livre plus intéressant que j'aie lu cette année.",
          "C'est le plus intéressant livre que j'aie lu cette année.",
          "C'est le livre le plus intéressant que j'aie lu cette année.",
        ],
        correctIndex: 2,
        rule:
          "Quand l'adjectif est placé après le nom (position habituelle pour la plupart des adjectifs en français), l'article se répète au superlatif : « le livre le plus intéressant ». L'adjectif « intéressant » ne se place pas avant le nom.",
        consolidation: {
          type: "qcm",
          prompt: "Quelle phrase est correcte ?",
          context: "",
          options: [
            "C'est l'idée la plus brillante que j'aie jamais eue.",
            "C'est l'idée plus brillante que j'aie jamais eue.",
            "C'est la plus brillante idée que j'aie jamais eue.",
          ],
          correctIndex: 0,
          rule:
            "Adjectif après le nom → article répété : « l'idée la plus brillante ». À noter : « la plus brillante idée » est également correct (adjectif antéposé), mais moins courant.",
        },
      },
      {
        before: "Plus on étudie une langue, ",
        hint: "structure parallèle",
        after: " on découvre sa richesse et ses subtilités.",
        answers: ["plus"],
        rule:
          "Structure parallèle « plus… plus » exprime une progression conjointe. La virgule sépare les deux propositions. Construction symétrique : moins… moins, plus… moins, moins… plus.",
        consolidation: {
          before: "Moins il dort, ",
          hint: "structure parallèle",
          after: " il est productif au travail.",
          answers: ["moins"],
        },
      },
      {
        type: "transform",
        instruction:
          "Réécrivez la phrase en utilisant la structure « d'autant plus… que » pour souligner la cause.",
        source:
          "Sa victoire est impressionnante parce qu'il a peu d'expérience.",
        answers: [
          "Sa victoire est d'autant plus impressionnante qu'il a peu d'expérience",
        ],
        rule:
          "« D'autant plus + adjectif + que » renforce la cause : l'adjectif est plus fort à cause d'une circonstance. Construction très utile pour mettre en relief un mérite ou une difficulté.",
        consolidation: {
          type: "transform",
          instruction:
            "Réécrivez avec « d'autant plus… que ».",
          source:
            "Cette réussite est admirable parce qu'elle est arrivée après plusieurs échecs.",
          answers: [
            "Cette réussite est d'autant plus admirable qu'elle est arrivée après plusieurs échecs",
          ],
          rule:
            "« D'autant plus admirable que » : l'adjectif est renforcé par la circonstance qui suit.",
        },
      },
      {
        before: "La situation économique est ",
        hint: "comparatif irrégulier de « mauvais »",
        after: " qu'on ne le pensait au début de l'année.",
        answers: ["pire"],
        rule:
          "« Pire » est le comparatif irrégulier de « mauvais » et s'emploie quand on parle d'une qualité abstraite ou d'une situation. Il est plus expressif que « plus mauvais ». Forme invariable au singulier (la pire, le pire), -s au pluriel (les pires).",
        consolidation: {
          before: "Ce film est sans doute le ",
          hint: "superlatif irrégulier de « mauvais »",
          after: " que j'aie vu cette année.",
          answers: ["pire"],
        },
      },
    ],
  },

  /* ---------- 21. NÉGATION ---------- */
  {
    id: "negation",
    number: 21,
    section: "Logique de la phrase",
    title: "La",
    titleEm: "négation",
    lesson: {
      intro:
        "La négation française est plus riche qu'il n'y paraît. Au-delà du simple « ne… pas », le B2 demande de manier des restrictions (ne… que), des cumuls (ne… ni… ni), des nuances temporelles (ne… plus, ne… jamais, ne… pas encore), la place exacte aux temps composés et à l'infinitif, et la nuance du « ne » seul (littéraire) ou explétif (sans valeur négative).",
      formation: [
        "Négation simple : ne + verbe + pas. Aux temps composés, « pas » se place entre l'auxiliaire et le participe : je n'ai pas compris. À l'infinitif, « ne pas » se place en bloc devant le verbe : ne pas oublier.",
        "Autres adverbes négatifs : ne… plus, ne… jamais, ne… pas encore, ne… nulle part, ne… aucunement. Mêmes règles de place.",
        "Pronoms négatifs : ne… personne, ne… rien, ne… aucun(e). « Personne » et « rien » se placent comme « pas » à un temps simple, mais après le participe à un temps composé : je n'ai vu personne, je n'ai rien dit.",
        "Restriction : ne… que (= seulement). Le « que » se place juste avant le mot ou groupe sur lequel porte la restriction. Ex. : Il ne boit que de l'eau (= seulement de l'eau).",
        "Négation double : ne… ni… ni. Les articles indéfinis et partitifs disparaissent après « ni ». Ex. : Je ne bois ni café ni thé (pas « ni du café »).",
        "À l'oral familier, le « ne » disparaît souvent (j'sais pas, j'veux rien). À l'écrit, il est obligatoire.",
      ],
      irregulars: [],
      triggers: [
        {
          title: "Place de la négation",
          subcases: [
            {
              label: "Temps simple : ne… pas autour du verbe",
              example: "Je ne comprends pas vraiment ta réaction.",
              highlight: "ne comprends pas",
            },
            {
              label: "Temps composé : ne… pas autour de l'auxiliaire",
              example: "Elle n'a pas répondu à mon message d'hier.",
              highlight: "n'a pas répondu",
            },
            {
              label: "Infinitif : « ne pas » en bloc devant le verbe",
              example: "Je te demande de ne pas insister sur ce point.",
              highlight: "ne pas insister",
            },
            {
              label: "Personne / rien à un temps composé : différents !",
              example: "Je n'ai rien vu mais je n'ai vu personne non plus.",
              highlight: "rien vu / vu personne",
            },
          ],
        },
        {
          title: "Restriction avec « ne… que »",
          subcases: [
            {
              label: "« Ne… que » = seulement (n'est pas une vraie négation)",
              example: "Je ne dispose que de cinq minutes pour vous répondre.",
              highlight: "ne… que",
            },
            {
              label: "Le « que » se place juste devant l'élément restreint",
              example: "Elle ne pense qu'à son travail ces derniers temps.",
              highlight: "qu'à son travail",
            },
            {
              label: "Distinction : « ne pas que » = pas seulement, il y a autre chose",
              example: "Il n'y a pas que toi qui aies des soucis dans cette équipe.",
              highlight: "pas que toi",
            },
          ],
        },
        {
          title: "Négation double et combinaisons",
          subcases: [
            {
              label: "Ne… ni… ni : les déterminants partitifs et indéfinis disparaissent",
              example: "Cette enfant ne mange ni viande ni poisson par principe.",
              highlight: "ni viande ni poisson",
            },
            {
              label: "Combinaisons : plus jamais, plus personne, plus rien",
              example: "Depuis cette dispute, ils ne se parlent plus jamais.",
              highlight: "plus jamais",
            },
            {
              label: "Sans + nom / infinitif (négation implicite)",
              example: "Il est parti sans dire au revoir à personne.",
              highlight: "sans dire au revoir à personne",
            },
          ],
        },
        {
          title: "Pièges fréquents",
          body:
            "« Ne… que » n'est pas une vraie négation : on garde « du, de la, des » après. On dit « il ne boit que du vin » et non « il ne boit que de vin ». Avec « ni… ni », au contraire, on supprime les articles indéfinis et partitifs : « ni du café ni du thé » est incorrect. Le « ne » explétif (sans valeur négative) apparaît après avant que, à moins que, de peur que, et après les verbes de crainte (je crains qu'il ne vienne = j'ai peur qu'il vienne). À l'oral familier, on omet souvent le « ne », mais à l'écrit c'est une faute.",
        },
      ],
    },
    items: [
      {
        type: "transform",
        instruction:
          "Réécrivez la phrase à la forme négative avec « ne… que » pour exprimer une restriction.",
        source:
          "Je dispose de cinq minutes pour vous répondre.",
        answers: [
          "Je ne dispose que de cinq minutes pour vous répondre",
        ],
        rule:
          "« Ne… que » exprime une restriction (= seulement). Le « que » se place juste avant l'élément restreint (cinq minutes). Attention : la préposition « de » est maintenue, car « ne… que » n'est pas une vraie négation.",
        consolidation: {
          type: "transform",
          instruction:
            "Réécrivez la phrase avec « ne… que ».",
          source:
            "Elle pense à son travail ces derniers temps.",
          answers: [
            "Elle ne pense qu'à son travail ces derniers temps",
          ],
          rule:
            "« Que » se place juste devant l'élément restreint (à son travail). Élision en « qu' » devant la voyelle.",
        },
      },
      {
        type: "qcm",
        prompt:
          "Laquelle de ces phrases est correctement construite ?",
        context: "",
        options: [
          "Il ne boit ni du café ni du thé.",
          "Il ne boit ni café ni thé.",
          "Il ne boit ne café ne thé.",
        ],
        correctIndex: 1,
        rule:
          "Avec « ne… ni… ni », les articles indéfinis (un, une, des) et partitifs (du, de la) disparaissent. On dit « ni café ni thé », jamais « ni du café ni du thé ». Les articles définis (le, la, les), eux, sont maintenus : « ni la viande ni le poisson ».",
        consolidation: {
          type: "qcm",
          prompt: "Laquelle est correcte ?",
          context: "",
          options: [
            "Je n'ai ni amis ni famille dans cette ville.",
            "Je n'ai ni des amis ni de la famille dans cette ville.",
            "Je n'ai ne amis ne famille dans cette ville.",
          ],
          correctIndex: 0,
          rule:
            "Les articles indéfinis (des, une) tombent après « ni » : ni amis ni famille.",
        },
      },
      {
        before: "Aux temps composés, attention à la place : je n'ai ",
        hint: "négation totale",
        after: " compris votre demande.",
        answers: ["pas"],
        rule:
          "Aux temps composés, « pas » se place entre l'auxiliaire et le participe passé : n'ai pas compris, n'avait pas vu. Même règle pour plus, jamais, pas encore : il n'a plus appelé, je n'ai jamais vu.",
        consolidation: {
          before: "Elle n'avait ",
          hint: "négation = à aucun moment dans le passé",
          after: " imaginé que la situation tournerait ainsi.",
          answers: ["jamais"],
        },
      },
      {
        type: "qcm",
        prompt: "Quelle est la place correcte de « personne » dans cette phrase ?",
        context: "Hier soir, à la fête, je…",
        options: [
          "n'ai personne vu d'intéressant.",
          "n'ai vu personne d'intéressant.",
          "ne personne ai vu d'intéressant.",
        ],
        correctIndex: 1,
        rule:
          "« Personne » et « nulle part » se placent toujours après le participe passé aux temps composés (et après l'infinitif). À distinguer de « rien », qui se place entre l'auxiliaire et le participe : « je n'ai rien vu » mais « je n'ai vu personne ».",
        consolidation: {
          type: "qcm",
          prompt: "Quelle phrase est correctement construite ?",
          context: "",
          options: [
            "Je n'ai rien dit à personne au sujet de cette histoire.",
            "Je n'ai dit rien à personne au sujet de cette histoire.",
            "Je n'ai personne dit rien au sujet de cette histoire.",
          ],
          correctIndex: 0,
          rule:
            "Combinaison classique : « rien » entre auxiliaire et participe (n'ai rien dit), « personne » après (à personne). Cumul de deux négations possible et fréquent.",
        },
      },
      {
        before: "Je te demande expressément de ",
        hint: "négation à l'infinitif",
        after: " sur ce point lors de la réunion.",
        answers: ["ne pas insister"],
        rule:
          "À l'infinitif, « ne pas » se place en bloc devant le verbe : ne pas insister, ne pas oublier, ne plus fumer. C'est la seule position correcte ; « insister ne pas » ou « ne insister pas » sont fautifs.",
        consolidation: {
          before: "Pour ",
          hint: "négation à l'infinitif",
          after: " la pluie, prends ton parapluie en sortant.",
          answers: ["ne pas oublier"],
        },
      },
      {
        type: "transform",
        instruction:
          "Réécrivez la phrase à la forme négative avec « ne… plus ».",
        source:
          "Depuis l'an dernier, il fume régulièrement.",
        answers: [
          "Depuis l'an dernier, il ne fume plus régulièrement",
          "Depuis l'an dernier, il ne fume plus",
        ],
        rule:
          "« Ne… plus » exprime l'arrêt d'une action ou d'un état qui durait. À ne pas confondre avec « ne… pas plus » (= pas davantage). La place est la même que « ne… pas ».",
        consolidation: {
          type: "transform",
          instruction:
            "Réécrivez à la forme négative avec « ne… jamais ».",
          source:
            "J'ai vu un spectacle aussi extraordinaire dans ma vie.",
          answers: [
            "Je n'ai jamais vu un spectacle aussi extraordinaire dans ma vie",
            "Je n'ai jamais vu de spectacle aussi extraordinaire dans ma vie",
          ],
          rule:
            "« Jamais » se place entre auxiliaire et participe au temps composé. Construction très idiomatique pour souligner l'unicité d'une expérience.",
        },
      },
      {
        before: "Cette équipe ",
        hint: "négation = aucun joueur",
        after: " joueur expérimenté pour ce championnat.",
        answers: ["n'a aucun"],
        rule:
          "« Aucun(e) » s'accorde en genre avec le nom et exprime l'absence totale. Il fonctionne comme un déterminant négatif et ne nécessite pas « pas » : « il n'a aucun ami », jamais « il n'a pas aucun ami ».",
        consolidation: {
          before: "Nous ",
          hint: "négation = aucune raison",
          after: " raison de douter de sa parole.",
          answers: ["n'avons aucune"],
        },
      },
      {
        type: "qcm",
        prompt:
          "Quelle phrase exprime correctement « il y a d'autres personnes aussi, pas seulement toi » ?",
        context: "",
        options: [
          "Il n'y a que toi qui aies des soucis.",
          "Il n'y a pas que toi qui aies des soucis.",
          "Il y a personne que toi qui aies des soucis.",
        ],
        correctIndex: 1,
        rule:
          "« Ne… que » exprime la restriction (seulement toi). « Ne… pas que » indique au contraire qu'il y a autre chose ou quelqu'un d'autre. C'est une distinction très utile pour nuancer un propos.",
        consolidation: {
          type: "qcm",
          prompt:
            "Quelle phrase signifie « il fait autre chose, en plus de travailler » ?",
          context: "",
          options: [
            "Il ne fait que travailler.",
            "Il ne fait pas que travailler.",
            "Il ne fait rien que travailler.",
          ],
          correctIndex: 1,
          rule:
            "« Ne fait que travailler » = il fait seulement cela. « Ne fait pas que travailler » = il fait autre chose en plus. La présence du « pas » inverse complètement le sens.",
        },
      },
    ],
  },

  /* ---------- 22. ARTICLES ---------- */
  {
    id: "articles",
    number: 22,
    section: "Déterminants et groupe nominal",
    title: "Les",
    titleEm: "articles",
    lesson: {
      intro:
        "Article défini, indéfini ou partitif ? Le choix paraît simple, mais en réalité il dépend de subtilités qui échappent souvent aux apprenants : on parle d'un objet précis ou abstrait, d'une quantité indéterminée, d'une généralité, d'une habitude. Au B2, l'enjeu se déplace : il s'agit de gérer les disparitions de l'article (après de, avec ni… ni, dans certaines locutions) et les contractions obligatoires.",
      formation: [
        "Articles définis : le, la, l', les. S'emploient pour un objet identifié, une généralité, une espèce. Contractions obligatoires avec à et de : au, aux, du, des.",
        "Articles indéfinis : un, une, des. Un objet non identifié ou un parmi d'autres. Au pluriel, « des » devient « de » devant un adjectif antéposé : de beaux jardins (mais des jardins fleuris).",
        "Articles partitifs : du, de la, de l', des. Quantité indéfinie de quelque chose qui ne se compte pas : de l'eau, du courage, de la patience.",
        "Disparition de l'article : après une négation totale, du/de la/des deviennent « de » ou « d' ». Ex. : Je bois du café → Je ne bois pas de café. Mais après être : ce n'est pas du café (l'article reste).",
        "Disparition aussi : après une expression de quantité (beaucoup de, peu de, trop de…) et après les prépositions « sans » et « avec » + nom abstrait (sans difficulté, avec courage).",
      ],
      irregulars: [],
      triggers: [
        {
          title: "Défini, indéfini ou partitif ?",
          subcases: [
            {
              label: "Défini → objet identifié, généralité, espèce",
              example: "Le chien est généralement considéré comme le meilleur ami de l'homme.",
              highlight: "Le chien",
            },
            {
              label: "Indéfini → un parmi d'autres, premier mention",
              example: "J'ai croisé un voisin charmant dans l'ascenseur ce matin.",
              highlight: "un voisin",
            },
            {
              label: "Partitif → quantité indéterminée d'une chose non comptable",
              example: "Pour réussir ce projet, il faudra du temps et de la patience.",
              highlight: "du temps et de la patience",
            },
          ],
        },
        {
          title: "Disparition de l'article",
          subcases: [
            {
              label: "Après une négation totale : du/de la/des → de",
              example: "Je n'ai plus de patience pour ce genre de discussions.",
              highlight: "de patience",
            },
            {
              label: "Après une expression de quantité (beaucoup de, peu de, trop de…)",
              example: "Cette région produit beaucoup de vin et peu de céréales.",
              highlight: "beaucoup de vin et peu de céréales",
            },
            {
              label: "« Des » → « de » devant un adjectif antéposé",
              example: "Le jardin abrite de magnifiques rosiers anciens.",
              highlight: "de magnifiques rosiers",
            },
          ],
        },
        {
          title: "Article avec ou sans, dans certaines constructions",
          subcases: [
            {
              label: "« Avec » + nom concret → article ; + nom abstrait → souvent sans article",
              example: "Elle a répondu avec gentillesse mais avec une fermeté désarmante.",
              highlight: "avec gentillesse",
            },
            {
              label: "Devant un nom de profession après être, devenir, rester → souvent sans article",
              example: "Sa fille est devenue médecin il y a deux ans.",
              highlight: "est devenue médecin",
            },
            {
              label: "Avec un complément qualifiant ce nom → on remet l'article",
              example: "Sa fille est une excellente médecin reconnue dans toute la région.",
              highlight: "une excellente médecin",
            },
          ],
        },
        {
          title: "Pièges fréquents",
          body:
            "« Ce n'est pas + article complet » et non « ce n'est pas de » : on dit « ce n'est pas du café, c'est du thé », car la négation porte sur l'identification, pas sur la quantité. Avec « ne… que » (restriction, pas vraie négation), l'article complet est maintenu : « il ne boit que du vin », jamais « il ne boit que de vin ». Pour les noms de pays : article défini obligatoire (la France, les États-Unis), sauf après « en » pour les pays féminins (en France) et après « de » dans certaines locutions (les vins de France).",
        },
      ],
    },
    items: [
      {
        type: "qcm",
        prompt: "Quel article convient dans ce contexte ?",
        context:
          "Pour réussir ce projet, il nous faudra ___ patience et ___ courage.",
        options: ["la / le", "de la / du", "une / un"],
        correctIndex: 1,
        rule:
          "« Patience » et « courage » sont ici des qualités envisagées comme des quantités indéterminées → article partitif (de la patience, du courage). L'article défini désignerait la patience en général ; l'indéfini introduirait un objet, ce qui ne convient pas pour ces noms abstraits.",
        consolidation: {
          type: "qcm",
          prompt: "Quel article convient ?",
          context:
            "Pour cette recette, il vous faut ___ farine, ___ beurre et ___ sel.",
          options: ["la / le / le", "de la / du / du", "une / un / un"],
          correctIndex: 1,
          rule:
            "Ingrédients en quantité indéterminée → partitifs : de la farine, du beurre, du sel.",
        },
      },
      {
        before: "Je ne bois jamais ",
        hint: "article après négation",
        after: " café après dix-huit heures, sinon je n'arrive plus à dormir.",
        answers: ["de"],
        rule:
          "Après une négation totale (ne… jamais), le partitif « du » devient « de ». Règle systématique : pas de café, plus de patience, jamais de vin.",
        consolidation: {
          before: "Cette boulangerie ne fait plus ",
          hint: "article après négation",
          after: " pain le dimanche depuis qu'elle a changé de propriétaire.",
          answers: ["de"],
        },
      },
      {
        type: "qcm",
        prompt:
          "Laquelle de ces phrases est correctement construite ?",
        context: "",
        options: [
          "Ce parc abrite des magnifiques arbres centenaires.",
          "Ce parc abrite de magnifiques arbres centenaires.",
          "Ce parc abrite les magnifiques arbres centenaires.",
        ],
        correctIndex: 1,
        rule:
          "Quand un adjectif est placé avant un nom au pluriel, l'article indéfini « des » devient « de ». On dit « de beaux jardins », « de grands hommes », « de magnifiques arbres ». Si l'adjectif est après le nom, « des » est maintenu : des arbres magnifiques.",
        consolidation: {
          type: "qcm",
          prompt: "Quelle phrase est correcte ?",
          context: "",
          options: [
            "Il a écrit des excellents romans.",
            "Il a écrit d'excellents romans.",
            "Il a écrit les excellents romans.",
          ],
          correctIndex: 1,
          rule:
            "« Des » devient « d' » devant un adjectif commençant par une voyelle : d'excellents romans, d'anciens amis.",
        },
      },
      {
        type: "transform",
        instruction:
          "Réécrivez la phrase à la forme négative en ajustant l'article si nécessaire.",
        source:
          "Cette région produit du vin et des fromages d'exception.",
        answers: [
          "Cette région ne produit pas de vin ni de fromages d'exception",
          "Cette région ne produit ni vin ni fromages d'exception",
          "Cette région ne produit pas de vin et pas de fromages d'exception",
        ],
        rule:
          "Après négation totale, les partitifs et indéfinis (du, des) deviennent « de ». Variante très idiomatique : « ne… ni… ni » fait disparaître complètement l'article (ni vin ni fromages).",
        consolidation: {
          type: "transform",
          instruction:
            "Réécrivez à la forme négative.",
          source:
            "Elle a des amis dans cette ville et de la famille à proximité.",
          answers: [
            "Elle n'a pas d'amis dans cette ville ni de famille à proximité",
            "Elle n'a ni amis dans cette ville ni famille à proximité",
          ],
          rule:
            "Mêmes règles : « des amis » → « d'amis » ; « de la famille » → « de famille ». La variante en « ni… ni » fait disparaître totalement l'article.",
        },
      },
      {
        before: "Sa sœur cadette est devenue ",
        hint: "article ou pas d'article",
        after: " avocate dans un grand cabinet parisien.",
        answers: [""],
        rule:
          "Devant un nom de profession après être, devenir, rester, etc., l'article est habituellement absent : il est médecin, elle est devenue avocate. L'article apparaît dès qu'un complément qualifie la profession : c'est une avocate brillante.",
        consolidation: {
          before: "Mon père a longtemps été ",
          hint: "article ou pas d'article",
          after: " enseignant avant de prendre sa retraite.",
          answers: [""],
        },
      },
      {
        type: "qcm",
        prompt:
          "Quelle phrase est correctement construite ?",
        context: "",
        options: [
          "Ce n'est pas de café, c'est de thé.",
          "Ce n'est pas du café, c'est du thé.",
          "Ce n'est pas le café, c'est le thé.",
        ],
        correctIndex: 1,
        rule:
          "Avec « être » à la forme négative, la négation porte sur l'identification, pas sur la quantité : l'article complet est maintenu (« du café, du thé »). C'est l'exception qui confirme la règle « pas de + nom ».",
        consolidation: {
          type: "qcm",
          prompt: "Quelle phrase est correcte ?",
          context: "",
          options: [
            "Ce n'est pas de fromage, c'est de viande.",
            "Ce n'est pas du fromage, c'est de la viande.",
            "Ce n'est pas de la fromage, c'est du viande.",
          ],
          correctIndex: 1,
          rule:
            "Après « être » négatif, l'article partitif est maintenu : du fromage, de la viande. Genre à respecter : du (masc.), de la (fém.).",
        },
      },
      {
        before: "Cette région produit beaucoup ",
        hint: "préposition après quantité",
        after: " vin et peu ",
        after2: " céréales.",
        answers: ["de"],
        rule:
          "Après une expression de quantité (beaucoup de, peu de, assez de, trop de, plus de, moins de…), on n'emploie jamais l'article partitif ni indéfini : juste « de » ou « d' » devant voyelle.",
        consolidation: {
          before: "Il a trop ",
          hint: "préposition après quantité",
          after: " travail et pas assez ",
          after2: " temps libre.",
          answers: ["de"],
        },
      },
      {
        type: "transform",
        instruction:
          "Réécrivez la phrase en plaçant l'adjectif AVANT le nom (et ajustez l'article si nécessaire).",
        source:
          "Le voyage nous a fait découvrir des paysages spectaculaires.",
        answers: [
          "Le voyage nous a fait découvrir de spectaculaires paysages",
        ],
        rule:
          "Quand l'adjectif passe avant le nom au pluriel, l'article indéfini « des » devient obligatoirement « de ». Cette inversion donne souvent une coloration plus littéraire à la phrase.",
        consolidation: {
          type: "transform",
          instruction:
            "Réécrivez en plaçant l'adjectif avant le nom.",
          source:
            "Cette boutique vend des bijoux anciens à des prix raisonnables.",
          answers: [
            "Cette boutique vend d'anciens bijoux à des prix raisonnables",
          ],
          rule:
            "« Des bijoux anciens » → « d'anciens bijoux » (élision devant voyelle). Le second « des » reste car « raisonnables » est placé après le nom.",
        },
      },
    ],
  },

  /* ---------- 23. INDÉFINIS ---------- */
  {
    id: "indefinis",
    number: 23,
    section: "Déterminants et groupe nominal",
    title: "Les déterminants et pronoms",
    titleEm: "indéfinis",
    lesson: {
      intro:
        "Les indéfinis (tout, chacun, quelques, plusieurs, n'importe quel, quiconque, autre…) servent à exprimer la quantité de façon floue, la totalité, l'individualité, le choix arbitraire. Au B2, on attend la maîtrise des nuances : quel mot pour quel sens, et un usage correct des constructions parfois piégeuses (tout + adjectif, n'importe quel vs n'importe lequel).",
      formation: [
        "« Tout » est variable comme déterminant ou pronom : tout, toute, tous, toutes. Ex. : toute la journée, tous les livres, toutes mes amies.",
        "« Tout » est invariable comme adverbe (= entièrement) : ils sont tout étonnés, elle est tout heureuse. Exception : devant un adjectif féminin commençant par une consonne, « tout » devient « toute » : elle est toute surprise.",
        "« Chacun(e) » est toujours pronom, jamais déterminant. Le déterminant correspondant est « chaque » + nom singulier. Ex. : chaque étudiant a son livre ; chacun a son livre.",
        "« Quelques » (déterminant) / « quelques-uns / quelques-unes » (pronom) : exprime un petit nombre.",
        "« Plusieurs » est invariable, toujours pluriel : plusieurs amis, plusieurs idées.",
        "« N'importe quel(le)(s) » + nom : choix arbitraire d'objet. « N'importe lequel / laquelle / lesquels / lesquelles » : pronom équivalent.",
        "« Quiconque » est un pronom indéfini littéraire ou juridique : quiconque enfreint la loi sera puni.",
      ],
      irregulars: [],
      triggers: [
        {
          title: "Tout : déterminant, pronom ou adverbe ?",
          subcases: [
            {
              label: "Déterminant : tout + déterminant + nom (variable)",
              example: "Tous les passagers doivent regagner leur siège.",
              highlight: "Tous les",
            },
            {
              label: "Pronom : remplace un nom (variable)",
              example: "Mes amis sont arrivés : tous m'attendaient déjà.",
              highlight: "tous",
            },
            {
              label: "Adverbe = entièrement (invariable, sauf devant adjectif fém. consonne)",
              example: "Elle est rentrée tout épuisée mais toute heureuse de son voyage.",
              highlight: "tout épuisée / toute heureuse",
            },
          ],
        },
        {
          title: "Chacun vs chaque",
          subcases: [
            {
              label: "« Chaque » + nom singulier (déterminant)",
              example: "Chaque participant recevra un dossier à l'entrée.",
              highlight: "Chaque participant",
            },
            {
              label: "« Chacun(e) » = pronom (jamais suivi d'un nom)",
              example: "Les invités sont arrivés : chacun a apporté quelque chose.",
              highlight: "chacun",
            },
            {
              label: "Expression : « chacun de » + groupe nominal pluriel",
              example: "Chacun des candidats a passé un entretien individuel.",
              highlight: "Chacun des candidats",
            },
          ],
        },
        {
          title: "Choix arbitraire : n'importe quel, n'importe lequel…",
          subcases: [
            {
              label: "« N'importe quel(le)(s) » + nom = choix arbitraire d'un objet",
              example: "Prenez n'importe quel stylo, peu importe la couleur.",
              highlight: "n'importe quel stylo",
            },
            {
              label: "« N'importe lequel / laquelle » = pronom (sans nom)",
              example: "Voici trois propositions, choisis n'importe laquelle.",
              highlight: "n'importe laquelle",
            },
            {
              label: "« N'importe qui / quoi / où / quand / comment » = adverbes / pronoms invariables",
              example: "Tu peux m'appeler n'importe quand, je serai disponible.",
              highlight: "n'importe quand",
            },
          ],
        },
        {
          title: "Pièges fréquents",
          body:
            "Erreur classique : « chacun étudiant » (incorrect : chacun ne se place pas devant un nom). Soit on dit « chaque étudiant », soit on dit « chacun des étudiants ». Avec « tout » adverbe, l'accord est piégeux : « ils sont tout étonnés » (invariable, masculin) mais « elles sont toutes étonnées » (féminin pluriel commençant par voyelle → invariable) et « elles sont toutes surprises » (féminin commençant par consonne → accord). « Quelque » varie quand il est déterminant (quelques amis) mais reste invariable comme adverbe (quelque cent personnes = environ).",
        },
      ],
    },
    items: [
      {
        before: "",
        hint: "déterminant indéfini de totalité",
        after: " les invités étaient présents pour la cérémonie.",
        answers: ["Tous"],
        rule:
          "« Tout » comme déterminant s'accorde avec le nom : tous les invités (masculin pluriel). On dit aussi : toute la journée (fém. sing.), toutes mes amies (fém. plur.).",
        consolidation: {
          before: "",
          hint: "déterminant indéfini de totalité",
          after: " les décisions importantes sont prises en équipe.",
          answers: ["Toutes"],
        },
      },
      {
        type: "qcm",
        prompt:
          "Laquelle de ces phrases est correctement orthographiée ?",
        context: "",
        options: [
          "Elles sont arrivées tout épuisées du voyage.",
          "Elles sont arrivées toutes épuisées du voyage.",
          "Elles sont arrivées tous épuisées du voyage.",
        ],
        correctIndex: 0,
        rule:
          "« Tout » adverbe (= entièrement, complètement) est invariable devant un adjectif féminin commençant par une voyelle. Devant un h muet aussi : tout heureuses. Mais devant une consonne, il devient « toute(s) » : toutes surprises.",
        consolidation: {
          type: "qcm",
          prompt: "Laquelle est correctement orthographiée ?",
          context: "",
          options: [
            "Les enfants étaient tout heureux d'avoir leurs cadeaux.",
            "Les enfants étaient tous heureux d'avoir leurs cadeaux.",
            "Les enfants étaient toutes heureux d'avoir leurs cadeaux.",
          ],
          correctIndex: 0,
          rule:
            "« Tout » adverbe + adjectif masculin = invariable. Toutefois, la phrase « ils étaient tous heureux » serait aussi correcte, mais avec un sens différent (= ils étaient tous, sans exception, heureux). Ici le sens « entièrement » est plus naturel.",
        },
      },
      {
        type: "qcm",
        prompt: "Quel mot convient dans cette phrase ?",
        context:
          "___ participant a reçu un badge dès son arrivée.",
        options: ["Chacun", "Chaque", "Tous"],
        correctIndex: 1,
        rule:
          "« Chaque » est un déterminant : il se place devant un nom singulier. « Chacun », pronom, ne peut jamais précéder un nom. Phrase correcte alternative : « Chacun des participants a reçu un badge. »",
        consolidation: {
          type: "qcm",
          prompt: "Quel mot convient ?",
          context:
            "Les invités ont été placés : ___ avait sa place attribuée à l'avance.",
          options: ["chacun", "chaque", "tous"],
          correctIndex: 0,
          rule:
            "Ici, on ne reprend pas le mot « invité » : on a besoin d'un pronom. « Chacun » fonctionne seul. « Chaque » exigerait un nom (chaque invité avait sa place).",
        },
      },
      {
        before: "Prends ",
        hint: "déterminant de choix arbitraire",
        after: " livre dans la bibliothèque, ils sont tous excellents.",
        answers: ["n'importe quel"],
        rule:
          "« N'importe quel(le)(s) » + nom exprime un choix arbitraire (= peu importe lequel). Déterminant, il s'accorde avec le nom : n'importe quel livre (masc.), n'importe quelle idée (fém.), n'importe quels jours (masc. pl.).",
        consolidation: {
          before: "Tu peux poser ",
          hint: "déterminant de choix arbitraire",
          after: " question, je ferai de mon mieux pour y répondre.",
          answers: ["n'importe quelle"],
        },
      },
      {
        type: "transform",
        instruction:
          "Réécrivez la phrase en remplaçant le groupe nominal par un pronom indéfini équivalent.",
        source:
          "Voici trois propositions intéressantes : choisis n'importe quelle proposition.",
        answers: [
          "Voici trois propositions intéressantes : choisis n'importe laquelle",
        ],
        rule:
          "Quand le nom n'est pas répété, « n'importe quel(le) » devient le pronom « n'importe lequel / laquelle / lesquels / lesquelles ». Accord avec le nom remplacé (proposition = féminin singulier → laquelle).",
        consolidation: {
          type: "transform",
          instruction:
            "Réécrivez en utilisant un pronom indéfini.",
          source:
            "Tu peux prendre n'importe quels gâteaux dans le placard.",
          answers: [
            "Tu peux prendre n'importe lesquels dans le placard",
          ],
          rule:
            "« N'importe quels gâteaux » → « n'importe lesquels ». Le pronom s'accorde au masculin pluriel.",
        },
      },
      {
        before: "J'ai invité ",
        hint: "déterminant indiquant un petit nombre",
        after: " amis pour le dîner de samedi.",
        answers: ["quelques"],
        rule:
          "« Quelques » (déterminant pluriel) indique un petit nombre indéterminé. « Plusieurs » exprime à peu près la même idée mais avec un nuance de « plus que deux ». Au pluriel uniquement.",
        consolidation: {
          before: "Donne-moi ",
          hint: "déterminant indiquant un petit nombre",
          after: " minutes, je termine et j'arrive.",
          answers: ["quelques"],
        },
      },
      {
        type: "qcm",
        prompt:
          "Laquelle de ces phrases est correcte ?",
        context: "",
        options: [
          "Quiconque enfreindra le règlement sera sanctionné.",
          "Quiconque enfreindrait le règlement sera sanctionné.",
          "Quiconque enfreigne le règlement sera sanctionné.",
        ],
        correctIndex: 0,
        rule:
          "« Quiconque » (= toute personne qui) est suivi de l'indicatif. Registre juridique ou littéraire. La forme la plus naturelle en français contemporain pour la même idée : « toute personne qui enfreindra » ou « celui qui enfreindra ».",
        consolidation: {
          type: "qcm",
          prompt: "Quelle phrase est correcte ?",
          context: "",
          options: [
            "N'importe qui peut s'inscrire à cette formation.",
            "N'importe quelqu'un peut s'inscrire à cette formation.",
            "N'importe quel peut s'inscrire à cette formation.",
          ],
          correctIndex: 0,
          rule:
            "« N'importe qui » = pronom indéfini de personne, invariable. « N'importe quel » exige obligatoirement un nom après. « Quelqu'un » ne se combine pas avec « n'importe ».",
        },
      },
      {
        type: "transform",
        instruction:
          "Réécrivez la phrase en utilisant « chacun de » + groupe nominal au lieu de « chaque ».",
        source:
          "Chaque candidat a reçu une réponse personnalisée.",
        answers: [
          "Chacun des candidats a reçu une réponse personnalisée",
        ],
        rule:
          "« Chaque » + nom singulier ↔ « chacun de + les + nom pluriel » (avec contraction de + les = des). Construction quasi équivalente, mais « chacun de » insiste davantage sur l'individualité.",
        consolidation: {
          type: "transform",
          instruction:
            "Réécrivez en utilisant « chacune de » + groupe nominal.",
          source:
            "Chaque proposition mérite d'être examinée avec attention.",
          answers: [
            "Chacune des propositions mérite d'être examinée avec attention",
          ],
          rule:
            "« Chaque proposition » → « chacune des propositions ». Accord au féminin pluriel pour le complément.",
        },
      },
    ],
  },

  /* ---------- 24. PLACE ET SENS DE L'ADJECTIF ---------- */
  {
    id: "place-adjectif",
    number: 24,
    section: "Déterminants et groupe nominal",
    title: "Place et sens de",
    titleEm: "l'adjectif",
    lesson: {
      intro:
        "En français, la place de l'adjectif n'est pas neutre. Certains adjectifs changent carrément de sens selon qu'ils sont placés avant ou après le nom : un grand homme n'est pas un homme grand, un ancien collègue n'est pas un collègue ancien. Au B2, ces couples sont à connaître précisément, sous peine de produire des phrases ambiguës ou involontairement comiques.",
      formation: [
        "Place habituelle : après le nom pour la majorité des adjectifs, surtout les adjectifs de couleur, de forme, de nationalité, et les participes adjectivés. Ex. : une voiture rouge, une table ronde, un ami italien, un livre fatigué.",
        "Avant le nom : adjectifs courts et fréquents (BAGS : Beau, Âge, Bon/Bonté, Gentil, Grand, Joli, Mauvais, Méchant, Nouveau, Petit, Vieux). Ex. : un bel arbre, un jeune homme, un bon repas, une grande maison.",
        "Adjectifs à double place : ils changent de sens selon leur position. Avant le nom : sens figuré ou subjectif. Après le nom : sens littéral, objectif, descriptif.",
        "Quand plusieurs adjectifs qualifient un même nom, ils peuvent se répartir avant et après : une jolie petite maison ancienne. Lien possible avec « et » s'ils sont de même nature : un repas long et copieux.",
      ],
      irregulars: [],
      triggers: [
        {
          title: "Adjectifs à sens variable selon la place",
          subcases: [
            {
              label: "Ancien : avant = ex- / après = vieux dans le temps",
              example: "Mon ancien collègue habite dans un appartement ancien du dix-neuvième siècle.",
              highlight: "ancien / ancien",
            },
            {
              label: "Grand : avant (homme/figure) = remarquable / après = de grande taille",
              example: "Victor Hugo est un grand écrivain ; physiquement, c'était aussi un homme grand.",
              highlight: "grand / grand",
            },
            {
              label: "Pauvre : avant = à plaindre / après = sans argent",
              example: "Ce pauvre enfant a perdu son chat ; ces familles pauvres ont besoin d'aide.",
              highlight: "pauvre / pauvres",
            },
            {
              label: "Propre : avant = à soi / après = nettoyé",
              example: "C'est ma propre voiture, et elle est toujours très propre.",
              highlight: "propre / propre",
            },
          ],
        },
        {
          title: "D'autres couples B2 à connaître",
          subcases: [
            {
              label: "Cher : avant = aimé / après = coûteux",
              example: "Mon cher ami, ce restaurant est vraiment trop cher pour mon budget.",
              highlight: "cher / cher",
            },
            {
              label: "Seul : avant = unique / après = solitaire",
              example: "Voici mon seul espoir : un homme seul qui pourrait m'aider.",
              highlight: "seul / seul",
            },
            {
              label: "Certain : avant = quelque, indéterminé / après = sûr",
              example: "J'ai un certain doute mais c'est une chose certaine.",
              highlight: "certain / certaine",
            },
            {
              label: "Curieux : avant = étrange / après = qui montre de la curiosité",
              example: "Quelle curieuse histoire ! Cet enfant curieux pose mille questions.",
              highlight: "curieuse / curieux",
            },
          ],
        },
        {
          title: "Place stylistique",
          subcases: [
            {
              label: "Adjectif descriptif (couleur, forme, origine) → toujours après le nom",
              example: "Une voiture rouge italienne, une table ronde, un drapeau bleu.",
              highlight: "rouge italienne",
            },
            {
              label: "Plusieurs adjectifs courts → avant et après",
              example: "Un joli petit village médiéval niché dans la montagne.",
              highlight: "joli petit / médiéval",
            },
            {
              label: "Adjectif long ou suivi d'un complément → toujours après",
              example: "Un livre passionnant et facile à lire en une nuit.",
              highlight: "passionnant / facile à lire",
            },
          ],
        },
        {
          title: "Pièges fréquents",
          body:
            "« Un homme grand » désigne quelqu'un de taille élevée, « un grand homme » désigne une personne d'exception (= une grande figure). Confondre les deux peut prêter à sourire. Avec « propre », attention : « ma propre chambre » signifie « la chambre qui m'appartient », pas « ma chambre nettoyée ». Pour « pauvre », c'est l'inverse de l'ordre habituel des sens figurés et littéraux : le sens financier (concret) est après, le sens compassionnel (figuré) est avant.",
        },
      ],
    },
    items: [
      {
        type: "qcm",
        prompt:
          "Laquelle de ces phrases signifie : « C'est mon collègue d'autrefois, je ne travaille plus avec lui » ?",
        context: "",
        options: [
          "C'est un collègue ancien qui me téléphone parfois.",
          "C'est un ancien collègue qui me téléphone parfois.",
          "C'est un collègue grand qui me téléphone parfois.",
        ],
        correctIndex: 1,
        rule:
          "« Ancien » avant le nom = ex- (ancien collègue = avec qui on ne travaille plus). Après le nom, « ancien » signifie « âgé » (un meuble ancien). C'est l'un des couples les plus piégeux du français.",
        consolidation: {
          type: "qcm",
          prompt: "Quelle phrase signifie « le bâtiment date d'il y a très longtemps » ?",
          context: "",
          options: [
            "C'est un ancien bâtiment du quartier.",
            "C'est un bâtiment ancien du quartier.",
            "C'est un grand bâtiment du quartier.",
          ],
          correctIndex: 1,
          rule:
            "« Bâtiment ancien » = vieux, qui date. « Ancien bâtiment » signifierait « bâtiment qui n'en est plus un » (= ex-bâtiment), ce qui n'a guère de sens.",
        },
      },
      {
        before: "Victor Hugo est considéré comme un ",
        hint: "adjectif placé pour exprimer la valeur, pas la taille",
        after: " écrivain du dix-neuvième siècle.",
        answers: ["grand"],
        rule:
          "« Grand écrivain » = écrivain d'exception, figure remarquable. Si on disait « écrivain grand », on ferait référence à sa taille physique (taille élevée), ce qui serait incongru en parlant de son œuvre.",
        consolidation: {
          before: "Sa grand-mère était une ",
          hint: "adjectif placé pour exprimer la valeur",
          after: " dame que tout le monde admirait.",
          answers: ["grande"],
        },
      },
      {
        type: "transform",
        instruction:
          "Réécrivez la phrase pour qu'elle signifie « ces familles n'ont pas d'argent ». Déplacez l'adjectif si nécessaire.",
        source:
          "Ces pauvres familles vivent dans des conditions difficiles.",
        answers: [
          "Ces familles pauvres vivent dans des conditions difficiles",
        ],
        rule:
          "« Pauvre » avant le nom = à plaindre, malheureux (sens compassionnel). Après le nom = sans argent (sens littéral). Pour insister sur la précarité financière, on place « pauvre » après le nom.",
        consolidation: {
          type: "transform",
          instruction:
            "Réécrivez pour exprimer un sentiment de pitié (sans préciser la situation financière).",
          source:
            "Cet homme malheureux a tout perdu dans l'incendie.",
          answers: [
            "Ce pauvre homme a tout perdu dans l'incendie",
          ],
          rule:
            "« Pauvre homme » avant le nom = à plaindre. Ici on remplace « malheureux » par cette expression compassionnelle classique.",
        },
      },
      {
        type: "qcm",
        prompt:
          "Quelle phrase signifie « la voiture m'appartient » ?",
        context: "",
        options: [
          "C'est ma voiture propre que tu vois là.",
          "C'est ma propre voiture que tu vois là.",
          "C'est ma voiture neuve que tu vois là.",
        ],
        correctIndex: 1,
        rule:
          "« Propre » avant le nom = qui appartient en propre (à soi). « Propre » après le nom = nettoyé, sans saleté. « Ma propre voiture » = la mienne. « Ma voiture propre » = ma voiture lavée.",
        consolidation: {
          type: "qcm",
          prompt: "Quelle phrase signifie « la chemise vient d'être lavée » ?",
          context: "",
          options: [
            "Je porte une propre chemise.",
            "Je porte une chemise propre.",
            "Je porte ma propre chemise.",
          ],
          correctIndex: 1,
          rule:
            "« Chemise propre » = nettoyée. « Propre chemise » est syntaxiquement bancal sans possessif ; et « ma propre chemise » signifierait « celle qui m'appartient ».",
        },
      },
      {
        before: "Il a un ",
        hint: "adjectif placé pour exprimer le caractère étrange",
        after: " comportement depuis quelques jours, je trouve ça inquiétant.",
        answers: ["curieux"],
        rule:
          "« Curieux » avant le nom = étrange, bizarre. Après le nom, l'adjectif signifie « qui montre de la curiosité » : un enfant curieux veut tout savoir. Ici, « curieux comportement » = comportement étrange.",
        consolidation: {
          before: "C'est une ",
          hint: "adjectif placé pour exprimer le caractère étrange",
          after: " coïncidence : nous étions au même endroit à la même heure.",
          answers: ["curieuse"],
        },
      },
      {
        type: "qcm",
        prompt:
          "Laquelle de ces phrases signifie « c'est mon unique possibilité » ?",
        context: "",
        options: [
          "C'est ma chance seule.",
          "C'est ma seule chance.",
          "C'est ma grande chance.",
        ],
        correctIndex: 1,
        rule:
          "« Seul » avant le nom = unique, exclusif. « Seul » après le nom = solitaire, isolé. « Ma seule chance » = ma seule (= unique) possibilité. « Une chance seule » serait grammaticalement bancal et n'a pas vraiment de sens.",
        consolidation: {
          type: "qcm",
          prompt: "Quelle phrase signifie « il est isolé » ?",
          context: "",
          options: [
            "Voici un seul homme.",
            "Voici un homme seul.",
            "Voici un grand homme.",
          ],
          correctIndex: 1,
          rule:
            "« Un homme seul » = isolé, solitaire. « Un seul homme » = un unique homme, un seul (le nombre).",
        },
      },
      {
        before: "Ce restaurant est devenu trop ",
        hint: "adjectif placé pour exprimer le prix élevé",
        after: " pour notre budget de famille.",
        answers: ["cher"],
        rule:
          "« Cher » après le nom (ou en attribut) = coûteux, qui coûte beaucoup. « Cher » avant le nom (souvent dans une formule de politesse) = aimé, apprécié : mon cher ami, ma chère cousine.",
        consolidation: {
          before: "",
          hint: "adjectif placé en attribut pour exprimer l'affection",
          after: " amie, comme je suis heureuse de te revoir après si longtemps !",
          answers: ["Chère"],
        },
      },
      {
        type: "transform",
        instruction:
          "Réécrivez la phrase en déplaçant l'adjectif « certain » pour qu'elle signifie « j'ai quelques doutes, sans en être complètement sûr ».",
        source:
          "J'ai un doute certain sur la véracité de ses propos.",
        answers: [
          "J'ai un certain doute sur la véracité de ses propos",
        ],
        rule:
          "« Certain » après le nom = sûr, indubitable (un fait certain). « Certain » avant le nom = quelque, indéterminé, vague (un certain doute = une pointe de doute). Le déplacement modifie complètement le degré de certitude.",
        consolidation: {
          type: "transform",
          instruction:
            "Réécrivez pour que la phrase signifie « il y a une nuance d'ironie, mais ce n'est pas net ».",
          source:
            "Sa réponse comportait une ironie certaine.",
          answers: [
            "Sa réponse comportait une certaine ironie",
          ],
          rule:
            "« Une ironie certaine » = sûre, évidente. « Une certaine ironie » = un peu d'ironie, sans que ce soit net. Différence subtile mais cruciale.",
        },
      },
    ],
  },

  /* ---------- 25. ADVERBES ---------- */
  {
    id: "adverbes",
    number: 25,
    section: "Adverbes et prépositions",
    title: "Formation et place des",
    titleEm: "adverbes",
    lesson: {
      intro:
        "L'adverbe modifie un verbe, un adjectif ou un autre adverbe. Sa formation à partir de l'adjectif suit des règles précises, mais comporte plusieurs cas particuliers qu'il faut connaître au B2. Sa place dans la phrase, surtout aux temps composés, est l'autre grand défi : un déplacement de quelques mots peut changer complètement le sens.",
      formation: [
        "Formation standard : féminin de l'adjectif + suffixe -ment. Lent → lentement. Doux → doucement. Vrai → vraiment.",
        "Adjectifs terminés en -ant → adverbe en -amment : constant → constamment, savant → savamment, élégant → élégamment.",
        "Adjectifs terminés en -ent → adverbe en -emment : prudent → prudemment, fréquent → fréquemment, évident → évidemment.",
        "Exception phonétique : « lent » suit la règle générale (lentement), pas la règle des -ent. Quelques autres adjectifs gardent un -é à la place du -e : précis → précisément, profond → profondément, énorme → énormément.",
        "Adverbes irréguliers : bon → bien, mauvais → mal, meilleur → mieux, petit → peu (dans certains emplois).",
        "Place : aux temps simples, l'adverbe se place après le verbe (je travaille beaucoup, elle parle vite). Aux temps composés, les adverbes courts et fréquents (bien, mal, déjà, encore, souvent, toujours, vraiment, beaucoup, trop, peu) se placent entre l'auxiliaire et le participe. Les adverbes longs se placent généralement après le participe.",
      ],
      irregulars: [
        { v: "bon", forms: "bien" },
        { v: "mauvais", forms: "mal" },
        { v: "meilleur", forms: "mieux" },
        { v: "rapide", forms: "rapidement (régulier) ou vite (irrégulier)" },
        { v: "gentil", forms: "gentiment (avec un seul « l »)" },
      ],
      triggers: [
        {
          title: "Formation à partir de l'adjectif",
          subcases: [
            {
              label: "Règle générale : féminin de l'adjectif + -ment",
              example: "Elle a répondu sèchement à toutes ses questions.",
              highlight: "sèchement",
            },
            {
              label: "Adjectifs en -ant → adverbes en -amment",
              example: "Il a abordé le sujet avec tact et a expliqué les choses élégamment.",
              highlight: "élégamment",
            },
            {
              label: "Adjectifs en -ent → adverbes en -emment (sauf « lent »)",
              example: "Cela arrive fréquemment dans ce genre de situation.",
              highlight: "fréquemment",
            },
            {
              label: "Adjectifs gardant un -é avant -ment",
              example: "Elle a profondément réfléchi avant de prendre sa décision.",
              highlight: "profondément",
            },
          ],
        },
        {
          title: "Place aux temps simples",
          subcases: [
            {
              label: "Après le verbe pour les adverbes de manière et de degré",
              example: "Cet enfant parle déjà couramment trois langues.",
              highlight: "parle déjà couramment",
            },
            {
              label: "Adverbes de temps et de lieu : souvent en début ou fin de phrase",
              example: "Hier, nous avons rencontré Pierre dans le parc.",
              highlight: "Hier",
            },
            {
              label: "Adverbes modificateurs d'adjectifs ou d'autres adverbes : devant",
              example: "Cette idée est particulièrement intéressante et très bien expliquée.",
              highlight: "particulièrement",
            },
          ],
        },
        {
          title: "Place aux temps composés",
          subcases: [
            {
              label: "Adverbes courts (bien, mal, déjà, encore, souvent, toujours, vraiment, beaucoup, trop) → entre auxiliaire et participe",
              example: "Elle a beaucoup voyagé pendant ses études supérieures.",
              highlight: "a beaucoup voyagé",
            },
            {
              label: "Adverbes longs en -ment → généralement après le participe",
              example: "Elle a répondu honnêtement à toutes mes questions.",
              highlight: "a répondu honnêtement",
            },
            {
              label: "Adverbes de temps précis (hier, demain, lundi) → en tête ou fin de phrase",
              example: "Hier, j'ai longuement discuté avec mon directeur.",
              highlight: "Hier",
            },
          ],
        },
        {
          title: "Pièges fréquents",
          body:
            "Confusion classique : les adverbes en -amment et -emment se prononcent de la même manière (le « a » et le « e » sonnent /a/ dans les deux cas). Seule l'orthographe diffère, qui dépend de la terminaison de l'adjectif d'origine. « Gentiment » s'écrit avec un seul « l » (de l'ancien adjectif « gentil » dont le féminin était « gentille » mais l'adverbe garde l'ancienne forme). Ne pas placer un adverbe long entre l'auxiliaire et le participe : on dit « elle a répondu honnêtement », non « elle a honnêtement répondu » (qui est moins naturel, sauf insistance stylistique).",
        },
      ],
    },
    items: [
      {
        type: "transform",
        instruction:
          "Transformez l'adjectif entre parenthèses en adverbe et insérez-le à la bonne place.",
        source:
          "Le directeur a expliqué la situation. (clair)",
        answers: [
          "Le directeur a clairement expliqué la situation",
          "Le directeur a expliqué clairement la situation",
        ],
        rule:
          "« Clair » → féminin « claire » + -ment → clairement. Aux temps composés, un adverbe de manière en -ment peut se placer entre l'auxiliaire et le participe (a clairement expliqué) ou après le participe (a expliqué clairement). Les deux sont naturels.",
        consolidation: {
          type: "transform",
          instruction:
            "Transformez l'adjectif en adverbe et insérez-le à la bonne place.",
          source:
            "Elle a accepté notre proposition. (immédiat)",
          answers: [
            "Elle a immédiatement accepté notre proposition",
            "Elle a accepté immédiatement notre proposition",
          ],
          rule:
            "« Immédiat » → immédiate + -ment → immédiatement. Adverbe en -ment dérivé d'un adjectif terminé par une voyelle.",
        },
      },
      {
        type: "qcm",
        prompt: "Quelle est la forme adverbiale correcte de « patient » ?",
        context: "Il a attendu son tour ___ pendant plus de deux heures.",
        options: ["patiamment", "patientement", "patiemment"],
        correctIndex: 2,
        rule:
          "Les adjectifs terminés en -ent forment leur adverbe en -emment (et non en -entement ni en -amment). Patient → patiemment. Même règle pour : fréquent → fréquemment, évident → évidemment, prudent → prudemment.",
        consolidation: {
          type: "qcm",
          prompt: "Quelle est la forme correcte ?",
          context: "Cette erreur revient ___ dans son travail.",
          options: ["fréquentement", "fréquemment", "fréquamment"],
          correctIndex: 1,
          rule:
            "Fréquent (en -ent) → fréquemment. Attention à la double consonne « mm » et au -e initial (et non -a).",
        },
      },
      {
        type: "qcm",
        prompt: "Quelle est la forme adverbiale correcte de « savant » ?",
        context: "Le conférencier a exposé sa théorie ___ devant un auditoire conquis.",
        options: ["savantement", "savemment", "savamment"],
        correctIndex: 2,
        rule:
          "Les adjectifs en -ant forment leur adverbe en -amment. Savant → savamment. À ne pas confondre avec les adjectifs en -ent qui donnent -emment. Astuce : se rappeler de l'adjectif d'origine.",
        consolidation: {
          type: "qcm",
          prompt: "Quelle forme est correcte ?",
          context: "Elle a abordé ce sujet délicat ___, sans froisser personne.",
          options: ["élégantement", "élégamment", "élégemment"],
          correctIndex: 1,
          rule:
            "Élégant (en -ant) → élégamment. La double consonne « mm » est typique de cette terminaison.",
        },
      },
      {
        before: "Mon frère cuisine ",
        hint: "adverbe irrégulier dérivé de « bon »",
        after: " que moi, c'est pour cela qu'il s'occupe toujours du repas.",
        answers: ["mieux"],
        rule:
          "« Bon » → « bien » comme adverbe ; au comparatif, « bien » devient « mieux ». À ne pas confondre avec « meilleur », comparatif de l'adjectif « bon » (qui qualifie un nom). Ici, c'est l'action de cuisiner qui est qualifiée, donc adverbe → mieux.",
        consolidation: {
          before: "Cette équipe joue ",
          hint: "comparatif adverbial",
          after: " que l'année dernière, le travail porte ses fruits.",
          answers: ["mieux"],
        },
      },
      {
        type: "transform",
        instruction:
          "Placez l'adverbe « toujours » à la bonne position dans cette phrase au passé composé.",
        source:
          "Elle a refusé de prendre l'avion.",
        answers: [
          "Elle a toujours refusé de prendre l'avion",
        ],
        rule:
          "« Toujours » est un adverbe court et fréquent : aux temps composés, il se place entre l'auxiliaire et le participe. Mêmes règles : déjà, encore, souvent, jamais, bien, mal, vraiment, beaucoup, trop.",
        consolidation: {
          type: "transform",
          instruction:
            "Placez l'adverbe « déjà » à la bonne position.",
          source:
            "Nous avons rencontré cette personne quelque part.",
          answers: [
            "Nous avons déjà rencontré cette personne quelque part",
          ],
          rule:
            "« Déjà » se place entre l'auxiliaire et le participe : « avons déjà rencontré ». Position fixe au passé composé.",
        },
      },
      {
        type: "qcm",
        prompt:
          "Laquelle de ces phrases est la plus naturelle en français ?",
        context: "",
        options: [
          "Elle a honnêtement répondu à toutes mes questions.",
          "Elle a répondu honnêtement à toutes mes questions.",
          "Elle honnêtement a répondu à toutes mes questions.",
        ],
        correctIndex: 1,
        rule:
          "Les adverbes longs en -ment se placent généralement APRÈS le participe passé, contrairement aux adverbes courts (bien, déjà, toujours). « Elle a répondu honnêtement » est la forme standard. Placer l'adverbe entre auxiliaire et participe est possible mais marqué stylistiquement.",
        consolidation: {
          type: "qcm",
          prompt: "Quelle est la phrase la plus naturelle ?",
          context: "",
          options: [
            "Il a complètement oublié notre rendez-vous.",
            "Il a oublié complètement notre rendez-vous.",
            "Il complètement a oublié notre rendez-vous.",
          ],
          correctIndex: 0,
          rule:
            "Exception : certains adverbes en -ment très usuels (complètement, totalement, vraiment) acceptent volontiers la position entre auxiliaire et participe. « A complètement oublié » est même plus courant que « a oublié complètement ». À retenir parmi les exceptions, avec : vraiment, totalement, absolument, profondément.",
        },
      },
      {
        before: "Ce vieux monsieur m'a dit ",
        hint: "adverbe irrégulier dérivé de « gentil »",
        after: " qu'il connaissait mon grand-père de longue date.",
        answers: ["gentiment"],
        rule:
          "« Gentil » → « gentiment » (avec un seul « l ») et non « gentillement » comme on serait tenté de l'écrire. Forme irrégulière à mémoriser, l'une des plus piégeuses du français.",
        consolidation: {
          before: "Elle nous a accueillis ",
          hint: "adverbe irrégulier",
          after: " malgré l'heure tardive.",
          answers: ["gentiment"],
        },
      },
      {
        type: "transform",
        instruction:
          "Transformez l'adjectif « profond » en adverbe et insérez-le à la bonne place.",
        source:
          "Elle a réfléchi avant de prendre sa décision finale.",
        answers: [
          "Elle a profondément réfléchi avant de prendre sa décision finale",
          "Elle a réfléchi profondément avant de prendre sa décision finale",
        ],
        rule:
          "« Profond » → « profondément » (avec un accent aigu sur le -é, qui remplace le -e du féminin « profonde »). Même règle pour : précis → précisément, énorme → énormément, intense → intensément.",
        consolidation: {
          type: "transform",
          instruction:
            "Transformez l'adjectif en adverbe et placez-le.",
          source:
            "Ce dossier nous a impressionnés. (énorme)",
          answers: [
            "Ce dossier nous a énormément impressionnés",
            "Ce dossier nous a impressionnés énormément",
          ],
          rule:
            "« Énorme » → « énormément » (avec accent aigu sur le -é qui remplace le -e final de l'adjectif).",
        },
      },
    ],
  },

  /* ---------- 26. PRÉPOSITIONS ---------- */
  {
    id: "prepositions",
    number: 26,
    section: "Adverbes et prépositions",
    title: "Prépositions à, de, en",
    titleEm: "et les verbes",
    lesson: {
      intro:
        "Les prépositions sont la peur secrète de tous les apprenants : pourquoi « commencer à » mais « finir de » ? Pourquoi « penser à » mais « rêver de » ? Il n'existe pas de règle générale infaillible — il faut connaître les constructions verbe par verbe. Au B2, on attend la maîtrise des cas les plus fréquents et la capacité à raisonner sur les patrons qui se dégagent.",
      formation: [
        "Trois grandes prépositions devant un infinitif : à, de, ou rien (construction directe).",
        "Construction directe (sans préposition) : verbes de volonté, de désir, de mouvement, d'opinion. Ex. : vouloir partir, aimer voyager, aller chercher, penser pouvoir, espérer venir, sembler comprendre.",
        "Verbes avec « à » : commencer à, apprendre à, parvenir à, hésiter à, chercher à, s'attendre à, tenir à, songer à, renoncer à, contribuer à, réussir à, arriver à (= parvenir à).",
        "Verbes avec « de » : finir de, arrêter de, cesser de, oublier de, éviter de, refuser de, accepter de, décider de, choisir de, essayer de, tenter de, regretter de, se souvenir de, avoir besoin de, avoir envie de, s'occuper de, mériter de.",
        "Devant un nom (lieu, moyen, manière) : « en » exprime le moyen, la matière, la durée approximative ; « à » exprime la direction, la localisation, la possession ; « de » exprime l'origine, la possession, la cause, la matière (concurrence avec « en »).",
      ],
      irregulars: [],
      triggers: [
        {
          title: "Verbes + à + infinitif",
          subcases: [
            {
              label: "Apprentissage, effort, progression",
              example: "Cet enfant apprend à lire avec beaucoup de plaisir.",
              highlight: "apprend à lire",
            },
            {
              label: "Hésitation, recherche",
              example: "J'hésite à accepter cette nouvelle proposition.",
              highlight: "hésite à accepter",
            },
            {
              label: "Engagement, attachement",
              example: "Je tiens à vous remercier pour votre accueil chaleureux.",
              highlight: "tiens à vous remercier",
            },
            {
              label: "Réussite, parvenir à",
              example: "Nous avons enfin réussi à trouver une solution acceptable.",
              highlight: "réussi à trouver",
            },
          ],
        },
        {
          title: "Verbes + de + infinitif",
          subcases: [
            {
              label: "Arrêt, achèvement, refus",
              example: "Il a finalement décidé d'arrêter de fumer.",
              highlight: "décidé d'arrêter de fumer",
            },
            {
              label: "Tentative, essai, choix",
              example: "Elle essaie depuis longtemps de convaincre ses parents.",
              highlight: "essaie de convaincre",
            },
            {
              label: "Regret, mémoire, sentiment",
              example: "Je me souviens d'avoir lu ce livre il y a longtemps.",
              highlight: "me souviens d'avoir lu",
            },
            {
              label: "Conseil, ordre indirect, recommandation",
              example: "Le médecin lui a conseillé de prendre du repos.",
              highlight: "conseillé de prendre",
            },
          ],
        },
        {
          title: "Constructions directes (sans préposition)",
          subcases: [
            {
              label: "Verbes de volonté, désir, espoir",
              example: "Je voudrais sincèrement vous aider mais je ne peux pas.",
              highlight: "voudrais vous aider",
            },
            {
              label: "Verbes de perception et de jugement",
              example: "Je sens venir un orage, l'air est devenu lourd.",
              highlight: "sens venir",
            },
            {
              label: "Verbes modaux",
              example: "Tu devrais lui parler avant la fin de la semaine.",
              highlight: "devrais lui parler",
            },
            {
              label: "Verbes de mouvement (aller, venir, monter…)",
              example: "Je vais chercher du pain à la boulangerie d'à côté.",
              highlight: "vais chercher",
            },
          ],
        },
        {
          title: "Pièges fréquents",
          body:
            "Distinction entre « décider de » (active) et « se décider à » (avec hésitation préalable) : je décide de partir / je me décide à partir. Différence entre « venir » + infinitif (= je viens manger = j'arrive pour manger) et « venir de » + infinitif (= passé immédiat : je viens de manger = j'ai mangé il y a peu). Construction « commencer à » est la plus fréquente, mais « commencer par » existe aussi avec un sens différent : commencer par lire (= la première étape est de lire). Avec « penser » : penser à quelque chose (réfléchir) vs penser de quelque chose (opinion) vs penser + infinitif sans préposition (= avoir l'intention).",
        },
      ],
    },
    items: [
      {
        before: "J'ai enfin réussi ",
        hint: "préposition",
        after: " convaincre mes parents de me laisser partir seul en voyage.",
        answers: ["à"],
        rule:
          "« Réussir » se construit avec « à » + infinitif : réussir à faire, parvenir à finir, arriver à comprendre. Famille des verbes d'effort et de progression vers un but, qui prennent presque tous « à ».",
        consolidation: {
          before: "Cet enfant apprend ",
          hint: "préposition",
          after: " lire avec une vitesse étonnante.",
          answers: ["à"],
        },
      },
      {
        type: "qcm",
        prompt: "Quelle préposition convient ?",
        context: "Mon médecin m'a conseillé ___ faire du sport régulièrement.",
        options: ["à", "de", "—"],
        correctIndex: 1,
        rule:
          "« Conseiller » se construit avec « de » + infinitif. Famille des verbes de conseil, recommandation, ordre indirect : conseiller de, recommander de, suggérer de, ordonner de, défendre de, interdire de. Tous prennent « de ».",
        consolidation: {
          type: "qcm",
          prompt: "Quelle préposition convient ?",
          context: "Le règlement interdit ___ fumer dans tous les locaux.",
          options: ["à", "de", "—"],
          correctIndex: 1,
          rule:
            "« Interdire » + de + infinitif. Construction parallèle à « défendre de » et opposée à « permettre de » (qui prend aussi « de »).",
        },
      },
      {
        type: "qcm",
        prompt:
          "Laquelle de ces phrases est correctement construite ?",
        context: "",
        options: [
          "Je voudrais à vous remercier pour votre aide précieuse.",
          "Je voudrais de vous remercier pour votre aide précieuse.",
          "Je voudrais vous remercier pour votre aide précieuse.",
        ],
        correctIndex: 2,
        rule:
          "« Vouloir » se construit directement avec un infinitif, sans préposition. Même famille : pouvoir, devoir, savoir, aimer, désirer, espérer, préférer, oser, sembler, paraître. Tous se construisent sans préposition.",
        consolidation: {
          type: "qcm",
          prompt: "Laquelle est correcte ?",
          context: "",
          options: [
            "Elle espère à terminer son projet avant l'été.",
            "Elle espère de terminer son projet avant l'été.",
            "Elle espère terminer son projet avant l'été.",
          ],
          correctIndex: 2,
          rule:
            "« Espérer » + infinitif direct, comme « vouloir » et « souhaiter » (sans préposition). À distinguer de « rêver de » qui, lui, prend « de ».",
        },
      },
      {
        before: "Elle hésite encore ",
        hint: "préposition",
        after: " accepter cette proposition d'embauche très intéressante.",
        answers: ["à"],
        rule:
          "« Hésiter » + à + infinitif. Famille des verbes d'hésitation et de doute : hésiter à, tarder à, songer à, penser à (au sens de « envisager »).",
        consolidation: {
          before: "Je n'arrive pas ",
          hint: "préposition",
          after: " me décider, ce choix est trop important.",
          answers: ["à"],
        },
      },
      {
        type: "transform",
        instruction:
          "Réécrivez la phrase en utilisant « venir de » + infinitif pour exprimer un passé immédiat.",
        source:
          "Je l'ai vu partir il y a quelques secondes seulement.",
        answers: [
          "Je viens de le voir partir",
          "Je viens de le voir partir il y a quelques secondes seulement",
        ],
        rule:
          "« Venir de » + infinitif exprime un passé très récent (= il y a quelques instants). À distinguer de « venir » + infinitif sans préposition (= je viens pour faire). C'est la préposition qui fait toute la différence.",
        consolidation: {
          type: "transform",
          instruction:
            "Réécrivez avec « venir de » + infinitif.",
          source:
            "Elle a terminé son rapport à l'instant.",
          answers: [
            "Elle vient de terminer son rapport",
          ],
          rule:
            "« À l'instant » → « venir de » + infinitif. Construction très idiomatique pour exprimer un passé immédiat.",
        },
      },
      {
        type: "qcm",
        prompt: "Quelle préposition convient dans ce contexte ?",
        context:
          "Je pense souvent ___ mon enfance et aux moments passés chez mes grands-parents.",
        options: ["à", "de", "en"],
        correctIndex: 0,
        rule:
          "« Penser à quelque chose ou à quelqu'un » = avoir en tête, réfléchir à. À distinguer de « penser de quelque chose » = avoir une opinion sur (« qu'en penses-tu ? »). Et « penser » + infinitif sans préposition = avoir l'intention de.",
        consolidation: {
          type: "qcm",
          prompt: "Quelle préposition convient ?",
          context:
            "Que penses-tu ___ cette nouvelle proposition ? J'aimerais avoir ton avis.",
          options: ["à", "de", "en"],
          correctIndex: 1,
          rule:
            "« Penser de » = demander un avis, une opinion. Très spécifique à cette construction ; le reste du temps, « penser » prend « à ».",
        },
      },
      {
        before: "Le ministre a finalement décidé ",
        hint: "préposition",
        after: " annoncer sa démission lors de la conférence de presse.",
        answers: ["d'"],
        rule:
          "« Décider de » + infinitif. À distinguer de « se décider à » (qui implique une hésitation préalable surmontée). Devant une voyelle, « de » s'élide en « d' ».",
        consolidation: {
          before: "Nous avons accepté ",
          hint: "préposition",
          after: " participer à ce projet ambitieux.",
          answers: ["de"],
        },
      },
      {
        type: "transform",
        instruction:
          "Réécrivez la phrase en utilisant le verbe « tenir à » + infinitif pour exprimer un attachement, une volonté forte.",
        source:
          "Il est important pour moi de vous remercier personnellement.",
        answers: [
          "Je tiens à vous remercier personnellement",
        ],
        rule:
          "« Tenir à » + infinitif exprime un attachement fort, une volonté insistante. Tournure très idiomatique et plus élégante que « il est important pour moi de ». Au B2, c'est un marqueur de fluidité.",
        consolidation: {
          type: "transform",
          instruction:
            "Réécrivez en utilisant « tenir à » + infinitif.",
          source:
            "C'est important pour nous de vous présenter nos excuses.",
          answers: [
            "Nous tenons à vous présenter nos excuses",
          ],
          rule:
            "« Tenir à » s'accorde au sujet : je tiens, tu tiens, il tient, nous tenons, vous tenez, ils tiennent. Forme irrégulière à mémoriser.",
        },
      },
    ],
  },
];
