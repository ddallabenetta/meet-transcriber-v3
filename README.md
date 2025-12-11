# Meet Transcriber v3

Piattaforma standalone per registrazione e trascrizione di riunioni con generazione automatica di report tramite AI.

## ✨ Caratteristiche

- ✅ **Registrazione audio** da microfono in tempo reale
- ✅ **Trascrizione locale** con OpenAI Whisper (offline, nessun cloud)
- ✅ **Trascrizione in tempo reale** durante la registrazione per riunioni lunghe
- ✅ **Player audio integrato** per riascoltare le registrazioni dallo storico
- ✅ **Report automatici** tramite LLM configurabile (OpenAI, Anthropic, Ollama)
- ✅ **Storico riunioni** con database SQLite locale
- ✅ **Interfaccia in italiano** moderna e intuitiva
- ✅ **Cross-platform** (macOS, Windows, Linux)

## 🛠️ Stack Tecnologico

- **Desktop App**: Tauri 2.0 (Rust + React/TypeScript)
- **Frontend**: React 18, TypeScript, Tailwind CSS, Shadcn/ui
- **Backend**: Rust (audio capture con cpal, database SQLite)
- **Trascrizione**: OpenAI Whisper (Python sidecar)
- **LLM**: OpenAI, Anthropic, Ollama (configurabile)

## 📋 Requisiti

### Sviluppo

- Node.js 18+ e npm
- Rust 1.77.2+
- Python 3.14 (o 3.11+)
- (Opzionale) Ollama per LLM locale

### Runtime

- Python 3.14 installato nel sistema
- (Opzionale) Ollama o API key per OpenAI/Anthropic

## 🚀 Installazione

### 1. Clona il repository

```bash
git clone <repository-url>
cd meet-transcriber-v3
```

### 2. Installa le dipendenze Node.js

```bash
npm install
```

### 3. Le dipendenze Python sono già installate

Il virtual environment Python con openai-whisper è già configurato in `python/venv/`.
Il primo avvio dell'app scaricherà automaticamente il modello Whisper.

### 4. Avvia in modalità dev

```bash
npm run tauri:dev
```

Questo comando:
1. Avvia il dev server Vite sulla porta 5173
2. Compila il backend Rust
3. Lancia l'applicazione Tauri

## 📖 Come Usare

### 1. Registrare una Riunione

1. Vai alla tab **"Registrazione"**
2. Inserisci un **titolo** per la riunione
3. **Opzionale**: Abilita **"Trascrizione in Tempo Reale"** per registrazioni lunghe
4. Clicca **"Inizia Registrazione"** (cerchio rosso)
5. Parla nel microfono
6. Clicca **"Ferma Registrazione"** (quadrato)

**Nota**: La prima volta potrebbe impiegare alcuni minuti per scaricare il modello Whisper (~150MB per "base")

### 2. Trascrizione in Tempo Reale

Durante la registrazione, se abiliti "Trascrizione in Tempo Reale":
- ✅ La trascrizione inizia automaticamente dopo ~8 secondi di audio
- ✅ Aggiornamenti ogni 8 secondi con nuovo testo
- ✅ Il testo appare progressivamente sotto i controlli
- ✅ Viene salvato automaticamente quando fermi la registrazione

**Ideale per**: Riunioni lunghe (>30 minuti) dove vuoi vedere il testo in tempo reale

### 3. Riascoltare una Registrazione 🎧

1. Vai alla tab **"Storico"**
2. Clicca su una riunione dalla lista
3. **Usa il player audio** in alto nella scheda dettaglio:
   - Play/Pausa
   - Barra di avanzamento
   - Controllo volume
   - Velocità riproduzione (1x, 1.5x, 2x)
4. La trascrizione (se disponibile) è visualizzata sotto il player

**Il player audio è sempre visibile** in cima alla scheda dettaglio riunione!

### 4. Trascrivere Manualmente

Se non hai usato la trascrizione in tempo reale:
1. Vai alla tab **"Storico"**
2. Seleziona la riunione
3. Clicca **"Avvia Trascrizione"**
4. Attendi (può impiegare tempo per riunioni lunghe)
5. La trascrizione apparirà automaticamente nella tab "Trascrizione"

### 5. Generare un Report AI

1. Assicurati che la riunione sia trascritta
2. Configura un provider LLM nelle **Impostazioni**
3. Clicca **"Genera Report con AI"** nella tab "Report"
4. Il report include:
   - 📌 Highlights principali
   - 👥 Partecipanti rilevati
   - ✅ Action items

## ⚙️ Configurazione

### Modelli Whisper

Nelle impostazioni puoi scegliere tra diversi modelli (trade-off velocità/accuratezza):
- **tiny** (~75 MB) - Velocissimo, meno accurato
- **base** (~142 MB) - Bilanciato ⭐ (predefinito)
- **small** (~466 MB) - Buona accuratezza
- **medium** (~1.5 GB) - Alta accuratezza
- **large-v3** (~3 GB) - Massima accuratezza
- **turbo** (~800 MB) - Veloce e accurato (novità 2024)

Il modello viene scaricato automaticamente al primo utilizzo.

### Provider LLM per Report

#### Ollama (Locale, Gratuito) ⭐ Consigliato

1. Installa Ollama: https://ollama.ai
2. Scarica un modello:
   ```bash
   ollama pull llama3.2
   ```
3. Configura nell'app:
   - Provider: **Ollama**
   - Base URL: `http://localhost:11434`
   - Model: `llama3.2` (o `mistral`, `qwen2.5`, etc.)

#### OpenAI

1. Ottieni API key da https://platform.openai.com
2. Configura nell'app:
   - Provider: **OpenAI**
   - API Key: `sk-...`
   - Model: `gpt-4o-mini` (economico) o `gpt-4o` (potente)

#### Anthropic (Claude)

1. Ottieni API key da https://console.anthropic.com
2. Configura nell'app:
   - Provider: **Anthropic**
   - API Key: `sk-ant-...`
   - Model: `claude-3-5-sonnet-20241022`

## 📁 Struttura Progetto

```
meet-transcriber-v3/
├── src/                      # Frontend React + TypeScript
│   ├── components/          # Componenti UI
│   │   ├── recording/       # Controlli registrazione
│   │   ├── history/         # Storico e player audio
│   │   ├── report/          # Visualizzazione report
│   │   └── settings/        # Impostazioni
│   ├── stores/              # State management (Zustand)
│   └── lib/                 # Utilities e comandi Tauri
├── src-tauri/               # Backend Rust
│   └── src/
│       ├── audio/           # Cattura audio (cpal)
│       ├── database/        # SQLite
│       ├── llm/             # Provider LLM
│       ├── transcription/   # Python sidecar
│       └── commands/        # Comandi Tauri
└── python/                  # Sidecar trascrizione
    ├── venv/               # Virtual environment
    └── src/
        ├── main.py          # Entry point
        ├── transcription.py # OpenAI Whisper batch
        └── streaming_transcription.py # Whisper streaming
```

## 🐛 Risoluzione Problemi

### La trascrizione non funziona

- **Prima volta**: Il modello Whisper deve essere scaricato (fino a 3 GB per large)
- **Controlla i log**: Guarda la console per errori Python
- **Verifica Python**: Assicurati che Python 3.14 sia installato
- **Verifica venv**: Il venv in `python/venv/` deve contenere openai-whisper

### Nessun audio registrato

- **Permessi microfono**: macOS richiede permessi per il microfono in Impostazioni → Privacy
- **Controlla dispositivo**: Verifica che il microfono funzioni in altre app
- **Seleziona dispositivo**: Prova a selezionare esplicitamente il microfono

### Il player audio non funziona

- **File non trovato**: Controlla che il file audio esista (path mostrato nei dettagli)
- **Formato non supportato**: Solo file WAV sono supportati
- **Browser**: Assicurati che il browser Tauri supporti la riproduzione audio

### La trascrizione in tempo reale è lenta

- **Primo chunk**: Il primo chunk può impiegare più tempo (caricamento modello)
- **Modello grande**: Prova un modello più piccolo (base o tiny)
- **CPU**: La trascrizione su CPU è lenta, considera GPU se disponibile

## 🔧 Build per Produzione

```bash
npm run tauri:build
```

L'applicazione compilata si troverà in:
- **macOS**: `src-tauri/target/release/bundle/macos/`
- **Windows**: `src-tauri/target/release/bundle/msi/`
- **Linux**: `src-tauri/target/release/bundle/appimage/`

## 📝 Note Tecniche

### Python Sidecar

- Il sidecar Python viene eseguito come processo separato
- Comunica con Rust via stdin/stdout usando JSON
- Il venv Python è in `python/venv/` e include openai-whisper + PyTorch
- Lo script principale è `python/src/main.py`

### Streaming Transcription

- Monitora il file audio in crescita ogni 3 secondi
- Trascrive incrementalmente ogni 8 secondi di nuovo audio
- Usa gli stessi modelli Whisper della trascrizione batch
- Eventi real-time tramite Tauri events

### Audio Format

- Formato: WAV PCM
- Sample rate: 44100 Hz
- Channels: Mono o Stereo (dipende dal dispositivo)
- Bit depth: 16-bit o 32-bit float

## 🎯 Future Features

- [ ] Cattura audio di sistema (videochiamate)
- [ ] Diarizzazione speaker (chi ha detto cosa)
- [ ] Export trascrizioni in formato SRT, VTT
- [ ] Ricerca full-text nelle trascrizioni
- [ ] Tag e categorie per riunioni
- [ ] Sincronizzazione cloud (opzionale)

## 📄 Licenza

MIT

## 🤝 Contributi

Contributi benvenuti! Apri una issue o una PR.

## 👨‍💻 Autore

Daniel Dalla Benetta
