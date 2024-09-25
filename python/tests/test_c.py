import calendar
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


def gcc_build_successful():
    try:
        with chdir("../c"):
            result = subprocess.run(
                [
                    "gcc",
                    "-g",
                    "-Wall",
                    "-Wextra",
                    "-Wpedantic",
                    "-Werror",
                    "-o",
                    "hypothesis",
                    "hypothesis.c",
                ],
                check=True,
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
            "../c/hypothesis",
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
        )
    return _proc


@pytest.mark.skipif(not gcc_build_successful(), reason="gcc build failed")
@settings(max_examples=1000000)
@given(
    datetimes(
        min_value=datetime(2024, 1, 1),
        max_value=datetime(2075, 12, 31, 23, 59, 59),
    )
)
def test_c(date: datetime):
    ctag = to_datetag(date)
    proc = proc_hypothesis()
    unix_timestamp = str(int(calendar.timegm(date.timetuple())))
    proc.stdin.write(unix_timestamp.encode("UTF-8"))
    proc.stdin.write(b"\n")
    proc.stdin.flush()
    tag = proc.stdout.readline().decode("UTF-8").strip()
    try:
        assert tag == ctag
        assert proc.poll() is None
    except Exception:
        breakpoint()
