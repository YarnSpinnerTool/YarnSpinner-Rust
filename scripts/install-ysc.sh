#!/bin/bash

pwd

# Build and install ysc to ./tools
TOOLS_DIR=./tools
CSPROJ_PATH=./third-party/YarnSpinner-Console/src/YarnSpinner.Console/ysc.csproj
# https://stackoverflow.com/questions/70433320/the-sdk-microsoft-net-sdk-workloadautoimportpropslocator-specified-could-not-b
export MSBuildEnableWorkloadResolver=false

if ! which dotnet; then
    echo "You must install the dotnet SDK before installing ysc."
    echo "https://dotnet.microsoft.com/en-us/download"
    exit 1
fi

dotnet publish "$CSPROJ_PATH" -c Release -o "$TOOLS_DIR"
