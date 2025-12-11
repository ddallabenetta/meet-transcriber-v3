"""
Modulo per la trascrizione audio usando openai-whisper.
"""

import logging
from pathlib import Path
from typing import Dict, List, Optional
import whisper

logger = logging.getLogger(__name__)

# Cache dei modelli caricati
_models_cache: Dict[str, whisper.Whisper] = {}


def get_model(model_size: str = "base", device: str = "cpu") -> whisper.Whisper:
    """
    Ottiene un modello Whisper, caricandolo se necessario.
    I modelli vengono cachati per evitare ricaricamenti.

    Args:
        model_size: Dimensione del modello (tiny, base, small, medium, large-v3)
        device: Device da usare (cpu, cuda)

    Returns:
        Istanza del modello Whisper
    """
    cache_key = f"{model_size}_{device}"

    if cache_key not in _models_cache:
        logger.info(f"Loading Whisper model: {model_size} on {device}")
        model = whisper.load_model(model_size, device=device)
        _models_cache[cache_key] = model
        logger.info(f"Model {model_size} loaded successfully")

    return _models_cache[cache_key]


def transcribe_audio(
    audio_path: str,
    model_size: str = "base",
    language: Optional[str] = None,
    device: str = "cpu",
) -> Dict:
    """
    Trascrizione di un file audio.

    Args:
        audio_path: Percorso del file audio
        model_size: Dimensione del modello Whisper
        language: Lingua del audio (es. 'it', 'en'), None per auto-detect
        device: Device da usare

    Returns:
        Dizionario con:
        - text: Testo completo trascritto
        - language: Lingua rilevata
        - segments: Lista di segmenti con timestamp
    """
    audio_file = Path(audio_path)

    if not audio_file.exists():
        raise FileNotFoundError(f"Audio file not found: {audio_path}")

    # Ottieni il modello
    model = get_model(model_size, device)

    logger.info(f"Starting transcription of {audio_path}")

    # Esegui la trascrizione
    transcribe_params = {}
    if language:
        transcribe_params["language"] = language

    result_raw = model.transcribe(str(audio_file), **transcribe_params)

    # Converti i segmenti
    segments = []
    for segment in result_raw.get("segments", []):
        segments.append({
            "start": segment["start"],
            "end": segment["end"],
            "text": segment["text"].strip()
        })

    result = {
        "text": result_raw["text"].strip(),
        "language": result_raw.get("language", language),
        "segments": segments,
    }

    logger.info(
        f"Transcription completed: {len(segments)} segments, language: {result['language']}"
    )

    return result
