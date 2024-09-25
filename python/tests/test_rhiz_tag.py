from datetime import datetime

from hypothesis import given, settings
from hypothesis.strategies import datetimes

from rhiz_tag import to_datetimes, to_tag


@settings(max_examples=10000)
@given(datetimes(min_value=datetime(2024, 1, 1), max_value=datetime(2075, 1, 1)))
def test_rhiz_tag(date):
    tag = to_tag(date)
    date0, date1 = to_datetimes(tag)
    try:
        assert date0 <= date
        assert date <= date1
    except Exception:
        breakpoint()
