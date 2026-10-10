#!/usr/bin/env sh
# Refresh data/gamecontrollerdb-linux.txt from SDL_GameControllerDB (zlib).
set -eu
cd "$(dirname "$0")/.."
curl -fsSL https://raw.githubusercontent.com/mdqinc/SDL_GameControllerDB/master/gamecontrollerdb.txt \
  | grep 'platform:Linux' > data/gamecontrollerdb-linux.txt
curl -fsSL https://raw.githubusercontent.com/mdqinc/SDL_GameControllerDB/master/LICENSE > data/LICENSE-SDL_GameControllerDB
wc -l data/gamecontrollerdb-linux.txt
