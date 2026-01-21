web: css-web
  dx serve --package web

css-web:
  tailwindcss -i ./tailwind.css -o ./packages/web/assets/tailwind.css

css-web-watch:
  tailwindcss -i ./tailwind.css -o ./packages/web/assets/tailwind.css --watch

desktop: css-desktop
  cd packages/desktop
  dx serve --package dot-repl-desktop

css-desktop:
  tailwindcss -i ./tailwind.css -o ./packages/desktop/assets/tailwind.css

install-tailwind:
  curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/download/v4.1.17/tailwindcss-linux-x64
  chmod +x tailwindcss-linux-x64
  sudo mv tailwindcss-linux-x64 /usr/local/bin/tailwindcss
