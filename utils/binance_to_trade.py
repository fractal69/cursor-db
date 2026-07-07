#!/usr/bin/env python3

import csv
from decimal import Decimal, ROUND_HALF_UP

SCALE = Decimal("100000000")

INPUT = "./input/input.csv"
OUTPUT = "output.csv"

def normalize_time(value: str) -> int:
    t = int(value)

    while t > 9_999_999_999_999:  # > 13 digits
        t //= 1000

    return t

def scale(value: str) -> int:
    return int((Decimal(value) * SCALE).to_integral_value(rounding=ROUND_HALF_UP))


with open(INPUT, newline="") as fin, open(OUTPUT, "w", newline="") as fout:
    reader = csv.DictReader(fin)
    writer = csv.writer(fout)

    for row in reader:
        writer.writerow([
            int(row["id"]),
            normalize_time(row["time"]),
            scale(row["price"]),
            scale(row["qty"]),

            1 if row["is_buyer_maker"].lower() == "true" else 0,
        ])

print(f"Archivo generado: {OUTPUT}")