# How to use

Create cluster with `kind`

```bash
 kind create cluster --name task-kube
```

Apply required changes with `kubectl`

```bash
kubectl apply -f charts/manifest.yaml
```
