#include <stdint.h>
#include <stddef.h>

struct TableEntry {
    uint32_t year_eve;
    uint32_t week_eve : 31;
    unsigned int week : 1;
};

struct TableEntryCorrected {
    int64_t year_eve;
    int64_t week_eve;
    char week;
};

// Include this in a single C file and wrap `to_datetag()` in a function. The goal
// is to make it easy for distribution (for me) without any bells and whistles. I'm
// not a C developer, so this is kept simple for portability and use in embedded
// systems. `stdint.h` and `stddef.h` can easily be removed to ensure there are no
// external dependencies.

// This program assumes the system is always in UTC. When converting a date to a
// Unix timestamp, treat everything as UTC and disregard timezones. If necessary,
// the table can be regenerated using `table.py`.

const char BASE54[] = "abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ34689";
const int BASE54_LEN = 54; 

const int64_t DAY_SECONDS = 60 * 60 * 24;
const int64_t YEAR_SECONDS = DAY_SECONDS * 366 + 2;
const int64_t WEEK_SECONDS = DAY_SECONDS * 7;
const int64_t TIME_BASE = 1704067200; // 2024-01-01 00:00:00 UTC
const int UNIX_YEAR = 1970;
const int BASE_YEAR = 2024;
const int WEEK_TICK = (WEEK_SECONDS / 52) + 1; // Equivalent to ceil(WEEK_SECONDS / 52)

const int TABLE_SIZE = 52;

static struct TableEntry entries[TABLE_SIZE] = {
    {0, 0, 1},
    {31622400, 172800, 1},
    {63158400, 259200, 1},
    {94694400, 345600, 0},
    {126230400, 432000, 0},
    {157852800, 0, 1},
    {189388800, 86400, 1},
    {220924800, 172800, 1},
    {252460800, 259200, 1},
    {284083200, 432000, 0},
    {315619200, 518400, 0},
    {347155200, 0, 1},
    {378691200, 86400, 1},
    {410313600, 259200, 1},
    {441849600, 345600, 0},
    {473385600, 432000, 0},
    {504921600, 518400, 0},
    {536544000, 86400, 1},
    {568080000, 172800, 1},
    {599616000, 259200, 1},
    {631152000, 345600, 0},
    {662774400, 518400, 0},
    {694310400, 0, 1},
    {725846400, 86400, 1},
    {757382400, 172800, 1},
    {789004800, 345600, 0},
    {820540800, 432000, 0},
    {852076800, 518400, 0},
    {883612800, 0, 1},
    {915235200, 172800, 1},
    {946771200, 259200, 1},
    {978307200, 345600, 0},
    {1009843200, 432000, 0},
    {1041465600, 0, 1},
    {1073001600, 86400, 1},
    {1104537600, 172800, 1},
    {1136073600, 259200, 1},
    {1167696000, 432000, 0},
    {1199232000, 518400, 0},
    {1230768000, 0, 1},
    {1262304000, 86400, 1},
    {1293926400, 259200, 1},
    {1325462400, 345600, 0},
    {1356998400, 432000, 0},
    {1388534400, 518400, 0},
    {1420156800, 86400, 1},
    {1451692800, 172800, 1},
    {1483228800, 259200, 1},
    {1514764800, 345600, 0},
    {1546387200, 518400, 0},
    {1577923200, 0, 1},
    {1609459200, 86400, 1},
};

static int base_x(char *buf, size_t buf_size, int num, const char *alphabet, const char base) {
    if (base <= 1 || buf_size < 2) {
        return 1;
    }

    size_t buf_idx = 0;
    do {
        if (buf_idx >= buf_size - 1) {
            return 1;
        }
        int remainder = num % base;
        num /= base;
        buf[buf_idx++] = alphabet[remainder];
    } while (num > 0);

    buf[buf_idx] = '\0';

    return 0;
}

static inline void correct(struct TableEntry* entry, struct TableEntryCorrected* corrected) {
    corrected->week = entry->week;
    corrected->year_eve = TIME_BASE + ((int64_t) entry->year_eve);
    corrected->week_eve = corrected->year_eve - ((int64_t) entry->week_eve);

}

// Generate a Ubiquitous General Purpose Tag to use in your notes, zettelkasten,
// field research or archival projects (photos, drafts, scrapbooking). Use it as a
// date-identifier for your note's header or as a reference. You can use it between
// media: Digital, typewritten, or handwritten. Reference a note in a book, in a
// paper, or in your source- code. Everything with the same tag is connected.
// 
// The tag contains a date, based on base54 and 52nds: The first letter is the year,
// beginning from 2024, the second letter is the week of the year using ISO weeks.
// The third letter is a 52nd of a week. It's primarily meant to give the tags an
// order, but if you use the tags daily, you learn to read them.
// 
// - Example: aQu-TWr (a: year 2024, Q: week 39, u: tick 19)
//   - 2024-09-25 13:23:09 - 2024-09-25 16:37:00
// 
// For more details please see the documentation.

// buf must be a `char[4]`, with buf_size < 4 the function never returns 0.
// returns 0 on sucess and 1 on error.
static inline int to_datetag(char* buf, size_t buf_size, const int64_t datetime) {
    const int year_est = UNIX_YEAR + datetime / YEAR_SECONDS; // Should never overshoot the actual year_est
    int index = year_est - BASE_YEAR;
    int end = TABLE_SIZE - 1;
    struct TableEntryCorrected entry0;
    struct TableEntryCorrected entry1;
    while (index < end) {
        correct(&entries[index], &entry0);
        correct(&entries[index + 1], &entry1);
        if (datetime >= entry0.year_eve && datetime < entry1.year_eve) {
            break;
        }  
        index += 1;
    }
    if (index == 51) {
        correct(&entries[index], &entry0);
        if (!(datetime >= entry0.year_eve && datetime < (entry0.year_eve + YEAR_SECONDS))) {
            return 1;
        }
    } else {
        if (!(datetime >= entry0.year_eve && datetime < entry1.year_eve)) {
            return 1;
        }
    }
    const int64_t year_seconds = datetime - entry0.week_eve;
    const int week = year_seconds / WEEK_SECONDS;
    const int tick = (year_seconds % WEEK_SECONDS) / WEEK_TICK;
    if (base_x(buf, buf_size, index, BASE54, BASE54_LEN)) {
      return 1;
    }
    if (base_x((buf + 1), buf_size, entry0.week + week, BASE54, BASE54_LEN)) {
      return 1;
    }
    if (base_x((buf + 2), buf_size, tick, BASE54, BASE54_LEN)) {
      return 1;
    }
    return 0;
}
