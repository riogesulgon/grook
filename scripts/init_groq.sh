#!/bin/bash

# Check if GROQ_API_KEY is already set
if [ -n "$GROQ_API_KEY" ]; then
  echo "GROQ_API_KEY is already set"
  exit 0
fi

# Prompt user for API key
read -p "Enter your GROQ API key: " api_key

# Basic validation - check if key is 32 characters (typical API key length)
if [ ${#api_key} -ne 32 ]; then
  echo "Error: Invalid API key length. Expected 32 characters."
  exit 1
fi

# Export the key
export GROQ_API_KEY=$api_key
echo "GROQ_API_KEY has been set successfully"

# Add to shell profile if desired
read -p "Would you like to add this to your shell profile? [y/N] " add_to_profile
if [[ "$add_to_profile" =~ ^[Yy]$ ]]; then
  echo "export GROQ_API_KEY=$api_key" >> ~/.bashrc
  echo "Added to ~/.bashrc. Please restart your shell or run 'source ~/.bashrc'"
fi