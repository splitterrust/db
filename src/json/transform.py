#!/usr/bin/env python3
import json
import os

FILENAME = os.path.join(os.path.abspath(os.getcwd()), 'spells.json')
# Get spells.json from https://raw.githubusercontent.com/cvaliente/Splittermond/master/Zauberbuch/zauber.json
FILENAME_OUT = os.path.join(os.path.abspath(os.getcwd()), 'spells_new.json')


spells = []

with open(FILENAME) as f:
    data = json.load(f)
    for k, v in data.items():
        v['Name'] = v['name']
        del v['name']
        schulen = {}
        for s, l in v['schulen'].items():
            schulen[s] = int(l) if l else 1
        del v['schulen']
        v['Schulen'] = schulen

        spells.append(v)

with open(FILENAME_OUT, 'w+') as f:
    f.write(json.dumps(spells, indent=4))
