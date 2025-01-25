#!/bin/bash

# Check if GROQ_API_KEY is already set
if [ -n "$GROQ_API_KEY" ]; then
  echo "GROQ_API_KEY is already set"
  exit 0
fi

# Prompt user for API key
read -p "Enter your GROQ API key: " api_key

# Export the key
export GROQ_API_KEY=$api_key
echo "GROQ_API_KEY has been set successfully"

# Add to shell profile if desired
read -p "Would you like to add this to your shell profile? [y/N] " add_to_profile
if [[ "$add_to_profile" =~ ^[Yy]$ ]]; then
  # Determine shell profile based on OS
  if [[ "$(uname -s)" == "Darwin" ]]; then
    PROFILE=~/.zshrc
  else
    PROFILE=~/.bashrc
  fi
  echo "export GROQ_API_KEY=$api_key" >> "$PROFILE"
  echo "Added to $PROFILE. Please restart your shell or run 'source $PROFILE'"
fi
