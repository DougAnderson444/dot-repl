css:
  tailwindcss -i ./tailwind.css -o ./packages/web/assets/tailwind.css --watch

css-desktop:
  tailwindcss -i ./tailwind.css -o ./packages/desktop/assets/tailwind.css --watch
serve:
  dx serve
