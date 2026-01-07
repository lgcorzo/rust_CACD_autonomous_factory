import os

import pandas as pd
from sklearn.model_selection import train_test_split

# Constants
TEST_SIZE = 0.2
SHUFFLE = False
SAMPLE_RATIO = 0.15
SAMPLE_RANDOM_STATE = 0

INPUTS_TRAIN_FILE = "data/inputs_train.parquet"
INPUTS_TEST_FILE = "data/inputs_test.parquet"
TARGETS_TRAIN_FILE = "data/targets_train.parquet"
TARGETS_TEST_FILE = "data/targets_test.parquet"
INPUTS_SAMPLE_FILE = "tests/data/inputs_sample.parquet"
TARGETS_SAMPLE_FILE = "tests/data/targets_sample.parquet"

# Ensure directories exist
os.makedirs("data", exist_ok=True)
os.makedirs("tests/data", exist_ok=True)

print("Downloading dataset...")
try:
    df_llm = pd.read_json("hf://datasets/Vezora/Tested-143k-Python-Alpaca/143k-Tested-Python-Alpaca-Vezora.json")
except Exception as e:
    print(f"Error downloading dataset: {e}")
    # Fallback or exit?
    # Trying alternative URL or method if pandas hf:// fails?
    # It might require 'huggingface_hub' and 'fsspec' installed.
    # Attempting to read input directly if pandas supports it.
    raise

print("Processing dataset...")
df_input = df_llm.drop(columns=["input", "output"])
df_input = df_input.rename(columns={"instruction": "input"})

df_target = df_llm.drop(columns=["instruction"])
df_target = df_target.rename(columns={"output": "response"})
df_target = df_target.rename(columns={"input": "input_target"})

print("Splitting dataset...")
inputs_train, inputs_test, targets_train, targets_test = train_test_split(
    df_input, df_target, test_size=TEST_SIZE, shuffle=SHUFFLE
)

print("Sampling dataset...")
inputs_train_sample = inputs_train.sample(frac=SAMPLE_RATIO, random_state=SAMPLE_RANDOM_STATE)
targets_train_sample = targets_train.sample(frac=SAMPLE_RATIO, random_state=SAMPLE_RANDOM_STATE)

print("Saving to parquet...")
inputs_train.to_parquet(INPUTS_TRAIN_FILE)
inputs_test.to_parquet(INPUTS_TEST_FILE)
targets_train.to_parquet(TARGETS_TRAIN_FILE)
targets_test.to_parquet(TARGETS_TEST_FILE)
inputs_train_sample.to_parquet(INPUTS_SAMPLE_FILE)
targets_train_sample.to_parquet(TARGETS_SAMPLE_FILE)

print("Data generation complete.")
