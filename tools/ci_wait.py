#!/usr/bin/env python3
"""ci_wait.py — espera a la ejecución del workflow `port` (runner local) de un
commit y descarga los assets de texto de su release (sólo REST, sin `gh`).

    GH_TOKEN=... python3 tools/ci_wait.py [--sha HEAD] [--out build/ci]
"""
import argparse, json, os, subprocess, sys, time, urllib.request

API = "https://api.github.com"
REPO = "correo415415/mslug-decompiled"


def get(tok, url, raw=False):
    r = urllib.request.Request(url, headers={
        "Authorization": f"Bearer {tok}", "User-Agent": "mslug-ci",
        "Accept": "application/octet-stream" if raw else "application/vnd.github+json"})
    with urllib.request.urlopen(r, timeout=120) as f:
        b = f.read()
    return b if raw else json.loads(b)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--sha", default="HEAD")
    ap.add_argument("--out", default="build/ci")
    ap.add_argument("--timeout", type=int, default=3600)
    a = ap.parse_args()
    tok = os.environ.get("GH_TOKEN") or sys.exit("GH_TOKEN no definido")
    sha = subprocess.check_output(["git", "rev-parse", a.sha], text=True).strip()
    t0, run = time.time(), None
    while time.time() - t0 < a.timeout:
        rs = [r for r in get(tok, f"{API}/repos/{REPO}/actions/runs?head_sha={sha}&event=push")
              ["workflow_runs"] if r["name"] == "port"]
        if rs and rs[0]["status"] == "completed":
            run = rs[0]
            break
        time.sleep(30)
    if not run:
        sys.exit("timeout")
    print("run", run["html_url"], run["conclusion"])
    rel = [r for r in get(tok, f"{API}/repos/{REPO}/releases?per_page=30")
           if r["tag_name"].startswith("port-") and r["tag_name"].endswith(sha[:7])]
    if not rel:
        sys.exit("sin release para " + sha[:7])
    print("release", rel[0]["html_url"])
    print(rel[0]["body"])
    os.makedirs(a.out, exist_ok=True)
    for x in rel[0]["assets"]:
        if x["name"].endswith((".txt", ".json")):
            open(os.path.join(a.out, x["name"]), "wb").write(get(tok, x["url"], raw=True))
    sys.exit(0 if run["conclusion"] == "success" else 1)


if __name__ == "__main__":
    main()
