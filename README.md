# Docker-RL

Small program to check the current rate limit cap on Docker Hub, because they
can't be bothered to do it themselves.


**NOTE:** Each call will use one of the limit requests.

See [This blog](https://www.docker.com/blog/checking-your-current-docker-pull-rate-limits-and-status/)
for more information.

# Install

This needs the local operating systems ssl dev library that is needed by `reqwest`.
For Ubuntu this is `libssl-dev`.

```sh
git clone git@github.com:dorrella/docker-rl.git
cd docker-rl
sudo apt-get update
sudo apt-get install -yf libssl-dev
make install
```

# Use

## Anonymous Limit

```sh
$ docker-rl
97/100
```

## For User

```sh
$ docker-rl -u dorrella
Password for dorrella:
96/100
```

```sh
$ docker-rl -u dorrella -p 'some pass'
95/100
```
