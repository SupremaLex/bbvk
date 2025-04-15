set -e

echo "Installing bbvk..."

if [ "$(uname)" != "Darwin" ]; then
  echo "This script is only for macOS."
  exit 1
fi

if [ ! -d "/Users/$USER/.bin/" ]; then
  mkdir -p "/Users/$USER/.bin/"
fi

if [ ! -f "/Users/$USER/.bin/bbvk" ]; then
  wget "https://github.com/rarimo/bbvk/releases/download/v0.1.0/bbvk" -O "/Users/$USER/.bin/bbvk"
fi

chmod +x "/Users/$USER/.bin/bbvk"

if [ ! -f "/Users/$USER/.zshrc" ]; then
  touch "/Users/$USER/.zshrc"
fi
if ! grep -q 'export PATH="$PATH:$HOME/.bin"' "/Users/$USER/.zshrc"; then
  echo 'export PATH="$PATH:$HOME/.bin"' >> "/Users/$USER/.zshrc"
fi

echo "bbvk is installed"
