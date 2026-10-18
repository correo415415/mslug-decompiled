# uso: docs.py "<changelog en>" "<todo es>" "<progreso es>" ; lee /tmp/census.txt
# Herramienta de sesión: actualiza COVERAGE/CHANGELOG/TODO/HANDOFF/PROGRESO tras
# un paso del censo. Antes: python3 assets/tools/data_census.py > /tmp/census.txt
import re,sys
c=open('/tmp/census.txt').read().rstrip('\n')
tot=c.split('\n')[-1].split()[-1]; tes=tot.replace('.',',').replace('%',' %')
f='docs/COVERAGE.md'; s=open(f).read()
s=re.sub(r"```\nmódulo .*?\nTOTAL[^\n]*\n```", "```\n"+c+"\n```", s, count=1, flags=re.S); open(f,'w').write(s)
ch,todo,prog=sys.argv[1:4]
f='CHANGELOG.md'; s=open(f).read(); s=s.replace("### Added\n","### Added\n- "+ch.replace('{T}',tot.replace('%',' %'))+"\n",1); open(f,'w').write(s)
f='TODO.md'; s=open(f).read()
m=re.search(r'- \[ \] Datos "otro" \(([0-9,]+ %)\)',s)
s=s[:m.start()]+'- [x] '+todo+'\n- [ ] Datos "otro" ('+tes+')'+s[m.end():]; open(f,'w').write(s)
f='HANDOFF.md'; s=open(f).read()
s=re.sub(r'\(([0-9,]+ %) de los datos','('+tes+' de los datos',s,count=1); open(f,'w').write(s)
f='docs/PROGRESO.md'; s=open(f).read()
import re as _re
i=_re.search(r'^> \*\*[^\n]*\(20\d\d-\d\d-\d\d\)\*\*', s, _re.M).start()
s=s[:i]+'> **'+prog.replace('{T}',tes)+'\n\n'+s[i:]; open(f,'w').write(s)
print(tot)
