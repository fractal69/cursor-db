import csv
import time
import requests
from decimal import Decimal
from datetime import datetime
from dateutil.parser import isoparse

# ============================================================
# Configuración
# ============================================================

SYMBOL = "BTC-USD"
START_DATE = "2026-06-01T00:00:00Z"

LIMIT = 1000
BASE_URL = "https://indexer.dydx.trade/v4/trades/perpetualMarket"

SCALE = Decimal("100000000")

# ============================================================

def scale(value):
    return int(Decimal(str(value)) * SCALE)


def iso_to_millis(value: str) -> int:
    return int(isoparse(value).timestamp() * 1000)


def parse_date(value: str) -> datetime:
    return isoparse(value)


#{
#  "id": "05e2e0c20000000200000002",
#  "side": "BUY",
#  "size": "0.0014",
#  "price": "64944",
#  "type": "LIMIT",
#  "createdAt": "2026-07-23T20:06:41.378Z",
#  "createdAtHeight": "98754754"
#}
def get_trades(symbol: str, cursor: str | None):

    params = {
        "limit": LIMIT
    }

    if cursor:
        params["createdBeforeOrAt"] = cursor

    response = requests.get(
        f"{BASE_URL}/{symbol}",
        params=params,
        timeout=30
    )

    response.raise_for_status()

    return response.json().get("trades", [])


def download_history(symbol: str, start_date: datetime):

    cursor = None
    rows = []

    while True:

        trades = get_trades(symbol, cursor)

        if not trades:
            break

        stop = False

        for trade in trades:

            created = parse_date(trade["createdAt"])

            if created < start_date:
                stop = True
                break

            rows.append(trade)

        print(
            f"Ticks: {len(rows):,} | "
            f"Cursor: {trades[-1]['createdAt']}"
        )

        if stop:
            break

        cursor = trades[-1]["createdAt"]

        time.sleep(0.2)

    return rows


def save_csv(filename: str, rows: list):

    rows.sort(key=lambda x: x["createdAt"])

    seen = set()

    with open(filename, "w", newline="") as f:

        writer = csv.writer(f)

        writer.writerow([
            "id",
            "time",
            "price",
            "qty",
            "is_buyer_maker"
        ])

        for row in rows:

            if row["id"] in seen:
                continue

            seen.add(row["id"])

            writer.writerow([
                int(row["id"], 16),
                iso_to_millis(row["createdAt"]),
                scale(row["price"]),
                scale(row["size"]),
                
                1 if row["side"] == "SELL" else 0
            ])

    return len(seen)


def main():

    start_date = parse_date(START_DATE)

    print(f"Descargando {SYMBOL}...")

    rows = download_history(SYMBOL, start_date)

    filename = f"{SYMBOL}_{START_DATE[:10]}.csv"

    save_csv(filename, rows)

    print("Descarga finalizada")


if __name__ == "__main__":
    main()