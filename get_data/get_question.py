#!/usr/bin/python3
import requests
import sys
from get_session_id import get_session_id
from bs4 import BeautifulSoup

if __name__ == "__main__":
    SESSIONID = get_session_id()
    if not SESSIONID:
        raise Exception("SESSIONID is missing from environment")
    day = sys.argv[1]
    uri = f"https://adventofcode.com/2022/day/{day}"

    response = requests.post(uri,
                             cookies={'session': SESSIONID})

    soup = BeautifulSoup(response.text, features="html.parser")

    print("Example Input:")
    print(soup.find('code').text)
    print()

    for ptag in soup.find_all('p')[:-3]:
        print(ptag.text)

