/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: [
    "../../packages/ui/src/**/*.{rs,html,css}",
    "./src/**/*.{rs,html,css}",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
};
