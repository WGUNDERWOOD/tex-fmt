// Import wasm
import init, { main } from './pkg/tex_fmt.js';

// Initialize wasm
(async () => {
  try {
    await init();
    console.log('WASM initialized successfully.');
  } catch (error) {
    console.error('Error initializing WASM :', error);
    alert('Failed to initialize WASM. Check console for details.');
  }
})();

// Submit button logic
document.getElementById('formatButton').addEventListener(
  'click', async () => {
    const copyMessage = document.getElementById('copyMessage');
    copyMessage.innerText = ""
    const inputText = document.getElementById('inputText').value;
    const outputText = document.getElementById('outputText');
    const logText = document.getElementById('logText');
    try {
      const configText = document.getElementById('configText').value;
      const result = await main(inputText, configText);
      outputText.value = result.output;
      logText.value = result.logs;
    } catch (error) {
      console.error('Error calling WebAssembly function:', error);
      alert('An error occurred. Check the console for details.');
    }
  }
);

// Copy output text to clipboard
document.getElementById('copyButton').addEventListener(
  'click', () => {
    const outputText = document.getElementById('outputText');
    outputText.select();
    outputText.setSelectionRange(0, 99999);
    try {
      document.execCommand('copy');
      const copyMessage = document.getElementById('copyMessage');
      copyMessage.innerText = "Copied!"
      outputText.blur();
    } catch (err) {
      console.error('Failed to copy text: ', err);
    }
  }
);
