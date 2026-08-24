#!/usr/bin/env python3
"""Tek iş parçacıklı WASM galeri paketini yerel web konağına hazırlar."""

from __future__ import annotations

import hashlib
import json
import shutil
import subprocess
import sys
from pathlib import Path


KOK = Path(__file__).resolve().parent.parent
SANDIK = "gpui-bilesenleri-galeri-wasm"
WASM = KOK / "target" / "wasm32-unknown-unknown" / "release" / "gpui_bilesenleri_galeri_wasm.wasm"
CIKTI = KOK / "crates" / SANDIK / "web" / "pkg"


def calistir(*komut: str) -> None:
    subprocess.run(komut, cwd=KOK, check=True)


def main() -> int:
    arac = shutil.which("wasm-bindgen")
    if arac is None:
        print("HATA: wasm-bindgen CLI bulunamadı (beklenen sürüm: 0.2.127)")
        return 1
    surum = subprocess.run(
        (arac, "--version"), check=True, text=True, stdout=subprocess.PIPE
    ).stdout
    if "0.2.127" not in surum:
        print(f"HATA: {surum.strip()}; beklenen wasm-bindgen 0.2.127")
        return 1

    calistir(
        "cargo",
        "build",
        "--locked",
        "--release",
        "--target",
        "wasm32-unknown-unknown",
        "-p",
        SANDIK,
    )
    CIKTI.mkdir(parents=True, exist_ok=True)
    calistir(
        arac,
        "--target",
        "web",
        "--out-dir",
        str(CIKTI),
        "--out-name",
        "gpui_bilesenleri_galeri_wasm",
        str(WASM),
    )
    web_wasm = CIKTI / "gpui_bilesenleri_galeri_wasm_bg.wasm"
    build = {
        "sema_surumu": 1,
        "wasm_sha256": hashlib.sha256(web_wasm.read_bytes()).hexdigest(),
    }
    (CIKTI / "build.json").write_text(
        json.dumps(build, ensure_ascii=False, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )
    print(f"WASM galeri hazır: {CIKTI.relative_to(KOK)}")
    print("Sunucu: python3 tools/wasm_galeri_sunucu.py --port 8000")
    print(
        "Açık sayfa kendini yeniler: sunucu önbelleksiz sunar, sayfa paket "
        "sürümünü izler."
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
