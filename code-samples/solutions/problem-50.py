import csv
import io

def parse_csv_line(line):
    reader = csv.reader(io.StringIO(line))
    return next(reader)
