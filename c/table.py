import calendar
from datetime import datetime, timedelta

base_time = datetime(2024, 1, 1, 0, 0, 0)
unix_base_time = calendar.timegm(base_time.timetuple())
a_week = timedelta(weeks=1)


def main():
    for i in range(52):
        date = datetime(2024 + i, 1, 1)
        print(f"{table_entry(date)}".replace("(", "{").replace(")", "},"))


def table_entry(date: datetime) -> str:
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
    unix_year_eve = calendar.timegm(date.timetuple()) - unix_base_time
    unix_week_eve = unix_year_eve - (
        calendar.timegm(week_eve.timetuple()) - unix_base_time
    )
    return int(unix_year_eve), int(unix_week_eve), week


if __name__ == "__main__":
    main()
