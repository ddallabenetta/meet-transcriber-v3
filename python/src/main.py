#!/usr/bin/env python3
"""
Sidecar Python per la trascrizione audio con faster-whisper.
Supporta trascrizione batch e streaming.
Comunica con Tauri via stdin/stdout usando JSON.
"""

import json
import logging
import sys
import threading
from pathlib import Path

# Import relativi
try:
    from streaming_transcription import StreamingTranscriber
    from transcription import transcribe_audio
except ImportError:
    # Se eseguito come modulo
    from .streaming_transcription import StreamingTranscriber
    from .transcription import transcribe_audio

# Setup logging to stderr (stdout is used for communication)
logging.basicConfig(
    format="%(asctime)s - %(levelname)s - %(message)s",
    stream=sys.stderr,
)

logger = logging.getLogger(__name__)

# Streaming transcriber globale
streaming_transcriber: StreamingTranscriber = None
streaming_thread: threading.Thread = None
stop_event = threading.Event()


def send_response(success: bool, result=None, error=None):
    """Send JSON response to stdout."""
    response = {"success": success, "result": result, "error": error}
    print(json.dumps(response), flush=True)


def send_streaming_update(segments):
    """Send streaming transcription update."""
    update = {"type": "streaming_update", "segments": segments}
    print(json.dumps(update), flush=True)


def start_streaming_transcription(audio_path, model_size, language):
    """Start streaming transcription in background"""
    global streaming_transcriber, streaming_thread, stop_event

    # Reset stop event
    stop_event.clear()

    streaming_transcriber = StreamingTranscriber(
        model_size=model_size, device="cpu", language=language
    )

    def transcribe_loop():
        try:
            streaming_transcriber.monitor_and_transcribe(
                audio_path=Path(audio_path),
                callback=send_streaming_update,
                stop_event=stop_event,
                check_interval=3.0,  # Check every 3 seconds
                min_chunk_duration=8.0,  # Transcribe when 8s of new audio available
            )
        except Exception as e:
            logger.error(f"Streaming transcription error: {e}", exc_info=True)
            send_response(success=False, error=f"Streaming error: {str(e)}")

    streaming_thread = threading.Thread(target=transcribe_loop, daemon=True)
    streaming_thread.start()

    send_response(success=True, result={"status": "streaming_started"})


def stop_streaming_transcription():
    """Stop streaming transcription"""
    global stop_event
    stop_event.set()
    send_response(success=True, result={"status": "streaming_stopped"})


def main():
    """Main loop - read commands from stdin and execute them."""
    logger.info("Transcription sidecar started")

    try:
        # Read one line from stdin (the command)
        for line in sys.stdin:
            line = line.strip()
            if not line:
                continue

            try:
                request = json.loads(line)
                command = request.get("command")

                if command == "transcribe":
                    # Batch transcription
                    audio_path = request.get("audio_path")
                    model_size = request.get("model_size", "base")
                    language = request.get("language")

                    logger.info(f"Transcribing: {audio_path} with model {model_size}")

                    result = transcribe_audio(
                        audio_path=audio_path, model_size=model_size, language=language
                    )

                    send_response(success=True, result=result)
                    logger.info("Transcription completed successfully")

                elif command == "start_streaming":
                    # Start streaming transcription
                    audio_path = request.get("audio_path")
                    model_size = request.get("model_size", "base")
                    language = request.get("language")

                    logger.info(f"Starting streaming transcription: {audio_path}")
                    start_streaming_transcription(audio_path, model_size, language)

                elif command == "stop_streaming":
                    # Stop streaming transcription
                    logger.info("Stopping streaming transcription")
                    stop_streaming_transcription()

                else:
                    send_response(success=False, error=f"Unknown command: {command}")

            except json.JSONDecodeError as e:
                logger.error(f"JSON decode error: {e}")
                send_response(success=False, error=f"Invalid JSON: {str(e)}")

            except Exception as e:
                logger.error(f"Error processing request: {e}", exc_info=True)
                send_response(success=False, error=str(e))

    except KeyboardInterrupt:
        logger.info("Sidecar interrupted")
    except Exception as e:
        logger.error(f"Fatal error: {e}", exc_info=True)
        send_response(success=False, error=str(e))


if __name__ == "__main__":
    main()
