#!/bin/sh
printf 'ready\n'
while IFS= read -r line; do
  case "$line" in
    *"ping"*)
      printf '{"status":"ok"}\n'
      ;;
    *)
      printf '{"status":"unknown"}\n'
      ;;
  esac
done
