#!/usr/bin/env python3
"""Run local compatibility, correctness and benchmark checks; never publish."""
import argparse
import os
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--no-benchmarks', action='store_true',
                        help='Run the Noir/Rust/consumer suite only; omit circuit and witness benchmarks')
    args = parser.parse_args()
    nargo = os.environ.get('NARGO', 'nargo')
    environment = dict(os.environ)
    environment.setdefault('HEGEL_SEED', '42')
    # Check the pinned compiler and downstream installation before the main suite.
    commands = [
        [sys.executable, 'scripts/test-consumer.py'],
        [nargo, 'check', '--workspace', '--deny-warnings'],
        [nargo, 'fmt', '--check'],
        [nargo, 'test', '--workspace', '--deny-warnings'],
        ['cargo', 'fmt', '--check'],
        ['cargo', 'test', '--locked'],
        [nargo, 'execute', '--package', 'ratio_example'],
    ]
    if not args.no_benchmarks:
        commands.append([sys.executable, 'scripts/benchmark.py'])
        commands.append([sys.executable, 'scripts/benchmark-witness.py'])
        commands.append([sys.executable, 'scripts/benchmark-witness.py', '--width', '256'])
    for command in commands:
        print('+ ' + ' '.join(command), flush=True)
        subprocess.run(command, cwd=ROOT, env=environment, check=True)


if __name__ == '__main__':
    main()
