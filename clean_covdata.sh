#!/bin/bash

# コメント

cargo clean

find ./coverage_prof -type f ! -name ".gitkeep" -exec rm -f {} +
find ./coverage -type f ! -name ".gitkeep" -exec rm -f {} +