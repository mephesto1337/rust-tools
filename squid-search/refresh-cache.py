import sys
import datetime
import collections
import typing

SquidAccessEntry = collections.namedtuple('SquidAccessEntry', ('time', 'method', 'url'))


def parse_squid_access_entry(line: str) -> SquidAccessEntry:
    """
    Parse a squid access line
    """
    line = line.strip().lstrip('\x00')
    parts = [p for p in line.split(' ') if p]
    time = datetime.datetime.fromtimestamp(float(parts[0]))
    method = parts[5]
    url = parts[6]
    return SquidAccessEntry(time, method, url)


def parse_squid_access_file(content: str) -> typing.List[SquidAccessEntry]:
    """
    Parse a all squid access file
    """
    last_day = datetime.datetime.now() - datetime.timedelta(1)

    entries = []
    for line in content.splitlines():
        entry = parse_squid_access_entry(line)
        if entry.time < last_day:
            continue
        entries.append(entry)
    return entries


def main():
    patterns = sys.argv[1:]
    with open('/var/log/squid/access.log') as f:
        content = f.read()

    entries = parse_squid_access_file(content)

    for entry in entries:
        if entry.method != 'GET':
            continue
        if any(map(lambda p: p in entry.url, patterns)):
            print(entry.url)


if __name__ == '__main__':
    main()
