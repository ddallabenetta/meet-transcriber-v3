# Meet Transcriber v3

Piattaforma standalone per registrazione e trascrizione di riunioni con generazione automatica di report tramite AI.

## Caratteristiche

- ✅ **Registrazione audio** in tempo reale (microfono)
- ✅ **Trascrizione locale** con faster-whisper (offline, nessun cloud)
- ✅ **Report automatici** tramite LLM configurabile (OpenAI, Anthropic, Ollama)
- ✅ **Storico riunioni** con database SQLite locale
- ✅ **Interfaccia in italiano** moderna e intuitiva
- ✅ **Cross-platform** (macOS, Windows, Linux)

## Stack Tecnologico

- **Desktop App**: Tauri 2.0 (Rust + React/TypeScript)
- **Frontend**: React 18, TypeScript, Tailwind CSS
- **Backend**: Rust (audio capture con cpal, database SQLite)
- **Trascrizione**: faster-whisper (Python sidecar)
- **LLM**: OpenAI, Anthropic, Ollama (configurabile)

## Requisiti

### Sviluppo

- Node.js 18+ e npm
- Rust 1.77.2+
- Python 3.8+
- (Opzionale) Ollama per LLM locale

### Runtime

- Python 3.8+ installato nel sistema
- (Opzionale) Ollama o API key per OpenAI/Anthropic

## Installazione

### 1. Clona il repository

```bash
git clone <repository-url>
cd meet-transcriber-v3
```

### 2. Installa le dipendenze Node.js

```bash
npm install
```

### 3. Installa le dipendenze Python

```bash
cd python
pip install -r requirements.txt
cd ..
```

### 4. Build del progetto

```bash
npm run tauri build
```

## Sviluppo

### Avvio in modalità dev

```bash
npm run tauri dev
```

Questo comando:
1. Avvia il dev server Vite sulla porta 5173
2. Compila il backend Rust
3. Lancia l'applicazione Tauri

### Test della trascrizione Python

Per testare il modulo Python separatamente:

```bash
cd python/src
python3 main.py
```

Poi invia un comando JSON via stdin:

```json
{"command": "transcribe", "audio_path": "/path/to/audio.wav", "model_size": "base", "language": "it"}
```

## Configurazione

### Modelli Whisper

I modelli disponibili sono:
- **tiny** (~75 MB) - Veloce, meno accurato
- **base** (~142 MB) - Bilanciato (predefinito)
- **small** (~466 MB) - Buona accuratezza
- **medium** (~1.5 GB) - Alta accuratezza
- **large-v3** (~3 GB) - Massima accuratezza

I modelli vengono scaricati automaticamente al primo utilizzo.

### Provider LLM

#### Ollama (Locale, consigliato)

1. Installa Ollama: https://ollama.ai
2. Scarica un modello: `ollama pull llama3`
3. Configura nell'app:
   - Provider: Ollama
   - Base URL: http://localhost:11434
   - Model: llama3

#### OpenAI

1. Ottieni API key da https://platform.openai.com
2. Configura nell'app:
   - Provider: OpenAI
   - API Key: sk-...
   - Model: gpt-4o

#### Anthropic (Claude)

1. Ottieni API key da https://console.anthropic.com
2. Configura nell'app:
   - Provider: Anthropic
   - API Key: sk-ant-...
   - Model: claude-3-5-sonnet-20241022

## Utilizzo

### 1. Registra una riunione

1. Vai alla tab "Registrazione"
2. Inserisci il titolo della riunione
3. Seleziona il dispositivo audio
4. Clicca "Avvia Registrazione"
5. Clicca "Ferma Registrazione" quando hai finito

### 2. Trascrivi

1. Vai alla tab "Storico"
2. Seleziona la riunione
3. Clicca "Avvia Trascrizione"
4. Attendi il completamento

### 3. Genera report

1. Dopo la trascrizione, vai alla tab "Report"
2. Clicca "Genera Report con AI"
3. Visualizza punti salienti, partecipanti e action items

## Struttura Progetto

```
meet-transcriber-v3/
├── src/                      # Frontend React
│   ├── components/          # Componenti UI
│   ├── stores/              # State management (Zustand)
│   └── lib/                 # Utilities e comandi Tauri
├── src-tauri/               # Backend Rust
│   └── src/
│       ├── audio/           # Cattura audio
│       ├── database/        # SQLite
│       ├── llm/             # Provider LLM
│       ├── transcription/   # Comunicazione Python
│       └── commands/        # Comandi Tauri
└── python/                  # Sidecar trascrizione
    └── src/
        ├── main.py          # Entry point
        └── transcription.py # faster-whisper
```

## Troubleshooting

### Nessun dispositivo audio trovato (macOS)

Assicurati di dare i permessi microfono all'app in:
Impostazioni → Privacy e Sicurezza → Microfono

### Errore trascrizione

- Verifica che Python 3.8+ sia installato: `python3 --version`
- Verifica che faster-whisper sia installato: `pip list | grep faster-whisper`
- Controlla i log nella console sviluppatore

### Errore LLM

- **Ollama**: Verifica che Ollama sia in esecuzione: `curl http://localhost:11434`
- **OpenAI/Anthropic**: Verifica l'API key e il credito disponibile

## Note sullo sviluppo

### Audio di sistema (cattura audio delle videochiamate)

Non ancora implementato. Richiede:
- **macOS**: ScreenCaptureKit o virtual audio device (BlackHole)
- **Windows**: WASAPI loopback
- **Linux**: PulseAudio monitor

### Diarizzazione speaker

Non ancora implementata. Può essere aggiunta con pyannote-audio.

## License

MIT

## Contributi

Contributi benvenuti! Apri una issue o una PR.
