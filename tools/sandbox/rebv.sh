#!/bin/bash
# Para la verificación en curso y lanza otra con el código actual.
pkill -f "^/bin/bash /tmp/bv.sh"; pkill -f "^/bin/bash /tmp/verify"; pkill -f "^/tmp/mslug_"
rm -rf /tmp/v /tmp/v3 /tmp/mslug_v*
setsid -f /tmp/bv.sh > /dev/null 2>&1 < /dev/null
