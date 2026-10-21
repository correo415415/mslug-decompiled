"""API mínima de GitHub con el token de ~/.git-credentials."""
import json, urllib.request
TOKEN = open('/home/user/.git-credentials').read().split(':')[2].split('@')[0]
REPO = 'correo415415/mslug-decompiled'
def api(path, data=None, method=None):
    url = path if path.startswith('http') else f'https://api.github.com/repos/{REPO}{path}'
    req = urllib.request.Request(url, data=json.dumps(data).encode() if data is not None else None,
        method=method or ('POST' if data is not None else 'GET'),
        headers={'Authorization': f'token {TOKEN}', 'Accept': 'application/vnd.github+json'})
    with urllib.request.urlopen(req) as r:
        b = r.read()
        return json.loads(b) if b else {}
