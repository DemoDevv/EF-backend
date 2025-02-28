from locust import HttpUser, between, task
from scenarios.authentification import AuthenticationBehavior
from scenarios.exceeded_rate_limit import ExceededRateLimiteBehaviour
from scenarios.users import UsersBehavior


class ApiUser(HttpUser):
    wait_time = between(1, 3)

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.auth = AuthenticationBehavior(self)
        self.users = UsersBehavior(self)
        self.exceeded_rate_limiter = ExceededRateLimiteBehaviour()

    def on_start(self):
        self.auth.register()

    @task(2)
    def test_user_profile(self):
        self.users.get_profile()
