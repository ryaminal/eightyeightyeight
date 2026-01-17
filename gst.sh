#!/bin/bash
# this has evolved from me piecing together a gst-launch command for writing an encrypted file and reading an encrypted file
# to then adding the stream/receive over rtp with some help from gemini
# to this helpful script that i worked with gemini to build.
#
# Total time taken so far: 79 minutes. Majority was on the first step just piecing together the commands.
#
# # Takeaways:
#  1. encrypting a file with gstreamer has some gotchas. critical pieces are using serialize-iv=true and per-buffer-padding=false
#     this is crucial without support for a streaming encryption cipher.
#  2. this is currently tuned to my system and camera, but it did take some finagling(raw vs jpeg and other things) to figure out a Pipeline
#     that was low latency.
#
# # Plan: use this script as a launching point with the conductor extension in gemini to do some context-driven development
#         and work on the next steps. This will include a non-bash interface, a tui(perhaps), better configuration, and some Encode
#         to restart the stream and things in case it all goes wrong.
#         Will also extract some metrics from gstreamer.
#
# # Mininum Viable Product:
#  1. Capture video from a local webcam or camera device
#  2. Stream frames into a storage backend on disk
#  3. Encrypt data at restart
#  4. Handle runtime failures(pipeline errors, disk full, buffer overflows, etc.)
#  5. Be configurable for different formats and cameras
#  6. Written in a language of my choice(rust) that showcases my taste in best practices, architecture, and coding style.

ACTION=$1
shift # Shift arguments so $1 becomes the next arg

# Configuration Defaults
KEY="00112233445566778899aabbccddeeff"
IV="00112233445566778899aabbccddeeff"
DEFAULT_IP="127.0.0.1"
DEFAULT_LISTEN_IP="0.0.0.0" # Listen on all interfaces by default for receive
DEFAULT_PORT="5000"
DEFAULT_FILE="live.ts.enc"

# Shared Source Pipeline (Camera -> Optimized Encode)
# - Raw I420 format to avoid JPEG decode overhead and compatibility issues
# - Ultrafast preset for lowest latency
SOURCE_PIPELINE="v4l2src device=/dev/video4 \
  ! video/x-raw,width=640,height=480,framerate=30/1 \
  ! videoconvert \
  ! video/x-raw,format=I420 \
  ! queue \
  ! x264enc tune=zerolatency speed-preset=ultrafast bitrate=1000 \
  ! queue \
  ! h264parse"

case "$ACTION" in
stream)
  DEST_IP=${1:-$DEFAULT_IP}
  PORT=${2:-$DEFAULT_PORT}

  echo "====================================================================="
  echo " STARTING STREAMER"
  echo " Destination: $DEST_IP:$PORT"
  echo " Video:       640x480 @ 30fps (Raw -> H.264 Ultrafast)"
  echo " Encryption:  AES-128-CBC (Packetized, Serialized IV)"
  echo " Note:        Use 'receive' mode on the other end."
  echo "====================================================================="

  gst-launch-1.0 -e $SOURCE_PIPELINE \
    ! rtph264pay config-interval=1 mtu=1400 \
    ! queue \
    ! aesenc key=$KEY iv=$IV per-buffer-padding=true serialize-iv=true \
    ! udpsink host=$DEST_IP port=$PORT
  ;;

receive)
  LISTEN_IP=${1:-$DEFAULT_LISTEN_IP}
  PORT=${2:-$DEFAULT_PORT}

  echo "====================================================================="
  echo " STARTING RECEIVER"
  echo " Listen IP:   $LISTEN_IP"
  echo " Listen Port: $PORT"
  echo " Buffer:      Minimal (Low Latency Mode)"
  echo " Encryption:  AES-128-CBC (Packetized, Serialized IV)"
  echo "====================================================================="

  gst-launch-1.0 udpsrc address=$LISTEN_IP port=$PORT \
    ! "application/x-rtp,media=(string)video,clock-rate=(int)90000,encoding-name=(string)H264" \
    ! aesdec key=$KEY iv=$IV per-buffer-padding=true serialize-iv=true \
    ! rtph264depay \
    ! decodebin \
    ! queue \
    ! autovideosink sync=false
  ;;

record)
  FILENAME=${1:-$DEFAULT_FILE}

  echo "====================================================================="
  echo " STARTING RECORDING"
  echo " Output File: $FILENAME"
  echo " Video:       640x480 @ 30fps (Raw -> H.264 Ultrafast)"
  echo " Encryption:  AES-128-CBC (Continuous Stream)"
  echo " IMPORTANT:   Press Ctrl+C to stop! (Required to finalize file)"
  echo "====================================================================="

  gst-launch-1.0 -e $SOURCE_PIPELINE \
    ! mpegtsmux \
    ! queue \
    ! rndbuffersize min=4096 max=4096 \
    ! aesenc key=$KEY serialize-iv=true per-buffer-padding=false \
    ! filesink location=$FILENAME
  ;;

play)
  FILENAME=${1:-$DEFAULT_FILE}

  echo "====================================================================="
  echo " STARTING PLAYBACK"
  echo " Input File:  $FILENAME"
  echo " Encryption:  AES-128-CBC (Continuous Stream)"
  echo "====================================================================="

  gst-launch-1.0 filesrc location=$FILENAME \
    ! aesdec key=$KEY serialize-iv=true per-buffer-padding=false \
    ! tsdemux \
    ! h264parse \
    ! decodebin \
    ! autovideosink
  ;;

*)
  echo "Usage: $0 {stream|receive|record|play} [options]"
  echo ""
  echo "Commands:"
  echo "  stream [IP] [PORT]   Start streaming (Default: $DEFAULT_IP $DEFAULT_PORT)"
  echo "  receive [IP] [PORT]  Start receiving (Default: $DEFAULT_LISTEN_IP $DEFAULT_PORT)"
  echo "  record [FILENAME]    Start recording (Default: $DEFAULT_FILE)"
  echo "  play [FILENAME]      Play a recording (Default: $DEFAULT_FILE)"
  echo ""
  echo "Examples:"
  echo "  $0 stream 192.168.1.50"
  echo "  $0 receive 0.0.0.0 5001"
  echo "  $0 record my_vacation.ts.enc"
  exit 1
  ;;
esac
