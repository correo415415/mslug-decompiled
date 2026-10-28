# CI del port: `port.yml` corregido

Copia de referencia de `.github/workflows/port.yml` (ya instalado en `main`,
`port` y `port_next`). Si se cambia uno, hay que cambiar los dos.

## Por qué no se podía lanzar

GitHub sólo muestra **Run workflow** (workflow_dispatch) para workflows que
existen en la rama por defecto (`main`). `port.yml` sólo existía en `port`, así
que no había forma de activarlo a mano.

## Qué cambia

* Sigue siendo **sólo manual** (no hay `push` ni `pull_request`).
* `workflow_dispatch`: Actions → port → Run workflow → elegir **cualquier rama**.
* `repository_dispatch` (tipo `port`): lanzarlo por API para cualquier rama con
  `POST /repos/correo415415/mslug-decompiled/dispatches`
  `{"event_type":"port","client_payload":{"ref":"port_next"}}`.
  El checkout, la etiqueta de la release y el commit destino usan esa rama.

## Instalación (hecha; repetir sólo si cambia)

1. Copiar `tools/ci/port.yml` a `.github/workflows/port.yml` en **main**.
2. Copiar el mismo fichero en la rama `port` (y `port_next`).
