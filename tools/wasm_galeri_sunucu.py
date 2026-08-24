#!/usr/bin/env python3
"""WASM galerisini önbelleksiz sunan yerel geliştirme sunucusu.

`python3 -m http.server` yanıtlarına yalnız `Last-Modified` koyar. Tarayıcı
`Cache-Control` görmediği belgeyi sezgisel olarak önbelleğe alabilir; o zaman
yeniden derlenen paket açık sekmede görünmez. Bu, iki tarafın aynı adrese
bakıp farklı sürüm çalıştırmasına yol açar — yerel geliştirmede fark edilmesi
en zor hata sınıfı budur.

Bu sunucu her yanıta `Cache-Control: no-store` koyar. Yayımlanan bir yüzey
değildir; yalnız `localhost` üzerinde geliştirme içindir.
"""

from __future__ import annotations

import argparse
import functools
import http.server
import os
from pathlib import Path

KOK = Path(__file__).resolve().parent.parent
WEB = KOK / "crates" / "gpui-bilesenleri-galeri-wasm" / "web"


class ÖnbelleksizVekil(http.server.SimpleHTTPRequestHandler):
    """Her yanıtı taze tutar; `wasm` MIME türünü de doğru bildirir."""

    def end_headers(self) -> None:
        self.send_header("Cache-Control", "no-store, must-revalidate")
        self.send_header("Pragma", "no-cache")
        self.send_header("Expires", "0")
        super().end_headers()

    def log_message(self, biçim: str, *argümanlar: object) -> None:
        # Varsayılan günlük her istek için satır basar; geliştirme akışında
        # gürültüden başka bir şey üretmiyor.
        del biçim, argümanlar


ÖnbelleksizVekil.extensions_map[".wasm"] = "application/wasm"


def main() -> int:
    ayrıştırıcı = argparse.ArgumentParser(description=__doc__)
    # Önizleme paneli portu `PORT` ile atar; port çakışırsa kendi seçtiğini
    # verir. Bu yüzden port komut satırında sabitlenmez, ortamdan okunur.
    ayrıştırıcı.add_argument(
        "--port", type=int, default=int(os.environ.get("PORT", "8000"))
    )
    ayrıştırıcı.add_argument("--directory", default=str(WEB))
    seçenekler = ayrıştırıcı.parse_args()

    # `ThreadingHTTPServer` şart: sayfa paket sürümünü yoklarken bağlantı açık
    # tutuyor; tek iş parçacıklı sunucu ilk kalıcı bağlantıda kilitlenir ve
    # sessizce yanıt vermez olur.
    kur = functools.partial(ÖnbelleksizVekil, directory=seçenekler.directory)
    http.server.ThreadingHTTPServer.allow_reuse_address = True
    with http.server.ThreadingHTTPServer(("127.0.0.1", seçenekler.port), kur) as sunucu:
        print(f"WASM galeri sunucusu: http://localhost:{seçenekler.port}/ (önbelleksiz)")
        sunucu.serve_forever()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
