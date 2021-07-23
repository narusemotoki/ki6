DOCKER-COMMAND=podman

.PHONY: buildrun
buildrun:
	${DOCKER-COMMAND} build --tag ki6 .
	${DOCKER-COMMAND} run --shm-size=1g --userns=keep-id -e DISPLAY=:0 --net=host --rm --interactive --tty --volume="${PWD}/..:/home/ubuntu/ki6" --workdir="/home/ubuntu/ki6" ki6
