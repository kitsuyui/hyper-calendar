#!/usr/bin/env python3
"""Validate the workspace version and print publishable crates in dependency order."""
import json
import subprocess
import sys

if len(sys.argv) != 2 or not sys.argv[1].startswith('v'):
    sys.exit('usage: plan.py v<workspace-version>')
tag = sys.argv[1]
data = json.loads(subprocess.check_output(['cargo', 'metadata', '--no-deps', '--format-version', '1'], text=True))
members = set(data['workspace_members'])
packages = {p['name']: p for p in data['packages'] if p['id'] in members}
publishable = {n: p for n, p in packages.items() if p['publish'] is None or 'crates-io' in p['publish']}
if not publishable:
    sys.exit('no crates are configured for crates.io publication')
for name, package in publishable.items():
    if tag != 'v' + package['version']:
        sys.exit(f"{name}: tag {tag} does not match version {package['version']}")
ordered = []
remaining = set(publishable)
while remaining:
    ready = []
    for name in sorted(remaining):
        dependencies = {d['name'] for d in publishable[name]['dependencies'] if d.get('path')}
        unavailable = (dependencies & packages.keys()) - publishable.keys()
        if unavailable:
            sys.exit(f'{name}: non-publishable workspace dependencies: {sorted(unavailable)}')
        if not dependencies & remaining:
            ready.append(name)
    if not ready:
        sys.exit('workspace dependency cycle detected')
    ordered.extend(ready)
    remaining.difference_update(ready)
print('\n'.join(ordered))
