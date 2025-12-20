#!/usr/bin/bash

ASSET_PATH="$PWD/events.json"
GIT_REPO="https://github.com/Jeremy-Gstein/seemsgood_guild"
REPO_NAME="seemsgood_guild"

# git remote set-url origin git@github.com:Jeremy-Gstein/seemsgood_guild
# Get the latest events.json from wowaudit and save to templates/assets/events.json
get_events() {
  curl -X 'GET' \
    'https://wowaudit.com/v1/historical_data' \
    -H 'accept: application/json' \
    -H 'Authorization: '$WOWAUDIT_TOKEN > "$ASSET_PATH"
}

# get_repo

# diff the current events.json and check if its actually populated.
#
# format_roster() {
#   curl -s -X GET "https://wowaudit.com/v1/characters" \
#     -H 'accept: application/json' \
#     -H 'Authorization: '$WOWAUDIT_TOKEN \
#     | jq -r '.[] 
#            | select(.rank == "Officer" or .rank == "Raider") 
#            | "Player { name: \"\(.name)\", class: PlayerClass::\(.class), realm: \"\(.realm)\" },"'
# }
format_roster() {
  curl -s -X GET "https://wowaudit.com/v1/characters" \
    -H "accept: application/json" \
    -H "Authorization: $WOWAUDIT_TOKEN" \
    | jq -r '.[] | select(.rank == "Officer" or .rank == "Raider") | "Player { name: \"\(.name)\", class: PlayerClass::\(.class), realm: \"\(.realm)\" },"'
}

help() {
  cat << 'EOF'
json_helpers.sh - a few tools to gather information from APIs we use.

USAGE:
          -e|--events) get events from wowaudit (mythic keystones)
          -r|--roster) get roster from wowaudit 
          -t|--token)  set wowaudit API token
          -h|--help)   print this help menu
EXAMPLE:
          Get roster:
            ./json_helpers.sh -t $token -r 
          Get Events And Roster:
            ./json_helpers.sh -t $token -r -e
EOF
}



WOWAUDIT_TOKEN=""
# Parse arguments properly
while [[ $# -gt 0 ]]; do
  case "$1" in
    -t|--token)
      shift
      if [[ -z "$1" ]]; then
        echo "Error: --token requires a value"
        exit 1
      fi
      WOWAUDIT_TOKEN="$1"
      ;;
    -e|--events) get_events ;;
    -r|--roster) format_roster ;;
    -u|--help) help ;;
    *) echo "Unknown argument: $1" ;;
  esac
  shift
done

# If no args, show help
if [[ -z "$WOWAUDIT_TOKEN" && $# -eq 0 ]]; then
  help
fi
