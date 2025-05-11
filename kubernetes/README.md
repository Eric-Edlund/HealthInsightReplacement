Minikube cluster of 3:

```bash
minikube start --nodes=3
```


Kind 

https://kind.sigs.k8s.io/docs/user/quick-start#configuring-your-kind-cluster

```bash
go install sigs.k8s.io/kind@v0.27.0 && kind create cluster

kind create cluster --config kind-cluster-config-1.yaml

kind export logs ./kind-cluster-logs-export
```

# Test Setup 1

```
kind create cluster --config kind-cluster-config-1.yaml

kubectl config use-config kind-cluster-config-1

kubectl apply -f config1/

kind delete cluster --name 'test-cluster-1'
```

Helm - Automates the automation

It appears that helm has pretty immature packages and I'd rather avoid it.

```
helm repo add bitnami https://charts.bitnami.com/bitnami
helm repo update

helm install kafka-cluster oci://registry-1.docker.io/bitnamicharts/kafka -f config1/kafka-values.yaml

helm uninstall kafka-cluster
```


Creating a local kafka image

- The official apache kafka image requires that the node id be statically specified
- StatefulSets don't support this
- I created an image derived from official kafka which checks the ordinal pod id
  to derive the node id.

```bash
# Set up local registry
docker run -d -p 5000:5000 --restart=always --name registry registry:2
# Build custom image +
# Assign a tag to the image
# this tag is for the immutable binary result image.
#
# Here we put the registry host as the first component of the
# container name for when we push it. There is a weird coupling
# going on between the tag and registry.
docker build -f ./Dockerfile.kafka . --tag localhost:5000/k8s-kafka:latest

# We could have tagged an image instead
# docker tag imagename:tag localhost:5000/imagename:tag

# Push it to the local repository
docker push localhost:5000/k8s-kafka:latest

# Confirm the upload
curl http://localhost:5000/v2/_catalog

# Alas, the kind cluster is on an isolate network and so it cannot reach the local
# container registry.

# Option 1:
Join the local repository container to the kind cluster network
```

