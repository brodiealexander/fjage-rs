# docker build . -t fjage-rs:testcontainer && docker run -p 5081:5081 --rm -it fjage-rs:testcontainer
FROM alpine:latest

# Dependencies
RUN mkdir fjage
RUN apk add curl openjdk8-jre bash

# Quickstart
WORKDIR /fjage

RUN curl -O https://raw.githubusercontent.com/org-arl/fjage/master/src/sphinx/fjage_quickstart.sh

RUN bash fjage_quickstart.sh

# To see error upon failure
COPY dummy.groovy /fjage/dummy.groovy
ENTRYPOINT /fjage/fjage.sh || true && cat /fjage/logs/log-0.txt