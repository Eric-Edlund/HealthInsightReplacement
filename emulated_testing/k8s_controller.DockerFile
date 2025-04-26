FROM ubuntu:24.04

RUN apt update
RUN apt install -y kubernetes
# RUN kubeadm init

# RUN snap install kubectl --classic
# RUN apt install kubernetes
# RUN kubeadm init
