import secrets
from pathlib import Path


def main() -> None:
    root = Path(__file__).resolve().parents[1]
    env_path = root / ".env"
    key = secrets.token_urlsafe(48)

    lines = []
    if env_path.exists():
        lines = env_path.read_text(encoding="utf-8").splitlines()

    updated = False
    out = []
    for line in lines:
        if line.startswith("MASTER_API_2026="):
            out.append(f"MASTER_API_2026={key}")
            updated = True
        else:
            out.append(line)

    if not updated:
        out.append(f"MASTER_API_2026={key}")

    env_path.write_text("\n".join(out).rstrip() + "\n", encoding="utf-8")
    print("Wrote MASTER_API_2026 to .env")
    print(f"Key prefix: {key[:8]}... (length={len(key)})")


if __name__ == "__main__":
    main()
