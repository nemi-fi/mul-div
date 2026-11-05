#!/usr/bin/env python3
"""Verify a checkout dependency from a consumer outside the library workspace."""
import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile

from common import ROOT, require_noir, run


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--canary', action='store_true',
                        help='Probe the installed compiler without claiming Noir 1.0 compatibility')
    args = parser.parse_args()
    nargo = os.environ.get('NARGO', 'nargo')
    version = run([nargo, '--version'])
    print(version, flush=True)
    if not args.canary:
        require_noir(version)
    with tempfile.TemporaryDirectory(prefix='mul-div-consumer-') as directory:
        consumer = Path(directory) / 'consumer'
        shutil.copytree(ROOT / 'tests/integration/consumer', consumer,
                        ignore=shutil.ignore_patterns('target'))
        manifest = consumer / 'Nargo.toml'
        source = manifest.read_text()
        # JSON string quoting also produces valid TOML strings for absolute paths.
        source = source.replace('"../../../library"', json.dumps(str(ROOT / 'library')))
        manifest.write_text(source)
        for command in [
            ['check', '--deny-warnings'], ['fmt', '--check'],
            ['test', '--deny-warnings'], ['execute'],
        ]:
            subprocess.run([nargo, *command], cwd=consumer, check=True)
    print('Isolated checkout consumer passed', flush=True)


if __name__ == '__main__':
    main()
