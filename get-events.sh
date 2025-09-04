#!/usr/bin/bash

WOWAUDIT_TOKEN=$1
ASSET_PATH="/tmp/seemsgood_guild/templates/assets/"
GIT_REPO="https://github.com/Jeremy-Gstein/seemsgood_guild"
REPO_NAME="seemsgood_guild"

git remote set-url origin git@github.com:Jeremy-Gstein/seemsgood_guild
# Get the latest events.json from wowaudit and save to templates/assets/events.json
get_events() {
  curl -X 'GET' \
    'https://wowaudit.com/v1/historical_data' \
    -H 'accept: application/json' \
    -H 'Authorization: '$WOWAUDIT_TOKEN > "$ASSET_PATH/events.json"
}

# upload to git.
get_repo() {
  cd /tmp
  git clone $GIT_REPO && cd $REPO_NAME 
  get_events && \
    git add ./templates/assets/events.json && \
    git commit -am "Update events.json" && \
    git remote set-url origin git@github.com:Jeremy-Gstein/seemsgood_guild && \
    git push
}

get_repo

# diff the current events.json and check if its actually populated.
