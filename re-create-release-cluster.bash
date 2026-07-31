kind delete cluster --name task-kube  

kind create cluster --name task-kube --config charts/kind-config.yaml 

bash publish-local-images.bash 

kubectl apply -f charts/manifest.yaml

kubectl apply -f https://github.com/kubernetes-sigs/metrics-server/releases/latest/download/components.yaml

kubectl patch deployment metrics-server -n kube-system   --type='json'   -p='[
    {
      "op": "add",
      "path": "/spec/template/spec/containers/0/args/-",
      "value": "--kubelet-insecure-tls"
    }
  ]'
