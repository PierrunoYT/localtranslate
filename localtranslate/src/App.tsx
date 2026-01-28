import { useState } from "react";
import "./App.css";

function App() {
  const [sourceText, setSourceText] = useState("");
  const [translatedText, setTranslatedText] = useState("");
  const [sourceLang, setSourceLang] = useState("en");
  const [targetLang, setTargetLang] = useState("es");

  const languages = [
    { code: "en", name: "English" },
    { code: "es", name: "Spanish" },
    { code: "fr", name: "French" },
    { code: "de", name: "German" },
    { code: "it", name: "Italian" },
    { code: "pt", name: "Portuguese" },
    { code: "ja", name: "Japanese" },
    { code: "zh", name: "Chinese" },
  ];

  const handleTranslate = () => {
    // Mock translation - replace with actual translation logic
    setTranslatedText(`[Translated from ${sourceLang} to ${targetLang}]\n${sourceText}`);
  };

  const handleSwapLanguages = () => {
    const tempLang = sourceLang;
    setSourceLang(targetLang);
    setTargetLang(tempLang);
    const tempText = sourceText;
    setSourceText(translatedText);
    setTranslatedText(tempText);
  };

  return (
    <main className="app-container">
      <header className="app-header">
        <h1>LocalTranslate</h1>
      </header>

      <div className="translation-panel">
        <div className="language-selector">
          <select
            value={sourceLang}
            onChange={(e) => setSourceLang(e.target.value)}
            className="lang-select"
          >
            {languages.map((lang) => (
              <option key={lang.code} value={lang.code}>
                {lang.name}
              </option>
            ))}
          </select>

          <button
            onClick={handleSwapLanguages}
            className="swap-button"
            title="Swap languages"
          >
            ⇄
          </button>

          <select
            value={targetLang}
            onChange={(e) => setTargetLang(e.target.value)}
            className="lang-select"
          >
            {languages.map((lang) => (
              <option key={lang.code} value={lang.code}>
                {lang.name}
              </option>
            ))}
          </select>
        </div>

        <div className="translation-area">
          <div className="text-area-container">
            <textarea
              className="text-input"
              placeholder="Enter text to translate..."
              value={sourceText}
              onChange={(e) => setSourceText(e.target.value)}
            />
          </div>

          <div className="text-area-container">
            <textarea
              className="text-output"
              placeholder="Translation will appear here..."
              value={translatedText}
              readOnly
            />
          </div>
        </div>

        <button onClick={handleTranslate} className="translate-button">
          Translate
        </button>
      </div>
    </main>
  );
}

export default App;
