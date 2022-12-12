import dotenv
import os


def get_session_id():
    dotenv.load_dotenv()
    return os.environ.get("SESSIONID")
