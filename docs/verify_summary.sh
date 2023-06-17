#!/bin/bash

set -e

# Verify that all pages of the design book are listed in `src/SUMMARY.md`.
# This is required for them to show up in the book.

function is_in_summary {
  # Count the number of occurrences of the file in the summary
  occurrences=$(grep $1 ./SUMMARY.md -c)

  # If it's not in the summary, print out an error
  if [[ $occurrences -eq 0 ]]
  then
    echo "'$1' missing in 'SUMMARY.md'"
    exit 1
  fi
}

# Export the function to use with 'xargs' later
export -f is_in_summary

# The pages are the 'src' folder
cd src

`# Get all pages in the book` \
find . -name "*.md" \
`# ...except for SUMMARY.md` \
| grep -F './SUMMARY.md' -v \
`# ...and check if they are included in the summary` \
`# see <https://stackoverflow.com/a/11003457>` \
| xargs -I {} bash -c 'is_in_summary "$@"' "$0" {}
