#!/bin/sh
printf 'ready\n'
while IFS= read -r line; do
  case "$line" in
    *"ping"*)
      printf '{"status":"fail"}\n'
      ;;
    *)
      printf '{"status":"fail"}\n'
      ;;
  esac
done
