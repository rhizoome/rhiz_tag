import os
import subprocess
from contextlib import contextmanager
from datetime import datetime

import pytest
from hypothesis import given, settings
from hypothesis.strategies import datetimes
from rhiz_tag import to_datetag


@contextmanager
def chdir(target_dir):
    original_dir = os.getcwd()
    try:
        os.chdir(target_dir)
        yield
    finally:
        os.chdir(original_dir)


def cargo_build_successful():
    try:
        with chdir("../rust"):
            result = subprocess.run(
                ["cargo", "build", "--bin", "hypothesis"], check=True
            )
        return result.returncode == 0
    except subprocess.CalledProcessError:
        # Command failed
        return False


_proc = None


def proc_hypothesis():
    global _proc
    if _proc is None or _proc.poll() is not None:
        _proc = subprocess.Popen(
            "../rust/target/debug/hypothesis",
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
        )
    return _proc


@pytest.mark.skipif(
    not cargo_build_successful(), reason="cargo build --bin hypothesis failed"
)
@settings(max_examples=100)
@given(datetimes(min_value=datetime(2024, 1, 1), max_value=datetime(2075, 1, 1)))
def test_rust(date: datetime):
    ctag = to_datetag(date)
    proc = proc_hypothesis()
    str_date = date.strftime("%Y-%m-%d %H:%M:%S")
    proc.stdin.write(str_date.encode("UTF-8"))
    proc.stdin.write(b"\n")
    proc.stdin.flush()
    tag = proc.stdout.readline().decode("UTF-8").strip()
    assert tag == ctag
    assert proc.poll() is None
