import React, { useState, useEffect, useRef } from "react";
import { Check, ArrowRight, RotateCcw, Sparkles, ArrowLeft } from "lucide-react";
import { CHAPTERS } from "./chapters.js";

/* ============================================================
   COULEURS & TYPO
   ============================================================ */
const PAPER = "#f3ead4";
const PAPER_LIGHT = "#faf4e0";
const INK = "#1c1d26";
const INK_SOFT = "#4a4b58";
const MUTE = "#8a8676";
const RULE_LINE = "#dccfa8";
const ACCENT = "#a83232";
const NAVY = "#1e3a5f";
const SUCCESS = "#4a7c4e";
const SERIF = '"Fraunces", "EB Garamond", Georgia, serif';

/* ============================================================
   DONNÉES DES CHAPITRES
   Tout est rédigé spécifiquement pour cette application.
   ============================================================ */


/* ============================================================
   UTILITAIRES
   ============================================================ */

function normalize(s) {
  return s
    .trim()
    .toLowerCase()
    .replace(/[\u2019\u2018]/g, "'")
    .replace(/\s+/g, " ")
    .replace(/[.!?]+$/, "");
}

function checkAnswer(input, answers) {
  const norm = normalize(input);
  return answers.some((a) => normalize(a) === norm);
}

/* ============================================================
   APP RACINE
   ============================================================ */

export default function GrammaireB2() {
  useEffect(() => {
    const id = "fraunces-font";
    if (document.getElementById(id)) return;
    const link = document.createElement("link");
    link.id = id;
    link.rel = "stylesheet";
    link.href =
      "https://fonts.googleapis.com/css2?family=Fraunces:ital,opsz,wght@0,9..144,300..700;1,9..144,300..700&display=swap";
    document.head.appendChild(link);
  }, []);

  const [view, setView] = useState("home"); // "home" | "chapter"
  const [currentChapterId, setCurrentChapterId] = useState(null);

  const openChapter = (id) => {
    setCurrentChapterId(id);
    setView("chapter");
    window.scrollTo({ top: 0, behavior: "instant" });
  };

  const goHome = () => {
    setView("home");
    setCurrentChapterId(null);
    window.scrollTo({ top: 0, behavior: "instant" });
  };

  return (
    <div
      style={{
        backgroundColor: PAPER,
        minHeight: "100vh",
        fontFamily: SERIF,
        color: INK,
        backgroundImage:
          "radial-gradient(rgba(0,0,0,0.025) 1px, transparent 1px)",
        backgroundSize: "3px 3px",
      }}
    >
      {view === "home" ? (
        <HomeScreen onSelect={openChapter} />
      ) : (
        <ChapterRunner
          chapter={CHAPTERS.find((c) => c.id === currentChapterId)}
          onGoHome={goHome}
        />
      )}

      <style>{`
        @keyframes fadeIn {
          from { opacity: 0; transform: translateY(4px); }
          to { opacity: 1; transform: translateY(0); }
        }
        input::placeholder { color: #c4b896; }
        button:focus-visible { outline: 2px solid ${NAVY}; outline-offset: 3px; }
      `}</style>
    </div>
  );
}

/* ============================================================
   HOME / SOMMAIRE
   ============================================================ */

function HomeScreen({ onSelect }) {
  // Regrouper les chapitres par section, en conservant l'ordre.
  const sections = [];
  for (const ch of CHAPTERS) {
    let s = sections.find((x) => x.name === ch.section);
    if (!s) {
      s = { name: ch.section, chapters: [] };
      sections.push(s);
    }
    s.chapters.push(ch);
  }

  return (
    <div
      style={{
        maxWidth: "640px",
        margin: "0 auto",
        padding: "48px 20px 80px",
        animation: "fadeIn 0.4s ease",
      }}
    >
      <header style={{ marginBottom: "48px" }}>
        <span
          style={{
            fontSize: "11px",
            letterSpacing: "0.22em",
            textTransform: "uppercase",
            color: MUTE,
            fontWeight: 500,
          }}
        >
          Niveau B2 · Défi
        </span>
        <h1
          style={{
            fontFamily: SERIF,
            fontWeight: 400,
            fontSize: "clamp(36px, 7vw, 52px)",
            lineHeight: 1.05,
            letterSpacing: "-0.02em",
            fontVariationSettings: '"opsz" 144',
            margin: "10px 0 16px",
          }}
        >
          Grammaire{" "}
          <em
            style={{
              fontStyle: "italic",
              color: ACCENT,
              fontVariationSettings: '"opsz" 144',
            }}
          >
            essentielle
          </em>
        </h1>
        <p
          style={{
            fontFamily: SERIF,
            fontStyle: "italic",
            fontSize: "18px",
            color: INK_SOFT,
            margin: 0,
            maxWidth: "50ch",
            lineHeight: 1.5,
            fontVariationSettings: '"opsz" 24',
          }}
        >
          Une leçon, huit exercices. Choisis un chapitre.
        </p>
      </header>

      {sections.map((sec) => (
        <section key={sec.name} style={{ marginBottom: "44px" }}>
          <div
            style={{
              display: "flex",
              alignItems: "center",
              gap: "12px",
              marginBottom: "18px",
            }}
          >
            <span
              style={{
                display: "inline-block",
                width: "22px",
                height: "1px",
                backgroundColor: NAVY,
              }}
            />
            <h2
              style={{
                fontSize: "11px",
                letterSpacing: "0.22em",
                textTransform: "uppercase",
                color: NAVY,
                margin: 0,
                fontWeight: 500,
              }}
            >
              {sec.name}
            </h2>
          </div>
          <ul
            style={{
              listStyle: "none",
              padding: 0,
              margin: 0,
            }}
          >
            {sec.chapters.map((ch) => (
              <ChapterRow key={ch.id} ch={ch} onSelect={onSelect} />
            ))}
          </ul>
        </section>
      ))}

      <p
        style={{
          marginTop: "56px",
          fontSize: "12px",
          color: MUTE,
          fontStyle: "italic",
          textAlign: "center",
          lineHeight: 1.5,
        }}
      >
        Autres chapitres à venir.
      </p>
    </div>
  );
}

function ChapterRow({ ch, onSelect }) {
  const [hover, setHover] = useState(false);
  return (
    <li
      style={{
        borderBottom: `1px solid ${RULE_LINE}`,
      }}
    >
      <button
        onClick={() => onSelect(ch.id)}
        onMouseEnter={() => setHover(true)}
        onMouseLeave={() => setHover(false)}
        style={{
          width: "100%",
          background: "none",
          border: "none",
          padding: "16px 0",
          textAlign: "left",
          cursor: "pointer",
          display: "flex",
          alignItems: "baseline",
          justifyContent: "space-between",
          gap: "16px",
          fontFamily: SERIF,
          color: INK,
          transition: "color 0.2s ease",
        }}
      >
        <span style={{ display: "flex", alignItems: "baseline", gap: "16px" }}>
          <span
            style={{
              fontSize: "13px",
              color: MUTE,
              fontVariantNumeric: "tabular-nums",
              fontStyle: "italic",
              minWidth: "20px",
            }}
          >
            {String(ch.number).padStart(2, "0")}
          </span>
          <span
            style={{
              fontFamily: SERIF,
              fontSize: "20px",
              fontVariationSettings: '"opsz" 36',
              color: hover ? ACCENT : INK,
              transition: "color 0.2s ease",
            }}
          >
            {ch.title}{" "}
            <em style={{ fontStyle: "italic", color: hover ? ACCENT : INK_SOFT }}>
              {ch.titleEm}
            </em>
          </span>
        </span>
        <ArrowRight
          size={16}
          strokeWidth={1.6}
          style={{
            color: hover ? ACCENT : MUTE,
            transition: "transform 0.2s ease, color 0.2s ease",
            transform: hover ? "translateX(3px)" : "translateX(0)",
            flexShrink: 0,
          }}
        />
      </button>
    </li>
  );
}

/* ============================================================
   RUNNER DE CHAPITRE (leçon + exercices + écran final)
   ============================================================ */

function ChapterRunner({ chapter, onGoHome }) {
  const [currentIndex, setCurrentIndex] = useState(0);
  const [phase, setPhase] = useState("lesson");
  const [input, setInput] = useState("");
  const [selectedIndex, setSelectedIndex] = useState(null);
  const [lastResult, setLastResult] = useState(null);
  const [score, setScore] = useState(0);
  const inputRef = useRef(null);

  const item = chapter.items[currentIndex];
  const total = chapter.items.length;
  const isConsolidation =
    phase === "consolidating" || phase === "consolidation-feedback";
  const isFeedback =
    phase === "feedback" || phase === "consolidation-feedback";
  const currentPrompt = isConsolidation ? item.consolidation : item;
  const promptType = (currentPrompt && currentPrompt.type) || "fill";
  const isInputDisabled = isFeedback || phase === "done" || phase === "lesson";

  useEffect(() => {
    if (
      (phase === "answering" || phase === "consolidating") &&
      promptType !== "qcm"
    ) {
      const t = setTimeout(() => inputRef.current?.focus(), 50);
      return () => clearTimeout(t);
    }
  }, [phase, currentIndex, promptType]);

  const canSubmit = () => {
    if (promptType === "qcm") return selectedIndex !== null;
    return input.trim().length > 0;
  };

  const evaluate = (prompt) => {
    if ((prompt.type || "fill") === "qcm") {
      return selectedIndex === prompt.correctIndex;
    }
    return checkAnswer(input, prompt.answers);
  };

  const handleSubmit = () => {
    if (!canSubmit() || isInputDisabled) return;
    if (phase === "answering") {
      const ok = evaluate(item);
      setLastResult(ok ? "right" : "wrong");
      if (ok) setScore((s) => s + 1);
      setPhase("feedback");
    } else if (phase === "consolidating") {
      const ok = evaluate(item.consolidation);
      setLastResult(ok ? "right" : "wrong");
      setPhase("consolidation-feedback");
    }
  };

  const handleAdvance = () => {
    if (phase === "feedback" && lastResult === "wrong") {
      setInput("");
      setSelectedIndex(null);
      setLastResult(null);
      setPhase("consolidating");
      return;
    }
    if (currentIndex >= total - 1) {
      setPhase("done");
      return;
    }
    setCurrentIndex((i) => i + 1);
    setInput("");
    setSelectedIndex(null);
    setLastResult(null);
    setPhase("answering");
  };

  const handleStartExercises = () => setPhase("answering");
  const handleRevisitLesson = () => setPhase("lesson");
  const handleRestart = () => {
    setCurrentIndex(0);
    setInput("");
    setSelectedIndex(null);
    setLastResult(null);
    setScore(0);
    setPhase("lesson");
  };

  const onKeyDown = (e) => {
    if (e.key === "Enter" && !e.shiftKey) {
      if (isFeedback) {
        e.preventDefault();
        handleAdvance();
      } else if (promptType !== "transform") {
        // for transform we allow Shift+Enter, but Enter still submits if single-line
        e.preventDefault();
        handleSubmit();
      }
    }
  };

  const advanceLabel = () => {
    if (phase === "feedback" && lastResult === "wrong") return "Cas similaire";
    if (
      currentIndex >= total - 1 &&
      (phase === "feedback" || phase === "consolidation-feedback")
    )
      return "Terminer";
    return "Continuer";
  };

  return (
    <div
      style={{
        maxWidth: "640px",
        margin: "0 auto",
        padding: "32px 20px 80px",
      }}
    >
      {/* Navigation barre */}
      <button
        onClick={onGoHome}
        style={{
          background: "none",
          border: "none",
          cursor: "pointer",
          fontFamily: SERIF,
          fontSize: "12px",
          fontStyle: "italic",
          color: NAVY,
          padding: 0,
          marginBottom: "20px",
          display: "inline-flex",
          alignItems: "center",
          gap: "6px",
          textDecoration: "underline",
          textUnderlineOffset: "3px",
          textDecorationColor: "#a0b4c8",
        }}
      >
        <ArrowLeft size={13} strokeWidth={2} />
        sommaire
      </button>

      {/* Header */}
      <header style={{ marginBottom: phase === "lesson" ? "28px" : "36px" }}>
        <div
          style={{
            display: "flex",
            justifyContent: "space-between",
            alignItems: "baseline",
            marginBottom: "8px",
          }}
        >
          <span
            style={{
              fontSize: "11px",
              letterSpacing: "0.22em",
              textTransform: "uppercase",
              color: MUTE,
              fontWeight: 500,
            }}
          >
            Chapitre {String(chapter.number).padStart(2, "0")} ·{" "}
            {phase === "lesson" ? "Leçon" : "Défi"}
          </span>
          {phase !== "done" && phase !== "lesson" && (
            <div
              style={{
                display: "flex",
                alignItems: "baseline",
                gap: "16px",
              }}
            >
              <button
                onClick={handleRevisitLesson}
                style={{
                  background: "none",
                  border: "none",
                  cursor: "pointer",
                  fontFamily: SERIF,
                  fontSize: "12px",
                  fontStyle: "italic",
                  color: NAVY,
                  padding: 0,
                  textDecoration: "underline",
                  textUnderlineOffset: "3px",
                  textDecorationColor: "#a0b4c8",
                }}
              >
                revoir la leçon
              </button>
              <span
                style={{
                  fontSize: "13px",
                  color: MUTE,
                  fontVariantNumeric: "tabular-nums",
                  fontStyle: "italic",
                }}
              >
                {currentIndex + 1}
                <span style={{ color: "#c4b896", margin: "0 4px" }}>/</span>
                {total}
              </span>
            </div>
          )}
        </div>
        <h1
          style={{
            fontFamily: SERIF,
            fontWeight: 400,
            fontSize: "clamp(30px, 6vw, 44px)",
            lineHeight: 1.05,
            letterSpacing: "-0.02em",
            fontVariationSettings: '"opsz" 144',
            margin: 0,
          }}
        >
          {chapter.title}{" "}
          <em
            style={{
              fontStyle: "italic",
              color: ACCENT,
              fontVariationSettings: '"opsz" 144',
            }}
          >
            {chapter.titleEm}
          </em>
        </h1>
      </header>

      {phase !== "done" && phase !== "lesson" && (
        <div
          style={{
            display: "flex",
            gap: "3px",
            marginBottom: "44px",
          }}
        >
          {chapter.items.map((_, i) => (
            <div
              key={i}
              style={{
                height: "2px",
                flex: 1,
                backgroundColor:
                  i < currentIndex
                    ? NAVY
                    : i === currentIndex
                    ? ACCENT
                    : RULE_LINE,
                transition: "background-color 0.4s ease",
              }}
            />
          ))}
        </div>
      )}

      {phase === "lesson" ? (
        <Lesson lesson={chapter.lesson} onStart={handleStartExercises} />
      ) : phase === "done" ? (
        <DoneScreen
          score={score}
          total={total}
          onRestart={handleRestart}
          onGoHome={onGoHome}
        />
      ) : (
        <Exercise
          item={item}
          currentPrompt={currentPrompt}
          isConsolidation={isConsolidation}
          isFeedback={isFeedback}
          lastResult={lastResult}
          phase={phase}
          input={input}
          setInput={setInput}
          selectedIndex={selectedIndex}
          setSelectedIndex={setSelectedIndex}
          onKeyDown={onKeyDown}
          isInputDisabled={isInputDisabled}
          inputRef={inputRef}
          handleSubmit={handleSubmit}
          handleAdvance={handleAdvance}
          advanceLabel={advanceLabel()}
          canSubmit={canSubmit()}
        />
      )}
    </div>
  );
}

/* ============================================================
   LEÇON
   ============================================================ */

function Lesson({ lesson, onStart }) {
  return (
    <div style={{ animation: "fadeIn 0.4s ease" }}>
      <p
        style={{
          fontFamily: SERIF,
          fontStyle: "italic",
          fontSize: "clamp(17px, 3vw, 19px)",
          lineHeight: 1.6,
          color: INK_SOFT,
          margin: "0 0 40px",
          fontVariationSettings: '"opsz" 24',
          maxWidth: "56ch",
        }}
      >
        {lesson.intro}
      </p>

      <Section title="Formation">
        {lesson.formation.map((p, i) => (
          <p key={i} style={pStyle}>
            {p}
          </p>
        ))}
      </Section>

      {lesson.irregulars && lesson.irregulars.length > 0 && (
        <Section title="Formes à mémoriser">
          <div
            style={{
              display: "flex",
              flexDirection: "column",
              gap: "10px",
              marginTop: "4px",
            }}
          >
            {lesson.irregulars.map(({ v, forms }) => (
              <div
                key={v}
                style={{
                  display: "flex",
                  flexDirection: "column",
                  gap: "2px",
                  paddingBottom: "10px",
                  borderBottom: `1px dotted ${RULE_LINE}`,
                }}
              >
                <span
                  style={{
                    fontFamily: SERIF,
                    fontStyle: "italic",
                    fontSize: "17px",
                    color: ACCENT,
                    fontVariationSettings: '"opsz" 24',
                  }}
                >
                  {v}
                </span>
                <span
                  style={{
                    fontFamily: SERIF,
                    fontSize: "15.5px",
                    color: INK_SOFT,
                    fontVariationSettings: '"opsz" 14',
                    lineHeight: 1.5,
                  }}
                >
                  {forms}
                </span>
              </div>
            ))}
          </div>
        </Section>
      )}

      <Section title="Quand l'employer ?">
        {lesson.triggers.map(({ title, body, note, subcases }, i) => (
          <div
            key={i}
            style={{
              marginBottom: i === lesson.triggers.length - 1 ? 0 : "24px",
            }}
          >
            <h3
              style={{
                fontFamily: SERIF,
                fontStyle: "italic",
                fontWeight: 400,
                fontSize: "17px",
                color: NAVY,
                margin: "0 0 6px",
                fontVariationSettings: '"opsz" 24',
              }}
            >
              {title}
            </h3>
            {body && (
              <p style={{ ...pStyle, whiteSpace: "pre-line", margin: 0 }}>
                {body}
              </p>
            )}
            {subcases &&
              subcases.map((sc, j) => (
                <div key={j} style={{ marginTop: j === 0 ? "8px" : "14px" }}>
                  <p style={{ ...pStyle, margin: "0 0 4px" }}>{sc.label}</p>
                  <p
                    style={{
                      fontFamily: SERIF,
                      fontSize: "15.5px",
                      fontStyle: "italic",
                      color: INK_SOFT,
                      margin: 0,
                      paddingLeft: "14px",
                      borderLeft: `2px solid ${RULE_LINE}`,
                      fontVariationSettings: '"opsz" 14',
                      lineHeight: 1.5,
                    }}
                  >
                    <HighlightedExample
                      text={sc.example}
                      highlight={sc.highlight}
                    />
                  </p>
                </div>
              ))}
            {note && (
              <p
                style={{
                  ...pStyle,
                  fontStyle: "italic",
                  color: MUTE,
                  marginTop: "6px",
                  fontSize: "14px",
                }}
              >
                {note}
              </p>
            )}
          </div>
        ))}
      </Section>

      <div
        style={{
          marginTop: "48px",
          paddingTop: "32px",
          borderTop: `1px solid ${RULE_LINE}`,
          display: "flex",
          justifyContent: "space-between",
          alignItems: "center",
          flexWrap: "wrap",
          gap: "16px",
        }}
      >
        <p
          style={{
            fontFamily: SERIF,
            fontStyle: "italic",
            fontSize: "15px",
            color: MUTE,
            margin: 0,
          }}
        >
          8 items, niveau défi.
        </p>
        <Button
          onClick={onStart}
          primary
          label="Commencer les exercices"
          icon={<ArrowRight size={16} strokeWidth={2} />}
        />
      </div>
    </div>
  );
}

/* ============================================================
   EXERCICE
   ============================================================ */

function Exercise({
  item,
  currentPrompt,
  isConsolidation,
  isFeedback,
  lastResult,
  phase,
  input,
  setInput,
  selectedIndex,
  setSelectedIndex,
  onKeyDown,
  isInputDisabled,
  inputRef,
  handleSubmit,
  handleAdvance,
  advanceLabel,
  canSubmit,
}) {
  const type = currentPrompt.type || "fill";

  // Étiquette du haut : « Cas voisin » prime, sinon on annonce le format non-standard
  let topLabel = null;
  if (isConsolidation) topLabel = "Cas voisin";
  else if (type === "qcm") topLabel = "Choix multiple";
  else if (type === "transform") topLabel = "Transformation";

  // Réponse attendue à afficher en cas d'erreur
  let expectedAnswer = "";
  if (type === "qcm") {
    expectedAnswer = currentPrompt.options[currentPrompt.correctIndex];
  } else {
    expectedAnswer = currentPrompt.answers && currentPrompt.answers[0];
  }

  return (
    <div>
      {topLabel && (
        <div
          style={{
            display: "flex",
            alignItems: "center",
            gap: "10px",
            marginBottom: "20px",
            color: NAVY,
          }}
        >
          <span
            style={{
              display: "inline-block",
              width: "20px",
              height: "1px",
              backgroundColor: NAVY,
            }}
          />
          <span
            style={{
              fontSize: "11px",
              letterSpacing: "0.22em",
              textTransform: "uppercase",
              fontWeight: 500,
            }}
          >
            {topLabel}
          </span>
        </div>
      )}

      {type === "fill" && (
        <FillPrompt
          currentPrompt={currentPrompt}
          inputRef={inputRef}
          input={input}
          setInput={setInput}
          onKeyDown={onKeyDown}
          isInputDisabled={isInputDisabled}
          isFeedback={isFeedback}
          lastResult={lastResult}
        />
      )}

      {type === "qcm" && (
        <QcmPrompt
          currentPrompt={currentPrompt}
          selectedIndex={selectedIndex}
          setSelectedIndex={setSelectedIndex}
          isInputDisabled={isInputDisabled}
          isFeedback={isFeedback}
          lastResult={lastResult}
        />
      )}

      {type === "transform" && (
        <TransformPrompt
          currentPrompt={currentPrompt}
          inputRef={inputRef}
          input={input}
          setInput={setInput}
          onKeyDown={onKeyDown}
          isInputDisabled={isInputDisabled}
          isFeedback={isFeedback}
          lastResult={lastResult}
        />
      )}

      <div style={{ minHeight: "180px", marginTop: "32px" }}>
        {isFeedback && (
          <div
            style={{
              paddingTop: "24px",
              borderTop: `1px solid ${RULE_LINE}`,
              animation: "fadeIn 0.3s ease",
            }}
          >
            {lastResult === "right" ? (
              <div
                style={{
                  display: "flex",
                  alignItems: "center",
                  gap: "12px",
                  color: SUCCESS,
                }}
              >
                <Check size={20} strokeWidth={2.2} />
                <span
                  style={{
                    fontFamily: SERIF,
                    fontStyle: "italic",
                    fontSize: "20px",
                    fontVariationSettings: '"opsz" 36',
                  }}
                >
                  {phase === "consolidation-feedback"
                    ? "Parfait — la règle est consolidée."
                    : "C'est juste."}
                </span>
              </div>
            ) : (
              <>
                <p
                  style={{
                    fontSize: "11px",
                    letterSpacing: "0.22em",
                    textTransform: "uppercase",
                    color: MUTE,
                    margin: "0 0 10px",
                    fontWeight: 500,
                  }}
                >
                  {type === "qcm" ? "Bonne réponse" : "Forme attendue"}
                </p>
                <p
                  style={{
                    fontFamily: SERIF,
                    fontStyle: "italic",
                    fontSize: type === "transform" ? "22px" : "28px",
                    color: ACCENT,
                    fontWeight: 500,
                    margin: "0 0 22px",
                    fontVariationSettings: '"opsz" 60',
                    lineHeight: type === "transform" ? 1.4 : 1.2,
                    maxWidth: "56ch",
                  }}
                >
                  {expectedAnswer}
                </p>
                <p
                  style={{
                    fontSize: "15.5px",
                    lineHeight: 1.65,
                    color: INK_SOFT,
                    margin: 0,
                    fontVariationSettings: '"opsz" 14',
                    maxWidth: "56ch",
                  }}
                >
                  {currentPrompt.rule || item.rule}
                </p>
              </>
            )}
          </div>
        )}
      </div>

      <div
        style={{
          display: "flex",
          justifyContent: "flex-end",
          marginTop: "8px",
        }}
      >
        {!isFeedback ? (
          <Button
            onClick={handleSubmit}
            disabled={!canSubmit}
            primary
            label="Valider"
          />
        ) : (
          <Button
            onClick={handleAdvance}
            primary
            label={advanceLabel}
            icon={<ArrowRight size={16} strokeWidth={2} />}
          />
        )}
      </div>

      <p
        style={{
          fontSize: "12px",
          color: MUTE,
          marginTop: "32px",
          fontStyle: "italic",
          textAlign: "center",
          lineHeight: 1.5,
        }}
      >
        {type === "qcm"
          ? "Sélectionne une option puis valide."
          : "Appuie sur Entrée pour valider ou avancer."}
      </p>
    </div>
  );
}

/* --- Fill-in (formulation à trou) --- */
function FillPrompt({
  currentPrompt,
  inputRef,
  input,
  setInput,
  onKeyDown,
  isInputDisabled,
  isFeedback,
  lastResult,
}) {
  return (
    <p
      style={{
        fontFamily: SERIF,
        fontSize: "clamp(20px, 3.6vw, 26px)",
        lineHeight: 1.7,
        fontVariationSettings: '"opsz" 36',
        fontWeight: 400,
        margin: "0 0 8px",
      }}
    >
      {currentPrompt.before}
      <span style={{ display: "inline-block", whiteSpace: "nowrap" }}>
        <input
          ref={inputRef}
          type="text"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={onKeyDown}
          disabled={isInputDisabled}
          autoComplete="off"
          autoCapitalize="off"
          autoCorrect="off"
          spellCheck={false}
          style={{
            backgroundColor: "transparent",
            border: "none",
            borderBottom: `2px solid ${
              isFeedback
                ? lastResult === "right"
                  ? SUCCESS
                  : ACCENT
                : "#c4b896"
            }`,
            outline: "none",
            fontFamily: SERIF,
            fontSize: "inherit",
            fontStyle: "italic",
            fontVariationSettings: '"opsz" 36',
            color: isFeedback
              ? lastResult === "right"
                ? SUCCESS
                : ACCENT
              : INK,
            padding: "0 6px 3px",
            width: "7em",
            textAlign: "center",
            WebkitTextFillColor: isFeedback
              ? lastResult === "right"
                ? SUCCESS
                : ACCENT
              : INK,
            opacity: 1,
          }}
        />
        <span
          style={{
            fontSize: "0.62em",
            color: MUTE,
            fontStyle: "italic",
            marginLeft: "6px",
            fontVariationSettings: '"opsz" 14',
          }}
        >
          ({currentPrompt.hint})
        </span>
      </span>
      {currentPrompt.after}
    </p>
  );
}

/* --- QCM --- */
function QcmPrompt({
  currentPrompt,
  selectedIndex,
  setSelectedIndex,
  isInputDisabled,
  isFeedback,
  lastResult,
}) {
  const correct = currentPrompt.correctIndex;
  const letters = ["A", "B", "C", "D", "E"];

  return (
    <div>
      <p
        style={{
          fontFamily: SERIF,
          fontSize: "clamp(17px, 2.6vw, 19px)",
          lineHeight: 1.55,
          color: INK_SOFT,
          margin: "0 0 14px",
          fontStyle: "italic",
          fontVariationSettings: '"opsz" 24',
        }}
      >
        {currentPrompt.prompt}
      </p>

      {currentPrompt.context && (
        <p
          style={{
            fontFamily: SERIF,
            fontSize: "clamp(19px, 3.2vw, 23px)",
            lineHeight: 1.6,
            color: INK,
            margin: "0 0 26px",
            paddingLeft: "16px",
            borderLeft: `2px solid ${RULE_LINE}`,
            fontVariationSettings: '"opsz" 36',
          }}
        >
          {currentPrompt.context}
        </p>
      )}

      <div style={{ display: "flex", flexDirection: "column", gap: "8px" }}>
        {currentPrompt.options.map((opt, i) => {
          const isSelected = selectedIndex === i;
          const isCorrect = i === correct;
          let bg = "transparent";
          let border = `1px solid ${RULE_LINE}`;
          let color = INK;
          let letterColor = MUTE;

          if (isFeedback) {
            if (isCorrect) {
              bg = "rgba(74, 124, 78, 0.08)";
              border = `1px solid ${SUCCESS}`;
              color = SUCCESS;
              letterColor = SUCCESS;
            } else if (isSelected) {
              bg = "rgba(168, 50, 50, 0.06)";
              border = `1px solid ${ACCENT}`;
              color = ACCENT;
              letterColor = ACCENT;
            }
          } else if (isSelected) {
            bg = "rgba(28, 29, 38, 0.06)";
            border = `1px solid ${INK}`;
            letterColor = INK;
          }

          return (
            <button
              key={i}
              onClick={() => !isInputDisabled && setSelectedIndex(i)}
              disabled={isInputDisabled}
              style={{
                display: "flex",
                alignItems: "baseline",
                gap: "16px",
                padding: "14px 18px",
                background: bg,
                border,
                borderRadius: "2px",
                cursor: isInputDisabled ? "default" : "pointer",
                textAlign: "left",
                fontFamily: SERIF,
                fontSize: "17px",
                fontVariationSettings: '"opsz" 24',
                color,
                transition: "all 0.18s ease",
                width: "100%",
              }}
            >
              <span
                style={{
                  fontFamily: SERIF,
                  fontStyle: "italic",
                  fontSize: "15px",
                  color: letterColor,
                  fontVariationSettings: '"opsz" 24',
                  minWidth: "16px",
                }}
              >
                {letters[i]}
              </span>
              <span style={{ flex: 1 }}>{opt}</span>
              {isFeedback && isCorrect && (
                <Check size={16} strokeWidth={2.2} style={{ color: SUCCESS }} />
              )}
            </button>
          );
        })}
      </div>
    </div>
  );
}

/* --- Transformation --- */
function TransformPrompt({
  currentPrompt,
  inputRef,
  input,
  setInput,
  onKeyDown,
  isInputDisabled,
  isFeedback,
  lastResult,
}) {
  return (
    <div>
      <p
        style={{
          fontFamily: SERIF,
          fontSize: "clamp(16px, 2.4vw, 17px)",
          lineHeight: 1.55,
          color: INK_SOFT,
          margin: "0 0 16px",
          fontStyle: "italic",
          fontVariationSettings: '"opsz" 24',
        }}
      >
        {currentPrompt.instruction}
      </p>

      <p
        style={{
          fontFamily: SERIF,
          fontSize: "clamp(19px, 3.2vw, 23px)",
          lineHeight: 1.55,
          color: INK,
          margin: "0 0 22px",
          paddingLeft: "16px",
          borderLeft: `2px solid ${RULE_LINE}`,
          fontVariationSettings: '"opsz" 36',
        }}
      >
        « {currentPrompt.source} »
      </p>

      <textarea
        ref={inputRef}
        value={input}
        onChange={(e) => setInput(e.target.value)}
        onKeyDown={onKeyDown}
        disabled={isInputDisabled}
        autoComplete="off"
        autoCapitalize="off"
        autoCorrect="off"
        spellCheck={false}
        rows={2}
        placeholder="Votre réécriture…"
        style={{
          width: "100%",
          minHeight: "70px",
          resize: "vertical",
          backgroundColor: PAPER_LIGHT,
          border: `1px solid ${
            isFeedback
              ? lastResult === "right"
                ? SUCCESS
                : ACCENT
              : "#c4b896"
          }`,
          borderRadius: "2px",
          outline: "none",
          fontFamily: SERIF,
          fontSize: "17px",
          fontStyle: "italic",
          fontVariationSettings: '"opsz" 24',
          color: isFeedback
            ? lastResult === "right"
              ? SUCCESS
              : ACCENT
            : INK,
          padding: "12px 14px",
          lineHeight: 1.5,
          boxSizing: "border-box",
          WebkitTextFillColor: isFeedback
            ? lastResult === "right"
              ? SUCCESS
              : ACCENT
            : INK,
          opacity: 1,
        }}
      />
    </div>
  );
}

/* ============================================================
   ÉCRAN FINAL
   ============================================================ */

function DoneScreen({ score, total, onRestart, onGoHome }) {
  const pct = score / total;
  let remark;
  if (pct === 1) remark = "Maîtrise complète. Tu peux passer au chapitre suivant.";
  else if (pct >= 0.85) remark = "Excellente performance — quelques nuances à raffiner.";
  else if (pct >= 0.6) remark = "Solide. Reviens revoir les cas qui t'ont fait hésiter.";
  else if (pct >= 0.4) remark = "Bon début. Une seconde session consolidera les règles.";
  else remark = "Ce chapitre mérite une seconde lecture — tu vas progresser vite.";

  return (
    <div style={{ paddingTop: "24px", animation: "fadeIn 0.4s ease" }}>
      <div
        style={{
          display: "flex",
          alignItems: "center",
          gap: "10px",
          marginBottom: "24px",
          color: NAVY,
        }}
      >
        <Sparkles size={16} />
        <span
          style={{
            fontSize: "11px",
            letterSpacing: "0.22em",
            textTransform: "uppercase",
            fontWeight: 500,
          }}
        >
          Fin du chapitre
        </span>
      </div>
      <p
        style={{
          fontFamily: SERIF,
          fontSize: "13px",
          letterSpacing: "0.2em",
          textTransform: "uppercase",
          color: MUTE,
          margin: "0 0 8px",
        }}
      >
        Score
      </p>
      <p
        style={{
          fontFamily: SERIF,
          fontSize: "clamp(56px, 12vw, 88px)",
          fontWeight: 300,
          lineHeight: 1,
          margin: "0 0 24px",
          fontVariationSettings: '"opsz" 144',
          letterSpacing: "-0.03em",
        }}
      >
        {score}
        <span style={{ color: RULE_LINE, fontStyle: "italic", margin: "0 8px" }}>
          /
        </span>
        <span style={{ color: MUTE, fontStyle: "italic" }}>{total}</span>
      </p>
      <p
        style={{
          fontFamily: SERIF,
          fontStyle: "italic",
          fontSize: "20px",
          lineHeight: 1.45,
          color: INK_SOFT,
          margin: "0 0 40px",
          maxWidth: "30ch",
          fontVariationSettings: '"opsz" 36',
        }}
      >
        {remark}
      </p>
      <div style={{ display: "flex", gap: "12px", flexWrap: "wrap" }}>
        <Button
          onClick={onRestart}
          primary
          label="Recommencer"
          icon={<RotateCcw size={15} strokeWidth={2} />}
        />
        <Button
          onClick={onGoHome}
          label="Retour au sommaire"
          icon={<ArrowLeft size={15} strokeWidth={2} />}
          iconFirst
        />
      </div>
    </div>
  );
}

/* ============================================================
   COMPOSANTS PARTAGÉS
   ============================================================ */

const pStyle = {
  fontFamily: SERIF,
  fontSize: "15.5px",
  lineHeight: 1.65,
  color: INK_SOFT,
  margin: "0 0 10px",
  fontVariationSettings: '"opsz" 14',
  maxWidth: "60ch",
};

function Section({ title, children }) {
  return (
    <section style={{ marginBottom: "36px" }}>
      <div
        style={{
          display: "flex",
          alignItems: "center",
          gap: "12px",
          marginBottom: "16px",
        }}
      >
        <span
          style={{
            display: "inline-block",
            width: "22px",
            height: "1px",
            backgroundColor: NAVY,
          }}
        />
        <h2
          style={{
            fontSize: "11px",
            letterSpacing: "0.22em",
            textTransform: "uppercase",
            color: NAVY,
            margin: 0,
            fontWeight: 500,
          }}
        >
          {title}
        </h2>
      </div>
      {children}
    </section>
  );
}

function HighlightedExample({ text, highlight }) {
  if (!highlight) return <>{text}</>;
  const idx = text.toLowerCase().indexOf(highlight.toLowerCase());
  if (idx === -1) return <>{text}</>;
  return (
    <>
      {text.slice(0, idx)}
      <span style={{ color: ACCENT, fontWeight: 500 }}>
        {text.slice(idx, idx + highlight.length)}
      </span>
      {text.slice(idx + highlight.length)}
    </>
  );
}

function Button({ onClick, disabled, primary, label, icon, iconFirst }) {
  const [hover, setHover] = useState(false);
  const base = {
    fontFamily: SERIF,
    fontStyle: "italic",
    fontSize: "16px",
    fontVariationSettings: '"opsz" 24',
    padding: "12px 24px",
    borderRadius: "2px",
    cursor: disabled ? "not-allowed" : "pointer",
    letterSpacing: "0.01em",
    transition: "all 0.2s ease",
    display: "inline-flex",
    alignItems: "center",
    gap: "8px",
    opacity: disabled ? 0.35 : 1,
  };
  const primaryStyle = {
    ...base,
    border: "none",
    backgroundColor: hover && !disabled ? "#2a2c38" : INK,
    color: PAPER_LIGHT,
  };
  const secondaryStyle = {
    ...base,
    border: `1px solid ${hover && !disabled ? INK : "#c4b896"}`,
    backgroundColor: "transparent",
    color: INK,
  };
  return (
    <button
      onClick={onClick}
      disabled={disabled}
      onMouseEnter={() => setHover(true)}
      onMouseLeave={() => setHover(false)}
      style={primary ? primaryStyle : secondaryStyle}
    >
      {iconFirst && icon}
      <span>{label}</span>
      {!iconFirst && icon}
    </button>
  );
}
