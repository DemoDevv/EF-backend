from tests.load_tests.helpers.data_helpers import (
    generate_random_email,
    generate_random_password,
)


class AuthenticationBehavior:
    def __init__(self, user):
        self.user = user
        self.headers = {"Content-Type": "application/json"}

    def register(self):
        self.credentials = {
            "email": generate_random_email(),
            "password": generate_random_password(),
        }

        response = self.user.client.post(
            "/api/v1/auth/register", json=self.credentials, headers=self.headers
        )
        if response.status_code == 200:
            token = response.json().get("access_token")
            if token:
                self.headers["Authorization"] = f"Bearer {token}"
            else:
                print(f"Error: No access token in response {response.text}")
        else:
            print(f"Failed to register: {response.status_code}")
