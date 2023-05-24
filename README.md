# CSV Chunk Processing

This is a Rust application that reads a CSV file and sends each record to the OpenAI API to generate embeddings. The embeddings received from the API are then saved to a local text file. The API key for OpenAI is read from an environment variable.

## Prerequisites

1. [Rust](https://www.rust-lang.org/tools/install)
2. An API key for [OpenAI](https://www.openai.com/)

## Usage

1. First, you will need to set the environment variable `OPENAI_API_KEY` to your OpenAI API key. This can be done in a terminal like so:
    ```bash
    export OPENAI_API_KEY=your-api-key-here
    ```

2. Run the program with the path to the CSV file as an argument. Each record in the CSV file should be a single chunk of text. The program can be run like so:
    ```bash
    cargo run /path/to/your/file.csv
    ```

After running, the program will create a new file named `embedding_vector.txt` in the same directory. This file contains the embeddings for each chunk of text from the CSV file.

## Note

This program does not currently support CSV files with headers. Please ensure your CSV file does not contain a header row.

This application is meant to be a simple demonstration of interacting with the OpenAI API and handling CSV files in Rust. It may need modifications based on the exact requirements of your use-case.
