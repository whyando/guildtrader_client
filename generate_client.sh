#!/bin/bash

if [ ! -f "generate_client.sh" ]; then
    echo "This script must be run from the root of the repository"
    exit 1
fi

rm -rf guildtrader
openapi-generator-cli generate -i "https://guildtrader.whyando.com/openapi.json" -g rust -o guildtrader | tee openapi-generator.log

