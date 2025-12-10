"""
Modulo per la trascrizione in streaming mentre avviene la registrazione.
"""

import logging
import time
import wave
from pathlib import Path
from typing import Callable, Dict, List, Optional

from faster_whisper import WhisperModel

logger = logging.getLogger(__name__)


class StreamingTranscriber:
    """
    Trascrittore che monitora un file audio in crescita e trascrive
    in tempo reale man mano che arrivano nuovi dati.
    """

    def __init__(
        self,
        model_size: str = "base",
        device: str = "cpu",
        language: Optional[str] = None,
    ):
        self.model_size = model_size
        self.device = device
        self.language = language
        self.model: Optional[WhisperModel] = None
        self.last_transcribed_position = 0

    def load_model(self):
        """Carica il modello Whisper"""
        if self.model is None:
            logger.info(f"Loading Whisper model: {self.model_size}")
            compute_type = "int8" if self.device == "cpu" else "float16"
            self.model = WhisperModel(
                self.model_size, device=self.device, compute_type=compute_type
            )
            logger.info("Model loaded successfully")

    def get_audio_duration(self, audio_path: Path) -> float:
        """Ottiene la durata corrente del file audio in secondi"""
        try:
            with wave.open(str(audio_path), "rb") as wav_file:
                frames = wav_file.getnframes()
                rate = wav_file.getframerate()
                duration = frames / float(rate)
                return duration
        except Exception as e:
            logger.error(f"Error getting audio duration: {e}")
            return 0.0

    def transcribe_chunk(self, audio_path: Path, start_time: float = 0.0) -> List[Dict]:
        """
        Trascrive un chunk del file audio a partire da start_time.
        """
        self.load_model()

        segments_list = []

        try:
            # Parametri di trascrizione
            transcribe_params = {
                "beam_size": 5,
                "vad_filter": True,
                "vad_parameters": {
                    "threshold": 0.5,
                    "min_speech_duration_ms": 250,
                },
            }

            if self.language:
                transcribe_params["language"] = self.language

            # Trascrive solo la parte nuova del file
            segments_iter, _ = self.model.transcribe(
                str(audio_path), **transcribe_params
            )

            for segment in segments_iter:
                # Filtra solo i segmenti dopo start_time
                if segment.start >= start_time:
                    segments_list.append(
                        {
                            "start": segment.start,
                            "end": segment.end,
                            "text": segment.text.strip(),
                        }
                    )

        except Exception as e:
            logger.error(f"Error transcribing chunk: {e}")

        return segments_list

    def monitor_and_transcribe(
        self,
        audio_path: Path,
        callback: Callable[[List[Dict]], None],
        stop_event: threading.Event,
        check_interval: float = 5.0,
        min_chunk_duration: float = 10.0,
    ):
        """
        Monitora il file audio e trascrive incrementalmente.

        Args:
            audio_path: Percorso del file audio in registrazione
            callback: Funzione chiamata con i nuovi segmenti trascritti
            stop_event: Event per terminare il monitoraggio
            check_interval: Intervallo in secondi tra i controlli
            min_chunk_duration: Durata minima del chunk per avviare trascrizione
        """
        self.last_transcribed_position = 0.0

        logger.info(f"Starting streaming transcription of {audio_path}")

        while not stop_event.is_set():
            # Verifica se il file esiste
            if not audio_path.exists():
                time.sleep(check_interval)
                continue

            # Ottieni durata corrente
            current_duration = self.get_audio_duration(audio_path)

            # Calcola quanto audio nuovo è disponibile
            new_audio_duration = current_duration - self.last_transcribed_position

            if new_audio_duration >= min_chunk_duration:
                logger.info(
                    f"Transcribing chunk from {self.last_transcribed_position}s to {current_duration}s"
                )

                # Trascrivi il nuovo chunk
                new_segments = self.transcribe_chunk(
                    audio_path, start_time=self.last_transcribed_position
                )

                if new_segments:
                    # Invia i nuovi segmenti tramite callback
                    callback(new_segments)

                    # Aggiorna la posizione
                    self.last_transcribed_position = current_duration
                    logger.info(f"Transcribed {len(new_segments)} new segments")

            time.sleep(check_interval)

        logger.info("Streaming transcription stopped")
