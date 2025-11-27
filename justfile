css:
  tailwindcss -i ./tailwind.css -o ./packages/web/assets/tailwind.css --watch &
  tailwindcss -i ./tailwind.css -o ./packages/desktop/assets/tailwind.css --watch &
  wait

serve:
  dx serve
