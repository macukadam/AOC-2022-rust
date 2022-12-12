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
    answer = sys.argv[2]
    uri = f"https://adventofcode.com/2022/day/{day}/answer"

    response = requests.post(uri, data={'level': day, 'answer': answer},
                             cookies={'session': SESSIONID})

    soup = BeautifulSoup(response.text, features="html.parser")

    print(soup.find('p').text)
