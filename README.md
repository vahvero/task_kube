# CPU frying task_kube solution

Pointless auto-scaling kubernetes cluster which may be used increase processor temperatures for cooking eggs in a pinch.

## Contents

Central server with task delegation is implemented in task_server.rs. This is called by N executors implemented in consumer.rs.

## Dependencies

Required dependencies

- [Kind](https://kind.sigs.k8s.io/docs/user/quick-start/#installation)
- [Docker](https://docs.docker.com/engine/install/)
- [Kubectl](https://kubernetes.io/docs/tasks/tools/install-kubectl-linux/)

[Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html),
[Angular](https://angular.dev/installation) and node/nvm/npm are useful, but not stricly necessary to run this solution.

## Building

Run

`bash re-create-cluster.sh`

This will build and create kubernetes cluster. After
building, application is available at `localhost:30000`.
Remove created cluster with

`bash delete-cluster.sh`

Create large set of tasks by

`cargo run --bin create_tasks`

Now calculation should be starting on the server.

## Development

Run file watching development server with `docker compose up --watch` which deploys to `localhost:5000`. It runs single consumer. Active changes may cause some tasks to hang due to consumer re-build. Reset database after build from the UI if this happens.

## Authors

- [vahvero](https://github.com/vahvero)