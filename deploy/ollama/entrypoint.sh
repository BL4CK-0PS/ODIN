#!/bin/sh
set -e

ollama serve &
sleep 3

if [ -n "$OLLAMA_EMBED_MODEL" ]; then
  echo "Pulling embedding model: $OLLAMA_EMBED_MODEL"
  ollama pull "$OLLAMA_EMBED_MODEL"
fi

if [ -n "$OLLAMA_REASON_MODEL" ]; then
  echo "Pulling reasoning model: $OLLAMA_REASON_MODEL"
  ollama pull "$OLLAMA_REASON_MODEL"
fi

echo "Model pull complete. Ollama ready."

wait
