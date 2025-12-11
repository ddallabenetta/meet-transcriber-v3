#!/bin/bash

# Script per testare il Python sidecar

echo "🧪 Test Python Sidecar"
echo "======================="
echo ""

# 1. Test import whisper
echo "1. Verifica import whisper..."
cd "$(dirname "$0")/python"
source venv/bin/activate
python3 -c "import whisper; print('✅ Whisper importato correttamente')" || { echo "❌ Errore import whisper"; exit 1; }
echo ""

# 2. Test modelli disponibili
echo "2. Modelli disponibili:"
python3 -c "import whisper; print(whisper.available_models())"
echo ""

# 3. Test script main.py
echo "3. Test esecuzione main.py..."
echo '{"command": "transcribe", "audio_path": "/nonexistent.wav", "model_size": "base", "language": "it"}' | python3 src/main.py 2>&1 | head -5
echo ""

echo "✅ Test completato!"
echo ""
echo "Per testare con un file audio reale:"
echo "1. Registra qualcosa nell'app"
echo "2. Trova il path del file audio registrato"
echo "3. Esegui: echo '{\"command\": \"transcribe\", \"audio_path\": \"<PATH>\", \"model_size\": \"base\", \"language\": \"it\"}' | python3 src/main.py"
