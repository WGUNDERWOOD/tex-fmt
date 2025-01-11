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
document.getElementById('submitButton').addEventListener(
  'click', async () => {
    const inputText = document.getElementById('textInput').value;
    const outputBox = document.getElementById('textOutput');
    const logBox = document.getElementById('textLog');
    try {
      const configText = document.getElementById('textConfig').value;
      const result = await main(inputText, configText);
      outputBox.value = result.output;
      logBox.value = result.logs;
    } catch (error) {
      console.error('Error calling WebAssembly function:', error);
      alert('An error occurred. Check the console for details.');
    }
  }
);

// Copy output text to clipboard
document.getElementById('copyButton').addEventListener(
  'click', () => {
    const outputBox = document.getElementById('textOutput');
    outputBox.select();
    outputBox.setSelectionRange(0, 99999);
    try {
      document.execCommand('copy');
      alert('Copied to clipboard:\n\n' + outputBox.value);
    } catch (err) {
      console.error('Failed to copy text: ', err);
    }
  }
);
