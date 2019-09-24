#shell
docker-compose run $1 2>&1 | tee ./log/$1.txt
cat ./log/$1.txt
