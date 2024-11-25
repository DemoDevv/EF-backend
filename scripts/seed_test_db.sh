source ./scripts/env.sh

load_env

psql -d echofetch_tests -f ./scripts/drop_db.sql
psql -d echofetch_tests -f ./scripts/seeders/seed.sql
