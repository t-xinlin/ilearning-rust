#!/bin/bash

set +x

function init_env() {
	export tmp_build_dir="/tmp/docker_build_new"
	rm -rf "${tmp_build_dir}" && mkdir -p "${tmp_build_dir}"
	cp -r Dockerfile start.sh supervisord.conf CentOS-7-reg.repo "${tmp_build_dir}"
	cp -r actix-web-example "${tmp_build_dir}"
	cp -r actix-web-example.db "${tmp_build_dir}"
	mkdir -p "${tmp_build_dir}/conf"
	cp -r conf/* "${tmp_build_dir}/conf"
	mkdir -p "${tmp_build_dir}/scripts"
	cp -r scripts/* "${tmp_build_dir}/scripts/"
}

function build_image() {
    init_env
	export IMAGE_REPO="mytest"
	export IMAGE_TAG="1.0.0"
	echo "docker image start"
	echo "tmp_build_dir: ${tmp_build_dir}/"
	ls -l "${tmp_build_dir}"
    pushd "${tmp_build_dir}" > /dev/null || { echo "push ${tmp_build_dir} failed" && return 1;}
    docker build --no-cache -t "${IMAGE_REPO}:${IMAGE_TAG}" -f Dockerfile . || { echo "build image failed" && return 1;}
    popd > /dev/null || { echo "pop ${tmp_build_dir} failed" && return 1;}
}

function main() {
    build_image
}

main
