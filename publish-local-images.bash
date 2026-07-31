#!/bin/sh
set -o errexit

docker build -f docker/task_server.release.Dockerfile . --tag task_server:v1.0
echo "Build task server"
kind load docker-image task_server:v1.0 --name task-kube

docker build -f docker/task_viewer.release.Dockerfile . --tag task_viewer:v1.0
echo "Build task viewer"
kind load docker-image task_viewer:v1.0 --name task-kube

docker build -f docker/consumer.release.Dockerfile . --tag task_consumer:v1.0
echo "Build consumer"
kind load docker-image task_consumer:v1.0 --name task-kube
