import random
import string


def generate_random_email():
    name = "".join(random.choices(string.ascii_lowercase, k=8))
    return f"{name}@example.com"


def generate_random_password():
    return "".join(random.choices(string.ascii_letters + string.digits, k=12))
