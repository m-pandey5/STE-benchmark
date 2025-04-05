import os
import json
import csv
import re
from bs4 import BeautifulSoup
from collections import defaultdict

# Base directory and output file
BASE_DIR = "./target/criterion"
OUTPUT_CSV = "benchmark_data_grouped.csv"

# List of JSON files to extract data from
JSON_FILES = ["benchmark.json"]

# Function to extract data from JSON files
def extract_data_from_folder(folder_path):
    data = {}

    for phase in ['base', 'new']:
        phase_path = os.path.join(folder_path, phase)

        if os.path.exists(phase_path):
            for json_file in JSON_FILES:
                json_path = os.path.join(phase_path, json_file)
                if os.path.exists(json_path):
                    with open(json_path, 'r') as f:
                        try:
                            content = json.load(f)
                            data[f"{phase}_{json_file.replace('.json', '')}"] = content
                        except json.JSONDecodeError:
                            print(f"Failed to parse {json_path}")
                            data[f"{phase}_{json_file.replace('.json', '')}"] = {}

    # Extract mean value from HTML
    mean_value = extract_mean_from_html(folder_path)
    data['mean_value'] = mean_value

    # Extract parameters from full_id field
    full_id = extract_full_id(folder_path)
    if full_id:
        parameters = extract_parameters_from_full_id(full_id)
        data.update(parameters)

    return data

# Function to extract mean value from HTML file
def extract_mean_from_html(folder_path):
    """Extracts the mean value from the HTML report."""
    html_file = os.path.join(folder_path, "report", "index.html")

    if os.path.exists(html_file):
        with open(html_file, "r", encoding="utf-8") as f:
            soup = BeautifulSoup(f, "html.parser")

            # Locate the mean row in the table
            mean_value = "N/A"
            stats_table = soup.find("section", class_="stats")

            if stats_table:
                rows = stats_table.find_all("tr")

                for row in rows:
                    cols = row.find_all("td")
                    if cols and "Mean" in cols[0].get_text():
                        mean_value = cols[2].get_text(strip=True)
                        break

            return mean_value
    return "N/A"

# Function to extract full_id from JSON file
def extract_full_id(folder_path):
    """Extracts the full_id field from the base benchmark.json file."""
    json_path = os.path.join(folder_path, "base", "benchmark.json")

    if os.path.exists(json_path):
        with open(json_path, 'r') as f:
            content = json.load(f)
            return content.get("full_id", "")

    return ""

# Function to extract parameters from full_id
def extract_parameters_from_full_id(full_id):
    """Parses the full_id string to extract encryption/decryption, no_of_nodes, data_type."""
    pattern = (
        r"scheme_SilentThreshold_(encryption|decryption)_no_of_nodes_(\d+)_data_(U\d+|u\d+|Bytes\d+)"
    )

    match = re.search(pattern, full_id)

    if match:
        operation = match.group(1)
        no_of_nodes = int(match.group(2))
        # batch_size = int(match.group(3))
        data_type = match.group(3).lower()

        return {
            "operation": operation,
            "no_of_nodes": no_of_nodes,
            # "batch_size": batch_size,
            "data_type": data_type
        }

    return {
        "operation": "N/A",
        "no_of_nodes": "N/A",
        # "batch_size": "N/A",
        "data_type": "N/A"
    }

# Collect all benchmark folders and group them by (no_of_nodes, batch_size, data_type)
def collect_benchmark_data(base_dir):
    grouped_data = defaultdict(lambda: {"encryption": {}, "decryption": {}})

    for root, dirs, _ in os.walk(base_dir):
        for dir_name in dirs:
            if dir_name.startswith("scheme_SilentThreshold"):
                folder_path = os.path.join(root, dir_name)
                data = extract_data_from_folder(folder_path)

                params = (
                    data.get("no_of_nodes", "N/A"),
                    # data.get("batch_size", "N/A"),
                    data.get("data_type", "N/A")
                )

                operation = data.get("operation", "N/A")
                mean_value = data.get("mean_value", "N/A")

                # Group by params and add encryption/decryption separately
                if operation == "encryption":
                    grouped_data[params]["encryption"] = {
                        "mean_value": mean_value
                    }
                elif operation == "decryption":
                    grouped_data[params]["decryption"] = {
                        "mean_value": mean_value
                    }

    return grouped_data

def custom_sort_key(row):
    # row is a tuple: (no_of_nodes, batch_size, data_type, encryption_mean, decryption_mean)
    data_type_priority = {'u32': 1, 'u64': 2, 'u128': 3, 'bytes32': 4}

    # Access by index instead of dictionary key
    no_of_nodes = int(row['no_of_nodes'])
    # batch_size = int(row['batch_size'])
    data_type = row['data_type']

    # Prioritize data types as u32 -> u64 -> u128 -> bytes32
    data_type_rank = data_type_priority.get(data_type, 999)

    return (no_of_nodes, data_type_rank)

def flatten_benchmark_data(grouped_data):
    """Flattens the nested benchmark data into a list of dictionaries."""
    results = []

    for (no_of_nodes,  data_type), ops in grouped_data.items():
        encryption_mean = ops.get("encryption", {}).get("mean_value", "N/A")
        decryption_mean = ops.get("decryption", {}).get("mean_value", "N/A")

        results.append({
            "no_of_nodes": no_of_nodes,
            
            "data_type": data_type,
            "encryption_mean_value": encryption_mean,
            "decryption_mean_value": decryption_mean
        })

    return results

def write_to_csv(output_file, grouped_data):
    """Writes the flattened benchmark data to CSV."""
    results = flatten_benchmark_data(grouped_data)

    # Sort the results before writing
    sorted_results = sorted(results, key=custom_sort_key)

    # Write to CSV
    with open(output_file, mode='w', newline='') as csvfile:
        fieldnames = ['no_of_nodes',  'data_type', 'encryption_mean_value', 'decryption_mean_value']
        writer = csv.DictWriter(csvfile, fieldnames=fieldnames)

        writer.writeheader()
        writer.writerows(sorted_results)

    print(f"✅ CSV successfully saved to {output_file}")

def benchmark_to_markdown(grouped_data, output_file="README.md"):
    """Generates a markdown table from benchmark data and writes it to a file."""
    results = flatten_benchmark_data(grouped_data)
    sorted_results = sorted(results, key=custom_sort_key)

    # Create Markdown table header
    markdown = "| No. of Nodes |  Data Type  | Encryption Mean Value | Decryption Mean Value |\n"
    markdown += "|--------------|------------|------------------------|-----------------------|\n"

    # Add data rows
    for row in sorted_results:
        markdown += f"| {row['no_of_nodes']}  | {row['data_type']} | {row['encryption_mean_value']} | {row['decryption_mean_value']} |\n"

    # Write to file
    with open(output_file, "w") as f:
        f.write(markdown)

    print(f"✅ Markdown table saved to {output_file}")

if __name__ == "__main__":
    benchmark_data = collect_benchmark_data(BASE_DIR)
    write_to_csv(OUTPUT_CSV, benchmark_data)
#     benchmark_to_markdown(benchmark_data)

