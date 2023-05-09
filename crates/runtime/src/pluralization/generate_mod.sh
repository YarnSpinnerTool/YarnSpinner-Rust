#!/usr/bin/env bash
set -euo pipefail

icu4x-datagen --key-file ./icu_datagen_keys.txt --locales all --format mod --out generated_output
