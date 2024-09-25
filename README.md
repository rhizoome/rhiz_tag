# Rhiz-Tag

Generate a universal, general-purpose tag to use in your notes, Zettelkasten,
field research, or archival projects (photos, drafts, scrapbooking). Use it as a
date identifier for your note's header or as a reference. You can apply it
across different media: Digital, typewritten, or handwritten. Reference a note in
a book, a paper, or in your source code. Everything with the same tag is
connected.

The Rhiz-Tag encodes the date using a base54 system and is based on "52nds." It
consists of three characters:

1. **First Character (Year)**: Represents the year, starting from 2024, and can
  span 54 years before wrapping around.
2. **Second Character (Week)**: Represents the ISO week of the year. While there
  are typically 52 weeks, it can extend up to 54 weeks in special cases like leap
  years or calendar anomalies.
3. **Third Character (Tick within Week)**: Divides each week into 52 equal
  parts, allowing for precise identification within the week.

While the tag primarily provides an order to your entries, using it daily will
help you get used to reading and interpreting them.

- **Example**: `aQu` (`a`: year 2024, `Q`: week 39, `u`: tick 19)
  - Corresponds to: 2024-09-25 13:23:09 to 2024-09-25 16:37:00
- **Full example: `aQx-Yjk`**: the second part are 3 more random characters in
  base54.

For more details, please refer to the documentation.

## Purpose

This tagging system aims to facilitate better organization, retrieval, and
interconnection of information across various domains. By providing a compact
and consistent way to represent time, it helps users create a cohesive network
of content that is easy to navigate and reference. This is particularly valuable
in disciplines that require meticulous record-keeping and cross-referencing,
such as research, writing, and personal knowledge management.
