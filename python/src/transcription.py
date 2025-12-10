"""
Modulo per la trascrizione audio usando faster-whisper.
"""

import logging
from pathlib import Path
from typing import Dict, List, Optional

from faster_whisper import WhisperModel

logger = logging.getLogger(__name__)

# Cache dei modelli caricati
_models_cache: Dict[str, WhisperModel] = {}


def get_model(model_size: str = "base", device: str = "cpu") -> WhisperModel:
    """
    Ottiene un modello Whisper, caricandolo se necessario.
    I modelli vengono cachati per evitare ricaricamenti.

    Args:
        model_size: Dimensione del modello (tiny, base, small, medium, large-v3)
        device: Device da usare (cpu, cuda)

    Returns:
        Istanza del modello WhisperModel
    """
    cache_key = f"{model_size}_{device}"

    if cache_key not in _models_cache:
        logger.info(f"Loading Whisper model: {model_size} on {device}")

        # Per CPU, usa int8 per performance migliori
        compute_type = "int8" if device == "cpu" else "float16"

        model = WhisperModel(
            model_size,
            device=device,
            compute_type=compute_type,
            download_root=None,  # Usa la directory di cache predefinita
        )

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

    # Parametri di trascrizione
    transcribe_params = {
        "beam_size": 5,
        "vad_filter": True,  # Voice Activity Detection per rimuovere silenzi
        "vad_parameters": {
            "threshold": 0.5,
            "min_speech_duration_ms": 250,
        },
    }

    if language:
        transcribe_params["language"] = language

    logger.info(f"Starting transcription of {audio_path}")

    # Esegui la trascrizione
    segments_iter, info = model.transcribe(str(audio_file), **transcribe_params)

    # Converti i segmenti in lista
    segments = []
    full_text = []

    for segment in segments_iter:
        segments.append(
            {"start": segment.start, "end": segment.end, "text": segment.text.strip()}
        )
        full_text.append(segment.text.strip())

    result = {
        "text": " ".join(full_text),
        "language": info.language if not language else language,
        "segments": segments,
    }

    logger.info(
        f"Transcription completed: {len(segments)} segments, language: {result['language']}"
    )

    return result
