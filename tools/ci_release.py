#!/usr/bin/env python3
"""
ci_release.py — crea/actualiza una Release de GitHub y sube assets usando
solo la REST API (urllib), sin depender de `gh` en el runner self-hosted.

Uso:
    python3 tools/ci_release.py --repo OWNER/REPO --tag TAG --target SHA \
        --title "..." --notes-file notes.md [--prerelease] [--delete-old 15 \
        --prefix matcher-] ASSET...

Token: variable de entorno GH_TOKEN (PAT fine-grained con contents:write o
GITHUB_TOKEN del workflow).
"""
import argparse, json, mimetypes, os, sys, urllib.request, urllib.error, urllib.parse

API = "https://api.github.com"

def req(tok, url, method="GET", data=None, ctype="application/json", raw=False):
    hdr = {"Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json",
           "X-GitHub-Api-Version": "2022-11-28", "User-Agent": "mslug-ci"}
    body = None
    if data is not None:
        body = data if raw else json.dumps(data).encode()
        hdr["Content-Type"] = ctype
    r = urllib.request.Request(url, method=method, data=body, headers=hdr)
    try:
        with urllib.request.urlopen(r, timeout=300) as f:
            txt = f.read()
            return f.status, (json.loads(txt) if txt else {})
    except urllib.error.HTTPError as e:
        return e.code, json.loads(e.read() or b"{}")

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--repo", required=True); ap.add_argument("--tag", required=True)
    ap.add_argument("--target", required=True); ap.add_argument("--title", required=True)
    ap.add_argument("--notes-file", required=True); ap.add_argument("--prerelease", action="store_true")
    ap.add_argument("--delete-old", type=int, default=0, help="conservar N prereleases con --prefix")
    ap.add_argument("--prefix", default="matcher-")
    ap.add_argument("assets", nargs="*")
    a = ap.parse_args()
    tok = os.environ.get("GH_TOKEN") or os.environ.get("GITHUB_TOKEN")
    if not tok: sys.exit("GH_TOKEN no definido")
    notes = open(a.notes_file, encoding="utf-8").read()
    st, rel = req(tok, f"{API}/repos/{a.repo}/releases", "POST", {
        "tag_name": a.tag, "target_commitish": a.target, "name": a.title,
        "body": notes, "prerelease": a.prerelease, "draft": False})
    if st not in (200, 201):
        sys.exit(f"crear release: HTTP {st}: {rel}")
    up = rel["upload_url"].split("{")[0]
    for path in a.assets:
        if not os.path.isfile(path): continue
        name = os.path.basename(path)
        ctype = mimetypes.guess_type(name)[0] or "application/octet-stream"
        with open(path, "rb") as f: blob = f.read()
        st, r = req(tok, f"{up}?{urllib.parse.urlencode({'name': name})}", "POST", blob, ctype, raw=True)
        print(f"  asset {name}: HTTP {st}")
        if st not in (200, 201): sys.exit(f"subir asset {name}: {r}")
    print(f"release creada: {rel['html_url']}")
    if a.delete_old > 0:
        st, lst = req(tok, f"{API}/repos/{a.repo}/releases?per_page=100")
        if st == 200:
            old = sorted([x for x in lst if x["prerelease"] and x["tag_name"].startswith(a.prefix)],
                         key=lambda x: x["created_at"], reverse=True)[a.delete_old:]
            for x in old:
                req(tok, f"{API}/repos/{a.repo}/releases/{x['id']}", "DELETE")
                req(tok, f"{API}/repos/{a.repo}/git/refs/tags/{x['tag_name']}", "DELETE")
                print(f"  borrada prerelease antigua {x['tag_name']}")

if __name__ == "__main__":
    main()
