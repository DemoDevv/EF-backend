source ./scripts/env.sh

load_env

diesel --database-url $DATABASE_TEST_URL migration run
