#!/usr/bin/python3
import requests
import sys
from get_session_id import get_session_id

if __name__ == "__main__":
    SESSIONID = get_session_id()
    if not SESSIONID:
        raise Exception("SESSIONID is missing from environment")
    day = sys.argv[1]
    uri = f"https://adventofcode.com/2022/day/{day}/input"
    response = requests.get(uri, cookies={'session': SESSIONID})
    open(f"{day}_input.txt", "wb").write(response.content)
