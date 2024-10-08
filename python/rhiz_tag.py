import random
from datetime import datetime, timedelta

base54 = "abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ34689"
base_time = datetime(2024, 1, 1, 0, 0, 0)
week_tick = 11631  # ceil(60*60*24*7/52)
rand_range = 53 * 53 * 53
a_week = timedelta(weeks=1)


def base_x(num: int, alphabet=base54):
    base = len(alphabet)

    if num == 0:
        return alphabet[0]

    baseX_result = []
    while num > 0:
        num, remainder = divmod(num, base)
        baseX_result.append(alphabet[remainder])

    return "".join(reversed(baseX_result))


def rev_x(base_str: str, alphabet=base54):
    base = len(alphabet)
    num = 0

    for char in base_str:
        num = num * base + alphabet.index(char)

    return num


def to_datetag(date: datetime) -> str:
    year = date.year - base_time.year
    month = date.month
    week = date.isocalendar()[1]
    # Handle underflows and overflows in ISO calendar
    if month == 1 and week > 51:
        week = 0
    if month == 12 and week < 2:
        week = (date - a_week).isocalendar()[1] + 1
    day = date.weekday()
    eve_est = date - timedelta(days=day)
    week_eve = datetime(eve_est.year, eve_est.month, eve_est.day)
    delta = date - week_eve
    tick = int(delta.total_seconds() / week_tick)
    return f"{base_x(year)}{base_x(week)}{base_x(tick)}"


def to_tag(date: datetime) -> str:
    tag = to_datetag(date)
    rand = random.randint(0, rand_range)
    rand_base = base_x(rand)
    return f"{tag}-{rand_base}"


def to_datetimes(tag: str) -> (datetime, datetime):
    year = 2024 + rev_x(tag[0])
    week = rev_x(tag[1])
    tick = rev_x(tag[2])

    # fromisocalendar does not correctly round-trip, thanks to our extension
    # of the ISO calendar to week 0 and [last week of the year] + 1, we
    # can avoid the round-trip bugs.
    if week == 0:
        monday = datetime.fromisocalendar(year, 1, 1) - a_week
    else:
        try:
            monday = datetime.fromisocalendar(year, week, 1)
        except ValueError:
            monday = datetime.fromisocalendar(year, week - 1, 1) + a_week
    date0 = monday + timedelta(seconds=week_tick * tick)
    date1 = date0 + timedelta(seconds=week_tick)
    return (date0, date1)


def to_datetime(tag: str) -> str:
    dt = to_datetimes(tag)
    week = rev_x(tag[1])
    tick = rev_x(tag[2])
    return f"{dt[0]} - {dt[1]} (Week: {week}, Tick: {tick})"


try:
    import click

    @click.group()
    def cli():
        """
        Generate a universal, general-purpose tag to use in your notes, Zettelkasten,
        field research, or archival projects (photos, drafts, scrapbooking). Use it as a
        date identifier for your note's header or as a reference. You can apply it
        across different media: Digital, typewritten, or handwritten. Reference a note
        in a book, a paper, or in your source code. Everything with the same tag is
        connected.

        For more details please see the [documentation](https://github.com/rhizoome/rhiz_tag).
        """
        pass

    @cli.command(help="Generate a tag")
    def tag():
        print(to_tag(datetime.now()))

    @cli.command(help="Find the date of a tag")
    @click.argument("tag", type=str)
    def date(tag):
        print(to_datetime(tag))

except ImportError:

    def cli():
        print("to get the cli do: pip install rhiz_tag[cli]")
