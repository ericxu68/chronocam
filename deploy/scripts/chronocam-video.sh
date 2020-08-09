#!/usr/bin/env bash
set -euo pipefail

# make sure there's a place to store video output
if [ ! -d /opt/chronocam-video-output ]; then
    mkdir -p /opt/chronocam-video-output
fi

# move current photos off to their own directory so the camera service
# can keep saving photos
if [ ! -d "/opt/chronocam-video-work" ]; then
    mkdir -p /opt/chronocam-video-work
    mv /opt/chronocam-output/*.jpg /opt/chronocam-video-work
fi

# use ffmpeg to produce the video
echo "processing timelapse video"
ffmpeg \
    -r 24 -pattern_type \
    glob -i '/opt/chronocam-video-work/*.jpg' \
    -s hd1080 \
    -vcodec libx264 \
    "/opt/chronocam-video-output/$(date +"%m_%d_%Y_%H-%M-%S").mp4"

# we are done, so we can remove the working directory and reclaim the storage
# space occupied by the JPEG files
echo "removing chronocam video working directory"
rm -rf /opt/chronocam-video-work
