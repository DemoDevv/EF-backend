class UsersBehavior:
    def __init__(self, user):
        self.user = user

    def get_profile(self):
        return self.user.client.get(
            "/api/v1/users/profile", headers=self.user.auth.headers
        )
