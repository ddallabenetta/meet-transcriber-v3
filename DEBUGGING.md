# 🐛 Guida al Debugging

## Apertura DevTools (Console Browser)

L'applicazione Tauri usa un browser interno. Per vedere i log JavaScript:

### macOS
1. Con l'app aperta, premi: **`Cmd + Option + I`**
2. Oppure: Click destro nell'app → **"Inspect Element"**
3. Si aprirà la **Console DevTools** dove puoi vedere:
   - Errori JavaScript (rossi)
   - Warning (gialli)
   - Log (bianchi)

### Cosa cercare nella console

Quando clicchi su "Inizia Registrazione" o "Avvia Trascrizione", dovresti vedere:
```
Registrazione avviata: <id>
Trascrizione streaming avviata
```

Se vedi errori come:
- `TypeError: ... is not a function` → Problema nel frontend
- `Command transcribe_meeting failed` → Problema nel backend Rust
- `Error: ...` → Leggi il messaggio per capire il problema

## Test Componenti Individuali

### 1. Test Python Sidecar

```bash
cd /Users/danieldallabenetta/Documents/meet-transcriber-v3
chmod +x test_python_sidecar.sh
./test_python_sidecar.sh
```

Dovresti vedere:
```
✅ Whisper importato correttamente
['tiny', 'base', 'small', ...]
```

### 2. Test Registrazione Audio

1. Apri l'app
2. Inserisci titolo: "Test Audio"
3. Clicca "Inizia Registrazione"
4. Parla per 5 secondi
5. Clicca "Ferma Registrazione"
6. **Apri DevTools** (Cmd+Option+I)
7. Cerca nella console: `Registrazione salvata: <path>`
8. Copia il path del file audio
9. Verifica che esista:
   ```bash
   ls -lh "<path copiato>"
   ```

### 3. Test Player Audio

**Il player dovrebbe essere SEMPRE visibile** quando apri una riunione dallo storico.

Posizione: In alto nella scheda dettaglio, appena sotto il titolo della riunione.

Se NON vedi il player:
1. Apri DevTools (Cmd+Option+I)
2. Vai nella tab "Elements" o "Inspector"
3. Cerca nell'HTML: `<audio`
4. Se non c'è, il componente non si sta renderizzando

**Possibili cause**:
- `meeting.audio_path` è null/undefined
- Il file audio non esiste più
- Errore di rendering React

**Debug**:
```javascript
// Nella console DevTools, esegui:
console.log(document.querySelector('audio'))
```

Se ritorna `null`, il player non è nel DOM.

### 4. Test Trascrizione Manuale

1. Registra una riunione (almeno 5 secondi di audio)
2. Vai in "Storico"
3. Apri la riunione
4. Clicca "Avvia Trascrizione"
5. **Apri DevTools immediatamente**
6. Osserva i log:
   - Dovrebbe apparire una richiesta al backend
   - Poi "Transcription completed" o un errore

**Se appare un errore come**:
```
Command transcribe_meeting failed: Error: ...
```

Significa che il backend Rust non riesce a chiamare Python.

**Verifiche**:
```bash
# Controlla che il venv Python esista
ls -la python/venv/bin/python3

# Controlla che whisper sia installato
python/venv/bin/python3 -c "import whisper; print('OK')"

# Controlla il path corrente quando esegui l'app
pwd
```

### 5. Test Trascrizione Live

1. Inserisci titolo riunione
2. **✅ ABILITA** "Trascrizione in Tempo Reale"
3. Clicca "Inizia Registrazione"
4. **Apri DevTools** (Cmd+Option+I)
5. Parla nel microfono per almeno 10-15 secondi
6. Osserva i log:
   - Dopo ~8 secondi dovresti vedere: "Transcribing chunk from 0.0s to X.Xs"
   - Poi appariranno eventi "transcription-update"

**Se non vedi nulla dopo 10 secondi**:
- Il processo Python potrebbe non essere partito
- Controlla se ci sono errori nella console
- Controlla stderr del processo Tauri (nel terminale)

## Log del Backend (Rust + Python)

I log del backend appaiono nel **terminale dove hai lanciato** `npm run tauri:dev`.

Cerca:
```
Error: ...
thread 'main' panicked at ...
Transcription sidecar started
Starting streaming transcription of ...
```

## Problemi Comuni

### "Nessun log in console"

**Soluzione**: Assicurati di aprire la Console DevTools dell'app Tauri, non del browser normale!

1. Focus sull'app Tauri
2. Premi `Cmd + Option + I`
3. Vai alla tab "Console"

### "Player audio non visibile"

**Debug step-by-step**:

1. Verifica che la riunione abbia un `audio_path`:
   ```javascript
   // In DevTools Console:
   // (Devi essere nella pagina dello storico con una riunione aperta)
   ```

2. Controlla nell'HTML (tab Elements):
   - Cerca: `Registrazione Audio`
   - Dovrebbe esserci un tag `<audio>` poco sotto

3. Verifica il file:
   ```bash
   # Nel terminale, usa il path che vedi nei dettagli riunione
   file "<path audio>"
   # Dovrebbe dire: "WAVE audio, ..."
   ```

### "Trascrizione non parte"

1. **Controlla che Python venv funzioni**:
   ```bash
   python/venv/bin/python3 -c "import whisper; print('OK')"
   ```

2. **Verifica il path dello script**:
   ```bash
   ls -la python/src/main.py
   chmod +x python/src/main.py
   ```

3. **Test manuale del comando**:
   ```bash
   cd python
   source venv/bin/activate
   echo '{"command":"transcribe","audio_path":"/Users/.../recording.wav","model_size":"base","language":"it"}' | python3 src/main.py
   ```

   Dovresti vedere output JSON con il risultato.

### "Trascrizione live non funziona"

1. **Verifica che streaming_transcription.py esista**:
   ```bash
   ls -la python/src/streaming_transcription.py
   ```

2. **Testa manualmente** (in un altro terminale mentre registri):
   ```bash
   cd python
   source venv/bin/activate
   # Sostituisci <path> con il path della registrazione in corso
   echo '{"command":"start_streaming","audio_path":"<path>","model_size":"tiny","language":"it"}' | python3 src/main.py
   ```

   Dovresti vedere output JSON progressivi mentre il file cresce.

## Logs Dettagliati

### Abilita logs Python più verbosi

In `python/src/main.py`, cambia:
```python
logging.basicConfig(
    format="%(asctime)s - %(levelname)s - %(message)s",
    stream=sys.stderr,
    level=logging.DEBUG,  # ← Aggiungi questa riga
)
```

Poi rilancia l'app e osserva il terminale per logs dettagliati.

## Contattami

Se nulla di questo funziona, fornisci:
1. Screenshot della Console DevTools
2. Output completo del terminale
3. Risultato di `./test_python_sidecar.sh`
