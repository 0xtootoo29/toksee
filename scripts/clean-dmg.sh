#!/bin/bash
# Post-build DMG cleanup: removes .VolumeIcon.icns from the DMG so that
# users with "show hidden files" turned on in Finder don't see a stray
# light-colored icon floating in the install window. Tauri's bundle_dmg.sh
# embeds a custom volume icon by default; we'd rather not, so we strip it
# after the build completes.
#
# Usage: scripts/clean-dmg.sh [path/to/file.dmg]
#        defaults to src-tauri/target/release/bundle/dmg/TokSee_*.dmg
#
# Requires: hdiutil (macOS), xattr.

set -euo pipefail

DMG_PATH="${1:-}"
if [ -z "$DMG_PATH" ]; then
  DMG_PATH=$(ls -1 src-tauri/target/release/bundle/dmg/TokSee_*.dmg 2>/dev/null | tail -n1)
  if [ -z "$DMG_PATH" ]; then
    echo "no DMG found in src-tauri/target/release/bundle/dmg/" >&2
    exit 1
  fi
fi

if [ ! -f "$DMG_PATH" ]; then
  echo "DMG does not exist: $DMG_PATH" >&2
  exit 1
fi

echo "→ cleaning $DMG_PATH"
ORIG_SIZE=$(ls -lh "$DMG_PATH" | awk '{print $5}')

TMP_RW=$(mktemp -t TokSee_RW.XXXXXX.dmg)
MOUNT=$(mktemp -d -t TokSee_RW_mount.XXXXXX)

# Convert to read-write so we can mutate
hdiutil convert "$DMG_PATH" -format UDRW -o "$TMP_RW" >/dev/null

# Mount it
hdiutil attach "$TMP_RW" -nobrowse -mountpoint "$MOUNT" >/dev/null

# Drop the custom volume icon
rm -f "$MOUNT/.VolumeIcon.icns"
xattr -d com.apple.FinderInfo "$MOUNT" 2>/dev/null || true

# Unmount + recompress
hdiutil detach "$MOUNT" >/dev/null
hdiutil convert "$TMP_RW" -format UDZO -ov -o "$DMG_PATH" >/dev/null

rm -f "$TMP_RW"
rmdir "$MOUNT" 2>/dev/null || true

NEW_SIZE=$(ls -lh "$DMG_PATH" | awk '{print $5}')
echo "✓ done · $ORIG_SIZE → $NEW_SIZE"
