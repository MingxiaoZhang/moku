/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs",   // Rust files containing Yew components
    "./index.html",    // Main HTML file
  ],
  theme: {
    extend: {},
  },
  plugins: [],
};

