import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [translatedWords, setTranslatedWords] = createSignal("");
  const [words, setWords] = createSignal("");

  async function translate() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setTranslatedWords(await invoke("translate_from_en_to_ru", { word: words() }));
  }


  
  

  return (
    <div class="container">

    <form
      class="list_element"
      onChange={(e) => {
        e.preventDefault();
        translate();
      }}>
      <input
        id="greet-input"
        onChange={(e) => setWords(e.currentTarget.value)}
        placeholder="Enter a name..."
      />
      <input
        id="greet-input"
        value={translatedWords()}
        placeholder="Enter a name..."
      />
    </form>
    

    <p>{translatedWords()}</p>
  </div>
  );
}

export default App;
