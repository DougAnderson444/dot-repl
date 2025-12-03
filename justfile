css:
  tailwindcss -i ./tailwind.css -o ./packages/web/assets/tailwind.css --watch

css-desktop:
  tailwindcss -i ./tailwind.css -o ./packages/desktop/assets/tailwind.css --watch
serve:
  dx serve

install-tailwind:
  curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/download/v4.1.17/tailwindcss-linux-x64
  chmod +x tailwindcss-linux-x64
  sudo mv tailwindcss-linux-x64 /usr/local/bin/tailwindcss
