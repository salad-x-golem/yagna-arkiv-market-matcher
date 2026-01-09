import argparse
import requests
import re

DEFAULT_URL = "http://polygongas.org:11500/offers/list"

def main(args):
    response = requests.get(args.url)
    response.raise_for_status()  # fail fast on HTTP errors

    data = response.json()

    name_pattern = re.compile(args.name_filter)
    actual_count = 0

    for entry in data:
        offer = entry.get("offer")
        if not offer:
            continue

        attributes = entry.get("attributes", {})
        if not attributes:
            raise ValueError("Missing attributes in offer entry")

        node_name = attributes.get("node_name", "")
        if not node_name:
            raise ValueError("Missing node_name in offer attributes")

        if name_pattern.search(node_name):
            actual_count += 1

    if actual_count == args.expected_count:
        print(f"Rental count OK: {actual_count}")
    else:
        print(
            f"Rental count MISMATCH: expected={args.expected_count}, actual={actual_count}"
        )
        raise ValueError(f"Rental count MISMATCH: expected={args.expected_count}, actual={actual_count}")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Validate rental offer count from an API endpoint"
    )
    parser.add_argument(
        "--url",
        default=DEFAULT_URL,
        help="Offers API URL (default: %(default)s)"
    )
    parser.add_argument(
        "--expected-count",
        type=int,
        required=True,
        help="Expected number of rental offers"
    )
    parser.add_argument(
        "--name-filter",
        type=str,
        required=True,
        help="Regex to match provider node_name"
    )

    args = parser.parse_args()
    main(args)
