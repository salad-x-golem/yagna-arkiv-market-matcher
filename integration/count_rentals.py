import sys
import requests

URL = "http://127.0.0.1:36200/rentals"

def main(expected_count: int):
    response = requests.get(URL)
    response.raise_for_status()  # fail fast on HTTP errors

    data = response.json()
    active_rentals = data.get("rentals", {}).get("activeRentals", [])
    actual_count = len(active_rentals)

    if actual_count != expected_count:
        raise ValueError(
            f"Rental count mismatch:\n"
            f"  Expected: {expected_count}\n"
            f"  Actual:   {actual_count}\n"
            f"  Rentals:  {active_rentals}"
        )

    print(f"Rental count OK: {actual_count}")

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python script.py <expected_rental_count>")
        sys.exit(1)

    try:
        expected = int(sys.argv[1])
    except ValueError:
        print("Expected rental count must be an integer")
        sys.exit(1)

    main(expected)
