#!/usr/bin/bash

WOWAUDIT_TOKEN=$1
ASSET_PATH="$PWD/templates/assets/"
GIT_REPO=""
REPO_NAME=""
# Get the latest events.json from wowaudit and save to templates/assets/events.json
curl -X 'GET' \
  'https://wowaudit.com/v1/historical_data' \
  -H 'accept: application/json' \
  -H 'Authorization: '$WOWAUDIT_TOKEN > "$ASSET_PATH/events.json"

# upload to git.
get_repo() {
  cd /tmp
  git clone $GIT_REPO && cd $REPO_NAME && \
    git add ./templates/assets/events.json && \
    git commit -am "Update events.json" && \
    git push
}

# diff the current events.json and check if its actually populated.
